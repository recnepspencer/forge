use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use super::OfflineMediaConsistencyBasis;

mod closure_validation;
mod file_identity;
mod media_file;
mod owned_allocation;
mod read_denial;
mod read_observation;

use closure_validation::reject_closure_path_mismatch;
use file_identity::{identity, open_revalidated, revalidate, revalidate_open_file};
pub use media_file::OfflineMediaFileIdentity;
use media_file::StableReadOnlyFile;
use owned_allocation::{allocation_for, enforce_owned_allocation, path_owned_bytes};
pub use read_denial::OfflineMediaReadDenial;
pub use read_observation::OfflineMediaReadObservation;

#[derive(Debug)]
pub struct ReadOnlyOfflineMediaCapability {
    basis: OfflineMediaConsistencyBasis,
    files: Vec<StableReadOnlyFile>,
    resident_owned_allocation_bytes: u64,
    peak_owned_allocation_bytes: u64,
}

impl ReadOnlyOfflineMediaCapability {
    pub fn open(
        paths: impl IntoIterator<Item = PathBuf>,
        basis: OfflineMediaConsistencyBasis,
    ) -> Result<Self, OfflineMediaReadDenial> {
        Self::open_bounded(paths, basis, u64::MAX)
    }

    pub fn open_bounded(
        paths: impl IntoIterator<Item = PathBuf>,
        basis: OfflineMediaConsistencyBasis,
        maximum_owned_allocation_bytes: u64,
    ) -> Result<Self, OfflineMediaReadDenial> {
        Self::open_bounded_accounting_input(paths, basis, maximum_owned_allocation_bytes, 0)
    }

    pub fn open_bounded_from_owned_paths(
        paths: Vec<PathBuf>,
        basis: OfflineMediaConsistencyBasis,
        maximum_owned_allocation_bytes: u64,
    ) -> Result<Self, OfflineMediaReadDenial> {
        let path_payload = paths.iter().try_fold(0_u64, |total, path| {
            total.checked_add(path_owned_bytes(path)?)
        });
        let transient_input_owned_bytes = allocation_for::<PathBuf>(paths.capacity())?
            .checked_add(path_payload.ok_or(OfflineMediaReadDenial::CounterOverflow)?)
            .ok_or(OfflineMediaReadDenial::CounterOverflow)?;
        Self::open_bounded_accounting_input(
            paths,
            basis,
            maximum_owned_allocation_bytes,
            transient_input_owned_bytes,
        )
    }

