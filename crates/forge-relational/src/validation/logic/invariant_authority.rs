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
use serde_json::Value;

impl RelationalRuntime {
    pub(crate) fn invariant_authority(&mut self) -> InvariantAuthority<'_> {
        InvariantAuthority::new(self)
    }

    pub fn certify_current_state(&mut self) -> Result<InvariantExecutionResult, PublicationError> {
        self.invariant_authority().enforce_certification_boundary()
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
        let collect_all = self.emit_collect_all_failure_diagnostics(&result);
        if let Some(failure) = result.summary().blocking_failure() {
            if !collect_all {
                self.emit_conflict_diagnostic(&result, failure);
            }
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
        let collect_all = self.emit_collect_all_failure_diagnostics(&result);
        if let Some(failure) = result.summary().blocking_failure() {
            if !collect_all {
                self.emit_conflict_diagnostic(&result, failure);
            }
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
        let collect_all = self.emit_collect_all_failure_diagnostics(&result);
        if let Some(failure) = result.summary().publication_failure() {
            if !collect_all {
                self.emit_publication_failure(&result, failure);
            }
            return Err(failure
                .clone()
                .into_publication_error(PublicationStage::InvariantCheck));
        }
        Ok(result)
    }

    pub(crate) fn enforce_certification_boundary(
        &mut self,
    ) -> Result<InvariantExecutionResult, PublicationError> {
        let result = self.runtime.invariant_access().certification_state();
        self.emit_preparation_diagnostics(&result);
        let collect_all = self.emit_collect_all_failure_diagnostics(&result);
        if let Some(failure) = result.summary().publication_failure() {
            if !collect_all {
                self.emit_publication_failure(&result, failure);
            }
            return Err(
                failure
                    .clone()
                    .into_publication_error(PublicationStage::InvariantCheck),
            );
        }
        Ok(result)
    }

    fn emit_conflict_diagnostic(
        &mut self,
        result: &InvariantExecutionResult,
        failure: &crate::validation::engine::InvariantFailure,
    ) {
        self.runtime
            .publication_authority()
            .diagnostic(DiagnosticsScope::Invariant)
            .failure()
            .emit_entry(
                failure.code(),
                failure.detail().to_string(),
                invariant_failure_fields(result, failure),
            );
    }

    fn emit_collect_all_failure_diagnostics(&mut self, result: &InvariantExecutionResult) -> bool {
        if !self
            .runtime
            .config
            .diagnostics
            .profile
            .collect_all_invariant_failures
        {
            return false;
        }

        let mut entries = Vec::new();
        for failure in result.blocking_failures() {
            entries.push(publication_failure_diagnostic(
                failure.code(),
                failure.detail().to_string(),
                invariant_failure_fields(result, &failure),
            ));
        }
        for failure in result.publication_failures() {
            entries.push(publication_failure_diagnostic(
                failure.code(),
                failure.detail().to_string(),
                invariant_failure_fields(result, &failure),
            ));
        }
        if entries.is_empty() {
            return false;
        }
        self.runtime
            .publication_authority()
            .push_bounded_diagnostic(
                DiagnosticsScope::Invariant,
                DiagnosticsArtifactKind::Failure,
                entries,
            );
        true
    }

    fn emit_publication_failure(
        &mut self,
        result: &InvariantExecutionResult,
        failure: &crate::validation::engine::InvariantFailure,
    ) {
        self.runtime
            .publication_authority()
            .push_bounded_diagnostic(
                DiagnosticsScope::Invariant,
                DiagnosticsArtifactKind::Failure,
                vec![publication_failure_diagnostic(
                    failure.code(),
                    failure.detail().to_string(),
                    invariant_failure_fields(result, failure),
                )],
            );
    }

    fn emit_preparation_diagnostics(&mut self, result: &InvariantExecutionResult) {
        if self
            .runtime
            .config
            .diagnostics
            .profile
            .detailed_traces_enabled
        {
            if let Some(proof_boundary) = result.metadata().proof_boundary() {
                self.runtime
                    .publication_authority()
                    .push_bounded_diagnostic(
                        DiagnosticsScope::Invariant,
                        DiagnosticsArtifactKind::DetailedTrace,
                        vec![RelationalDiagnosticsEntry {
                            code: DiagnosticCode::InvariantProofBoundaryObserved,
                            message: "invariant execution preserved an explicit planner/executor proof boundary"
                                .to_string(),
                            fields: json!({
                                "execution_point": result.metadata().execution_point().diagnostic_label(),
                                "scope_class": format!("{:?}", proof_boundary.scope_class()),
                                "widened_causes": proof_boundary.widened_causes().iter().map(|cause| format!("{cause:?}")).collect::<Vec<_>>(),
                                "packet_count": proof_boundary.packet_count(),
                                "touched_partition_count": proof_boundary.touched_partition_count(),
                            }),
                        }],
                    );
            }
            let custom_trace_entries = result
                .results()
                .iter()
                .filter_map(custom_invariant_trace_entry)
                .collect::<Vec<_>>();
            if !custom_trace_entries.is_empty() {
                self.runtime
                    .publication_authority()
                    .push_bounded_diagnostic(
                        DiagnosticsScope::Invariant,
                        DiagnosticsArtifactKind::DetailedTrace,
                        custom_trace_entries,
                    );
            }
        }
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

fn invariant_failure_fields(
    result: &InvariantExecutionResult,
    failure: &crate::validation::engine::InvariantFailure,
) -> Value {
    let proof_boundary = result.metadata().proof_boundary().map(|proof_boundary| {
        json!({
            "scope_class": format!("{:?}", proof_boundary.scope_class()),
            "widened_causes": proof_boundary.widened_causes().iter().map(|cause| format!("{cause:?}")).collect::<Vec<_>>(),
            "packet_count": proof_boundary.packet_count(),
            "touched_partition_count": proof_boundary.touched_partition_count(),
        })
    });

    json!({
        "execution_point": failure.execution_point().diagnostic_label(),
        "proof_boundary": proof_boundary,
        "violation": failure.fields(),
        "custom_provenance": matching_custom_provenance(result, failure),
    })
}

fn matching_custom_provenance(
    result: &InvariantExecutionResult,
    failure: &crate::validation::engine::InvariantFailure,
) -> Value {
    result
        .results()
        .iter()
        .find_map(|result| match &result.verdict {
            crate::validation::data::InvariantVerdict::Violation(violation)
                if *violation == *failure.violation() =>
            {
                result
                    .custom_provenance()
                    .and_then(|provenance| serde_json::to_value(provenance).ok())
            }
            _ => None,
        })
        .unwrap_or(Value::Null)
}

fn custom_invariant_trace_entry(
    result: &crate::validation::data::InvariantCheckResult,
) -> Option<RelationalDiagnosticsEntry> {
    let provenance = result.custom_provenance()?;
    let crate::validation::data::InvariantReportedRule::Custom(identity) = &result.rule else {
        return None;
    };
    Some(RelationalDiagnosticsEntry {
        code: DiagnosticCode::InvariantProofBoundaryObserved,
        message: "custom invariant structural provenance captured for deterministic debugging"
            .to_string(),
        fields: json!({
            "rule_id": identity.rule_id.as_str(),
            "semantic_version_major": identity.semantic_version.major,
            "semantic_version_minor": identity.semantic_version.minor,
            "execution_point": result.execution_point.diagnostic_label(),
            "verdict": format!("{:?}", result.verdict),
            "provenance": provenance,
        }),
    })
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
