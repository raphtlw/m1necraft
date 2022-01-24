use crate::GlobalPaths;

pub fn check_minecraft_launcher_paths() -> bool {
    let paths = GlobalPaths::get();

    paths.mcl_dir.exists()
        && paths.mcl_launcher_profiles.exists()
        && paths.mcl_versions_dir.exists()
}
