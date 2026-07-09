use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use fs4::available_space;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HeavyFixtureBackendProfile {
    StoreOwnedLocalDisk,
    NonCanonicalChaosCorpus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeavyFixtureMaterializationDirectory {
    path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeavyFixtureDiskPreflightReceipt {
    directory: HeavyFixtureMaterializationDirectory,
    required_bytes: u64,
    available_bytes: u64,
    backend_profile: HeavyFixtureBackendProfile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeavyFixtureTempFileMaterialization {
    path: PathBuf,
    bytes_written: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeavyFixtureCleanupReceipt {
    path: PathBuf,
    completed: bool,
}

impl HeavyFixtureMaterializationDirectory {
    pub fn named_heavy_fixture_root() -> Self {
        Self {
            path: PathBuf::from("target/worth-store-heavy-fixtures"),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl HeavyFixtureDiskPreflightReceipt {
    pub fn directory(&self) -> &HeavyFixtureMaterializationDirectory {
        &self.directory
    }

    pub const fn required_bytes(&self) -> u64 {
        self.required_bytes
    }

    pub const fn available_bytes(&self) -> u64 {
        self.available_bytes
    }

    pub const fn backend_profile(&self) -> HeavyFixtureBackendProfile {
        self.backend_profile
    }
}

impl HeavyFixtureTempFileMaterialization {
    pub fn begin(preflight: &HeavyFixtureDiskPreflightReceipt, stem: &str) -> io::Result<Self> {
        let filename = format!("{}.fixture.bin", sanitized_fixture_stem(stem));
        let path = preflight.directory.path().join(filename);
        File::create(&path)?;
        Ok(Self {
            path,
            bytes_written: 0,
        })
    }

    pub fn append_chunk(&mut self, bytes: &[u8]) -> io::Result<()> {
        let mut file = fs::OpenOptions::new().append(true).open(&self.path)?;
        file.write_all(bytes)?;
        self.bytes_written += bytes.len() as u64;
        Ok(())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub const fn bytes_written(&self) -> u64 {
        self.bytes_written
    }
}

impl HeavyFixtureCleanupReceipt {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub const fn completed(&self) -> bool {
        self.completed
    }
}

pub fn preflight_heavy_fixture_directory(
    directory: HeavyFixtureMaterializationDirectory,
    required_bytes: u64,
    backend_profile: HeavyFixtureBackendProfile,
) -> io::Result<HeavyFixtureDiskPreflightReceipt> {
    fs::create_dir_all(directory.path())?;
    let available_bytes = available_space(directory.path())?;
    if available_bytes < required_bytes {
        return Err(io::Error::other(format!(
            "heavy fixture preflight failed: required={required_bytes} available={available_bytes}"
        )));
    }
    Ok(HeavyFixtureDiskPreflightReceipt {
        directory,
        required_bytes,
        available_bytes,
        backend_profile,
    })
}

pub fn cleanup_heavy_fixture_materialization(
    materialization: HeavyFixtureTempFileMaterialization,
) -> io::Result<HeavyFixtureCleanupReceipt> {
    let path = materialization.path;
    if path.exists() {
        fs::remove_file(&path)?;
    }
    Ok(HeavyFixtureCleanupReceipt {
        path,
        completed: true,
    })
}

fn sanitized_fixture_stem(stem: &str) -> String {
    let mut sanitized = stem
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect::<String>();
    sanitized.truncate(80);
    sanitized
}