    fn open_bounded_accounting_input(
        paths: impl IntoIterator<Item = PathBuf>,
        basis: OfflineMediaConsistencyBasis,
        maximum_owned_allocation_bytes: u64,
        transient_input_owned_bytes: u64,
    ) -> Result<Self, OfflineMediaReadDenial> {
        if maximum_owned_allocation_bytes == 0 {
            return Err(OfflineMediaReadDenial::OwnedAllocationBudgetExceeded {
                admitted: 1,
                limit: 0,
            });
        }
        let basis_owned = basis
            .owned_allocation_bytes()
            .ok_or(OfflineMediaReadDenial::CounterOverflow)?;
        let input_peak = basis_owned
            .checked_add(transient_input_owned_bytes)
            .ok_or(OfflineMediaReadDenial::CounterOverflow)?;
        enforce_owned_allocation(input_peak, maximum_owned_allocation_bytes)?;
        let mut paths = paths.into_iter();
        let initial_rows = allocation_for::<PathBuf>(paths.size_hint().0)?;
        let initial_peak = input_peak
            .checked_add(initial_rows)
            .ok_or(OfflineMediaReadDenial::CounterOverflow)?;
        enforce_owned_allocation(initial_peak, maximum_owned_allocation_bytes)?;
        let mut collection_peak_owned_allocation_bytes = initial_peak;
        let mut collected_paths = Vec::new();
        collected_paths
            .try_reserve_exact(paths.size_hint().0)
            .map_err(|_| OfflineMediaReadDenial::AllocationFailed)?;
        let mut path_payload = 0_u64;
        for path in paths.by_ref() {
            path_payload = path_payload
                .checked_add(
                    path_owned_bytes(&path).ok_or(OfflineMediaReadDenial::CounterOverflow)?,
                )
                .ok_or(OfflineMediaReadDenial::CounterOverflow)?;
            if collected_paths.len() == collected_paths.capacity() {
                let requested_rows = allocation_for::<PathBuf>(
                    collected_paths
                        .capacity()
                        .checked_add(1)
                        .ok_or(OfflineMediaReadDenial::CounterOverflow)?,
                )?;
                let requested_peak = basis_owned
                    .checked_add(transient_input_owned_bytes)
                    .and_then(|value| value.checked_add(path_payload))
                    .and_then(|value| value.checked_add(requested_rows))
                    .ok_or(OfflineMediaReadDenial::CounterOverflow)?;
                enforce_owned_allocation(requested_peak, maximum_owned_allocation_bytes)?;
                collection_peak_owned_allocation_bytes =
                    collection_peak_owned_allocation_bytes.max(requested_peak);
                collected_paths
                    .try_reserve_exact(1)
                    .map_err(|_| OfflineMediaReadDenial::AllocationFailed)?;
            }
            let admitted_peak = basis_owned
                .checked_add(transient_input_owned_bytes)
                .and_then(|value| value.checked_add(path_payload))
                .and_then(|value| {
                    value.checked_add(allocation_for::<PathBuf>(collected_paths.capacity()).ok()?)
                })
                .ok_or(OfflineMediaReadDenial::CounterOverflow)?;
            enforce_owned_allocation(admitted_peak, maximum_owned_allocation_bytes)?;
            collection_peak_owned_allocation_bytes =
                collection_peak_owned_allocation_bytes.max(admitted_peak);
            collected_paths.push(path);
        }
        drop(paths);
        let mut paths = collected_paths;
        paths.sort();
        let path_rows = allocation_for::<PathBuf>(paths.capacity())?;
        enforce_owned_allocation(
            basis_owned
                .checked_add(path_payload)
                .and_then(|value| value.checked_add(path_rows))
                .ok_or(OfflineMediaReadDenial::CounterOverflow)?,
            maximum_owned_allocation_bytes,
        )?;
        match basis.closure_entries() {
            Some(expected) => reject_closure_path_mismatch(expected, &paths)?,
            None if paths.len() != 1 => {
                return Err(OfflineMediaReadDenial::UnprovenCrossFileConsistency);
            }
            _ => {}
        }
        let mut files = Vec::new();
        files
            .try_reserve_exact(paths.len())
            .map_err(|_| OfflineMediaReadDenial::AllocationFailed)?;
        let mut alias_groups = HashMap::<file_id::FileId, u64>::new();
        alias_groups
            .try_reserve(paths.len())
            .map_err(|_| OfflineMediaReadDenial::AllocationFailed)?;
        let file_rows = allocation_for::<StableReadOnlyFile>(files.capacity())?;
        let alias_rows = allocation_for::<(file_id::FileId, u64)>(alias_groups.capacity())?;
        let working_peak_owned_allocation_bytes = basis_owned
            .checked_add(path_payload)
            .and_then(|value| value.checked_add(path_rows))
            .and_then(|value| value.checked_add(file_rows))
            .and_then(|value| value.checked_add(alias_rows))
            .ok_or(OfflineMediaReadDenial::CounterOverflow)?;
        enforce_owned_allocation(
            working_peak_owned_allocation_bytes,
            maximum_owned_allocation_bytes,
        )?;
        for path in paths {
            let path_metadata =
                std::fs::metadata(&path).map_err(|source| OfflineMediaReadDenial::Io {
                    path: path.clone(),
                    source,
                })?;
            if !path_metadata.is_file() {
                return Err(OfflineMediaReadDenial::NotAFile { path });
            }
            let key_before =
                file_id::get_file_id(&path).map_err(|source| OfflineMediaReadDenial::Io {
                    path: path.clone(),
                    source,
                })?;
            let file = OpenOptions::new()
                .read(true)
                .open(&path)
                .map_err(|source| OfflineMediaReadDenial::Io {
                    path: path.clone(),
                    source,
                })?;
            let metadata = file
                .metadata()
                .map_err(|source| OfflineMediaReadDenial::Io {
                    path: path.clone(),
                    source,
                })?;
            if !metadata.is_file() {
                return Err(OfflineMediaReadDenial::NotAFile { path });
            }
            let key = file_id::get_file_id(&path).map_err(|source| OfflineMediaReadDenial::Io {
                path: path.clone(),
                source,
            })?;
            if key != key_before {
                return Err(OfflineMediaReadDenial::ConcurrentMutationIndeterminate { path });
            }
            let next_group = u64::try_from(alias_groups.len())
                .ok()
                .and_then(|groups| groups.checked_add(1))
                .ok_or(OfflineMediaReadDenial::CounterOverflow)?;
            let alias_group = *alias_groups.entry(key).or_insert(next_group);
            files.push(StableReadOnlyFile {
                identity: identity(path, &metadata, key, alias_group),
            });
        }
        if let Some(expected) = basis.closure_entries() {
            for (file, expected) in files.iter().zip(expected) {
                if file.identity.length() != expected.bytes() {
                    return Err(OfflineMediaReadDenial::ContentClosureArtifactMismatch {
                        path: file.identity.path.clone(),
                    });
                }
            }
        }
        let resident_owned_allocation_bytes = basis_owned
            .checked_add(path_payload)
            .and_then(|value| value.checked_add(file_rows))
            .ok_or(OfflineMediaReadDenial::CounterOverflow)?;
        let peak_owned_allocation_bytes = collection_peak_owned_allocation_bytes
            .max(working_peak_owned_allocation_bytes)
            .max(resident_owned_allocation_bytes);
        Ok(Self {
            basis,
            files,
            resident_owned_allocation_bytes,
            peak_owned_allocation_bytes,
        })
    }

