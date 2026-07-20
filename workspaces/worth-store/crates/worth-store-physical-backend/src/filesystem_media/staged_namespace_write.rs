use super::{
    CompletedStagedNamespaceWrite, MediaCausalBoundary, MediaOperationFailureKind,
    MediaOperationResult, PositionedWriteRequest, StagedNamespaceFile, StagedNamespaceWriteOutcome,
};

impl<'owner> StagedNamespaceFile<'owner> {
    pub fn write_all(self, bytes: &[u8]) -> StagedNamespaceWriteOutcome<'owner> {
        if bytes.is_empty() {
            let operation = self
                .owner
                .issue_operation_identity()
                .expect("media operation identity exhausted");
            let failure = super::failure_context::operation_failure(
                operation,
                super::MediaOperationRole::PositionedWrite,
                self.path.role(),
                Some(self.handle.identity()),
                MediaOperationFailureKind::DeniedBeforeEffect,
                None,
                MediaCausalBoundary::BeforeOsCall,
            );
            return StagedNamespaceWriteOutcome::Failed {
                staged: self,
                completed_bytes: 0,
                failure,
            };
        }
        let mut completed = 0_usize;
        let mut first_operation = None;
        let mut last_operation = None;
        let mut primitive_attempts = 0_u64;
        while completed < bytes.len() {
            let outcome = self.handle.positioned_write(PositionedWriteRequest::new(
                completed as u64,
                &bytes[completed..],
            ));
            first_operation.get_or_insert(outcome.operation());
            last_operation = Some(outcome.operation());
            primitive_attempts = primitive_attempts
                .checked_add(1)
                .expect("publication primitive-attempt count exhausted");
            match outcome.result() {
                MediaOperationResult::Completed(
                    super::CompletedMediaEffect::PositionedWriteCompleted(transfer),
                ) => completed += transfer.bytes() as usize,
                MediaOperationResult::Failed(failure) => {
                    if let MediaOperationFailureKind::PartialTransfer(transfer) = failure.kind() {
                        completed += transfer.completed_bytes() as usize;
                        self.owner.boundary().counters().retry_attempt();
                        continue;
                    }
                    return StagedNamespaceWriteOutcome::Failed {
                        staged: self,
                        completed_bytes: completed as u64,
                        failure,
                    };
                }
                _ => unreachable!("positioned write returned an unrelated completed effect"),
            }
        }
        StagedNamespaceWriteOutcome::Completed(CompletedStagedNamespaceWrite {
            staged: self,
            write: super::PublicationWriteSummary::new(
                first_operation.expect("nonempty staged write has an operation"),
                last_operation.expect("nonempty staged write has an operation"),
                primitive_attempts,
                completed as u64,
            ),
        })
    }
}
