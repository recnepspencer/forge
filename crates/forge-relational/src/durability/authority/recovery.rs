use crate::capabilities::{RuntimeIdentitySource, SchemaSource, SchemaVersionSource};
use crate::diagnostics::data::{DiagnosticsArtifactKind, DiagnosticsScope};
use crate::durability::data::{
    DurabilityError, RecoveryCompatibilityMismatch, RecoveryCoverage, RecoveryFailureClass,
    RecoveryPlan,
};
use crate::logic::runtime::{RecoveryOutcome as RuntimeRecoveryOutcome, RelationalRuntime};

use super::compatibility::{
    record_recovery_verification_counters, recovery_compatibility_diagnostic,
    validate_recovery_compatibility,
};
use super::diagnostics::{recovery_checkpoint_selected, recovery_range_replayed};
use super::runtime_rebuild::rebuild_runtime_from_plan;
use super::DurabilityAuthority;

impl<'runtime> DurabilityAuthority<'runtime> {
    pub fn recover(
        &mut self,
        plan: RecoveryPlan,
    ) -> Result<RuntimeRecoveryOutcome, DurabilityError> {
        let compatibility_entry = recovery_compatibility_diagnostic(&plan);
        let compatibility_artifact_kind = match &plan.compatibility.verification_outcome {
            crate::durability::data::RecoveryVerificationOutcome::VerifiedAtLayer(_) => {
                DiagnosticsArtifactKind::MinimalSummary
            }
            crate::durability::data::RecoveryVerificationOutcome::Rejected { .. } => {
                DiagnosticsArtifactKind::Failure
            }
        };
        self.runtime
            .publication_authority()
            .push_bounded_diagnostic(
                DiagnosticsScope::History,
                compatibility_artifact_kind,
                vec![compatibility_entry.clone()],
            );
        if matches!(
            plan.compatibility.verification_outcome,
            crate::durability::data::RecoveryVerificationOutcome::Rejected { .. }
        ) {
            record_recovery_verification_counters(self.runtime, &plan.compatibility);
        }
        validate_recovery_compatibility(self.runtime, &plan)?;
        if !plan.compatibility.schema_parity.is_verified() {
            return Err(DurabilityError::new(
                RecoveryFailureClass::SchemaMismatch,
                "recovery schema registry mismatch",
            )
            .with_compatibility_mismatch(
                RecoveryCompatibilityMismatch::SchemaRegistryShape {
                    expected_primary_schema_version: plan.config.primary_schema_version_id(),
                    found_primary_schema_version: self.runtime.primary_schema_version_id(),
                    expected_entity_kind_count: plan.config.schema.registry.entity_kinds.len(),
                    found_entity_kind_count: self.runtime.schema_registry().entity_kinds.len(),
                    expected_relation_kind_count: plan.config.schema.registry.relation_kinds.len(),
                    found_relation_kind_count: self.runtime.schema_registry().relation_kinds.len(),
                },
            ));
        }
        if !plan.compatibility.profile_parity.is_verified() {
            return Err(DurabilityError::new(
                RecoveryFailureClass::ProfileMismatch,
                "recovery profile mismatch",
            )
            .with_compatibility_mismatch(
                RecoveryCompatibilityMismatch::RuntimeProfile {
                    expected: format!("{:?}", plan.config.profile),
                    found: format!("{:?}", self.runtime.runtime_profile()),
                },
            ));
        }
        if !plan.compatibility.runtime_name_parity.is_verified() {
            return Err(DurabilityError::new(
                RecoveryFailureClass::RuntimeNameMismatch,
                "recovery runtime name mismatch",
            )
            .with_compatibility_mismatch(RecoveryCompatibilityMismatch::RuntimeName {
                expected: plan.config.execution.runtime_name.clone(),
                found: self.runtime.runtime_name().to_string(),
            }));
        }
        if plan.integrity_report.corrupt_segment_id.is_some() {
            return Err(DurabilityError::new(
                RecoveryFailureClass::CorruptSegment,
                "required durable segment is corrupt",
            ));
        }

        let tail_commits = plan.tail_log.len();
        let checkpoint_commits = plan
            .checkpoint
            .as_ref()
            .map(|checkpoint| checkpoint.envelopes.len())
            .unwrap_or(0);
        let mut restored = rebuild_runtime_from_plan(plan.clone())?;
        restored.durability.set_log(plan.tail_log);
        restored.durability.store = plan.store.clone();
        record_recovery_verification_counters(&restored, &plan.compatibility);
        restored.publication_authority().push_bounded_diagnostic(
            DiagnosticsScope::History,
            compatibility_artifact_kind,
            vec![compatibility_entry],
        );
        restored.publication_authority().push_bounded_diagnostic(
            DiagnosticsScope::History,
            DiagnosticsArtifactKind::MinimalSummary,
            vec![
                recovery_checkpoint_selected(
                    plan.cursor.checkpoint_id,
                    &plan.integrity_report.skipped_corrupt_checkpoints,
                ),
                recovery_range_replayed(&plan.cursor.segment_ids, tail_commits),
            ],
        );
        let outcome = RuntimeRecoveryOutcome {
            recovered_commits: restored.history.commit_envelopes.len(),
            latest_commit: restored.history().latest_commit().cloned(),
            restored_branches: restored.history.branch_heads.len(),
            cursor: plan.cursor,
            coverage: RecoveryCoverage {
                checkpoint_commits,
                replayed_tail_commits: tail_commits,
                recovered_through_commit: restored.history().latest_commit().cloned(),
            },
            integrity_report: plan.integrity_report,
        };
        *self.runtime = restored;
        Ok(outcome)
    }
}

impl RelationalRuntime {
    pub(crate) fn rebuild_runtime_from_plan(
        plan: RecoveryPlan,
    ) -> Result<RelationalRuntime, DurabilityError> {
        rebuild_runtime_from_plan(plan)
    }
}
