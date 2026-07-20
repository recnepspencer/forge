use std::path::Path;

use worth_store_physical_format::{BackupBundleManifest, OfflinePhysicalArtifactFamily};
use worth_store_recovery_physics::RecoveryCandidateObservation;

use super::owner_artifact_verification::{verify_owner_artifact, OwnerObservation};
use super::owner_family_mapping::offline_family;
use super::owner_media_read::OwnerMediaReadSession;
use super::BackupVerificationDefect;
use crate::backup_verification::owner_resource_budget::{
    actual_owner_result_owned_allocation_bytes, maximum_requested_owned_allocation_bytes,
    maximum_reserved_owned_allocation_bytes,
};
use crate::inspection::OwnerDecodedArtifactBinding;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct OwnerSemanticVerificationCounters {
    artifacts_attempted: u64,
    artifacts_verified: u64,
    bytes_verified: u64,
    bytes_read: u64,
    decoder_allocation_bytes: u64,
    peak_buffer_bytes: u64,
}

impl OwnerSemanticVerificationCounters {
    pub(super) fn record_attempt(mut self) -> Option<Self> {
        self.artifacts_attempted = self.artifacts_attempted.checked_add(1)?;
        Some(self)
    }

    pub(super) fn record(mut self, observation: OwnerObservation) -> Option<Self> {
        self.artifacts_verified = self.artifacts_verified.checked_add(1)?;
        self.bytes_verified = self.bytes_verified.checked_add(observation.bytes_read)?;
        self.decoder_allocation_bytes = self
            .decoder_allocation_bytes
            .max(observation.decoder_allocation_bytes);
        self.peak_buffer_bytes = self.peak_buffer_bytes.max(observation.peak_buffer_bytes);
        Some(self)
    }

    pub(super) fn record_read(mut self, bytes: u64) -> Option<Self> {
        self.bytes_read = self.bytes_read.checked_add(bytes)?;
        Some(self)
    }

    pub(super) const fn artifacts_verified(self) -> u64 {
        self.artifacts_verified
    }

    pub(super) const fn artifacts_attempted(self) -> u64 {
        self.artifacts_attempted
    }

    pub(super) const fn bytes_verified(self) -> u64 {
        self.bytes_verified
    }

    pub(super) const fn bytes_read(self) -> u64 {
        self.bytes_read
    }

    pub(super) const fn decoder_allocation_bytes(self) -> u64 {
        self.decoder_allocation_bytes
    }

    pub(super) const fn peak_buffer_bytes(self) -> u64 {
        self.peak_buffer_bytes
    }
}

pub(super) struct OwnerSemanticVerificationResult {
    pub(super) counters: OwnerSemanticVerificationCounters,
    pub(super) recovery_candidates: Vec<RecoveryCandidateObservation>,
    pub(super) owner_bindings: Vec<OwnerDecodedArtifactBinding>,
    pub(super) peak_owned_allocation_bytes: u64,
}

pub(super) struct OwnerSemanticVerificationResourceDenial {
    pub(super) required_bytes: u64,
    pub(super) limit_bytes: u64,
}

pub(super) enum OwnerSemanticVerificationDenial {
    Resource(OwnerSemanticVerificationResourceDenial),
    AllocationFailed,
    Media(worth_store_physical_backend::OfflineMediaReadDenial),
    Inspection(crate::OfflineInspectionDenial),
}

