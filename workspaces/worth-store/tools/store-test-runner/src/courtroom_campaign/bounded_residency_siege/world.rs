use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

#[path = "world/expectation.rs"]
mod expectation;

const CONFIGURATION_SCHEMA: &str =
    "worth.store.physical-work-courtroom.bounded-residency.configuration.v3";
pub(super) const WORKLOAD_SEED: u64 = 7_312_955_904_608_109_267;
pub(super) const INLINE_RECORD_BYTES: usize = 3_000;
pub(super) const INLINE_RECORDS: usize = 64;
pub(super) const EXTENT_RECORD_BYTES: usize = 1_048_576;
pub(super) const EXTENT_RECORDS: usize = 109;
pub(super) const TOTAL_BYTES: u64 = 6_979_584;
pub(super) const RESIDENT_BYTES: u64 = 65_536;
pub(super) const METADATA_BYTES: u64 = 32_768;
pub(super) const FRAME_ENTRIES: u32 = 12;
pub(super) const RESIDENT_FRAMES: u32 = 8;
pub(super) const PINNED_FRAMES: u32 = 4;
pub(super) const PIN_LEASES: u32 = 6;
pub(super) const DIRTY_FRAMES: u32 = 2;
pub(super) const DIRTY_REPLACEMENT_BYTES: u64 = 65_536;
pub(super) const OPERATION_BYTES: u64 = 6_815_744;
pub(super) const CHECKPOINT_MEMORY_BYTES: u64 = 1_048_576;
pub(super) const FOREGROUND_READ_SCOPE_BYTES: u64 = 2_097_152;
pub(super) const FOREGROUND_WRITE_SCOPE_BYTES: u64 = 6_815_744;
pub(super) const RECOVERY_SCOPE_BYTES: u64 = 2_359_296;
pub(super) const SCRUB_SCOPE_BYTES: u64 = 1_835_008;
pub(super) const MAINTENANCE_SCOPE_BYTES: u64 = 1_572_864;
pub(super) const VERIFICATION_SCOPE_BYTES: u64 = 1_048_576;
pub(super) const BLOB_SCOPE_BYTES: u64 = 1_310_720;
pub(super) const PREFETCH_FRAMES: u32 = 2;
pub(super) const READ_AHEAD_FRAMES: u32 = 2;
pub(super) const WRITE_BEHIND_FRAMES: u32 = 1;
pub(super) const SERVING_APPEND_RECORDS: u64 = 2;
static WORLD_SEQUENCE: AtomicU64 = AtomicU64::new(1);

pub(super) struct BoundedResidencySiegeWorld {
    ownership: WorldRootOwnership,
    store: PathBuf,
    configuration: PathBuf,
}

enum WorldRootOwnership {
    Temporary(tempfile::TempDir),
    Retained(PathBuf),
}

