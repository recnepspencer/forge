use serde_json::json;

use crate::authority::commit::preparation::diagnostics::failures::PreparationFailureClass;
use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::logic::runtime::{RelationalRuntime, WorkingState};
use crate::publication::data::{PublicationError, PublicationStage};
use crate::publication::logic::publication_failure_diagnostic;
use crate::transactions::data::{CommitConflict, MergedCommitPlan, TransactionCommitError};
use crate::validation::engine::InvariantExecutionResult;

impl RelationalRuntime {
    pub(crate) fn invariant_authority(&mut self) -> InvariantAuthority<'_> {
        InvariantAuthority::new(self)
    }
}

pub(crate) struct InvariantAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl<'runtime> InvariantAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub(crate) fn enforce_commit_boundary(
        &mut self,
        merged_plan: &MergedCommitPlan,
    ) -> Result<InvariantExecutionResult, TransactionCommitError> {
        let result = self.runtime.invariant_access().commit_boundary(merged_plan);
        self.emit_preparation_diagnostics(&result);
        if let Some(failure) = result.summary().blocking_failure() {
            self.emit_conflict_diagnostic(failure.execution_point(), failure.detail().to_string());
            return Err(TransactionCommitError::conflict(
                failure.clone().into_commit_conflict(),
            ));
        }
        Ok(result)
    }

    pub(crate) fn enforce_mutation_sensitive_for_working_state(
        &mut self,
        working_state: &WorkingState,
        version_id: crate::identity::data::VersionId,
        merged_plan: &MergedCommitPlan,
    ) -> Result<InvariantExecutionResult, CommitConflict> {
        let result = {
            let storage = self.runtime.storage_access();
            let overlay_state = storage.overlay_state_view(working_state);
            self.runtime
                .invariant_access()
                .mutation_sensitive_for_state(overlay_state, version_id, Some(merged_plan))
        };
        self.emit_preparation_diagnostics(&result);
        if let Some(failure) = result.summary().blocking_failure() {
            self.emit_conflict_diagnostic(failure.execution_point(), failure.detail().to_string());
            return Err(failure.clone().into_commit_conflict());
        }
        Ok(result)
    }

    pub(crate) fn enforce_snapshot_publication_for_working_state(
        &mut self,
        working_state: &WorkingState,
        version_id: crate::identity::data::VersionId,
        merged_plan: &MergedCommitPlan,
    ) -> Result<InvariantExecutionResult, PublicationError> {
        let result = {
            let storage = self.runtime.storage_access();
            let overlay_state = storage.overlay_state_view(working_state);
            self.runtime
                .invariant_access()
                .snapshot_publication_for_state(overlay_state, version_id, Some(merged_plan))
        };
        self.emit_preparation_diagnostics(&result);
        if let Some(failure) = result.summary().publication_failure() {
            self.emit_publication_failure(failure.detail().to_string());
            return Err(failure
                .clone()
                .into_publication_error(PublicationStage::InvariantCheck));
        }
        Ok(result)
    }

    fn emit_conflict_diagnostic(
        &mut self,
        execution_point: crate::validation::data::InvariantExecutionPoint,
        detail: String,
    ) {
        self.runtime
            .publication_authority()
            .diagnostic(DiagnosticsScope::Invariant)
            .failure()
            .emit_entry(
                DiagnosticCode::InvariantViolation,
                detail,
                json!({ "execution_point": execution_point.diagnostic_label() }),
            );
    }

    fn emit_publication_failure(&mut self, detail: String) {
        self.runtime
            .publication_authority()
            .push_bounded_diagnostic(
                DiagnosticsScope::Invariant,
                DiagnosticsArtifactKind::Failure,
                vec![publication_failure_diagnostic(detail)],
            );
    }

    fn emit_preparation_diagnostics(&mut self, result: &InvariantExecutionResult) {
        let fallback_reason = result
            .metadata()
            .preparation_strategy()
            .and_then(|strategy| strategy.fallback_reason);
        let failures = result.metadata().preparation_failures();
        if fallback_reason.is_none() && failures.is_empty() {
            return;
        }

        let mut entries = Vec::new();
        if let Some(reason) = fallback_reason {
            entries.push(RelationalDiagnosticsEntry {
                code: DiagnosticCode::PreparationFallback,
                message: "preparation fell back to serial execution".to_string(),
                fields: json!({
                    "execution_point": result.metadata().execution_point().diagnostic_label(),
                    "reason": format!("{reason:?}"),
                }),
            });
        }
        for failure in failures {
            entries.push(RelationalDiagnosticsEntry {
                code: DiagnosticCode::PreparationFailure,
                message: "preparation contract reported a structured failure".to_string(),
                fields: json!({
                    "execution_point": result.metadata().execution_point().diagnostic_label(),
                    "failure_class": preparation_failure_label(*failure),
                }),
            });
        }

        self.runtime
            .publication_authority()
            .push_bounded_diagnostic(
                DiagnosticsScope::Invariant,
                DiagnosticsArtifactKind::DetailedTrace,
                entries,
            );
    }
}

fn preparation_failure_label(failure: PreparationFailureClass) -> &'static str {
    match failure {
        PreparationFailureClass::PlanningProofInsufficient => "planning_proof_insufficient",
        PreparationFailureClass::PacketOverlapDetected => "packet_overlap_detected",
        PreparationFailureClass::ReductionIdentityConflict => "reduction_identity_conflict",
        PreparationFailureClass::FallbackToSerial => "fallback_to_serial",
        PreparationFailureClass::WorkerEvaluationFailure => "worker_evaluation_failure",
        PreparationFailureClass::FragmentCanonicalizationFailure => {
            "fragment_canonicalization_failure"
        }
        PreparationFailureClass::PublicationIsolationViolation => "publication_isolation_violation",
        PreparationFailureClass::ConsumerFailureNonAuthoritative => {
            "consumer_failure_non_authoritative"
        }
    }
}
