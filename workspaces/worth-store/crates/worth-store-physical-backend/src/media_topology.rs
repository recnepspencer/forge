use std::io;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilesystemFailureDomainIdentity {
    UnixDevice(u64),
    WindowsVolume(u64),
}

pub fn observe_filesystem_failure_domain(
    path: &Path,
) -> Result<FilesystemFailureDomainIdentity, io::Error> {
    let existing = path
        .ancestors()
        .find(|candidate| candidate.exists())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "physical media path has no observable existing ancestor",
            )
        })?;
    match file_id::get_file_id(existing)? {
        file_id::FileId::Inode { device_id, .. } => {
            Ok(FilesystemFailureDomainIdentity::UnixDevice(device_id))
        }
        file_id::FileId::LowRes {
            volume_serial_number,
            ..
        } => Ok(FilesystemFailureDomainIdentity::WindowsVolume(u64::from(
            volume_serial_number,
        ))),
        file_id::FileId::HighRes {
            volume_serial_number,
            ..
        } => Ok(FilesystemFailureDomainIdentity::WindowsVolume(
            volume_serial_number,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sibling_paths_observe_one_filesystem_failure_domain() {
        let root = tempfile::tempdir().expect("temporary filesystem root");
        let left = root.path().join("left");
        let right = root.path().join("right");
        std::fs::create_dir_all(&left).expect("left directory");
        std::fs::create_dir_all(&right).expect("right directory");

        assert_eq!(
            observe_filesystem_failure_domain(&left).expect("left filesystem"),
            observe_filesystem_failure_domain(&right).expect("right filesystem")
        );
    }
}
