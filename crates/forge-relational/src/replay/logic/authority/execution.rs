mod envelope_self_audit;
mod surface_audit;

use crate::capabilities::{RuntimeConfigSource, SchemaSource};
use crate::history::data::{BranchId, CommitId, HistoryDriftClass};
use crate::logic::runtime::RelationalRuntime;
use crate::replay::data::{
    CanonicalCommitEnvelope, RelationalReplayOutcome, RelationalReplayRequest,
    ReplayAuthorityBasisKind, ReplayExecutionMode, ReplayFailureClass, ReplayLineageAuthorityBasis,
    ReplayLineageDigestMode, ReplayMismatch, ReplayMismatchClass, ReplayObservableSurface,
    ReplayVerificationLayer, ReplayVerificationMode, ReplayVerificationPlan,
};

use super::super::diagnostics::record_replay_diagnostic;
use super::super::planning::{
    load_replay_envelope, promised_replay_surfaces, replay_commit_closure_by_commit_id_order,
    replay_recovery_plan_for_chain,
};
use super::continuity::validated_replay_continuity_envelope;
use super::lineage_authority::{
    lineage_decision_log_comparison_basis, lineage_event_batch_comparison_basis,
    select_published_lineage_authority,
};
use super::strategy_replay::verify_strategy_reexecution_surface;
use super::ReplayAuthority;

use self::envelope_self_audit::audit_retained_envelope_authority;
use self::surface_audit::compare_replay_surfaces;

impl<'runtime> ReplayAuthority<'runtime> {
    fn fail_and_record(
        &mut self,
        request: RelationalReplayRequest,
        envelope: Option<&CanonicalCommitEnvelope>,
        chain: Option<&[CommitId]>,
        failure: ReplayFailureClass,
        mismatch: Option<ReplayMismatch>,
    ) -> RelationalReplayOutcome {
        let outcome = match mismatch {
            Some(mismatch) => RelationalReplayOutcome::fail(request, envelope, chain, failure)
                .with_mismatch(mismatch),
            None => RelationalReplayOutcome::fail(request, envelope, chain, failure),
        };
        record_replay_diagnostic(self.runtime, &outcome.requested, &outcome);
        outcome
    }