impl BoundedResidencySiegeWorld {
    pub(super) fn create(target_root: Option<&Path>) -> Result<Self, String> {
        let ownership = match target_root {
            Some(target) => WorldRootOwnership::Retained(create_retained_root(target)?),
            None => WorldRootOwnership::Temporary(
                tempfile::Builder::new()
                    .prefix("worth-store-bounded-residency-")
                    .tempdir()
                    .map_err(|error| {
                        format!("cannot create bounded-residency courtroom world: {error}")
                    })?,
            ),
        };
        let store = ownership.path().join("store");
        let configuration = ownership.path().join("configuration");
        write_configuration(&configuration)?;
        Ok(Self {
            ownership,
            store,
            configuration,
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

    pub(super) const fn expected_records(&self) -> u64 {
        (INLINE_RECORDS + EXTENT_RECORDS) as u64
    }

    pub(super) const fn expected_payload_bytes(&self) -> u64 {
        (INLINE_RECORD_BYTES * INLINE_RECORDS + EXTENT_RECORD_BYTES * EXTENT_RECORDS) as u64
    }

    pub(super) const fn producer_records(&self) -> u64 {
        self.expected_records() - SERVING_APPEND_RECORDS
    }

    pub(super) const fn producer_payload_bytes(&self) -> u64 {
        self.expected_payload_bytes() - SERVING_APPEND_RECORDS * EXTENT_RECORD_BYTES as u64
    }

    pub(super) const fn admitted_byte_limit(&self) -> u64 {
        TOTAL_BYTES
    }

    pub(super) const fn resident_byte_limit(&self) -> u64 {
        RESIDENT_BYTES
    }

    pub(super) fn expectation_digest(&self) -> [u8; 32] {
        expectation::digest()
    }

    pub(super) const fn seed(&self) -> u64 {
        WORKLOAD_SEED
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

fn write_configuration(path: &Path) -> Result<(), String> {
    let encoded = format!(
        "{CONFIGURATION_SCHEMA}\nseed={WORKLOAD_SEED}\n\
         inline-record-bytes={INLINE_RECORD_BYTES}\ninline-records={INLINE_RECORDS}\n\
         extent-record-bytes={EXTENT_RECORD_BYTES}\nextent-records={EXTENT_RECORDS}\n\
         total-bytes={TOTAL_BYTES}\nresident-bytes={RESIDENT_BYTES}\n\
         metadata-bytes={METADATA_BYTES}\nframe-entries={FRAME_ENTRIES}\n\
         resident-frames={RESIDENT_FRAMES}\npinned-frames={PINNED_FRAMES}\n\
         pin-leases={PIN_LEASES}\ndirty-frames={DIRTY_FRAMES}\n\
         dirty-replacement-bytes={DIRTY_REPLACEMENT_BYTES}\n\
         operation-bytes={OPERATION_BYTES}\n\
         checkpoint-memory-bytes={CHECKPOINT_MEMORY_BYTES}\n\
         scope-foreground-read-bytes={FOREGROUND_READ_SCOPE_BYTES}\n\
         scope-foreground-write-bytes={FOREGROUND_WRITE_SCOPE_BYTES}\n\
         scope-recovery-bytes={RECOVERY_SCOPE_BYTES}\n\
         scope-scrub-bytes={SCRUB_SCOPE_BYTES}\n\
         scope-maintenance-bytes={MAINTENANCE_SCOPE_BYTES}\n\
         scope-verification-bytes={VERIFICATION_SCOPE_BYTES}\n\
         scope-blob-bytes={BLOB_SCOPE_BYTES}\n\
         speculative-prefetch-frames={PREFETCH_FRAMES}\n\
         speculative-read-ahead-frames={READ_AHEAD_FRAMES}\n\
         speculative-write-behind-frames={WRITE_BEHIND_FRAMES}\n"
    );
    std::fs::write(path, encoded)
        .map_err(|error| format!("cannot write bounded-residency configuration: {error}"))
}

fn create_retained_root(target: &Path) -> Result<PathBuf, String> {
    let target = absolute_target(target)?;
    if target.to_string_lossy().starts_with(r"\\?\") {
        return Err(
            "bounded-residency target root cannot use the Windows verbatim namespace".into(),
        );
    }
    std::fs::create_dir_all(&target)
        .map_err(|error| format!("cannot create bounded-residency target root: {error}"))?;
    for _ in 0..32 {
        let sequence = WORLD_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let candidate = target.join(format!(
            "bounded-residency-{}-{sequence}",
            std::process::id()
        ));
        match std::fs::create_dir(&candidate) {
            Ok(()) => return Ok(candidate),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(format!(
                    "cannot create retained bounded-residency root {}: {error}",
                    candidate.display()
                ))
            }
        }
    }
    Err("cannot allocate a unique retained bounded-residency root".into())
}

fn absolute_target(target: &Path) -> Result<PathBuf, String> {
    if target.is_absolute() {
        Ok(target.to_path_buf())
    } else {
        std::env::current_dir()
            .map(|current| current.join(target))
            .map_err(|error| format!("cannot resolve bounded-residency current directory: {error}"))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{
        BoundedResidencySiegeWorld, EXTENT_RECORD_BYTES, RESIDENT_BYTES, SERVING_APPEND_RECORDS,
        TOTAL_BYTES, WRITE_BEHIND_FRAMES,
    };

    #[test]
    fn world_declares_the_exact_hostile_ratios_without_a_decoded_oracle_file() {
        let world = BoundedResidencySiegeWorld::create(None).unwrap();
        assert!(world.expected_payload_bytes() >= 32 * RESIDENT_BYTES);
        assert!(world.expected_payload_bytes() >= 16 * TOTAL_BYTES);
        assert_eq!(
            world.producer_records() + SERVING_APPEND_RECORDS,
            world.expected_records()
        );
        assert_eq!(
            world.producer_payload_bytes() + SERVING_APPEND_RECORDS * EXTENT_RECORD_BYTES as u64,
            world.expected_payload_bytes()
        );
        assert!(world.producer_payload_bytes() >= 32 * RESIDENT_BYTES);
        assert!(world.producer_payload_bytes() >= 16 * TOTAL_BYTES);
        assert_eq!(WRITE_BEHIND_FRAMES, 1);
        assert!(!world.root().join("records.oracle").exists());
        assert!(!world.store().exists());
    }

    #[test]
    fn retained_world_accepts_an_ordinary_absolute_windows_path() {
        let target = tempfile::tempdir().unwrap();
        let world = BoundedResidencySiegeWorld::create(Some(Path::new(target.path()))).unwrap();
        assert!(world.root().starts_with(target.path()));
        assert!(!world.root().to_string_lossy().starts_with(r"\\?\"));
    }
}
