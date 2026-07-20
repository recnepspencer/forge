use super::{
    CompletedMediaEffect, MediaOperationFailureKind, MediaOperationOutcome, MediaOperationRole,
    MediaTransferPosition, MediaTransferProgress, NamespaceFileHandle,
};

pub struct PositionedReadRequest<'buffer> {
    pub(super) offset: u64,
    pub(super) buffer: &'buffer mut [u8],
}

impl<'buffer> PositionedReadRequest<'buffer> {
    pub fn new(offset: u64, buffer: &'buffer mut [u8]) -> Self {
        Self { offset, buffer }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PositionedWriteRequest<'buffer> {
    offset: u64,
    buffer: &'buffer [u8],
}

impl<'buffer> PositionedWriteRequest<'buffer> {
    pub const fn new(offset: u64, buffer: &'buffer [u8]) -> Self {
        Self { offset, buffer }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AppendRequest<'buffer> {
    buffer: &'buffer [u8],
}

impl<'buffer> AppendRequest<'buffer> {
    pub const fn new(buffer: &'buffer [u8]) -> Self {
        Self { buffer }
    }
}

impl NamespaceFileHandle<'_, super::MutableFileAccess> {
    pub fn positioned_write(&self, request: PositionedWriteRequest<'_>) -> MediaOperationOutcome {
        let operation = self.operation_identity();
        let requested = request.buffer.len() as u64;
        let attempt = self.owner().boundary().begin_operation(
            MediaOperationRole::PositionedWrite,
            requested,
            super::MediaOperationCoordinates::for_path(
                operation,
                self.role(),
                Some(self.identity()),
            )
            .at_offset(request.offset),
        );
        if requested == 0 {
            attempt.denied();
            return self.failed_write(
                operation,
                MediaOperationRole::PositionedWrite,
                MediaOperationFailureKind::DeniedBeforeEffect,
                None,
            );
        }
        if let Some(error) = attempt.fail_before_error() {
            attempt.denied();
            return self.failed_write(
                operation,
                MediaOperationRole::PositionedWrite,
                MediaOperationFailureKind::DeniedBeforeEffect,
                Some(&error),
            );
        }
        let Ok(_authority) = self.owner().begin_mutation() else {
            attempt.denied();
            return self.failed_write(
                operation,
                MediaOperationRole::PositionedWrite,
                MediaOperationFailureKind::DeniedBeforeEffect,
                None,
            );
        };
        let _guard = self.mutation_guard();
        let limit = attempt.transfer_limit(requested) as usize;
        let outcome = super::positioned_io::positioned_write(
            self.file(),
            &request.buffer[..limit],
            request.offset,
        );
        match outcome {
            Ok(completed) if attempt.effect_observation_is_indeterminate() => {
                attempt.indeterminate(completed as u64);
                self.failed_write(
                    operation,
                    MediaOperationRole::PositionedWrite,
                    MediaOperationFailureKind::IndeterminateEffect {
                        attempted: super::MediaAttemptedEffect::PositionedWrite {
                            requested_bytes: requested,
                        },
                        last_established: super::MediaEstablishedBoundary::BytePrefix {
                            completed_bytes: completed as u64,
                        },
                    },
                    None,
                )
            }
            Ok(completed) => self.classify_write(
                attempt,
                operation,
                requested,
                completed as u64,
                MediaTransferPosition::PositionedOffset(request.offset),
                MediaOperationRole::PositionedWrite,
            ),
            Err(error) => {
                attempt.denied();
                self.failed_write(
                    operation,
                    MediaOperationRole::PositionedWrite,
                    MediaOperationFailureKind::DeniedBeforeEffect,
                    Some(&error),
                )
            }
        }
    }

