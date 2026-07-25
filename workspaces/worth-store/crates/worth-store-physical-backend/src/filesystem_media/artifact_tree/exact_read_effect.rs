use std::io::Read;

use super::{ArtifactTreeFailure, ArtifactTreeFailureKind};
use crate::filesystem_media::{FilesystemMediaOwner, MediaOperationIdentity, MediaOperationRole};

pub(super) enum ExactReadEffect {
    Completed(MediaOperationIdentity),
    DeniedBeforeEffect(ArtifactTreeFailure),
}

pub(super) fn execute(
    owner: &FilesystemMediaOwner,
    file: &mut cap_std::fs::File,
    target: &mut [u8],
) -> ExactReadEffect {
    let requested = target.len() as u64;
    let Some((operation, attempt)) = super::super::artifact_tree_effects::begin_identified(
        owner,
        MediaOperationRole::PositionedRead,
        requested,
    ) else {
        return ExactReadEffect::DeniedBeforeEffect(ArtifactTreeFailure::structural(
            ArtifactTreeFailureKind::DeniedBeforeEffect,
        ));
    };
    if let Some(error) = attempt.fail_before_error() {
        attempt.denied();
        return ExactReadEffect::DeniedBeforeEffect(ArtifactTreeFailure::io(
            ArtifactTreeFailureKind::DeniedBeforeEffect,
            &error,
        ));
    }
    if attempt.transfer_limit(requested) != requested {
        attempt.denied();
        return ExactReadEffect::DeniedBeforeEffect(ArtifactTreeFailure::structural(
            ArtifactTreeFailureKind::AccessLimitExceeded,
        ));
    }
    match file.read_exact(target) {
        Ok(()) => {
            attempt.completed(requested);
            ExactReadEffect::Completed(operation)
        }
        Err(error) => {
            attempt.denied();
            ExactReadEffect::DeniedBeforeEffect(ArtifactTreeFailure::io(
                ArtifactTreeFailureKind::Damaged,
                &error,
            ))
        }
    }
}
