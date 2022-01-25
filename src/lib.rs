pub mod config;
pub mod mcl;
pub mod utils;

use glob::glob;
use once_cell::sync::OnceCell;
use std::path::PathBuf;

pub mod strings {
    pub const APP_BUNDLE_ID: &str = "raphtlw.apps.M1necraft";
    pub const ARTIFACT_URL: &str =
        "https://github.com/raphtlw/m1necraft/releases/download/resources";
    pub const JAVA_8_URL: &str =
        "https://cdn.azul.com/zulu/bin/zulu8.58.0.13-ca-jre8.0.312-macosx_aarch64.zip";
    pub const CHECKSUMS_TXT: &str = "checksums.txt";
    pub const LWJGLFAT_JAR: &str = "lwjglfat.jar";
    pub const LWJGL_NATIVES: &str = "lwjglnatives";
    pub const MCL_PROFILES: &str = "mcl_profiles";
    pub const JAVA_8_ZIP: &str = "jre8.zip";
    pub const LAUNCHER_PROFILE_KEY_PREFIX: &str = "m1necraft-";
    pub const ZIP_MACOSX_METADATA_FOLDER: &str = "__MACOSX";
}

pub mod values {
    pub const SUPPORTED_VERSIONS: &[&str] = &["1.16.5"];
}

// pub static APP_DATA_DIR: OnceCell<PathBuf> = OnceCell::new();
// pub static CONFIG_PATH: OnceCell<PathBuf> = OnceCell::new();
// pub static MINECRAFT_DATA_PATH: OnceCell<PathBuf> = OnceCell::new();
pub static PATHS: OnceCell<GlobalPaths> = OnceCell::new();

/// Only paths need to be initialized with OnceCell for platform independence.
/// ALL paths should be immutable.
#[derive(Debug, Clone)]
pub struct GlobalPaths {
    pub app_data_dir: PathBuf,
    pub config_file: PathBuf,

    pub mcl_dir: PathBuf,
    pub mcl_launcher_profiles: PathBuf,
    pub mcl_versions_dir: PathBuf,
    pub mcl_lwjglfat_jar: PathBuf,
    pub mcl_lwjglnatives: PathBuf,
    pub mcl_jre: PathBuf,
    pub mcl_runtime_dir: PathBuf,

    pub res_lwjgl: PathBuf,
    pub res_lwjglnatives: PathBuf,
    pub res_lwjglfat_jar: PathBuf,
    pub res_mcl_profiles: PathBuf,
    pub res_checksums: PathBuf,
    pub res_jre: Option<PathBuf>,
}

impl GlobalPaths {
    pub fn get() -> &'static Self {
        PATHS.get().clone().expect("PATHS is not initialized")
    }

    pub fn init() -> Result<Self> {
        let app_data_dir = dirs::data_local_dir().unwrap().join(strings::APP_BUNDLE_ID);
        let mcl_dir = dirs::data_local_dir().unwrap().join("minecraft");
        let res_lwjgl = app_data_dir.join("lwjgl");

        let mut paths = GlobalPaths {
            app_data_dir: app_data_dir.clone(),
            config_file: app_data_dir.join("config.json"),

            mcl_dir: mcl_dir.clone(),
            mcl_launcher_profiles: mcl_dir.join("launcher_profiles.json"),
            mcl_versions_dir: mcl_dir.join("versions"),
            mcl_lwjglfat_jar: mcl_dir.join("libraries").join(strings::LWJGLFAT_JAR),
            mcl_lwjglnatives: mcl_dir.join(strings::LWJGL_NATIVES),
            mcl_jre: mcl_dir.join("runtime/zulu-8.jre"),
            mcl_runtime_dir: mcl_dir.join("runtime"),

            res_lwjgl: res_lwjgl.clone(),
            res_lwjglnatives: res_lwjgl.join(strings::LWJGL_NATIVES),
            res_lwjglfat_jar: res_lwjgl.join(strings::LWJGLFAT_JAR),
            res_mcl_profiles: app_data_dir.join(strings::MCL_PROFILES),
            res_checksums: app_data_dir.join(strings::CHECKSUMS_TXT),
            res_jre: None,
        };

        paths.set_optional().unwrap();
        PATHS.set(paths.clone()).unwrap();

        Ok(paths)
    }

    pub fn set_optional(&mut self) -> Result<()> {
        self.res_jre = if let Some(path) = glob(&format!(
            "{}/zulu8*_aarch64/zulu-8.jre",
            self.app_data_dir.to_string_lossy()
        ))?
        .next()
        {
            path.ok()
        } else {
            None
        };

        Ok(())
    }
}

/// Initializes all the paths that are stored in the OnceCell's above.
// pub fn init_paths() {
//     APP_DATA_DIR
//         .set(dirs::data_local_dir().unwrap().join(APP_BUNDLE_ID))
//         .unwrap();
//     if !APP_DATA_DIR.get().unwrap().exists() {
//         log::debug!("App data directory does not exist, creating...");
//         fs::create_dir(APP_DATA_DIR.get().unwrap().clone())
//             .expect("Failed to create app data directory");
//     }
//     log::debug!("APP_DATA_DIR: {:#?}", APP_DATA_DIR.get().unwrap());

//     CONFIG_PATH
//         .set(APP_DATA_DIR.get().unwrap().clone().join("config.json"))
//         .unwrap();
//     log::debug!("CONFIG_PATH: {:#?}", CONFIG_PATH.get().unwrap());

//     MINECRAFT_DATA_PATH
//         .set(dirs::data_dir().unwrap().join("minecraft"))
//         .unwrap();
//     log::debug!(
//         "MINECRAFT_DATA_PATH: {:#?}",
//         MINECRAFT_DATA_PATH.get().unwrap()
//     );
// }

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
