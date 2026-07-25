use std::io::{ErrorKind, Read};

use super::{ArtifactTreeFailure, ArtifactTreeFailureKind};
use crate::filesystem_media::{FilesystemMediaOwner, MediaOperationIdentity, MediaOperationRole};

pub(super) enum ExactReadEffect {
    Completed {
        operation: MediaOperationIdentity,
        completed_bytes: u64,
    },
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
    let admitted = attempt.transfer_limit(requested);
    match read_admitted(file, &mut target[..admitted as usize]) {
        AdmittedRead::Completed => {
            if admitted == requested {
                attempt.completed(admitted);
            } else {
                attempt.partial(admitted);
            }
            ExactReadEffect::Completed {
                operation,
                completed_bytes: admitted,
            }
        }
        AdmittedRead::Interrupted {
            completed_bytes: 0,
            error,
        } => {
            attempt.denied();
            ExactReadEffect::DeniedBeforeEffect(ArtifactTreeFailure::io(
                ArtifactTreeFailureKind::Damaged,
                &error,
            ))
        }
        AdmittedRead::Interrupted {
            completed_bytes, ..
        } => {
            attempt.partial(completed_bytes);
            ExactReadEffect::Completed {
                operation,
                completed_bytes,
            }
        }
    }
}

enum AdmittedRead {
    Completed,
    Interrupted {
        completed_bytes: u64,
        error: std::io::Error,
    },
}

fn read_admitted(file: &mut cap_std::fs::File, target: &mut [u8]) -> AdmittedRead {
    let mut completed = 0;
    while completed < target.len() {
        match file.read(&mut target[completed..]) {
            Ok(0) => {
                return AdmittedRead::Interrupted {
                    completed_bytes: completed as u64,
                    error: std::io::Error::from(ErrorKind::UnexpectedEof),
                };
            }
            Ok(read) => completed += read,
            Err(error) if error.kind() == ErrorKind::Interrupted => {}
            Err(error) => {
                return AdmittedRead::Interrupted {
                    completed_bytes: completed as u64,
                    error,
                };
            }
        }
    }
    AdmittedRead::Completed
}