    pub const fn basis(&self) -> &OfflineMediaConsistencyBasis {
        &self.basis
    }
    pub fn file(&self, file_index: usize) -> Option<&OfflineMediaFileIdentity> {
        self.files.get(file_index).map(|file| &file.identity)
    }

    pub fn file_index(&self, path: &Path) -> Option<usize> {
        self.files
            .binary_search_by(|file| file.identity.path().cmp(path))
            .ok()
    }

    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    pub const fn resident_owned_allocation_bytes(&self) -> u64 {
        self.resident_owned_allocation_bytes
    }

    pub const fn peak_owned_allocation_bytes(&self) -> u64 {
        self.peak_owned_allocation_bytes
    }

    pub fn into_consistency_basis(self) -> OfflineMediaConsistencyBasis {
        self.basis
    }

    pub fn read_bounded_into(
        &mut self,
        file_index: usize,
        offset: u64,
        buffer: &mut [u8],
    ) -> Result<OfflineMediaReadObservation, OfflineMediaReadDenial> {
        if buffer.is_empty() {
            return Err(OfflineMediaReadDenial::ZeroReadBudget);
        }
        let file = self
            .files
            .get(file_index)
            .ok_or(OfflineMediaReadDenial::InvalidFileIndex)?;
        if offset > file.identity.length() {
            return Err(OfflineMediaReadDenial::InvalidReadOffset {
                path: file.identity.path.clone(),
                offset,
                length: file.identity.length(),
            });
        }
        let mut opened = open_revalidated(file)?;
        opened
            .seek(SeekFrom::Start(offset))
            .map_err(|source| OfflineMediaReadDenial::Io {
                path: file.identity.path.clone(),
                source,
            })?;
        let read = opened
            .read(buffer)
            .map_err(|source| OfflineMediaReadDenial::Io {
                path: file.identity.path.clone(),
                source,
            })?;
        if read == 0 && offset < file.identity.length() {
            return Err(OfflineMediaReadDenial::UnexpectedEof {
                path: file.identity.path.clone(),
                offset,
                length: file.identity.length(),
            });
        }
        revalidate_open_file(file, &opened)?;
        Ok(OfflineMediaReadObservation::new(file_index, offset, read))
    }

    pub fn revalidate_consistency(&self) -> Result<(), OfflineMediaReadDenial> {
        for file in &self.files {
            revalidate(file)?;
        }
        Ok(())
    }

    pub fn validate_content_closure<'a>(
        &self,
        observed: impl IntoIterator<Item = (&'a Path, u64, [u8; 32])>,
    ) -> Result<(), OfflineMediaReadDenial> {
        let Some(expected) = self.basis.closure_entries() else {
            return Ok(());
        };
        let mut observed = observed.into_iter();
        for expected in expected {
            let Some((path, bytes, digest)) = observed.next() else {
                return Err(OfflineMediaReadDenial::ContentClosureMissingArtifact {
                    path: expected.path().to_path_buf(),
                });
            };
            if path != expected.path()
                || bytes != expected.bytes()
                || digest != expected.content_digest()
            {
                return Err(OfflineMediaReadDenial::ContentClosureArtifactMismatch {
                    path: path.to_path_buf(),
                });
            }
        }
        if let Some((path, _, _)) = observed.next() {
            return Err(OfflineMediaReadDenial::ContentClosureUnexpectedArtifact {
                path: path.to_path_buf(),
            });
        }
        Ok(())
    }
}

pub(crate) use file_identity::physical_file_identity;
