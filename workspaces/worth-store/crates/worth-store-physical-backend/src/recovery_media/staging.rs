use sha2::{Digest, Sha256};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use crate::filesystem_media::{
    ArtifactNewWriteOutcome, ArtifactNewWriteRange, ArtifactTreeFailure,
    ArtifactTreePublicationEffectOutcome, CompletedArtifactNewWrite, CompletedArtifactRangeRead,
    CompletedArtifactTreePublicationEffect, IndeterminateArtifactNewWrite,
    IndeterminateArtifactTreePublicationEffect,
};
use crate::{
    BackendQueueExecutionAdaptation, BackendQueueExecutionCompletion,
    BackendQueueExecutionPlanBinding,
};

use super::AdmittedRecoveryFilesystemMedia;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryStagingWriteDisposition {
    Created,
    AlreadyMaterialized,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedRecoveryStagingWrite {
    artifact: RecordArtifactFile,
    coordinate: RecordFrameCoordinate,
    payload_digest: [u8; 32],
    disposition: RecoveryStagingWriteDisposition,
    created: Option<CompletedArtifactNewWrite>,
    verified: Option<CompletedArtifactRangeRead>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndeterminateRecoveryStagingWrite {
    artifact: RecordArtifactFile,
    physical: IndeterminateArtifactNewWrite,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedScheduledRecoveryStagingWrite {
    physical: CompletedRecoveryStagingWrite,
    queue: BackendQueueExecutionCompletion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeniedScheduledRecoveryStagingWrite {
    failure: ArtifactTreeFailure,
    queue: Option<BackendQueueExecutionCompletion>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndeterminateScheduledRecoveryStagingWrite {
    physical: IndeterminateRecoveryStagingWrite,
    queue: BackendQueueExecutionCompletion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryStagingWriteOutcome {
    Completed(Box<CompletedScheduledRecoveryStagingWrite>),
    DeniedBeforeEffect(Box<DeniedScheduledRecoveryStagingWrite>),
    Indeterminate(Box<IndeterminateScheduledRecoveryStagingWrite>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedScheduledRecoveryStagingSynchronization {
    physical: CompletedArtifactTreePublicationEffect,
    queue: BackendQueueExecutionCompletion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndeterminateScheduledRecoveryStagingSynchronization {
    physical: IndeterminateArtifactTreePublicationEffect,
    queue: BackendQueueExecutionCompletion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryStagingSynchronizationOutcome {
    Completed(Box<CompletedScheduledRecoveryStagingSynchronization>),
    DeniedBeforeEffect(Box<DeniedScheduledRecoveryStagingWrite>),
    Indeterminate(Box<IndeterminateScheduledRecoveryStagingSynchronization>),
}

impl AdmittedRecoveryFilesystemMedia {
    /// Executes one scheduler-bound C4 ensure-exact operation. The queue ticket
    /// is issued before any media observation, so the convergence read cannot
    /// become an unscheduled recovery side lane.
    pub fn stage_recovery_artifact_scheduled(
        &self,
        artifact: RecordArtifactFile,
        bytes: &[u8],
        binding: BackendQueueExecutionPlanBinding,
    ) -> RecoveryStagingWriteOutcome {
        let Some(coordinate) = u32::try_from(bytes.len())
            .ok()
            .and_then(|length| RecordFrameCoordinate::new(artifact, 0, length))
        else {
            return denied_without_queue();
        };
        let physical = match super::discovery::record_artifact(artifact) {
            Ok(physical) => physical,
            Err(_) => return denied_without_queue(),
        };
        let ticket = match crate::BackendQueueExecutionAuthority::store_owned().issue_ticket(
            binding,
            &self.parts.execution_capability,
            BackendQueueExecutionAdaptation::None,
        ) {
            Ok(ticket) => ticket,
            Err(_) => return denied_without_queue(),
        };
        let media = self.parts.artifact_tree();
        let completed = match media.file_exists(&physical) {
            Ok(true) => verify_existing(&media, artifact, physical, coordinate, bytes),
            Ok(false) => create(&media, artifact, physical, coordinate, bytes),
            Err(failure) => Err(RecoveryStagingPhysicalFailure::Denied(failure)),
        };
        let queue = ticket.begin_completion().observe_queue_depth(1).complete();
        match completed {
            Ok(physical) => RecoveryStagingWriteOutcome::Completed(Box::new(
                CompletedScheduledRecoveryStagingWrite { physical, queue },
            )),
            Err(RecoveryStagingPhysicalFailure::Denied(failure)) => {
                RecoveryStagingWriteOutcome::DeniedBeforeEffect(Box::new(
                    DeniedScheduledRecoveryStagingWrite {
                        failure,
                        queue: Some(queue),
                    },
                ))
            }
            Err(RecoveryStagingPhysicalFailure::Indeterminate(physical)) => {
                RecoveryStagingWriteOutcome::Indeterminate(Box::new(
                    IndeterminateScheduledRecoveryStagingWrite { physical, queue },
                ))
            }
        }
    }

    pub fn synchronize_recovery_artifact_scheduled(
        &self,
        artifact: RecordArtifactFile,
        binding: BackendQueueExecutionPlanBinding,
    ) -> RecoveryStagingSynchronizationOutcome {
        let physical = match super::discovery::record_artifact(artifact) {
            Ok(physical) => physical,
            Err(_) => {
                return RecoveryStagingSynchronizationOutcome::DeniedBeforeEffect(Box::new(
                    DeniedScheduledRecoveryStagingWrite {
                        failure: structural_denial(),
                        queue: None,
                    },
                ))
            }
        };
        let ticket = match crate::BackendQueueExecutionAuthority::store_owned().issue_ticket(
            binding,
            &self.parts.execution_capability,
            BackendQueueExecutionAdaptation::None,
        ) {
            Ok(ticket) => ticket,
            Err(_) => {
                return RecoveryStagingSynchronizationOutcome::DeniedBeforeEffect(Box::new(
                    DeniedScheduledRecoveryStagingWrite {
                        failure: structural_denial(),
                        queue: None,
                    },
                ))
            }
        };
        let physical = self
            .parts
            .artifact_tree()
            .synchronize_file_effect(&physical);
        let queue = ticket.begin_completion().observe_queue_depth(1).complete();
        match physical {
            ArtifactTreePublicationEffectOutcome::Completed(physical) => {
                RecoveryStagingSynchronizationOutcome::Completed(Box::new(
                    CompletedScheduledRecoveryStagingSynchronization { physical, queue },
                ))
            }
            ArtifactTreePublicationEffectOutcome::DeniedBeforeEffect(failure) => {
                RecoveryStagingSynchronizationOutcome::DeniedBeforeEffect(Box::new(
                    DeniedScheduledRecoveryStagingWrite {
                        failure,
                        queue: Some(queue),
                    },
                ))
            }
            ArtifactTreePublicationEffectOutcome::Indeterminate(physical) => {
                RecoveryStagingSynchronizationOutcome::Indeterminate(Box::new(
                    IndeterminateScheduledRecoveryStagingSynchronization { physical, queue },
                ))
            }
        }
    }
}

fn verify_existing(
    media: &crate::filesystem_media::ArtifactTreeMedia<'_>,
    artifact: RecordArtifactFile,
    physical: crate::filesystem_media::ArtifactTreeFile,
    coordinate: RecordFrameCoordinate,
    expected: &[u8],
) -> Result<CompletedRecoveryStagingWrite, RecoveryStagingPhysicalFailure> {
    if media.file_length(&physical) != Ok(expected.len() as u64) {
        return Err(RecoveryStagingPhysicalFailure::Denied(structural_denial()));
    }
    let mut observed = vec![0; expected.len()];
    let verified = match media.read_exact_range(&physical, coordinate, &mut observed) {
        crate::filesystem_media::ArtifactRangeReadOutcome::Completed(completed) => completed,
        crate::filesystem_media::ArtifactRangeReadOutcome::DeniedBeforeEffect(failure) => {
            return Err(RecoveryStagingPhysicalFailure::Denied(failure))
        }
    };
    if observed != expected {
        return Err(RecoveryStagingPhysicalFailure::Denied(structural_denial()));
    }
    Ok(CompletedRecoveryStagingWrite {
        artifact,
        coordinate,
        payload_digest: Sha256::digest(expected).into(),
        disposition: RecoveryStagingWriteDisposition::AlreadyMaterialized,
        created: None,
        verified: Some(verified),
    })
}

fn create(
    media: &crate::filesystem_media::ArtifactTreeMedia<'_>,
    artifact: RecordArtifactFile,
    physical: crate::filesystem_media::ArtifactTreeFile,
    coordinate: RecordFrameCoordinate,
    bytes: &[u8],
) -> Result<CompletedRecoveryStagingWrite, RecoveryStagingPhysicalFailure> {
    let range = ArtifactNewWriteRange::new(bytes.len() as u64).expect("nonempty coordinate");
    match media.write_new_exact(&physical, range, bytes) {
        ArtifactNewWriteOutcome::Completed(created) => Ok(CompletedRecoveryStagingWrite {
            artifact,
            coordinate,
            payload_digest: created.payload_digest(),
            disposition: RecoveryStagingWriteDisposition::Created,
            created: Some(created),
            verified: None,
        }),
        ArtifactNewWriteOutcome::DeniedBeforeEffect(failure) => {
            Err(RecoveryStagingPhysicalFailure::Denied(failure))
        }
        ArtifactNewWriteOutcome::Indeterminate(physical) => {
            Err(RecoveryStagingPhysicalFailure::Indeterminate(
                IndeterminateRecoveryStagingWrite { artifact, physical },
            ))
        }
    }
}

enum RecoveryStagingPhysicalFailure {
    Denied(ArtifactTreeFailure),
    Indeterminate(IndeterminateRecoveryStagingWrite),
}

fn denied_without_queue() -> RecoveryStagingWriteOutcome {
    RecoveryStagingWriteOutcome::DeniedBeforeEffect(Box::new(DeniedScheduledRecoveryStagingWrite {
        failure: structural_denial(),
        queue: None,
    }))
}

fn structural_denial() -> ArtifactTreeFailure {
    ArtifactTreeFailure::recovery_denial()
}

impl CompletedScheduledRecoveryStagingWrite {
    pub const fn physical(&self) -> &CompletedRecoveryStagingWrite {
        &self.physical
    }
    pub const fn queue(&self) -> BackendQueueExecutionCompletion {
        self.queue
    }
}

impl DeniedScheduledRecoveryStagingWrite {
    pub const fn failure(&self) -> ArtifactTreeFailure {
        self.failure
    }
    pub const fn queue(&self) -> Option<BackendQueueExecutionCompletion> {
        self.queue
    }
}

impl IndeterminateScheduledRecoveryStagingWrite {
    pub const fn physical(&self) -> &IndeterminateRecoveryStagingWrite {
        &self.physical
    }
    pub const fn queue(&self) -> BackendQueueExecutionCompletion {
        self.queue
    }
}

impl CompletedScheduledRecoveryStagingSynchronization {
    pub const fn physical(&self) -> &CompletedArtifactTreePublicationEffect {
        &self.physical
    }
    pub const fn queue(&self) -> BackendQueueExecutionCompletion {
        self.queue
    }
}

impl IndeterminateScheduledRecoveryStagingSynchronization {
    pub const fn physical(&self) -> &IndeterminateArtifactTreePublicationEffect {
        &self.physical
    }
    pub const fn queue(&self) -> BackendQueueExecutionCompletion {
        self.queue
    }
}

impl CompletedRecoveryStagingWrite {
    pub const fn artifact(&self) -> RecordArtifactFile {
        self.artifact
    }
    pub const fn coordinate(&self) -> RecordFrameCoordinate {
        self.coordinate
    }
    pub const fn byte_count(&self) -> u64 {
        self.coordinate.length() as u64
    }
    pub const fn payload_digest(&self) -> [u8; 32] {
        self.payload_digest
    }
    pub const fn disposition(&self) -> RecoveryStagingWriteDisposition {
        self.disposition
    }
    pub const fn created(&self) -> Option<&CompletedArtifactNewWrite> {
        self.created.as_ref()
    }
    pub const fn verified(&self) -> Option<CompletedArtifactRangeRead> {
        self.verified
    }
}

impl IndeterminateRecoveryStagingWrite {
    pub const fn artifact(&self) -> RecordArtifactFile {
        self.artifact
    }
    pub const fn physical(&self) -> &IndeterminateArtifactNewWrite {
        &self.physical
    }
    pub fn into_physical(self) -> IndeterminateArtifactNewWrite {
        self.physical
    }
}
