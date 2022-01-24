use std::{fs, io::BufReader, os::unix::prelude::PermissionsExt, process, thread, time::Duration};

use api::{
    mcl::check_minecraft_launcher_paths,
    strings,
    utils::{download_artifact, extract_zip_file},
    GlobalPaths, Result,
};
use chrono::Utc;
use clap::{Parser, Subcommand};
use colored::Colorize;
use fern::colors::{Color, ColoredLevelConfig};
use fs_extra::{dir, file};
use indicatif::ProgressBar;
use serde_json::{json, Value};
use unzpack::Unzpack;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    /// Enables verbose logging.
    #[clap(short, long)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Installs Minecraft and the ARM64 JDK into the official Minecraft Launcher
    Install {
        /// Specify the version to install.
        #[clap(short, long, required = true)]
        version: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::White)
        .trace(Color::BrightBlack);
    let colors_level = colors_line.clone().info(Color::Green);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            if args.verbose {
                out.finish(format_args!(
                    "{color_line}[{date}][{target}][{level}{color_line}] {message}\x1B[0m",
                    color_line = format_args!(
                        "\x1B[{}m",
                        colors_line.get_color(&record.level()).to_fg_str()
                    ),
                    date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    target = record.target(),
                    level = colors_level.color(record.level()),
                    message = message,
                ));
            } else {
                if record.target() == "cli" {
                    out.finish(format_args!(
                        "{color_line}{message}{color_line}",
                        color_line = format_args!(
                            "\x1B[{}m",
                            colors_line.get_color(&record.level()).to_fg_str()
                        ),
                        message = message,
                    ));
                }
            }
        })
        .level({
            if args.verbose {
                log::LevelFilter::Debug
            } else {
                log::LevelFilter::Info
            }
        })
        .chain(std::io::stdout())
        .apply()
        .expect("Failed to initialize logger");

    let mut paths = api::GlobalPaths::init().expect("Failed to initialize GlobalPaths");

    match &args.command {
        Commands::Install { version } => {
            log::debug!("START PATCHING MINECRAFT LAUNCHER");

            // Check if all the Minecraft Launcher paths exist
            if !check_minecraft_launcher_paths() {
                log::error!("Error locating Minecraft installation.");
                log::error!("Either Minecraft launcher isn't installed or Minecraft has not been launched before.");
                process::exit(1);
            }

            let bar = ProgressBar::new_spinner().with_message("Working on it...");

            /// Download required artifacts
            async fn download_all_artifacts() -> Result<()> {
                let mcl_profiles = download_artifact("mcl_profiles.zip");
                let lwjgl = download_artifact("lwjgl.zip");
                let checksums = download_artifact("checksums.txt");

                let artifacts_files_downloaded = tokio::try_join!(mcl_profiles, lwjgl, checksums);
                let paths = GlobalPaths::get();

                let (mcl_profiles_path, lwjgl_path, _) = artifacts_files_downloaded?;
                extract_zip_file(&mcl_profiles_path, &paths.app_data_dir, "mcl_profiles")?;
                extract_zip_file(&lwjgl_path, &paths.app_data_dir, "lwjgl")?;

                Ok(())
            }

            // Compare checksums of remote and local first
            if paths.res_checksums.exists() {
                let checksums_local = fs::read_to_string(&paths.res_checksums).unwrap();
                let checksums_remote =
                    reqwest::get(format!("{}/checksums.txt", strings::ARTIFACT_URL))
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                log::debug!("checksums_local: {}", checksums_local);
                log::debug!("checksums_remote: {}", checksums_remote);
                if checksums_local != checksums_remote {
                    bar.set_message("Downloading resources from GitHub...");
                    download_all_artifacts().await.unwrap();
                }
            } else {
                bar.set_message("Downloading resources from GitHub...");
                download_all_artifacts().await.unwrap();
            }

            // Download Azul 8 JRE
            if paths.res_jre.is_none() {
                bar.set_message("Downloading Java");
                let java_bundle_zip_bytes = reqwest::get(strings::JAVA_8_URL)
                    .await
                    .unwrap()
                    .bytes()
                    .await
                    .unwrap();
                let java_bundle_file_path = paths.app_data_dir.join(strings::JAVA_8_ZIP);
                fs::write(&java_bundle_file_path, java_bundle_zip_bytes).unwrap();
                Unzpack::extract(&java_bundle_file_path, &paths.app_data_dir).unwrap();
                paths.set_optional().unwrap();
                bar.set_message("Extracting Java");
                // set perms
                for file in
                    fs::read_dir(paths.res_jre.as_ref().unwrap().join("Contents/Home/bin")).unwrap()
                {
                    if let Ok(file) = file {
                        let mut perms = file.metadata().unwrap().permissions();
                        perms.set_mode(0o777);
                        fs::set_permissions(&file.path(), perms)
                            .expect("Failed to set file permissions");
                    }
                }
            } else {
                log::info!("JRE zip already exists, skipping download");
            }
            bar.set_message("Done");

            // Add files to Minecraft
            bar.set_message("Adding files to Minecraft");

            // lwjglnatives
            if !paths.mcl_lwjglnatives.exists() {
                fs_extra::dir::copy(
                    &paths.res_lwjglnatives,
                    &paths.mcl_lwjglnatives.parent().unwrap(),
                    &dir::CopyOptions::new(),
                )
                .unwrap();
            } else {
                log::info!("minecraft/lwjglnatives already exists, not adding");
            }

            // lwjglfat.jar
            if !paths.mcl_lwjglfat_jar.exists() {
                fs_extra::file::copy(
                    &paths.res_lwjglfat_jar,
                    &paths.mcl_lwjglfat_jar,
                    &file::CopyOptions::new(),
                )
                .unwrap();
            } else {
                log::info!("minecraft/libraries/lwjglfat.jar already exists, not adding");
            }

            // versions/1.16.5-arm
            let profile_name = format!("{}-arm", version);
            if !paths.mcl_versions_dir.join(&profile_name).exists() {
                fs_extra::dir::copy(
                    paths.res_mcl_profiles.join(&profile_name),
                    paths.mcl_versions_dir,
                    &dir::CopyOptions::new(),
                )
                .expect("Invalid version specified");
            } else {
                log::info!(
                    "minecraft/versions/{} already exists, not adding",
                    &profile_name
                )
            }

            // jre
            if !paths.mcl_jre.exists() {
                fs_extra::dir::copy(
                    paths.res_jre.as_ref().unwrap(),
                    paths.mcl_runtime_dir,
                    &dir::CopyOptions::new(),
                )
                .unwrap();
            } else {
                log::info!("ARM JRE already exists, not adding");
            }

            bar.set_message("Done");

            // add launcher profile
            bar.set_message("Adding launcher profile");
            match version.as_str() {
                "1.16.5" => {
                    let file = fs::File::open(&paths.mcl_launcher_profiles).unwrap();
                    let reader = BufReader::new(file);
                    // Using Value bc the type will always morph
                    let launcher_profiles: Value = serde_json::from_reader(reader).unwrap();
                    let launcher_profile_key =
                        format!("{}{}", strings::LAUNCHER_PROFILE_KEY_PREFIX, version);
                    if launcher_profiles["profiles"][&launcher_profile_key].is_null() {
                        log::info!("Launcher profile does not exist, creating...");
                        let custom_launcher_profile = json!({
                            "created": Utc::now().to_rfc3339(),
                            "icon": "Grass",
                            "javaDir": paths.mcl_jre.join("Contents/Home/bin/java"),
                            "lastVersionId": profile_name,
                            "name": "M1necraft",
                            "type": "custom"
                        });
                        let mut modified_launcher_profiles = launcher_profiles.clone();
                        modified_launcher_profiles["profiles"]
                            .as_object_mut()
                            .unwrap()
                            .insert(launcher_profile_key, custom_launcher_profile);
                        log::debug!(
                            "Modified launcher_profiles.json: {:#?}",
                            &modified_launcher_profiles
                        );
                        fs::write(
                            &paths.mcl_launcher_profiles,
                            serde_json::to_string_pretty(&modified_launcher_profiles).unwrap(),
                        )
                        .expect("Failed to write new launcher_profiles.json file");
                    }
                }
                _ => {
                    log::error!("ERROR: Invalid version specified {}", version);
                    process::exit(1);
                }
            };

            bar.set_message("Done");

            thread::sleep(Duration::from_secs(1)); // no idea why this is needed but fixes permission issues

            // final step - open minecraft launcher, let user know we are done
            let output = process::Command::new("open")
                .args(&["-a", "Minecraft"])
                .output()
                .expect("Failed to start Minecraft launcher");
            log::debug!(
                "{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            log::info!(
                "{}",
                r#"
🚀 Successfully added new Minecraft profile! Just hit "PLAY" to start the game."#
                    .green()
                    .bold()
            );
        }
    }
}
