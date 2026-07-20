use super::{
    CompletedMediaEffect, MediaAttemptedEffect, MediaCausalBoundary, MediaEstablishedBoundary,
    MediaOperationFailureKind, MediaOperationOutcome, MediaOperationRole, NamespaceFileHandle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TruncateRequest {
    logical_length: u64,
}

impl TruncateRequest {
    pub const fn new(logical_length: u64) -> Self {
        Self { logical_length }
    }

    pub const fn logical_length(self) -> u64 {
        self.logical_length
    }
}

impl NamespaceFileHandle<'_, super::MutableFileAccess> {
    pub fn truncate(&self, request: TruncateRequest) -> MediaOperationOutcome {
        let operation = self
            .owner()
            .issue_operation_identity()
            .expect("media operation identity exhausted");
        let attempt = self.owner().boundary().begin_operation(
            MediaOperationRole::Truncate,
            0,
            super::MediaOperationCoordinates::for_path(
                operation,
                self.role(),
                Some(self.identity()),
            )
            .at_offset(request.logical_length()),
        );
        if let Some(error) = attempt.fail_before_error() {
            attempt.denied();
            return self.truncate_failure(
                operation,
                MediaOperationFailureKind::DeniedBeforeEffect,
                Some(&error),
                MediaCausalBoundary::BeforeOsCall,
            );
        }
        let Ok(_authority) = self.owner().begin_mutation() else {
            attempt.denied();
            return self.truncate_failure(
                operation,
                MediaOperationFailureKind::DeniedBeforeEffect,
                None,
                MediaCausalBoundary::BeforeOsCall,
            );
        };
        let _guard = self.mutation_guard();
        match self.file().set_len(request.logical_length()) {
            Ok(()) if attempt.effect_observation_is_indeterminate() => {
                attempt.indeterminate(0);
                self.truncate_failure(
                    operation,
                    MediaOperationFailureKind::IndeterminateEffect {
                        attempted: MediaAttemptedEffect::LogicalLengthChange,
                        last_established: MediaEstablishedBoundary::LogicalLengthChangeIssued,
                    },
                    None,
                    MediaCausalBoundary::CompletionUnconfirmed,
                )
            }
            Ok(()) => {
                attempt.completed(0);
                MediaOperationOutcome::completed(
                    operation,
                    CompletedMediaEffect::LogicalLengthChanged,
                )
            }
            Err(error) => {
                attempt.indeterminate(0);
                self.truncate_failure(
                    operation,
                    MediaOperationFailureKind::IndeterminateEffect {
                        attempted: MediaAttemptedEffect::LogicalLengthChange,
                        last_established: MediaEstablishedBoundary::LogicalLengthChangeIssued,
                    },
                    Some(&error),
                    MediaCausalBoundary::CompletionUnconfirmed,
                )
            }
        }
    }

    fn truncate_failure(
        &self,
        operation: super::MediaOperationIdentity,
        kind: MediaOperationFailureKind,
        error: Option<&std::io::Error>,
        boundary: MediaCausalBoundary,
    ) -> MediaOperationOutcome {
        let failure = super::failure_context::operation_failure(
            operation,
            MediaOperationRole::Truncate,
            self.role(),
            Some(self.identity()),
            kind,
            error,
            boundary,
        );
        MediaOperationOutcome::failed(operation, failure)
    }
}
