use sha2::{Digest, Sha256};
use worth_store_physical_format::MaterializedBackupBundle;

use crate::inspection::OwnerObservationBindingDenial;
use crate::{OfflineFileTruthEvidence, OfflineTruthEvidenceSet};

use super::{
    verification_owned_memory::maximum_truth_evidence_owned_allocation_bytes,
    BackupStructuralVerificationDenial, BackupVerificationAllocationPhase,
    BackupVerificationDefect, BackupVerificationReadAccounting, BackupVerificationReport,
    BackupVerificationReportEvidence,
};

pub(super) const fn map_owner_binding_denial(
    denial: OwnerObservationBindingDenial,
) -> BackupStructuralVerificationDenial {
    match denial {
        OwnerObservationBindingDenial::DuplicateSource => {
            BackupStructuralVerificationDenial::OwnerBindingDuplicateSource
        }
        OwnerObservationBindingDenial::MissingSource => {
            BackupStructuralVerificationDenial::OwnerBindingMissingSource
        }
    }
}

pub(super) fn backup_truth_evidence(
    materialized: &MaterializedBackupBundle,
    maximum_owned_allocation_bytes: u64,
) -> Result<OfflineTruthEvidenceSet, BackupStructuralVerificationDenial> {
    let mut entries = Vec::new();
    let entry_count = materialized
        .manifest()
        .artifacts()
        .len()
        .checked_add(1)
        .ok_or(BackupStructuralVerificationDenial::VerificationAllocationFailed)?;
    let requested_owned_allocation_bytes =
        maximum_truth_evidence_owned_allocation_bytes(materialized.root(), materialized.manifest())
            .ok_or(BackupStructuralVerificationDenial::VerificationCounterOverflow)?;
    if requested_owned_allocation_bytes > maximum_owned_allocation_bytes {
        return Err(
            BackupStructuralVerificationDenial::OwnedAllocationBudgetExceeded {
                phase: BackupVerificationAllocationPhase::TruthEvidenceConstruction,
                admitted: requested_owned_allocation_bytes,
                limit: maximum_owned_allocation_bytes,
            },
        );
    }
    entries.try_reserve_exact(entry_count).map_err(|_| {
        BackupStructuralVerificationDenial::TruthEvidence(
            crate::OfflineTruthEvidenceAdmissionDenial::AllocationFailed,
        )
    })?;
    entries.push(
        OfflineFileTruthEvidence::new(materialized.root().join("backup.manifest"))
            .with_expected_digest(materialized.manifest_digest()),
    );
    entries.extend(materialized.manifest().artifacts().iter().map(|row| {
        OfflineFileTruthEvidence::new(materialized.root().join(row.output_name()))
            .with_expected_digest(row.content_digest())
    }));
    let evidence =
        OfflineTruthEvidenceSet::from_owned_entries(entries, maximum_owned_allocation_bytes)
            .map_err(BackupStructuralVerificationDenial::TruthEvidence)?;
    let actual_owned_allocation_bytes = evidence
        .owned_allocation_bytes()
        .ok_or(BackupStructuralVerificationDenial::VerificationCounterOverflow)?;
    if actual_owned_allocation_bytes > maximum_owned_allocation_bytes {
        return Err(
            BackupStructuralVerificationDenial::OwnedAllocationBudgetExceeded {
                phase: BackupVerificationAllocationPhase::TruthEvidenceConstruction,
                admitted: actual_owned_allocation_bytes,
                limit: maximum_owned_allocation_bytes,
            },
        );
    }
    Ok(evidence)
}

pub(super) fn verification_identity(
    bundle: &MaterializedBackupBundle,
    walked: &crate::StructurallyWalkedMedia,
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store:independent-backup-verification:v2\0");
    digest.update(bundle.manifest_digest());
    let basis = walked.consistency_basis().identity();
    digest.update((basis.len() as u64).to_le_bytes());
    digest.update(basis.as_bytes());
    for file in walked.files() {
        digest.update((file.source_index() as u64).to_le_bytes());
        digest.update(file.source().metadata_fingerprint());
        digest.update(file.source().physical_key_fingerprint());
        digest.update(file.length().to_le_bytes());
        digest.update(file.content_digest());
    }
    digest.finalize().into()
}

pub(super) fn closure_defect_report(
    denial: &worth_store_physical_backend::OfflineMediaReadDenial,
    root: &std::path::Path,
) -> Option<BackupVerificationReport> {
    use worth_store_physical_backend::OfflineMediaReadDenial;

    let defects = match denial {
        OfflineMediaReadDenial::ContentClosureMissingArtifact { path } => {
            if path.file_name().and_then(|name| name.to_str()) == Some("backup.manifest") {
                vec![BackupVerificationDefect::PublishedManifestChanged]
            } else {
                vec![BackupVerificationDefect::MissingComponent {
                    output_name: path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("<non-utf8-component>")
                        .to_owned(),
                }]
            }
        }
        OfflineMediaReadDenial::ContentClosureUnexpectedArtifact { path } => {
            vec![BackupVerificationDefect::ExtraComponent { path: path.clone() }]
        }
        OfflineMediaReadDenial::ContentClosureArtifactMismatch { path } => {
            if path == &root.join("backup.manifest") {
                vec![BackupVerificationDefect::PublishedManifestChanged]
            } else {
                vec![BackupVerificationDefect::ComponentDigestMismatch {
                    output_name: path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("<non-utf8-component>")
                        .to_owned(),
                }]
            }
        }
        _ => return None,
    };
    Some(BackupVerificationReport::new(
        BackupVerificationReportEvidence {
            defects,
            admitted_read_bytes: 0,
            inspected_bytes: 0,
            inspected_files: 0,
            peak_buffer_bytes: 0,
            owner_verified_artifacts: 0,
            owner_verified_bytes: 0,
            owner_decoder_allocation_bytes: 0,
            manifest_owned_allocation_bytes: 0,
            peak_owned_allocation_bytes: 0,
            read_accounting: BackupVerificationReadAccounting::UnavailableAfterAcquisitionDenial,
        },
    ))
}

pub(super) fn invalid_manifest() -> BackupStructuralVerificationDenial {
    BackupStructuralVerificationDenial::Format(
        worth_store_physical_format::BackupBundleFormatDenial::InvalidManifest,
    )
}

pub(super) fn hex(bytes: &[u8]) -> Result<String, BackupStructuralVerificationDenial> {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let capacity = bytes
        .len()
        .checked_mul(2)
        .ok_or(BackupStructuralVerificationDenial::VerificationAllocationFailed)?;
    let mut encoded = String::new();
    encoded
        .try_reserve_exact(capacity)
        .map_err(|_| BackupStructuralVerificationDenial::VerificationAllocationFailed)?;
    for byte in bytes {
        encoded.push(DIGITS[usize::from(byte >> 4)] as char);
        encoded.push(DIGITS[usize::from(byte & 0x0f)] as char);
    }
    Ok(encoded)
}