    pub fn append(&self, request: AppendRequest<'_>) -> MediaOperationOutcome {
        let operation = self.operation_identity();
        let requested = request.buffer.len() as u64;
        let attempt = self.owner().boundary().begin_operation(
            MediaOperationRole::Append,
            requested,
            super::MediaOperationCoordinates::for_path(
                operation,
                self.role(),
                Some(self.identity()),
            ),
        );
        if requested == 0 {
            attempt.denied();
            return self.failed_write(
                operation,
                MediaOperationRole::Append,
                MediaOperationFailureKind::DeniedBeforeEffect,
                None,
            );
        }
        if let Some(error) = attempt.fail_before_error() {
            attempt.denied();
            return self.failed_write(
                operation,
                MediaOperationRole::Append,
                MediaOperationFailureKind::DeniedBeforeEffect,
                Some(&error),
            );
        }
        let Ok(_authority) = self.owner().begin_mutation() else {
            attempt.denied();
            return self.failed_write(
                operation,
                MediaOperationRole::Append,
                MediaOperationFailureKind::DeniedBeforeEffect,
                None,
            );
        };
        let _guard = self.mutation_guard();
        let start = match self.file().metadata() {
            Ok(metadata) => metadata.len(),
            Err(error) => {
                attempt.denied();
                return self.failed_write(
                    operation,
                    MediaOperationRole::Append,
                    MediaOperationFailureKind::DeniedBeforeEffect,
                    Some(&error),
                );
            }
        };
        let limit = attempt.transfer_limit(requested) as usize;
        let outcome =
            super::positioned_io::positioned_write(self.file(), &request.buffer[..limit], start);
        match outcome {
            Ok(completed) if attempt.effect_observation_is_indeterminate() => {
                attempt.indeterminate(completed as u64);
                self.failed_write(
                    operation,
                    MediaOperationRole::Append,
                    MediaOperationFailureKind::IndeterminateEffect {
                        attempted: super::MediaAttemptedEffect::Append {
                            requested_bytes: requested,
                        },
                        last_established: super::MediaEstablishedBoundary::BytePrefix {
                            completed_bytes: completed as u64,
                        },
                    },
                    None,
                )
            }
            Ok(completed) => self.classify_write(
                attempt,
                operation,
                requested,
                completed as u64,
                MediaTransferPosition::KnownAppendPosition(start),
                MediaOperationRole::Append,
            ),
            Err(error) => {
                attempt.denied();
                self.failed_write(
                    operation,
                    MediaOperationRole::Append,
                    MediaOperationFailureKind::DeniedBeforeEffect,
                    Some(&error),
                )
            }
        }
    }

    fn classify_write(
        &self,
        attempt: super::fault_interposition::MediaBoundaryAttempt<'_>,
        operation: super::MediaOperationIdentity,
        requested: u64,
        completed: u64,
        position: MediaTransferPosition,
        role: MediaOperationRole,
    ) -> MediaOperationOutcome {
        match super::transfer::classify_media_transfer(requested, completed, position) {
            Ok(MediaTransferProgress::Completed(transfer)) => {
                attempt.completed(completed);
                MediaOperationOutcome::completed(
                    operation,
                    if role == MediaOperationRole::Append {
                        CompletedMediaEffect::AppendCompleted(transfer)
                    } else {
                        CompletedMediaEffect::PositionedWriteCompleted(transfer)
                    },
                )
            }
            Ok(MediaTransferProgress::Partial(transfer)) => {
                attempt.partial(transfer.completed_bytes());
                self.failed_write(
                    operation,
                    role,
                    MediaOperationFailureKind::PartialTransfer(transfer),
                    None,
                )
            }
            Ok(MediaTransferProgress::NoProgress) | Err(_) => {
                attempt.denied();
                self.failed_write(
                    operation,
                    role,
                    MediaOperationFailureKind::DeniedBeforeEffect,
                    None,
                )
            }
        }
    }

    fn failed_write(
        &self,
        operation: super::MediaOperationIdentity,
        role: MediaOperationRole,
        kind: MediaOperationFailureKind,
        error: Option<&std::io::Error>,
    ) -> MediaOperationOutcome {
        let failure = super::failure_context::operation_failure(
            operation,
            role,
            self.role(),
            Some(self.identity()),
            kind,
            error,
            super::failure_context::causal_boundary(error),
        );
        MediaOperationOutcome::failed(operation, failure)
    }
}

impl<Access> NamespaceFileHandle<'_, Access> {
    pub(super) fn operation_identity(&self) -> super::MediaOperationIdentity {
        self.owner()
            .issue_operation_identity()
            .expect("media operation identity exhausted")
    }
}
