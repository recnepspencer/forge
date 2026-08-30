use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use worth_store_physical_format::{
    DurablePhysicalRootManifest, DurableRootSelector, RecordArtifactFile, PHYSICAL_HEADER_LENGTH,
};

use super::RootArtifactRole;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ClosedStoreProcessManifest {
    store_identity: [u8; 16],
    current_selector: ProcessRootArtifact,
    current_root: ProcessRootArtifact,
    directories: BTreeSet<PathBuf>,
    files: BTreeMap<PathBuf, ProcessStoreFile>,
    identity: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ProcessRootArtifact {
    role: RootArtifactRole,
    relative_path: PathBuf,
    concrete_identity: u64,
    root_generation: u64,
    exact_length: u64,
    content_sha256: [u8; 32],
    covered_edit_offset: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ProcessStoreFile {
    exact_length: u64,
    content_sha256: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProcessTreeSnapshot {
    directories: BTreeSet<PathBuf>,
    files: BTreeMap<PathBuf, ProcessStoreFile>,
    contents: BTreeMap<PathBuf, Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ProcessManifestDenial {
    TreeRead,
    NonRegularEntry(PathBuf),
    MissingCurrentSelector,
    InvalidCurrentSelector,
    MissingCurrentRoot,
    InvalidCurrentRoot,
    RootGenerationMismatch,
    DestinationExists,
    CopyFailed,
    TreeMismatch,
    MutationMismatch,
    IdentityEncoding,
}

impl ClosedStoreProcessManifest {
    pub(crate) fn observe(root: &Path) -> Result<Self, ProcessManifestDenial> {
        let snapshot = ProcessTreeSnapshot::observe(root)?;
        let directories = &snapshot.directories;
        let files = &snapshot.files;
        let selector_path = PathBuf::from("families/records/root-current.selector");
        let selector_file = files
            .get(&selector_path)
            .ok_or(ProcessManifestDenial::MissingCurrentSelector)?;
        let selector_bytes = std::fs::read(root.join(&selector_path))
            .map_err(|_| ProcessManifestDenial::MissingCurrentSelector)?;
        let selector = DurableRootSelector::decode(&selector_bytes)
            .map_err(|_| ProcessManifestDenial::InvalidCurrentSelector)?;
        let root_generation = selector.root_generation();
        let root_path = PathBuf::from("families/records/roots").join(
            RecordArtifactFile::RootManifest {
                generation: root_generation,
            }
            .file_name(),
        );
        let root_file = files
            .get(&root_path)
            .ok_or(ProcessManifestDenial::MissingCurrentRoot)?;
        let root_bytes = std::fs::read(root.join(&root_path))
            .map_err(|_| ProcessManifestDenial::MissingCurrentRoot)?;
        let (root_manifest, _) = DurablePhysicalRootManifest::decode(&root_bytes, u16::MAX)
            .map_err(|_| ProcessManifestDenial::InvalidCurrentRoot)?;
        if root_manifest.generation() != root_generation {
            return Err(ProcessManifestDenial::RootGenerationMismatch);
        }
        let store_identity = selector.store_identity().bytes();
        let current_selector = ProcessRootArtifact {
            role: RootArtifactRole::CurrentSelector,
            relative_path: selector_path,
            concrete_identity: selector.identity().get(),
            root_generation,
            exact_length: selector_file.exact_length,
            content_sha256: selector_file.content_sha256,
            covered_edit_offset: u64::from(PHYSICAL_HEADER_LENGTH),
        };
        let current_root = ProcessRootArtifact {
            role: RootArtifactRole::AddressedRootManifest,
            relative_path: root_path,
            concrete_identity: root_generation,
            root_generation,
            exact_length: root_file.exact_length,
            content_sha256: root_file.content_sha256,
            covered_edit_offset: u64::from(PHYSICAL_HEADER_LENGTH)
                + std::mem::size_of::<u64>() as u64,
        };
        let identity = manifest_identity(
            store_identity,
            &current_selector,
            &current_root,
            directories,
            files,
        )?;
        Ok(Self {
            store_identity,
            current_selector,
            current_root,
            directories: snapshot.directories,
            files: snapshot.files,
            identity,
        })
    }

    pub(crate) fn copy_to(
        &self,
        source: &Path,
        destination: &Path,
    ) -> Result<(), ProcessManifestDenial> {
        if destination.exists() {
            return Err(ProcessManifestDenial::DestinationExists);
        }
        self.require_unchanged(source)?;
        std::fs::create_dir_all(destination).map_err(|_| ProcessManifestDenial::CopyFailed)?;
        for relative in &self.directories {
            std::fs::create_dir(destination.join(relative))
                .map_err(|_| ProcessManifestDenial::CopyFailed)?;
        }
        for relative in self.files.keys() {
            let target = destination.join(relative);
            let parent = target.parent().ok_or(ProcessManifestDenial::CopyFailed)?;
            std::fs::create_dir_all(parent).map_err(|_| ProcessManifestDenial::CopyFailed)?;
            std::fs::copy(source.join(relative), target)
                .map_err(|_| ProcessManifestDenial::CopyFailed)?;
        }
        self.require_unchanged(destination)
    }

    pub(crate) fn require_unchanged(&self, root: &Path) -> Result<(), ProcessManifestDenial> {
        let snapshot = ProcessTreeSnapshot::observe(root)?;
        if snapshot.directories == self.directories && snapshot.files == self.files {
            Ok(())
        } else {
            Err(ProcessManifestDenial::TreeMismatch)
        }
    }

    pub(crate) fn artifact(&self, role: RootArtifactRole) -> Option<&ProcessRootArtifact> {
        match role {
            RootArtifactRole::CurrentSelector => Some(&self.current_selector),
            RootArtifactRole::AddressedRootManifest => Some(&self.current_root),
            RootArtifactRole::PreviousSelector => None,
        }
    }

    pub(crate) const fn store_identity(&self) -> [u8; 16] {
        self.store_identity
    }
    pub(crate) const fn identity(&self) -> [u8; 32] {
        self.identity
    }
    pub(crate) fn file_count(&self) -> u64 {
        self.files.len() as u64
    }
    pub(crate) fn byte_count(&self) -> u64 {
        self.files.values().map(|file| file.exact_length).sum()
    }
}

impl ProcessTreeSnapshot {
    pub(crate) fn observe(root: &Path) -> Result<Self, ProcessManifestDenial> {
        let (directories, files, contents) = observe_tree(root)?;
        Ok(Self {
            directories,
            files,
            contents,
        })
    }

    pub(crate) fn require_unchanged(&self, root: &Path) -> Result<(), ProcessManifestDenial> {
        let observed = Self::observe(root)?;
        if observed == *self {
            Ok(())
        } else {
            Err(ProcessManifestDenial::TreeMismatch)
        }
    }

    pub(crate) fn require_exact_one_byte_delta(
        &self,
        root: &Path,
        target: &Path,
        offset: u64,
        xor_mask: u8,
    ) -> Result<([u8; 32], [u8; 32]), ProcessManifestDenial> {
        let observed = Self::observe(root)?;
        if self.directories != observed.directories
            || self.contents.keys().ne(observed.contents.keys())
        {
            return Err(ProcessManifestDenial::MutationMismatch);
        }
        let target_offset =
            usize::try_from(offset).map_err(|_| ProcessManifestDenial::MutationMismatch)?;
        let mut target_digests = None;
        for (relative, before) in &self.contents {
            let after = observed
                .contents
                .get(relative)
                .ok_or(ProcessManifestDenial::MutationMismatch)?;
            if relative != target {
                if before != after {
                    return Err(ProcessManifestDenial::MutationMismatch);
                }
                continue;
            }
            if before.len() != after.len()
                || target_offset >= before.len()
                || before
                    .iter()
                    .zip(after)
                    .enumerate()
                    .filter(|(_, (left, right))| left != right)
                    .map(|(index, _)| index)
                    .ne([target_offset])
                || after[target_offset] != before[target_offset] ^ xor_mask
            {
                return Err(ProcessManifestDenial::MutationMismatch);
            }
            target_digests = Some((Sha256::digest(before).into(), Sha256::digest(after).into()));
        }
        target_digests.ok_or(ProcessManifestDenial::MutationMismatch)
    }
}

impl ProcessRootArtifact {
    pub(crate) const fn role(&self) -> RootArtifactRole {
        self.role
    }
    pub(crate) fn relative_path(&self) -> &Path {
        &self.relative_path
    }
    pub(crate) const fn concrete_identity(&self) -> u64 {
        self.concrete_identity
    }
    pub(crate) const fn root_generation(&self) -> u64 {
        self.root_generation
    }
    pub(crate) const fn exact_length(&self) -> u64 {
        self.exact_length
    }
    pub(crate) const fn content_sha256(&self) -> [u8; 32] {
        self.content_sha256
    }
    pub(crate) const fn covered_edit_offset(&self) -> u64 {
        self.covered_edit_offset
    }
}

fn observe_tree(
    root: &Path,
) -> Result<
    (
        BTreeSet<PathBuf>,
        BTreeMap<PathBuf, ProcessStoreFile>,
        BTreeMap<PathBuf, Vec<u8>>,
    ),
    ProcessManifestDenial,
> {
    let mut observed_directories = BTreeSet::new();
    let mut files = BTreeMap::new();
    let mut contents = BTreeMap::new();
    let mut directories = vec![PathBuf::new()];
    while let Some(relative_directory) = directories.pop() {
        let mut entries = std::fs::read_dir(root.join(&relative_directory))
            .map_err(|_| ProcessManifestDenial::TreeRead)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ProcessManifestDenial::TreeRead)?;
        entries.sort_by_key(std::fs::DirEntry::file_name);
        for entry in entries.into_iter().rev() {
            let relative = relative_directory.join(entry.file_name());
            let kind = entry
                .file_type()
                .map_err(|_| ProcessManifestDenial::TreeRead)?;
            if kind.is_dir() {
                observed_directories.insert(relative.clone());
                directories.push(relative);
            } else if kind.is_file() {
                let bytes =
                    std::fs::read(entry.path()).map_err(|_| ProcessManifestDenial::TreeRead)?;
                files.insert(
                    relative.clone(),
                    ProcessStoreFile {
                        exact_length: bytes.len() as u64,
                        content_sha256: Sha256::digest(&bytes).into(),
                    },
                );
                contents.insert(relative, bytes);
            } else {
                return Err(ProcessManifestDenial::NonRegularEntry(relative));
            }
        }
    }
    Ok((observed_directories, files, contents))
}

fn manifest_identity(
    store_identity: [u8; 16],
    current_selector: &ProcessRootArtifact,
    current_root: &ProcessRootArtifact,
    directories: &BTreeSet<PathBuf>,
    files: &BTreeMap<PathBuf, ProcessStoreFile>,
) -> Result<[u8; 32], ProcessManifestDenial> {
    bincode::serialize(&(
        "worth-store-c9-production-root-manifest-v1",
        store_identity,
        current_selector,
        current_root,
        directories,
        files,
    ))
    .map(|bytes| Sha256::digest(bytes).into())
    .map_err(|_| ProcessManifestDenial::IdentityEncoding)
}
