use std::fs::{File, OpenOptions};
use std::io::{self, Write};
#[cfg(feature = "certification-test-authority")]
use std::io::{Seek, SeekFrom};
#[cfg(feature = "certification-test-authority")]
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(feature = "certification-test-authority")]
use fs4::FileExt;

use super::StoreDurabilityRequirement;
use crate::{
    reach_storage_boundary, ProductionStorageBoundaryControl, ProductionStorageBoundarySeam,
    StorageBoundaryRegion,
};

#[derive(Debug)]
pub(crate) struct StoreDurabilityTarget {
    file: File,
    staged_path: Option<PathBuf>,
    published_path: Option<PathBuf>,
    parent_directory: Option<File>,
    persisted_offset: u64,
    bytes_written: u64,
}

impl StoreDurabilityTarget {
    #[cfg(feature = "certification-test-authority")]
    pub(crate) fn append(
        root: &Path,
        relative_path: &Path,
        requirement: StoreDurabilityRequirement,
        encoded_frame: &[u8],
        observed_file_bytes: u64,
        valid_prefix_bytes: u64,
    ) -> io::Result<Self> {
        let path = root.join(relative_path);
        let directory = path.parent().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "append target needs a parent")
        })?;
        std::fs::create_dir_all(directory)?;
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(&path)?;
        file.lock_exclusive()?;
        let current_bytes = file.metadata()?.len();
        if current_bytes != observed_file_bytes {
            return Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "WAL segment changed after append planning",
            ));
        }
        if valid_prefix_bytes > current_bytes {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "WAL valid prefix exceeds the physical segment length",
            ));
        }
        if valid_prefix_bytes < current_bytes {
            file.set_len(valid_prefix_bytes)?;
        }
        file.seek(SeekFrom::Start(valid_prefix_bytes))?;
        file.write_all(encoded_frame)?;
        let parent_directory = requirement
            .requires_directory_sync()
            .then(|| open_directory(directory))
            .transpose()?;
        Ok(Self {
            file,
            staged_path: Some(path),
            published_path: None,
            parent_directory,
            persisted_offset: valid_prefix_bytes,
            bytes_written: encoded_frame.len() as u64,
        })
    }

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
            persisted_offset: 0,
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

    pub(crate) const fn persisted_offset(&self) -> u64 {
        self.persisted_offset
    }

    pub(crate) fn reach_boundary(
        &mut self,
        control: &impl ProductionStorageBoundaryControl,
        seam: ProductionStorageBoundarySeam,
    ) -> io::Result<()> {
        reach_storage_boundary(
            control,
            seam,
            &mut self.file,
            StorageBoundaryRegion::new(self.persisted_offset, self.bytes_written),
        )
    }

    #[cfg(windows)]
    pub(crate) fn sync_parent_namespace(&self, _rename_completed: bool) -> io::Result<()> {
        self.parent_directory
            .as_ref()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "directory namespace handle required",
                )
            })?
            .sync_all()
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
