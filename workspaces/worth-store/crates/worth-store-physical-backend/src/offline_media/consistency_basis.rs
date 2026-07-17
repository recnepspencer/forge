use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineMediaClosureEntry {
    path: PathBuf,
    bytes: u64,
    content_digest: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineMediaConsistencyBasis {
    identity: String,
    kind: OfflineMediaConsistencyKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineMediaConsistencyBasisDenial {
    EmptyIdentity,
    EmptyClosure,
    DuplicatePath,
    AllocationFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum OfflineMediaConsistencyKind {
    ContentAddressedClosure(Vec<OfflineMediaClosureEntry>),
    SingleArtifactMutationDetection,
}

impl OfflineMediaClosureEntry {
    pub fn new(path: impl Into<PathBuf>, bytes: u64, content_digest: [u8; 32]) -> Option<Self> {
        let path = path.into();
        (!path.as_os_str().is_empty()).then_some(Self {
            path,
            bytes,
            content_digest,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub const fn bytes(&self) -> u64 {
        self.bytes
    }

    pub const fn content_digest(&self) -> [u8; 32] {
        self.content_digest
    }

    fn owned_allocation_bytes(&self) -> Option<u64> {
        path_bytes(&self.path)
    }
}

impl OfflineMediaConsistencyBasis {
    pub fn content_addressed_closure(
        identity: impl Into<String>,
        entries: impl IntoIterator<Item = OfflineMediaClosureEntry>,
    ) -> Result<Self, OfflineMediaConsistencyBasisDenial> {
        let identity = identity.into();
        if identity.trim().is_empty() {
            return Err(OfflineMediaConsistencyBasisDenial::EmptyIdentity);
        }
        let entries = entries.into_iter();
        let mut collected = Vec::new();
        collected
            .try_reserve_exact(entries.size_hint().0)
            .map_err(|_| OfflineMediaConsistencyBasisDenial::AllocationFailed)?;
        for entry in entries {
            if collected.len() == collected.capacity() {
                collected
                    .try_reserve(1)
                    .map_err(|_| OfflineMediaConsistencyBasisDenial::AllocationFailed)?;
            }
            collected.push(entry);
        }
        Self::content_addressed_closure_from_owned_entries(identity, collected)
    }

    pub fn content_addressed_closure_from_owned_entries(
        identity: impl Into<String>,
        mut entries: Vec<OfflineMediaClosureEntry>,
    ) -> Result<Self, OfflineMediaConsistencyBasisDenial> {
        let identity = identity.into();
        if identity.trim().is_empty() {
            return Err(OfflineMediaConsistencyBasisDenial::EmptyIdentity);
        }
        if entries.is_empty() {
            return Err(OfflineMediaConsistencyBasisDenial::EmptyClosure);
        }
        entries.sort_by(|left, right| left.path.cmp(&right.path));
        if entries.windows(2).any(|pair| pair[0].path == pair[1].path) {
            return Err(OfflineMediaConsistencyBasisDenial::DuplicatePath);
        }
        Ok(Self {
            identity,
            kind: OfflineMediaConsistencyKind::ContentAddressedClosure(entries),
        })
    }

    pub(crate) fn single_artifact_mutation_detection() -> Self {
        Self {
            identity: "single-artifact-mutation-detection".to_owned(),
            kind: OfflineMediaConsistencyKind::SingleArtifactMutationDetection,
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn expected_artifact(&self, path: &Path) -> Option<&OfflineMediaClosureEntry> {
        let entries = self.closure_entries()?;
        entries
            .binary_search_by(|entry| entry.path().cmp(path))
            .ok()
            .map(|index| &entries[index])
    }

    pub const fn is_content_addressed_closure(&self) -> bool {
        matches!(
            self.kind,
            OfflineMediaConsistencyKind::ContentAddressedClosure(_)
        )
    }

    pub(crate) fn closure_entries(&self) -> Option<&[OfflineMediaClosureEntry]> {
        match &self.kind {
            OfflineMediaConsistencyKind::ContentAddressedClosure(entries) => Some(entries),
            OfflineMediaConsistencyKind::SingleArtifactMutationDetection => None,
        }
    }

    /// Returns the allocation payload and collection storage owned by this
    /// basis. Allocator bookkeeping and spare path capacity are deliberately
    /// outside this deterministic protocol counter.
    pub fn owned_allocation_bytes(&self) -> Option<u64> {
        let identity = u64::try_from(self.identity.capacity()).ok()?;
        let closure = match &self.kind {
            OfflineMediaConsistencyKind::ContentAddressedClosure(entries) => {
                let rows = u64::try_from(entries.capacity())
                    .ok()?
                    .checked_mul(std::mem::size_of::<OfflineMediaClosureEntry>() as u64)?;
                entries.iter().try_fold(rows, |total, entry| {
                    total.checked_add(entry.owned_allocation_bytes()?)
                })?
            }
            OfflineMediaConsistencyKind::SingleArtifactMutationDetection => 0,
        };
        identity.checked_add(closure)
    }
}

#[cfg(windows)]
fn path_bytes(path: &Path) -> Option<u64> {
    use std::os::windows::ffi::OsStrExt;
    u64::try_from(path.as_os_str().encode_wide().count())
        .ok()?
        .checked_mul(2)
}

#[cfg(unix)]
fn path_bytes(path: &Path) -> Option<u64> {
    use std::os::unix::ffi::OsStrExt;
    u64::try_from(path.as_os_str().as_bytes().len()).ok()
}

#[cfg(not(any(windows, unix)))]
fn path_bytes(path: &Path) -> Option<u64> {
    u64::try_from(path.to_string_lossy().len()).ok()
}
