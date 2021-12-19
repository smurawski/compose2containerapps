use std::env;
use std::path::{Path, PathBuf};

pub fn find_command<T>(command: T) -> Option<PathBuf>
where
    T: AsRef<Path>,
{
    // If the command path is absolute and a file exists, then use that.
    if command.as_ref().is_absolute() && command.as_ref().is_file() {
        return Some(command.as_ref().to_path_buf());
    }
    // Find the command by checking each entry in `PATH`. If we still can't find it, give up and
    // return `None`.
    match env::var_os("PATH") {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let candidate = PathBuf::from(&path).join(command.as_ref());
                if candidate.is_file() {
                    return Some(candidate);
                } else if let Some(result) = find_command_with_pathext(&candidate) {
                    return Some(result);
                }
            }
            None
        }
        None => None,
    }
}

fn find_command_with_pathext(candidate: &Path) -> Option<PathBuf> {
    if candidate.extension().is_none() {
        if let Some(pathexts) = env::var_os("PATHEXT") {
            for pathext in env::split_paths(&pathexts) {
                let mut source_candidate = candidate.to_path_buf();
                let extension = pathext.to_str().unwrap().trim_matches('.');
                source_candidate.set_extension(extension);
                let current_candidate = source_candidate.to_path_buf();
                if current_candidate.is_file() {
                    return Some(current_candidate);
                }
            }
        };
    }
    None
}
