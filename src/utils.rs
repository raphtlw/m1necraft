use std::{
    fs,
    path::{Path, PathBuf},
};

use unzpack::Unzpack;

use crate::{strings, GlobalPaths, Result};

pub async fn download_artifact<'a>(name: &'a str) -> Result<PathBuf> {
    log::debug!("Downloading artifact from GitHub: {}", name);
    let file_bytes = reqwest::get(format!("{}/{}", strings::ARTIFACT_URL, name))
        .await?
        .bytes()
        .await?;

    let paths = GlobalPaths::get();
    let file_path = paths.app_data_dir.join(name);
    log::debug!("Saved artifact to: {:?}", &file_path);
    fs::write(&file_path, file_bytes)?;

    Ok(file_path)
}

pub fn extract_zip_file<P, S>(filepath: P, outpath: P, inner_dir: S) -> Result<()>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let filepath = filepath.as_ref();
    let outpath = outpath.as_ref();
    let inner_dir_full = outpath.join(inner_dir.as_ref());
    if inner_dir_full.exists() {
        log::debug!("extract_zip_file + inner_dir: {:#?}", &inner_dir_full);
        fs::remove_dir_all(inner_dir_full)?;
        log::debug!("Removed");
    }

    Unzpack::extract(filepath, outpath)?;

    // handle __MACOSX folders
    let macosx_dir_full = outpath.join(strings::ZIP_MACOSX_METADATA_FOLDER);
    if macosx_dir_full.exists() {
        log::debug!(
            "Found {} in extracted zip",
            strings::ZIP_MACOSX_METADATA_FOLDER
        );
        fs::remove_dir_all(macosx_dir_full)?;
        log::debug!("Removed");
    }

    Ok(())
}
