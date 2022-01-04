use clap::{Parser, Subcommand};
use fern::colors::{Color, ColoredLevelConfig};

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
    /// Installs Minecraft into the official Minecraft Launcher
    /// to run Minecraft natively on Apple Silicon.
    Install {
        /// Specify the version to install.
        #[clap(short, long)]
        version: String,
    },
}

fn main() {
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
                    out.finish(format_args!("{}", message));
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

    api::init_paths();

    match &args.command {
        Commands::Install { version } => {
            log::debug!("START PATCHING MINECRAFT");
        }
    }
}
