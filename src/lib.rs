pub mod config;

use once_cell::sync::OnceCell;
use std::{fs, path::PathBuf};

const APP_BUNDLE_ID: &str = "raphtlw.apps.M1necraft";
static MC_LIBS_PATH: OnceCell<PathBuf> = OnceCell::new();
static APP_DATA_DIR: OnceCell<PathBuf> = OnceCell::new();
static CONFIG_PATH: OnceCell<PathBuf> = OnceCell::new();

/// Initializes all the paths that are stored in the OnceCell's above.
pub fn init_paths() {
    APP_DATA_DIR
        .set(dirs::data_local_dir().unwrap().join(APP_BUNDLE_ID))
        .unwrap();
    if !APP_DATA_DIR.get().unwrap().exists() {
        log::debug!("App data directory does not exist, creating...");
        fs::create_dir(APP_DATA_DIR.get().unwrap().clone())
            .expect("Failed to create app data directory");
    }
    log::debug!("APP_DATA_DIR: {:#?}", APP_DATA_DIR.get().unwrap());

    CONFIG_PATH
        .set(APP_DATA_DIR.get().unwrap().clone().join("config.json"))
        .unwrap();
    log::debug!("CONFIG_PATH: {:#?}", CONFIG_PATH.get().unwrap());

    MC_LIBS_PATH
        .set(PathBuf::from(
            APP_DATA_DIR.get().unwrap().clone().join("mc_libs"),
        ))
        .unwrap();
    log::debug!("MC_LIBS_PATH: {:#?}", MC_LIBS_PATH.get().unwrap());
}
