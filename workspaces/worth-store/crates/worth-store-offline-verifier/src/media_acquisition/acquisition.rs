use std::path::{Path, PathBuf};

use worth_store_physical_backend::{OfflineMediaReadDenial, ReadOnlyOfflineMediaCapability};

use crate::{OfflineInspectionBudget, OfflineMediaAcquisitionBudget};

use super::UntrustedOfflineMediaSet;

#[derive(Debug)]
pub enum OfflineMediaAcquisitionDenial {
    MissingRoot {
        root: PathBuf,
    },
    RootNotDirectory {
        root: PathBuf,
    },
    SymbolicLinkUnsupported {
        path: PathBuf,
    },
    DirectoryRead {
        path: PathBuf,
        source: std::io::Error,
    },
    BudgetExceeded {
        dimension: OfflineMediaAcquisitionDimension,
        admitted: u64,
        limit: u64,
    },
    Media(OfflineMediaReadDenial),
    AcquisitionAllocationFailed,
    SessionAllocationFailed,
    Interrupted(crate::OfflineInspectionDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineMediaAcquisitionDimension {
    Files,
    Directories,
    PathBytes,
    Depth,
    OwnedAllocationBytes,
}

pub(crate) fn acquire_read_only_media(
    media: UntrustedOfflineMediaSet,
    budget: OfflineInspectionBudget,
    cancellation: &crate::OfflineInspectionCancellation,
    started_at: std::time::Instant,
) -> Result<ReadOnlyOfflineMediaCapability, OfflineMediaAcquisitionDenial> {
    reject_interruption(budget, cancellation, started_at)?;
    let (root, basis) = media.into_parts();
    if !root.exists() {
        return Err(OfflineMediaAcquisitionDenial::MissingRoot { root });
    }
    if std::fs::symlink_metadata(&root)
        .map(|metadata| metadata.file_type().is_symlink())
        .unwrap_or(false)
    {
        return Err(OfflineMediaAcquisitionDenial::SymbolicLinkUnsupported { path: root });
    }
    if !root.is_dir() {
        return Err(OfflineMediaAcquisitionDenial::RootNotDirectory { root });
    }
    let basis_owned =
        basis
            .owned_allocation_bytes()
            .ok_or(OfflineMediaAcquisitionDenial::BudgetExceeded {
                dimension: OfflineMediaAcquisitionDimension::OwnedAllocationBytes,
                admitted: u64::MAX,
                limit: budget.maximum_owned_allocation_bytes(),
            })?;
    let files = collect_files(
        &root,
        budget.acquisition(),
        basis_owned,
        budget.maximum_owned_allocation_bytes(),
        budget,
        cancellation,
        started_at,
    )?;
    ReadOnlyOfflineMediaCapability::open_bounded_from_owned_paths(
        files,
        basis,
        budget.maximum_owned_allocation_bytes(),
    )
    .map_err(OfflineMediaAcquisitionDenial::Media)
}

fn collect_files(
    root: &Path,
    budget: OfflineMediaAcquisitionBudget,
    basis_owned_bytes: u64,
    maximum_owned_allocation_bytes: u64,
    inspection_budget: OfflineInspectionBudget,
    cancellation: &crate::OfflineInspectionCancellation,
    started_at: std::time::Instant,
) -> Result<Vec<PathBuf>, OfflineMediaAcquisitionDenial> {
    reject_interruption(inspection_budget, cancellation, started_at)?;
    let mut files = Vec::new();
    let mut directories = 1u64;
    let mut total_path_bytes = path_bytes(root);
    enforce(
        OfflineMediaAcquisitionDimension::PathBytes,
        total_path_bytes,
        budget.max_path_bytes(),
    )?;
    let mut pending = Vec::new();
    pending
        .try_reserve(1)
        .map_err(|_| OfflineMediaAcquisitionDenial::AcquisitionAllocationFailed)?;
    pending.push((root.to_path_buf(), 0u64));
    enforce_owned_allocation(
        basis_owned_bytes,
        total_path_bytes,
        &files,
        &pending,
        maximum_owned_allocation_bytes,
    )?;
    while let Some((path, depth)) = pending.pop() {
        reject_interruption(inspection_budget, cancellation, started_at)?;
        let entries = std::fs::read_dir(&path).map_err(|source| {
            OfflineMediaAcquisitionDenial::DirectoryRead {
                path: path.clone(),
                source,
            }
        })?;
        for entry in entries {
            reject_interruption(inspection_budget, cancellation, started_at)?;
            let entry = entry.map_err(|source| OfflineMediaAcquisitionDenial::DirectoryRead {
                path: path.to_path_buf(),
                source,
            })?;
            let entry_path = entry.path();
            total_path_bytes = total_path_bytes
                .checked_add(path_bytes(&entry_path))
                .ok_or(OfflineMediaAcquisitionDenial::BudgetExceeded {
                    dimension: OfflineMediaAcquisitionDimension::PathBytes,
                    admitted: u64::MAX,
                    limit: budget.max_path_bytes(),
                })?;
            enforce(
                OfflineMediaAcquisitionDimension::PathBytes,
                total_path_bytes,
                budget.max_path_bytes(),
            )?;
            let file_type = entry.file_type().map_err(|source| {
                OfflineMediaAcquisitionDenial::DirectoryRead {
                    path: entry_path.clone(),
                    source,
                }
            })?;
            if file_type.is_symlink() {
                return Err(OfflineMediaAcquisitionDenial::SymbolicLinkUnsupported {
                    path: entry_path,
                });
            }
            if file_type.is_dir() {
                let child_depth =
                    depth
                        .checked_add(1)
                        .ok_or(OfflineMediaAcquisitionDenial::BudgetExceeded {
                            dimension: OfflineMediaAcquisitionDimension::Depth,
                            admitted: u64::MAX,
                            limit: budget.max_depth(),
                        })?;
                enforce(
                    OfflineMediaAcquisitionDimension::Depth,
                    child_depth,
                    budget.max_depth(),
                )?;
                directories = directories.checked_add(1).ok_or(
                    OfflineMediaAcquisitionDenial::BudgetExceeded {
                        dimension: OfflineMediaAcquisitionDimension::Directories,
                        admitted: u64::MAX,
                        limit: budget.max_directories(),
                    },
                )?;
                enforce(
                    OfflineMediaAcquisitionDimension::Directories,
                    directories,
                    budget.max_directories(),
                )?;
                if pending.len() == pending.capacity() {
                    pending
                        .try_reserve(1)
                        .map_err(|_| OfflineMediaAcquisitionDenial::AcquisitionAllocationFailed)?;
                }
                pending.push((entry_path, child_depth));
            } else if file_type.is_file() {
                let admitted_files = u64::try_from(files.len())
                    .ok()
                    .and_then(|count| count.checked_add(1))
                    .ok_or(OfflineMediaAcquisitionDenial::BudgetExceeded {
                        dimension: OfflineMediaAcquisitionDimension::Files,
                        admitted: u64::MAX,
                        limit: budget.max_files(),
                    })?;
                enforce(
                    OfflineMediaAcquisitionDimension::Files,
                    admitted_files,
                    budget.max_files(),
                )?;
                if files.len() == files.capacity() {
                    files
                        .try_reserve(1)
                        .map_err(|_| OfflineMediaAcquisitionDenial::AcquisitionAllocationFailed)?;
                }
                files.push(entry_path);
            }
            enforce_owned_allocation(
                basis_owned_bytes,
                total_path_bytes,
                &files,
                &pending,
                maximum_owned_allocation_bytes,
            )?;
        }
    }
    Ok(files)
}

fn reject_interruption(
    budget: OfflineInspectionBudget,
    cancellation: &crate::OfflineInspectionCancellation,
    started_at: std::time::Instant,
) -> Result<(), OfflineMediaAcquisitionDenial> {
    crate::inspection::reject_inspection_interruption(budget, cancellation, started_at)
        .map_err(OfflineMediaAcquisitionDenial::Interrupted)
}

fn enforce_owned_allocation(
    basis_owned_bytes: u64,
    path_payload_bytes: u64,
    files: &Vec<PathBuf>,
    pending: &Vec<(PathBuf, u64)>,
    limit: u64,
) -> Result<(), OfflineMediaAcquisitionDenial> {
    let file_rows = allocation_bytes::<PathBuf>(files.capacity())?;
    let pending_rows = allocation_bytes::<(PathBuf, u64)>(pending.capacity())?;
    let admitted = basis_owned_bytes
        .checked_add(path_payload_bytes)
        .and_then(|bytes| bytes.checked_add(file_rows))
        .and_then(|bytes| bytes.checked_add(pending_rows))
        .ok_or(OfflineMediaAcquisitionDenial::BudgetExceeded {
            dimension: OfflineMediaAcquisitionDimension::OwnedAllocationBytes,
            admitted: u64::MAX,
            limit,
        })?;
    enforce(
        OfflineMediaAcquisitionDimension::OwnedAllocationBytes,
        admitted,
        limit,
    )
}

fn allocation_bytes<T>(capacity: usize) -> Result<u64, OfflineMediaAcquisitionDenial> {
    u64::try_from(capacity)
        .ok()
        .and_then(|count| count.checked_mul(std::mem::size_of::<T>() as u64))
        .ok_or(OfflineMediaAcquisitionDenial::BudgetExceeded {
            dimension: OfflineMediaAcquisitionDimension::OwnedAllocationBytes,
            admitted: u64::MAX,
            limit: u64::MAX,
        })
}

fn enforce(
    dimension: OfflineMediaAcquisitionDimension,
    admitted: u64,
    limit: u64,
) -> Result<(), OfflineMediaAcquisitionDenial> {
    if admitted > limit {
        Err(OfflineMediaAcquisitionDenial::BudgetExceeded {
            dimension,
            admitted,
            limit,
        })
    } else {
        Ok(())
    }
}

#[cfg(windows)]
fn path_bytes(path: &Path) -> u64 {
    use std::os::windows::ffi::OsStrExt;
    (path.as_os_str().encode_wide().count() as u64) * 2
}

#[cfg(unix)]
fn path_bytes(path: &Path) -> u64 {
    use std::os::unix::ffi::OsStrExt;
    path.as_os_str().as_bytes().len() as u64
}

#[cfg(not(any(windows, unix)))]
fn path_bytes(path: &Path) -> u64 {
    path.to_string_lossy().len() as u64
}
