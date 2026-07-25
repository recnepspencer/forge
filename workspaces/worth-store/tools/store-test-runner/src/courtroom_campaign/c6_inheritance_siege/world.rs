use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const CONFIGURATION_SCHEMA: &str = "worth.store.c5_1.c6-inheritance-siege.configuration.v1";
pub(super) const RECORD_BYTES: usize = 3_000;
pub(super) const RECORD_COUNT: usize = 192;
pub(super) const RESIDENT_BYTES: u64 = 65_536;
pub(super) const METADATA_BYTES: u64 = 16_384;
pub(super) const PINNED_FRAMES: u32 = 8;
pub(super) const PIN_LEASES: u32 = 2;
pub(super) const DIRTY_FRAMES: u32 = 2;
pub(super) const OPERATION_BYTES: u64 = 16_777_216;
pub(super) const FRAME_ENTRIES: u32 = 8;
static WORLD_SEQUENCE: AtomicU64 = AtomicU64::new(1);

pub(super) struct C6SiegeWorld {
    ownership: WorldRootOwnership,
    store: PathBuf,
    configuration: PathBuf,
    oracle: PathBuf,
}

enum WorldRootOwnership {
    Temporary(tempfile::TempDir),
    Retained(PathBuf),
}

impl C6SiegeWorld {
    pub(super) fn create(target_root: Option<&Path>) -> Result<Self, String> {
        let ownership = match target_root {
            Some(target) => WorldRootOwnership::Retained(create_retained_root(target)?),
            None => WorldRootOwnership::Temporary(
                tempfile::Builder::new()
                    .prefix("worth-store-c5-1-courtroom-c-")
                    .tempdir()
                    .map_err(|error| format!("cannot create Courtroom C world: {error}"))?,
            ),
        };
        let store = ownership.path().join("store");
        std::fs::create_dir(&store)
            .map_err(|error| format!("cannot create Courtroom C Store root: {error}"))?;
        let configuration = ownership.path().join("configuration");
        let oracle = ownership.path().join("records.oracle");
        write_configuration(&configuration)?;
        std::fs::write(&oracle, oracle_bytes())
            .map_err(|error| format!("cannot write Courtroom C oracle: {error}"))?;
        Ok(Self {
            ownership,
            store,
            configuration,
            oracle,
        })
    }

    pub(super) fn root(&self) -> &Path {
        self.ownership.path()
    }

    pub(super) fn store(&self) -> &Path {
        &self.store
    }

    pub(super) fn configuration(&self) -> &Path {
        &self.configuration
    }

    pub(super) fn oracle(&self) -> &Path {
        &self.oracle
    }

    pub(super) const fn expected_records(&self) -> u64 {
        RECORD_COUNT as u64
    }

    pub(super) const fn expected_payload_bytes(&self) -> u64 {
        (RECORD_BYTES * RECORD_COUNT) as u64
    }

    pub(super) const fn admitted_byte_limit(&self) -> u64 {
        RESIDENT_BYTES + METADATA_BYTES + OPERATION_BYTES
    }
}

impl WorldRootOwnership {
    fn path(&self) -> &Path {
        match self {
            Self::Temporary(root) => root.path(),
            Self::Retained(root) => root,
        }
    }
}

pub(super) fn oracle_bytes() -> Vec<u8> {
    let mut bytes = Vec::with_capacity(RECORD_BYTES * RECORD_COUNT);
    for ordinal in 0..RECORD_COUNT {
        bytes.resize(bytes.len() + RECORD_BYTES, (ordinal % 251) as u8);
    }
    bytes
}

fn write_configuration(path: &Path) -> Result<(), String> {
    let encoded = format!(
        "{CONFIGURATION_SCHEMA}\nrecord-bytes={RECORD_BYTES}\nrecord-count={RECORD_COUNT}\n\
         resident-bytes={RESIDENT_BYTES}\nmetadata-bytes={METADATA_BYTES}\n\
         pinned-frames={PINNED_FRAMES}\npin-leases={PIN_LEASES}\n\
         dirty-frames={DIRTY_FRAMES}\noperation-bytes={OPERATION_BYTES}\n\
         frame-entries={FRAME_ENTRIES}\n"
    );
    std::fs::write(path, encoded)
        .map_err(|error| format!("cannot write Courtroom C configuration: {error}"))
}

fn create_retained_root(target: &Path) -> Result<PathBuf, String> {
    let target = absolute_target(target)?;
    if target.to_string_lossy().starts_with(r"\\?\") {
        return Err("Courtroom C target root cannot use the Windows verbatim namespace".into());
    }
    std::fs::create_dir_all(&target)
        .map_err(|error| format!("cannot create Courtroom C target root: {error}"))?;
    for _ in 0..32 {
        let sequence = WORLD_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let candidate = target.join(format!("courtroom-c-{}-{sequence}", std::process::id()));
        match std::fs::create_dir(&candidate) {
            Ok(()) => return Ok(candidate),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(format!(
                    "cannot create retained Courtroom C root {}: {error}",
                    candidate.display()
                ))
            }
        }
    }
    Err("cannot allocate a unique retained Courtroom C root".into())
}

fn absolute_target(target: &Path) -> Result<PathBuf, String> {
    if target.is_absolute() {
        Ok(target.to_path_buf())
    } else {
        std::env::current_dir()
            .map(|current| current.join(target))
            .map_err(|error| format!("cannot resolve Courtroom C current directory: {error}"))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{oracle_bytes, C6SiegeWorld, RECORD_BYTES, RECORD_COUNT};

    #[test]
    fn world_owns_an_oversized_deterministic_oracle_and_isolated_store() {
        let world = C6SiegeWorld::create(None).unwrap();
        assert_eq!(oracle_bytes().len(), RECORD_BYTES * RECORD_COUNT);
        assert_eq!(std::fs::read(world.oracle()).unwrap(), oracle_bytes());
        assert!(world.store().starts_with(world.root()));
        assert!(world.expected_payload_bytes() > 8 * 65_536);
    }

    #[test]
    fn retained_world_accepts_an_ordinary_absolute_windows_path() {
        let target = tempfile::tempdir().unwrap();
        let world = C6SiegeWorld::create(Some(Path::new(target.path()))).unwrap();
        assert!(world.root().starts_with(target.path()));
        assert!(!world.root().to_string_lossy().starts_with(r"\\?\"));
    }
}
