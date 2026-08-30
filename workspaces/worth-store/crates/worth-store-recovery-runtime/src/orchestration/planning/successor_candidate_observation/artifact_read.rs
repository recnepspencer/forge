use worth_store::physical_runtime::{ObservedRecoveryArtifact, RecoveryDiscoveryFailure};
use worth_store_physical_format::RecordArtifactFile;

use super::artifact_generation;
use crate::entry::PhysicalRecoverySuccessorCandidateDenial;
use crate::progression::RecoveryObservedCandidateArtifact;

pub(super) fn retain_successor(
    successor: u64,
    artifact: RecordArtifactFile,
    bytes: Vec<u8>,
    artifacts: &mut Vec<RecoveryObservedCandidateArtifact>,
) -> bool {
    let generation = match artifact {
        RecordArtifactFile::RootRoutingBlock { generation, .. }
        | RecordArtifactFile::SegmentMembershipBlock { generation, .. }
        | RecordArtifactFile::FreeSpaceMembershipBlock { generation, .. } => generation,
        _ => 0,
    };
    if generation == successor {
        artifacts.push(observed(artifact, bytes));
        true
    } else {
        false
    }
}

pub(super) fn observed(
    artifact: RecordArtifactFile,
    bytes: Vec<u8>,
) -> RecoveryObservedCandidateArtifact {
    RecoveryObservedCandidateArtifact {
        artifact,
        bytes: bytes.into_boxed_slice(),
    }
}

pub(super) fn required(
    result: Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure>,
    artifact: RecordArtifactFile,
) -> Result<Vec<u8>, PhysicalRecoverySuccessorCandidateDenial> {
    optional(result, artifact)?.ok_or(PhysicalRecoverySuccessorCandidateDenial::MissingArtifact {
        artifact,
        generation: artifact_generation(artifact),
    })
}

pub(super) fn optional(
    result: Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure>,
    artifact: RecordArtifactFile,
) -> Result<Option<Vec<u8>>, PhysicalRecoverySuccessorCandidateDenial> {
    match result {
        Ok(observed) => Ok(observed.into_bytes()),
        Err(failure) => Err(PhysicalRecoverySuccessorCandidateDenial::Discovery {
            artifact,
            generation: artifact_generation(artifact),
            failure,
        }),
    }
}

pub(super) fn optional_source(
    result: Result<ObservedRecoveryArtifact, RecoveryDiscoveryFailure>,
    artifact: RecordArtifactFile,
) -> Result<Option<ObservedRecoveryArtifact>, PhysicalRecoverySuccessorCandidateDenial> {
    match result {
        Ok(observed) if observed.bytes().is_some() => Ok(Some(observed)),
        Ok(_) => Ok(None),
        Err(failure) => Err(PhysicalRecoverySuccessorCandidateDenial::Discovery {
            artifact,
            generation: artifact_generation(artifact),
            failure,
        }),
    }
}
