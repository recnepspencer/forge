use super::{
    MediaOperationFailureKind, MediaOperationRole, MediaTransferPosition, MediaTransferProgress,
    NamespaceFileHandle, PositionedReadOutcome, PositionedReadRequest, PositionedReadResult,
};

impl<Access> NamespaceFileHandle<'_, Access> {
    pub fn positioned_read(&self, request: PositionedReadRequest<'_>) -> PositionedReadOutcome {
        let operation = self.operation_identity();
        let requested = request.buffer.len() as u64;
        let attempt = self.owner().boundary().begin_operation(
            MediaOperationRole::PositionedRead,
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
            return self.failed_read(
                operation,
                MediaOperationFailureKind::DeniedBeforeEffect,
                None,
            );
        }
        if let Some(error) = attempt.fail_before_error() {
            attempt.denied();
            return self.failed_read(
                operation,
                MediaOperationFailureKind::DeniedBeforeEffect,
                Some(&error),
            );
        }
        let limit = attempt.transfer_limit(requested) as usize;
        let outcome = super::positioned_io::positioned_read(
            self.file(),
            &mut request.buffer[..limit],
            request.offset,
        );
        match outcome {
            Ok(0) => {
                self.owner().boundary().counters().eof_observation();
                attempt.completed(0);
                PositionedReadOutcome::new(
                    operation,
                    PositionedReadResult::EndOfFile {
                        requested_offset: request.offset,
                    },
                )
            }
            Ok(completed) => self.classify_read(
                attempt,
                operation,
                requested,
                completed as u64,
                request.offset,
            ),
            Err(error) => {
                attempt.denied();
                self.failed_read(
                    operation,
                    MediaOperationFailureKind::DeniedBeforeEffect,
                    Some(&error),
                )
            }
        }
    }

    fn classify_read(
        &self,
        attempt: super::fault_interposition::MediaBoundaryAttempt<'_>,
        operation: super::MediaOperationIdentity,
        requested: u64,
        completed: u64,
        offset: u64,
    ) -> PositionedReadOutcome {
        match super::transfer::classify_media_transfer(
            requested,
            completed,
            MediaTransferPosition::PositionedOffset(offset),
        ) {
            Ok(MediaTransferProgress::Completed(transfer)) => {
                attempt.completed(completed);
                PositionedReadOutcome::new(operation, PositionedReadResult::Transferred(transfer))
            }
            Ok(MediaTransferProgress::Partial(transfer)) => {
                attempt.partial(transfer.completed_bytes());
                self.failed_read(
                    operation,
                    MediaOperationFailureKind::PartialTransfer(transfer),
                    None,
                )
            }
            Ok(MediaTransferProgress::NoProgress) => {
                attempt.completed(0);
                PositionedReadOutcome::new(
                    operation,
                    PositionedReadResult::EndOfFile {
                        requested_offset: offset,
                    },
                )
            }
            Err(_) => {
                attempt.denied();
                self.failed_read(
                    operation,
                    MediaOperationFailureKind::DeniedBeforeEffect,
                    None,
                )
            }
        }
    }

    fn failed_read(
        &self,
        operation: super::MediaOperationIdentity,
        kind: MediaOperationFailureKind,
        error: Option<&std::io::Error>,
    ) -> PositionedReadOutcome {
        PositionedReadOutcome::new(
            operation,
            PositionedReadResult::Failed(super::failure_context::operation_failure(
                operation,
                MediaOperationRole::PositionedRead,
                self.role(),
                Some(self.identity()),
                kind,
                error,
                super::failure_context::causal_boundary(error),
            )),
        )
    }
}