pub(super) fn verify_owner_semantics(
    root: &Path,
    manifest: &BackupBundleManifest,
    max_buffer_bytes: usize,
    maximum_owned_allocation_bytes: u64,
    defects: &mut Vec<BackupVerificationDefect>,
    media: &mut OwnerMediaReadSession,
) -> Result<OwnerSemanticVerificationResult, OwnerSemanticVerificationDenial> {
    let requested_owned_allocation_bytes = maximum_requested_owned_allocation_bytes(root, manifest)
        .and_then(|bytes| bytes.checked_add(max_buffer_bytes as u64))
        .unwrap_or(u64::MAX);
    if requested_owned_allocation_bytes > maximum_owned_allocation_bytes {
        return Err(OwnerSemanticVerificationDenial::Resource(
            OwnerSemanticVerificationResourceDenial {
                required_bytes: requested_owned_allocation_bytes,
                limit_bytes: maximum_owned_allocation_bytes,
            },
        ));
    }
    let mut counters = OwnerSemanticVerificationCounters::default();
    let mut recovery_candidates = Vec::new();
    let mut owner_bindings = Vec::new();
    if recovery_candidates
        .try_reserve_exact(manifest.artifacts().len())
        .is_err()
        || owner_bindings
            .try_reserve_exact(manifest.artifacts().len().saturating_add(1))
            .is_err()
    {
        return Err(OwnerSemanticVerificationDenial::AllocationFailed);
    }
    let reserved_peak = maximum_reserved_owned_allocation_bytes(
        root,
        manifest,
        recovery_candidates.capacity(),
        owner_bindings.capacity(),
        max_buffer_bytes,
    )
    .unwrap_or(u64::MAX);
    if reserved_peak > maximum_owned_allocation_bytes {
        return Err(OwnerSemanticVerificationDenial::Resource(
            OwnerSemanticVerificationResourceDenial {
                required_bytes: reserved_peak,
                limit_bytes: maximum_owned_allocation_bytes,
            },
        ));
    }
    for row in manifest.artifacts() {
        media
            .reject_interruption()
            .map_err(OwnerSemanticVerificationDenial::Inspection)?;
        let Some(attempted) = counters.record_attempt() else {
            defects.push(BackupVerificationDefect::VerificationCounterOverflow);
            break;
        };
        counters = attempted;
        let path = root.join(row.output_name());
        let mut reader = match media.reader(&path) {
            Ok(reader) => reader,
            Err(worth_store_physical_backend::OfflineMediaReadDenial::InvalidFileIndex)
                if !path.exists() =>
            {
                // Structural comparison already recorded the absent component.
                // Continue owner decoding for the remaining immutable closure
                // so one omission cannot conceal independent corruption.
                continue;
            }
            Err(denial) => return Err(OwnerSemanticVerificationDenial::Media(denial)),
        };
        let actual_bytes = reader.length();
        let verification = verify_owner_artifact(
            &mut reader,
            actual_bytes,
            manifest.root_generation(),
            row,
            max_buffer_bytes,
        );
        let bytes_read = reader
            .finish()
            .map_err(OwnerSemanticVerificationDenial::Inspection)?;
        counters =
            counters
                .record_read(bytes_read)
                .ok_or(OwnerSemanticVerificationDenial::Inspection(
                    crate::OfflineInspectionDenial::CounterOverflow,
                ))?;
        match verification {
            Ok(verified) => {
                let observation = verified.observation();
                let Some(recorded) = counters.record(observation) else {
                    defects.push(BackupVerificationDefect::VerificationCounterOverflow);
                    continue;
                };
                counters = recorded;
                if let Some(candidate) = verified.into_recovery_candidate() {
                    recovery_candidates.push(candidate);
                }
                let Some(physical_owner) = row.reclaim_owner().generation_owner() else {
                    defects.push(BackupVerificationDefect::OwnerSemanticMismatch {
                        output_name: row.output_name().to_owned(),
                        format: row.format(),
                        kind: super::BackupArtifactSemanticDefectKind::OwnerReferenceInvalid,
                    });
                    continue;
                };
                owner_bindings.push(
                    OwnerDecodedArtifactBinding::with_physical_owner(
                        root.join(row.output_name()),
                        offline_family(row.family()),
                        row.generation(),
                        physical_owner,
                    )
                    .expect("admitted backup row has a nonzero generation"),
                );
            }
            Err(kind) => defects.push(BackupVerificationDefect::OwnerSemanticMismatch {
                output_name: row.output_name().to_owned(),
                format: row.format(),
                kind,
            }),
        }
    }
    owner_bindings.push(
        OwnerDecodedArtifactBinding::new(
            root.join("backup.manifest"),
            OfflinePhysicalArtifactFamily::Manifest,
            manifest.manifest_generation(),
        )
        .expect("admitted manifest generation is nonzero"),
    );
    let actual_owned_allocation_bytes = actual_owner_result_owned_allocation_bytes(
        &recovery_candidates,
        &owner_bindings,
        max_buffer_bytes,
    )
    .ok_or(OwnerSemanticVerificationDenial::Resource(
        OwnerSemanticVerificationResourceDenial {
            required_bytes: u64::MAX,
            limit_bytes: maximum_owned_allocation_bytes,
        },
    ))?;
    if actual_owned_allocation_bytes > maximum_owned_allocation_bytes {
        return Err(OwnerSemanticVerificationDenial::Resource(
            OwnerSemanticVerificationResourceDenial {
                required_bytes: actual_owned_allocation_bytes,
                limit_bytes: maximum_owned_allocation_bytes,
            },
        ));
    }
    Ok(OwnerSemanticVerificationResult {
        counters,
        recovery_candidates,
        owner_bindings,
        peak_owned_allocation_bytes: actual_owned_allocation_bytes,
    })
}
