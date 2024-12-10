use std::collections::VecDeque;
use std::env;
use std::ffi::OsStr;
use std::fs::{self, File, Metadata};
use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;
use uuid::Uuid;

#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

#[cfg(windows)]
use std::os::windows::fs::{MetadataExt, OpenOptionsExt};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

pub struct FileUtil {
    temp_files: VecDeque<PathBuf>,
}

impl FileUtil {
    pub fn new() -> Self {
        Self {
            temp_files: VecDeque::new(),
        }
    }

    /* TODO: need to implement
    pub fn conf_path() -> Result<PathBuf, Error> {
        if cfg!(target_os = "macos") {
            Ok(Self::framework_resources_path("ee.ria.digidocpp")?)
        } else if cfg!(target_os = "windows") {
            env::var("APPDATA").map(PathBuf::from)
        } else {
            env::var("HOME").map(|home| PathBuf::from(home).join(".digidocpp"))
        }
    }
    */

    #[cfg(target_os = "macos")]
    fn framework_resources_path(name: &str) -> Result<PathBuf, Error> {
        unimplemented!("Framework resources not implemented")
    }

    pub fn encode_name(file_name: &str) -> PathBuf {
        PathBuf::from(file_name)
    }

    pub fn file_exists(path: &str) -> bool {
        Path::new(path).is_file()
    }

    pub fn modified_time(path: &str) -> Result<SystemTime, Error> {
        fs::metadata(path).and_then(|meta| meta.modified())
    }

    pub fn update_modified_time(path: &str, time: SystemTime) -> Result<(), Error> {
        #[cfg(unix)]
        let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o644));
        filetime::set_file_mtime(path, filetime::FileTime::from_system_time(time))
    }

    pub fn file_extension(path: &str, extensions: &[&str]) -> bool {
        if let Some(ext) = Path::new(path).extension().and_then(OsStr::to_str) {
            extensions.iter().any(|&e| e.eq_ignore_ascii_case(ext))
        } else {
            false
        }
    }

    pub fn file_size(path: &str) -> Result<u64, Error> {
        fs::metadata(path).map(|meta| meta.len())
    }

    pub fn file_name(path: &str) -> Option<String> {
        Path::new(path)
            .file_name()
            .and_then(OsStr::to_str)
            .map(|s| s.to_string())
    }

    pub fn directory(path: &str) -> Option<String> {
        Path::new(path)
            .parent()
            .and_then(|p| p.to_str())
            .map(|s| s.to_string())
    }

    pub fn path(dir: &str, relative_path: &str) -> PathBuf {
        let mut base = PathBuf::from(dir);
        base.push(relative_path);
        base
    }

    pub fn temp_file_name(&mut self) -> Result<PathBuf, Error> {
        let temp_dir = env::temp_dir();
        let file_name = format!("tempfile_{}", Uuid::new_v4());
        let temp_path = temp_dir.join(file_name);
        self.temp_files.push_back(temp_path.clone());
        Ok(temp_path)
    }

    pub fn create_directory(path: &str) -> Result<(), Error> {
        fs::create_dir_all(path)
    }

    pub fn delete_temp_files(&mut self) -> Result<(), Error> {
        while let Some(temp) = self.temp_files.pop_front() {
            if temp.is_file() {
                fs::remove_file(&temp)?;
            } else if temp.is_dir() {
                fs::remove_dir_all(&temp)?;
            }
        }
        Ok(())
    }

    pub fn to_uri_path(path: &str) -> String {
        Url::from_file_path(path).unwrap().to_string()
    }

    pub fn from_uri_path(path: &str) -> Result<String, Error> {
        Url::parse(path)
            .map_err(|_| Error::new(std::io::ErrorKind::InvalidInput, "Invalid URI"))
            .and_then(|url| {
                url.to_file_path()
                    .map_err(|_| Error::new(std::io::ErrorKind::InvalidInput, "Invalid URI Path"))
            })
            .map(|p| p.to_string_lossy().to_string())
    }

    pub fn hex_to_bin(hex: &str) -> Vec<u8> {
        (0..hex.len())
            .step_by(2)
            .filter_map(|i| u8::from_str_radix(&hex[i..i + 2], 16).ok())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

   // TODO: tests
}
