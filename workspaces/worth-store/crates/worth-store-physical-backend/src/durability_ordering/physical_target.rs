use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::StoreDurabilityRequirement;

#[derive(Debug)]
pub(crate) struct StoreDurabilityTarget {
    file: File,
    staged_path: Option<PathBuf>,
    published_path: Option<PathBuf>,
    parent_directory: Option<File>,
    bytes_written: u64,
}

impl StoreDurabilityTarget {
    pub(crate) fn persist(
        root: &std::path::Path,
        requirement: StoreDurabilityRequirement,
        payload: &[u8],
    ) -> io::Result<Self> {
        static NEXT_TARGET: AtomicU64 = AtomicU64::new(1);
        let id = NEXT_TARGET.fetch_add(1, Ordering::Relaxed);
        let directory = root.join(format!(
            "worth-store-durability-{}-{id}",
            std::process::id()
        ));
        std::fs::create_dir_all(&directory)?;
        let staged_path = directory.join("staged");
        let published_path = directory.join("published");
        let mut file = OpenOptions::new()
            .create_new(true)
            .read(true)
            .write(true)
            .open(&staged_path)?;
        file.write_all(payload)?;
        let parent_directory = (requirement.requires_rename_durable()
            || requirement.requires_directory_sync())
        .then(|| open_directory(&directory))
        .transpose()?;
        Ok(Self {
            file,
            staged_path: Some(staged_path),
            published_path: requirement
                .requires_rename_durable()
                .then_some(published_path),
            parent_directory,
            bytes_written: payload.len() as u64,
        })
    }

    pub(crate) fn sync_data(&self) -> io::Result<()> {
        self.file.sync_data()
    }
    pub(crate) fn sync_all(&self) -> io::Result<()> {
        self.file.sync_all()
    }

    pub(crate) fn rename_publication(&self) -> io::Result<()> {
        let staged = self.staged_path.as_ref().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "atomic publication needs staged path",
            )
        })?;
        let published = self.published_path.as_ref().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "atomic publication needs final path",
            )
        })?;
        std::fs::rename(staged, published)
    }

    pub(crate) fn persisted_path(&self, rename_completed: bool) -> &std::path::Path {
        if rename_completed {
            self.published_path
                .as_deref()
                .expect("completed rename has a published target")
        } else {
            self.staged_path
                .as_deref()
                .expect("non-renamed durability has a staged target")
        }
    }

    pub(crate) const fn bytes_written(&self) -> u64 {
        self.bytes_written
    }

    #[cfg(windows)]
    pub(crate) fn sync_parent_namespace(&self, rename_completed: bool) -> io::Result<()> {
        if rename_completed {
            self.parent_directory
                .as_ref()
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "directory namespace handle required",
                    )
                })?
                .sync_all()
        } else {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "Windows namespace durability requires an atomic rename",
            ))
        }
    }

    #[cfg(not(windows))]
    pub(crate) fn sync_parent_namespace(&self, _rename_completed: bool) -> io::Result<()> {
        self.parent_directory
            .as_ref()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "directory sync target required",
                )
            })?
            .sync_all()
    }
}

#[cfg(windows)]
fn open_directory(path: &std::path::Path) -> io::Result<File> {
    use std::os::windows::fs::OpenOptionsExt;
    OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(0x0200_0000)
        .open(path)
}

#[cfg(not(windows))]
fn open_directory(path: &std::path::Path) -> io::Result<File> {
    File::open(path)
}