    pub fn replay_commit(&mut self, request: RelationalReplayRequest) -> RelationalReplayOutcome {
        let Some(envelope) = load_replay_envelope(self.runtime, request.commit_id) else {
            return self.fail_and_record(
                request,
                None,
                None,
                ReplayFailureClass::MissingCommit,
                None,
            );
        };
        if envelope.branch_context != request.branch_id {
            return self.fail_and_record(
                request,
                Some(&envelope),
                None,
                ReplayFailureClass::BranchMismatch,
                None,
            );
        }
        if envelope.schema_authority != self.runtime.schema_registry().authority_snapshot() {
            return self.fail_and_record(
                request,
                Some(&envelope),
                None,
                ReplayFailureClass::SchemaMismatch,
                None,
            );
        }

        let commit_closure = match replay_commit_closure_by_commit_id_order(
            self.runtime,
            self.runtime,
            request.commit_id,
        ) {
            Ok(chain) => chain,
            Err(failure) => {
                return self.fail_and_record(request, Some(&envelope), None, failure, None);
            }
        };

        let verification_plan = ReplayVerificationPlan::from_mode(request.verification_mode);
        let validated_envelope =
            match validated_replay_continuity_envelope(self.runtime, &envelope, &verification_plan)
            {
                Ok(validated) => validated,
                Err(mismatch) => {
                    return self.record_continuity_rejection(
                        request,
                        &envelope,
                        &commit_closure,
                        mismatch,
                    )
                }
            };

        let retained_envelope_surfaces =
            promised_replay_surfaces(self.runtime, &envelope, &commit_closure, None);
        let mut retained_envelope_mismatches = Vec::new();
        audit_retained_envelope_authority(
            self.runtime,
            &mut retained_envelope_mismatches,
            &envelope,
            None,
        );
        if !retained_envelope_mismatches.is_empty() {
            let outcome = RelationalReplayOutcome {
                requested: request,
                commit: Some(envelope.commit.clone()),
                reconstructed_commit_closure: commit_closure.clone(),
                snapshot_version: Some(envelope.commit.version_id),
                lineage_authority_basis: None,
                compared_surfaces: retained_envelope_surfaces,
                mismatches: retained_envelope_mismatches,
                failure: Some(ReplayFailureClass::ObservableMismatch),
            };
            record_replay_diagnostic(self.runtime, &outcome.requested, &outcome);
            return outcome;
        }

        let replay_plan = replay_recovery_plan_for_chain(
            self.runtime,
            self.runtime.runtime_config(),
            self.runtime.commit_strategy_executor_registry().clone(),
            &commit_closure,
            request.verification_mode,
        );
        let replay_runtime = match RelationalRuntime::rebuild_runtime_from_plan(replay_plan) {
            Ok(runtime) => runtime,
            Err(_) => {
                return self.fail_and_record(
                    request,
                    Some(&envelope),
                    Some(&commit_closure),
                    ReplayFailureClass::ObservableMismatch,
                    None,
                );
            }
        };

        let Some(replayed_envelope) = replay_runtime
            .replay()
            .canonical_commit_envelope(request.commit_id)
            .cloned()
        else {
            return self.fail_and_record(
                request,
                Some(&envelope),
                Some(&commit_closure),
                ReplayFailureClass::ObservableMismatch,
                Some(ReplayMismatch {
                    class: ReplayMismatchClass::HistoryDrift,
                    history_drift_class: Some(HistoryDriftClass::ReplayAuthorityDrift),
                    surface: ReplayObservableSurface::History,
                    verification_layer: ReplayVerificationLayer::DeepArtifactParity,
                    detail: "replayed target envelope was not reconstructed".to_string(),
                    expected: Some(format!("{:?}", envelope.commit.commit_id)),
                    observed: None,
                }),
            );
        };

        let validated_replayed_envelope = match validated_replay_continuity_envelope(
            self.runtime,
            &replayed_envelope,
            &verification_plan,
        ) {
            Ok(validated) => validated,
            Err(mismatch) => {
                return self.record_continuity_rejection(
                    request,
                    &envelope,
                    &commit_closure,
                    mismatch,
                )
            }
        };

        let compared_surfaces = promised_replay_surfaces(
            self.runtime,
            &envelope,
            &commit_closure,
            Some(&replayed_envelope),
        );
        let mut mismatches = Vec::new();
        if compared_surfaces.contains(&ReplayObservableSurface::Strategy) {
            verify_strategy_reexecution_surface(
                self.runtime,
                &mut mismatches,
                &envelope,
                &commit_closure,
                request.verification_mode,
            );
        }
        let selected_lineage_authority = if compared_surfaces
            .contains(&ReplayObservableSurface::Lineage)
        {
            let selected = select_published_lineage_authority(self.runtime, &envelope);
            self.runtime
                .performance_access()
                .count_replay_lineage_authority_basis(
                    selected.indexed_source,
                    selected.kind,
                    selected.artifact.digest_basis().lineage_event_count(),
                    selected.artifact.digest_basis().lineage_decision_count(),
                );
            if selected.kind == ReplayAuthorityBasisKind::HistoryEnvelopeFallback
                && request.verification_mode != ReplayVerificationMode::NormalRecoveryVerification
            {
                self.runtime
                    .performance_access()
                    .count_replay_lineage_authoritative_basis_rejection();
                return self.fail_and_record(
                    request,
                    Some(&envelope),
                    Some(&commit_closure),
                    ReplayFailureClass::AuthoritativeBasisUnavailable,
                    None,
                );
            }
            Some(selected)
        } else {
            None
        };

        audit_retained_envelope_authority(
            self.runtime,
            &mut mismatches,
            &envelope,
            selected_lineage_authority.as_ref(),
        );

        compare_replay_surfaces(
            self.runtime,
            &verification_plan,
            &mut mismatches,
            &compared_surfaces,
            &envelope,
            &replayed_envelope,
            &validated_envelope,
            &validated_replayed_envelope,
            &replay_runtime,
            &request,
            selected_lineage_authority.as_ref(),
        );

        let outcome = RelationalReplayOutcome {
            requested: request,
            commit: Some(envelope.commit.clone()),
            reconstructed_commit_closure: commit_closure.clone(),
            snapshot_version: Some(envelope.commit.version_id),
            lineage_authority_basis: selected_lineage_authority.as_ref().map(|selected| {
                ReplayLineageAuthorityBasis::new(
                    selected.kind,
                    envelope.commit.commit_id,
                    ReplayLineageDigestMode::ExactCanonicalArtifactDigest,
                    selected.artifact.digest_basis().lineage_event_count(),
                    selected.artifact.digest_basis().lineage_decision_count(),
                    lineage_event_batch_comparison_basis(selected.artifact),
                    lineage_decision_log_comparison_basis(selected.artifact),
                )
            }),
            compared_surfaces: compared_surfaces.clone(),
            mismatches: mismatches.clone(),
            failure: (!mismatches.is_empty()).then_some(ReplayFailureClass::ObservableMismatch),
        };
        record_replay_diagnostic(self.runtime, &outcome.requested, &outcome);
        outcome
    }

    pub fn replay_range(
        &mut self,
        branch_id: BranchId,
        commits: &[CommitId],
        verification_mode: ReplayVerificationMode,
    ) -> Vec<RelationalReplayOutcome> {
        commits
            .iter()
            .copied()
            .map(|commit_id| {
                self.replay_commit(RelationalReplayRequest {
                    commit_id,
                    branch_id: branch_id.clone(),
                    execution_mode: ReplayExecutionMode::SerialDeterministic,
                    verification_mode,
                })
            })
            .collect()
    }

    fn record_continuity_rejection(
        &mut self,
        request: RelationalReplayRequest,
        envelope: &CanonicalCommitEnvelope,
        commit_closure: &[CommitId],
        mismatch: ReplayMismatch,
    ) -> RelationalReplayOutcome {
        if mismatch.class == ReplayMismatchClass::DescriptorVersionDrift {
            self.runtime
                .performance_access()
                .count_descriptor_version_mismatch();
        }
        self.runtime
            .performance_access()
            .count_replay_verification_layer(mismatch.verification_layer);
        self.fail_and_record(
            request,
            Some(envelope),
            Some(commit_closure),
            ReplayFailureClass::ObservableMismatch,
            Some(mismatch),
        )
    }
}
