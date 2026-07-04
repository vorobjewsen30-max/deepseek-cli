pub(crate) use deepseek_skills::install_system_skills;
pub(crate) use deepseek_skills::system_cache_root_dir;

use deepseek_utils_absolute_path::AbsolutePathBuf;

pub(crate) fn uninstall_system_skills(deepseek_home: &AbsolutePathBuf) {
    let _ = std::fs::remove_dir_all(system_cache_root_dir(deepseek_home));
}
