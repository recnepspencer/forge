use crate::capabilities::{HistorySource, ReplayRead, RuntimeConfigSource, SchemaSource};
use crate::performance::logic::ReplayLineageAuthorityIndexedSource;
use crate::history::data::{BranchId, CommitId};
use crate::lineage::data::{CorrespondenceCandidateId, CorrespondencePromotionRejectionClass, PublishedLineageArtifact};
use crate::logic::runtime::RelationalRuntime;
use crate::replay::data::{
    CanonicalCommitEnvelope, CertifiedLineageSurfaceComparisonBasis,
    CertifiedLineageSurfaceDigest, DescriptorAuthorityKind, DescriptorComparisonBasis,
    DescriptorParityCheck, LineageCertifiedSurfaceKind, RelationalReplayOutcome,
    RelationalReplayRequest, ReplayAuthorityBasisKind, ReplayExecutionMode, ReplayFailureClass,
    ReplayLineageAuthorityBasis, ReplayLineageDigestMode, ReplayMismatch, ReplayMismatchClass,
    ReplayObservableSurface, ReplaySurfaceAuthorityKind, ReplaySurfaceComparisonBasis,
    ReplaySurfaceParityCheck, ReplayVerificationLayer, ReplayVerificationMode,
    ReplayVerificationPlan, VerifiedDescriptorDigest, VerifiedReplaySurfaceDigest,
};
use crate::schema::logic::{validate_schema_continuity_bundle, ValidatedSchemaContinuityBundle};

use super::diagnostics::record_replay_diagnostic;
use super::planning::{
    load_replay_envelope, promised_replay_surfaces, replay_chain, replay_recovery_plan_for_chain,
};

pub struct ReplayAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

struct ValidatedReplayContinuityEnvelope<'a> {
    _validated_bundle: ValidatedSchemaContinuityBundle<'a>,
    transition_basis: Option<DescriptorComparisonBasis>,
    continuation_basis: Option<DescriptorComparisonBasis>,
    reconciliation_basis: Option<DescriptorComparisonBasis>,
    lineage_basis: Option<DescriptorComparisonBasis>,
}

struct SelectedPublishedLineageAuthority<'a> {
    kind: ReplayAuthorityBasisKind,
    indexed_source: Option<ReplayLineageAuthorityIndexedSource>,
    artifact: &'a crate::lineage::data::PublishedLineageArtifact,
}

impl<'runtime> ReplayAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    fn fail_and_record(
        &mut self,
        request: RelationalReplayRequest,
        envelope: Option<&CanonicalCommitEnvelope>,
        chain: Option<&[CommitId]>,
        failure: ReplayFailureClass,
        mismatch: Option<ReplayMismatch>,
    ) -> RelationalReplayOutcome {
        let outcome = match mismatch {
            Some(mismatch) => {
                RelationalReplayOutcome::fail(request, envelope, chain, failure).with_mismatch(mismatch)
            }
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
        if envelope.schema_registry != *self.runtime.schema_registry() {
            return self.fail_and_record(
                request,
                Some(&envelope),
                None,
                ReplayFailureClass::SchemaMismatch,
                None,
            );
        }

        let chain = match replay_chain(self.runtime, request.commit_id) {
            Ok(chain) => chain,
            Err(failure) => {
                return self.fail_and_record(request, Some(&envelope), None, failure, None);
            }
        };

        let replay_plan = replay_recovery_plan_for_chain(
            self.runtime,
            self.runtime.runtime_config(),
            &chain,
            request.verification_mode,
        );
        let replay_runtime = match RelationalRuntime::rebuild_runtime_from_plan(replay_plan) {
            Ok(runtime) => runtime,
            Err(_) => {
                return self.fail_and_record(
                    request,
                    Some(&envelope),
                    Some(&chain),
                    ReplayFailureClass::ObservableMismatch,
                    None,
                );
            }
        };

        let Some(replayed_envelope) = replay_runtime
            .replay_access()
            .canonical_commit_envelope(request.commit_id)
            .cloned()
        else {
            return self.fail_and_record(
                request,
                Some(&envelope),
                Some(&chain),
                ReplayFailureClass::ObservableMismatch,
                Some(ReplayMismatch {
                    class: ReplayMismatchClass::HistoryDrift,
                    surface: ReplayObservableSurface::History,
                    verification_layer: ReplayVerificationLayer::DeepArtifactParity,
                    detail: "replayed target envelope was not reconstructed".to_string(),
                    expected: Some(format!("{:?}", envelope.commit.commit_id)),
                    observed: None,
                }),
            );
        };
        let verification_plan = ReplayVerificationPlan::from_mode(request.verification_mode);
        let validated_envelope =
            match validated_replay_continuity_envelope(self.runtime, &envelope, &verification_plan) {
            Ok(validated) => validated,
            Err(mismatch) => {
                if mismatch.class == ReplayMismatchClass::DescriptorVersionDrift {
                    self.runtime
                        .performance_access()
                        .count_descriptor_version_mismatch();
                }
                self.runtime
                    .performance_access()
                    .count_replay_verification_layer(mismatch.verification_layer);
                return self.fail_and_record(
                    request,
                    Some(&envelope),
                    Some(&chain),
                    ReplayFailureClass::ObservableMismatch,
                    Some(mismatch),
                )
                ;
            }
        };
        let validated_replayed_envelope =
            match validated_replay_continuity_envelope(
                self.runtime,
                &replayed_envelope,
                &verification_plan,
            ) {
                Ok(validated) => validated,
                Err(mismatch) => {
                    if mismatch.class == ReplayMismatchClass::DescriptorVersionDrift {
                        self.runtime
                            .performance_access()
                            .count_descriptor_version_mismatch();
                    }
                    self.runtime
                        .performance_access()
                        .count_replay_verification_layer(mismatch.verification_layer);
                    return self.fail_and_record(
                        request,
                        Some(&envelope),
                        Some(&chain),
                        ReplayFailureClass::ObservableMismatch,
                        Some(mismatch),
                    )
                    ;
                }
            };
        let compared_surfaces = promised_replay_surfaces(&envelope);
        let mut mismatches = Vec::new();
        let selected_lineage_authority =
            if compared_surfaces.contains(&ReplayObservableSurface::Lineage) {
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
                        Some(&chain),
                        ReplayFailureClass::AuthoritativeBasisUnavailable,
                        None,
                    );
                }
                Some(selected)
            } else {
                None
            };

        compare_replay_surface(
            self.runtime,
            &verification_plan,
            &mut mismatches,
            ReplayObservableSurface::Patch,
            ReplayMismatchClass::PatchDrift,
            surface_basis_for_patch(&envelope),
            surface_basis_for_patch(&replayed_envelope),
            "canonical patch artifact differed",
            || envelope.patch.canonicalized() == replayed_envelope.patch.canonicalized(),
            || format!("{:?}", envelope.patch),
            || format!("{:?}", replayed_envelope.patch),
        );
        compare_replay_surface(
            self.runtime,
            &verification_plan,
            &mut mismatches,
            ReplayObservableSurface::Diagnostics,
            ReplayMismatchClass::DiagnosticsDrift,
            surface_basis_for_diagnostics(&envelope),
            surface_basis_for_diagnostics(&replayed_envelope),
            "diagnostics summary differed",
            || envelope.diagnostics_summary == replayed_envelope.diagnostics_summary,
            || format!("{:?}", envelope.diagnostics_summary),
            || format!("{:?}", replayed_envelope.diagnostics_summary),
        );
        compare_replay_surface(
            self.runtime,
            &verification_plan,
            &mut mismatches,
            ReplayObservableSurface::History,
            ReplayMismatchClass::HistoryDrift,
            surface_basis_for_history(&envelope),
            surface_basis_for_history(&replayed_envelope),
            "history parent ordering differed",
            || {
                envelope.commit.parents == replayed_envelope.commit.parents
                    && envelope.merge_parent_branches == replayed_envelope.merge_parent_branches
                    && envelope.merge_base_commits == replayed_envelope.merge_base_commits
            },
            || {
                format!(
                    "{:?}|{:?}|{:?}",
                    envelope.commit.parents,
                    envelope.merge_parent_branches,
                    envelope.merge_base_commits
                )
            },
            || {
                format!(
                    "{:?}|{:?}|{:?}",
                    replayed_envelope.commit.parents,
                    replayed_envelope.merge_parent_branches,
                    replayed_envelope.merge_base_commits
                )
            },
        );
        if replayed_envelope.descriptor_semantics_version != envelope.descriptor_semantics_version {
            self.runtime
                .performance_access()
                .count_descriptor_version_mismatch();
            self.runtime
                .performance_access()
                .count_replay_verification_layer(ReplayVerificationLayer::DigestParity);
            mismatches.push(ReplayMismatch {
                class: ReplayMismatchClass::DescriptorVersionDrift,
                surface: ReplayObservableSurface::History,
                verification_layer: ReplayVerificationLayer::DigestParity,
                detail: "descriptor semantics version differed".to_string(),
                expected: Some(format!("{:?}", envelope.descriptor_semantics_version)),
                observed: Some(format!("{:?}", replayed_envelope.descriptor_semantics_version)),
            });
        }
        compare_descriptor_surface(
            self.runtime,
            &verification_plan,
            &mut mismatches,
            validated_envelope.transition_basis.clone(),
            validated_replayed_envelope.transition_basis.clone(),
            ReplayMismatchClass::SchemaTransitionDrift,
            "schema transition artifact differed",
            || envelope.schema_transition == replayed_envelope.schema_transition,
            || format!("{:?}", envelope.schema_transition),
            || format!("{:?}", replayed_envelope.schema_transition),
        );
        compare_descriptor_surface(
            self.runtime,
            &verification_plan,
            &mut mismatches,
            validated_envelope.continuation_basis.clone(),
            validated_replayed_envelope.continuation_basis.clone(),
            ReplayMismatchClass::SchemaContinuationDescriptorDrift,
            "schema continuation descriptor differed",
            || {
                envelope.schema_continuation_descriptor
                    == replayed_envelope.schema_continuation_descriptor
            },
            || format!("{:?}", envelope.schema_continuation_descriptor),
            || format!("{:?}", replayed_envelope.schema_continuation_descriptor),
        );
        compare_descriptor_surface(
            self.runtime,
            &verification_plan,
            &mut mismatches,
            validated_envelope.reconciliation_basis.clone(),
            validated_replayed_envelope.reconciliation_basis.clone(),
            ReplayMismatchClass::SchemaReconciliationDescriptorDrift,
            "schema reconciliation descriptor differed",
            || {
                envelope.schema_reconciliation_descriptor
                    == replayed_envelope.schema_reconciliation_descriptor
            },
            || format!("{:?}", envelope.schema_reconciliation_descriptor),
            || format!("{:?}", replayed_envelope.schema_reconciliation_descriptor),
        );
        compare_descriptor_surface(
            self.runtime,
            &verification_plan,
            &mut mismatches,
            validated_envelope.lineage_basis.clone(),
            validated_replayed_envelope.lineage_basis.clone(),
            ReplayMismatchClass::SchemaLineageDrift,
            "schema lineage artifact differed",
            || {
                envelope
                    .schema_reconciliation_descriptor
                    .as_ref()
                    .map(|descriptor| &descriptor.resulting_lineage)
                    == replayed_envelope
                        .schema_reconciliation_descriptor
                        .as_ref()
                        .map(|descriptor| &descriptor.resulting_lineage)
            },
            || {
                format!(
                    "{:?}",
                    envelope
                        .schema_reconciliation_descriptor
                        .as_ref()
                        .map(|descriptor| &descriptor.resulting_lineage)
                )
            },
            || {
                format!(
                    "{:?}",
                    replayed_envelope
                        .schema_reconciliation_descriptor
                        .as_ref()
                        .map(|descriptor| &descriptor.resulting_lineage)
                )
            },
        );
        if compared_surfaces.contains(&ReplayObservableSurface::Snapshot) {
            let original_surface = self
                .runtime
                .replay_snapshot_surface_at_version(envelope.commit.version_id);
            let replayed_surface = replay_runtime
                .replay_snapshot_surface_at_version(replayed_envelope.commit.version_id);
            compare_replay_surface(
                self.runtime,
                &verification_plan,
                &mut mismatches,
                ReplayObservableSurface::Snapshot,
                ReplayMismatchClass::SnapshotDrift,
                surface_basis_for_snapshot(&original_surface),
                surface_basis_for_snapshot(&replayed_surface),
                "snapshot-visible state differed",
                || original_surface == replayed_surface,
                || format!("{:?}", original_surface),
                || format!("{:?}", replayed_surface),
            );
        }
        let original_branch_head = Some(&envelope.commit);
        let replayed_branch_head = replay_runtime.branch_head_ref(&request.branch_id);
        compare_replay_surface(
            self.runtime,
            &verification_plan,
            &mut mismatches,
            ReplayObservableSurface::BranchHead,
            ReplayMismatchClass::BranchHeadDrift,
            surface_basis_for_branch_head(original_branch_head),
            surface_basis_for_branch_head(replayed_branch_head),
            "branch head movement differed",
            || replayed_branch_head == original_branch_head,
            || format!("{:?}", original_branch_head),
            || format!("{:?}", replayed_branch_head),
        );
        if compared_surfaces.contains(&ReplayObservableSurface::Lineage) {
            let Some(original_lineage) = selected_lineage_authority.as_ref() else {
                return self.fail_and_record(
                    request,
                    Some(&envelope),
                    Some(&chain),
                    ReplayFailureClass::ObservableMismatch,
                    Some(ReplayMismatch {
                        class: ReplayMismatchClass::LineageDrift,
                        surface: ReplayObservableSurface::Lineage,
                        verification_layer: ReplayVerificationLayer::DeepArtifactParity,
                        detail: "lineage replay parity promised a lineage surface without an authoritative lineage basis".to_string(),
                        expected: Some("authoritative lineage basis".to_string()),
                        observed: None,
                    }),
                );
            };
            let replayed_lineage = replayed_envelope.published_lineage();
            compare_replay_surface(
                self.runtime,
                &verification_plan,
                &mut mismatches,
                ReplayObservableSurface::Lineage,
                ReplayMismatchClass::LineageDrift,
                surface_basis_for_published_lineage(original_lineage.artifact),
                surface_basis_for_published_lineage(replayed_lineage),
                "lineage artifacts differed",
                || published_lineage_artifacts_match(original_lineage.artifact, replayed_lineage),
                || format!("{:?}", original_lineage.artifact),
                || format!("{:?}", replayed_lineage),
            );
        }
        if compared_surfaces.contains(&ReplayObservableSurface::DerivedIndexes) {
            let original_index_generations = self
                .runtime
                .index_generations_at_version(envelope.commit.version_id);
            let replayed_index_generations =
                replay_runtime.index_generations_at_version(envelope.commit.version_id);
            compare_replay_surface(
                self.runtime,
                &verification_plan,
                &mut mismatches,
                ReplayObservableSurface::DerivedIndexes,
                ReplayMismatchClass::DerivedIndexDrift,
                surface_basis_for_derived_indexes(&original_index_generations),
                surface_basis_for_derived_indexes(&replayed_index_generations),
                "derived index artifacts differed",
                || replayed_index_generations == original_index_generations,
                || format!("{:?}", original_index_generations),
                || format!("{:?}", replayed_index_generations),
            );
        }

        let outcome = RelationalReplayOutcome {
            requested: request,
            commit: Some(envelope.commit.clone()),
            reconstructed_parent_chain: chain.clone(),
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
}

fn compare_replay_surface(
    runtime: &RelationalRuntime,
    verification_plan: &ReplayVerificationPlan,
    mismatches: &mut Vec<ReplayMismatch>,
    surface: ReplayObservableSurface,
    mismatch_class: ReplayMismatchClass,
    expected: ReplaySurfaceComparisonBasis,
    observed: ReplaySurfaceComparisonBasis,
    detail: &str,
    deep_matches: impl FnOnce() -> bool,
    render_expected: impl FnOnce() -> String,
    render_observed: impl FnOnce() -> String,
) {
    let parity = expected.compare(&observed, mismatch_class, detail);
    match parity {
        ReplaySurfaceParityCheck::ExactDigestMatch { .. } => {
            runtime
                .performance_access()
                .count_replay_verification_layer(ReplayVerificationLayer::DigestParity);
        }
        ReplaySurfaceParityCheck::SummaryMatchDigestUnavailable { .. } => {
            runtime
                .performance_access()
                .count_replay_verification_layer(ReplayVerificationLayer::SummaryParity);
        }
        ReplaySurfaceParityCheck::Drift {
            mismatch_class,
            layer,
            detail,
            ..
        } => {
            runtime.performance_access().count_replay_verification_layer(layer);
            if layer != ReplayVerificationLayer::DeepArtifactParity
                && verification_plan.allows_deep_artifact_parity()
            {
                if deep_matches() {
                    runtime
                        .performance_access()
                        .count_replay_verification_layer(ReplayVerificationLayer::DeepArtifactParity);
                    return;
                }
                runtime
                    .performance_access()
                    .count_replay_verification_layer(ReplayVerificationLayer::DeepArtifactParity);
                let expected = render_expected();
                let observed = render_observed();
                mismatches.push(ReplayMismatch {
                    class: mismatch_class,
                    surface,
                    verification_layer: ReplayVerificationLayer::DeepArtifactParity,
                    detail: format!("{} (confirmed by audit/deep artifact parity)", detail),
                    expected: Some(expected),
                    observed: Some(observed),
                });
                return;
            }
            let expected = render_expected();
            let observed = render_observed();
            mismatches.push(ReplayMismatch {
                class: mismatch_class,
                surface,
                verification_layer: layer,
                detail,
                expected: Some(expected),
                observed: Some(observed),
            });
        }
    }
}

fn validated_replay_continuity_envelope<'a>(
    runtime: &RelationalRuntime,
    envelope: &'a crate::replay::data::CanonicalCommitEnvelope,
    verification_plan: &ReplayVerificationPlan,
) -> Result<ValidatedReplayContinuityEnvelope<'a>, ReplayMismatch> {
    let validated_bundle = validate_schema_continuity_bundle(envelope)
        .map_err(|issue| replay_mismatch_for_continuity_issue(issue, verification_plan))?;
    let canonicalization_policy = runtime
        .runtime_config()
        .schema
        .descriptor_canonicalization_policy
        .clone();
    if let Some(found) = envelope
        .schema_continuation_descriptor
        .as_ref()
        .map(|descriptor| descriptor.bridge.canonicalization_version)
        .into_iter()
        .chain(
            envelope
                .schema_reconciliation_descriptor
                .as_ref()
                .map(|descriptor| descriptor.canonicalization_version),
        )
        .find(|version| !canonicalization_policy.supports(*version))
    {
        return Err(replay_mismatch_for_continuity_issue(
            crate::schema::logic::SchemaContinuityBundleIssue::DescriptorCanonicalizationVersionMismatch {
                expected: canonicalization_policy.current_write_version(),
                found,
            },
            verification_plan,
        ));
    }
    Ok(ValidatedReplayContinuityEnvelope {
        transition_basis: descriptor_basis_for_transition(envelope),
        continuation_basis: descriptor_basis_for_continuation(envelope),
        reconciliation_basis: descriptor_basis_for_reconciliation(envelope),
        lineage_basis: descriptor_basis_for_lineage(envelope),
        _validated_bundle: validated_bundle,
    })
}

fn replay_mismatch_for_continuity_issue(
    issue: crate::schema::logic::SchemaContinuityBundleIssue,
    verification_plan: &ReplayVerificationPlan,
) -> ReplayMismatch {
    let (class, layer) = match issue {
        crate::schema::logic::SchemaContinuityBundleIssue::IncompleteBundle
        | crate::schema::logic::SchemaContinuityBundleIssue::TargetSchemaVersionMismatch => (
            ReplayMismatchClass::SchemaTransitionDrift,
            replay_issue_layer(verification_plan, ReplayVerificationLayer::DigestParity),
        ),
        crate::schema::logic::SchemaContinuityBundleIssue::ContinuationDescriptorDrift { .. }
        | crate::schema::logic::SchemaContinuityBundleIssue::ContinuationBoundaryFingerprintMismatch { .. }
        | crate::schema::logic::SchemaContinuityBundleIssue::VisibleBridgeProofMismatch
        | crate::schema::logic::SchemaContinuityBundleIssue::HistoricalReinterpretationViolation => (
            ReplayMismatchClass::SchemaContinuationDescriptorDrift,
            replay_issue_layer(verification_plan, ReplayVerificationLayer::DigestParity),
        ),
        crate::schema::logic::SchemaContinuityBundleIssue::ReconciliationDescriptorDrift => (
            ReplayMismatchClass::SchemaReconciliationDescriptorDrift,
            replay_issue_layer(verification_plan, ReplayVerificationLayer::DigestParity),
        ),
        crate::schema::logic::SchemaContinuityBundleIssue::DescriptorSemanticsVersionMismatch { .. }
        | crate::schema::logic::SchemaContinuityBundleIssue::DescriptorCanonicalizationVersionMismatch { .. } => (
            ReplayMismatchClass::DescriptorVersionDrift,
            replay_issue_layer(verification_plan, ReplayVerificationLayer::DigestParity),
        ),
        crate::schema::logic::SchemaContinuityBundleIssue::LineageSchemaVersionMismatch => (
            ReplayMismatchClass::SchemaLineageDrift,
            ReplayVerificationLayer::SummaryParity,
        ),
    };
    ReplayMismatch {
        class,
        surface: ReplayObservableSurface::History,
        verification_layer: layer,
        detail: issue.detail(),
        expected: None,
        observed: None,
    }
}

fn replay_issue_layer(
    verification_plan: &ReplayVerificationPlan,
    default_layer: ReplayVerificationLayer,
) -> ReplayVerificationLayer {
    if verification_plan.allows_deep_artifact_parity() {
        ReplayVerificationLayer::DeepArtifactParity
    } else {
        default_layer
    }
}

fn compare_descriptor_surface(
    runtime: &RelationalRuntime,
    verification_plan: &ReplayVerificationPlan,
    mismatches: &mut Vec<ReplayMismatch>,
    expected: Option<DescriptorComparisonBasis>,
    observed: Option<DescriptorComparisonBasis>,
    mismatch_class: ReplayMismatchClass,
    detail: &str,
    deep_matches: impl FnOnce() -> bool,
    render_expected: impl FnOnce() -> String,
    render_observed: impl FnOnce() -> String,
) {
    match (expected, observed) {
        (None, None) => {}
        (Some(expected), Some(observed)) => {
            let parity = expected.compare(&observed, mismatch_class, detail);
            match parity {
                DescriptorParityCheck::ExactDigestMatch { .. } => {
                    runtime
                        .performance_access()
                        .count_replay_verification_layer(ReplayVerificationLayer::DigestParity);
                }
                DescriptorParityCheck::SummaryMatchDigestUnavailable { .. } => {
                    runtime
                        .performance_access()
                        .count_replay_verification_layer(ReplayVerificationLayer::SummaryParity);
                }
                DescriptorParityCheck::Drift {
                    mismatch_class,
                    layer,
                    detail,
                    ..
                } => {
                    runtime.performance_access().count_replay_verification_layer(layer);
                    if layer == ReplayVerificationLayer::DigestParity
                        && verification_plan.allows_deep_artifact_parity()
                    {
                        if deep_matches() {
                            runtime.performance_access().count_replay_verification_layer(
                                ReplayVerificationLayer::DeepArtifactParity,
                            );
                            return;
                        }
                        runtime.performance_access().count_replay_verification_layer(
                            ReplayVerificationLayer::DeepArtifactParity,
                        );
                        let expected_debug = render_expected();
                        let observed_debug = render_observed();
                        mismatches.push(ReplayMismatch {
                            class: mismatch_class,
                            surface: ReplayObservableSurface::History,
                            verification_layer: ReplayVerificationLayer::DeepArtifactParity,
                            detail: format!(
                                "{} (confirmed by audit/deep artifact parity)",
                                detail
                            ),
                            expected: Some(expected_debug),
                            observed: Some(observed_debug),
                        });
                        return;
                    }
                    let expected_debug = render_expected();
                    let observed_debug = render_observed();
                    mismatches.push(ReplayMismatch {
                        class: mismatch_class,
                        surface: ReplayObservableSurface::History,
                        verification_layer: layer,
                        detail,
                        expected: Some(expected_debug),
                        observed: Some(observed_debug),
                    });
                }
            }
        }
        _ => {
            runtime
                .performance_access()
                .count_replay_verification_layer(if verification_plan.allows_deep_artifact_parity() {
                    ReplayVerificationLayer::DeepArtifactParity
                } else {
                    ReplayVerificationLayer::DigestParity
                });
            let expected_debug = render_expected();
            let observed_debug = render_observed();
            mismatches.push(ReplayMismatch {
                class: mismatch_class,
                surface: ReplayObservableSurface::History,
                verification_layer: if verification_plan.allows_deep_artifact_parity() {
                    ReplayVerificationLayer::DeepArtifactParity
                } else {
                    ReplayVerificationLayer::DigestParity
                },
                detail: detail.to_string(),
                expected: Some(expected_debug),
                observed: Some(observed_debug),
            })
        }
    }
}

fn descriptor_basis_for_transition(
    envelope: &crate::replay::data::CanonicalCommitEnvelope,
) -> Option<DescriptorComparisonBasis> {
    let transition = envelope.schema_transition.as_ref()?;
    Some(DescriptorComparisonBasis::new(
        DescriptorAuthorityKind::SchemaTransitionArtifact,
        Some(VerifiedDescriptorDigest::new(
            DescriptorAuthorityKind::SchemaTransitionArtifact,
            envelope.descriptor_semantics_version,
            None,
            transition,
        )),
        Some(crate::replay::data::stable_digest(&(
            transition.source_schema_id.clone(),
            transition.source_schema_version_id,
            transition.target_schema_id.clone(),
            transition.target_schema_version_id,
            transition.diff_atoms.len(),
        ))),
    ))
}

fn descriptor_basis_for_continuation(
    envelope: &crate::replay::data::CanonicalCommitEnvelope,
) -> Option<DescriptorComparisonBasis> {
    let descriptor = envelope.schema_continuation_descriptor.as_ref()?;
    Some(DescriptorComparisonBasis::new(
        DescriptorAuthorityKind::SchemaContinuationDescriptor,
        Some(VerifiedDescriptorDigest::new(
            DescriptorAuthorityKind::SchemaContinuationDescriptor,
            envelope.descriptor_semantics_version,
            Some(descriptor.bridge.canonicalization_version),
            descriptor,
        )),
        Some(crate::replay::data::stable_digest(&(
            descriptor.boundary_fingerprint,
            descriptor.bridge.continuation,
            descriptor.normalized_boundary_count,
        ))),
    ))
}

fn descriptor_basis_for_reconciliation(
    envelope: &crate::replay::data::CanonicalCommitEnvelope,
) -> Option<DescriptorComparisonBasis> {
    let descriptor = envelope.schema_reconciliation_descriptor.as_ref()?;
        Some(DescriptorComparisonBasis::new(
            DescriptorAuthorityKind::SchemaReconciliationDescriptor,
            Some(VerifiedDescriptorDigest::new(
                DescriptorAuthorityKind::SchemaReconciliationDescriptor,
                envelope.descriptor_semantics_version,
                Some(descriptor.canonicalization_version),
                descriptor,
            )),
            Some(crate::replay::data::stable_digest(&(
                descriptor.classification,
                descriptor.resulting_lineage.resulting_schema_version_id,
                descriptor.resulting_lineage.ordering_mode.clone(),
            ))),
        ))
}

fn descriptor_basis_for_lineage(
    envelope: &crate::replay::data::CanonicalCommitEnvelope,
) -> Option<DescriptorComparisonBasis> {
    let lineage = envelope
        .schema_reconciliation_descriptor
        .as_ref()
        .map(|descriptor| &descriptor.resulting_lineage)?;
        Some(DescriptorComparisonBasis::new(
            DescriptorAuthorityKind::SchemaLineageArtifact,
            None,
            Some(crate::replay::data::stable_digest(&(
                lineage.resulting_schema_id.clone(),
                lineage.resulting_schema_version_id,
                lineage.parent_schema_version_ids.clone(),
            ))),
    ))
}

fn surface_basis_for_patch(
    envelope: &crate::replay::data::CanonicalCommitEnvelope,
) -> ReplaySurfaceComparisonBasis {
    let patch = envelope.patch.canonicalized();
    ReplaySurfaceComparisonBasis::new(
        ReplaySurfaceAuthorityKind::Patch,
        Some(VerifiedReplaySurfaceDigest::new(
            ReplaySurfaceAuthorityKind::Patch,
            &patch,
        )),
        Some(crate::replay::data::stable_digest(&patch.records.len())),
    )
}

fn surface_basis_for_diagnostics(
    envelope: &crate::replay::data::CanonicalCommitEnvelope,
) -> ReplaySurfaceComparisonBasis {
    let summary = &envelope.diagnostics_summary;
    ReplaySurfaceComparisonBasis::new(
        ReplaySurfaceAuthorityKind::Diagnostics,
        Some(VerifiedReplaySurfaceDigest::new(
            ReplaySurfaceAuthorityKind::Diagnostics,
            summary,
        )),
        Some(crate::replay::data::stable_digest(&(summary.entries.len(), &summary.kind))),
    )
}

fn surface_basis_for_history(
    envelope: &crate::replay::data::CanonicalCommitEnvelope,
) -> ReplaySurfaceComparisonBasis {
    let history = (
        envelope.commit.parents.clone(),
        envelope.merge_parent_branches.clone(),
        envelope.merge_base_commits.clone(),
    );
    ReplaySurfaceComparisonBasis::new(
        ReplaySurfaceAuthorityKind::History,
        Some(VerifiedReplaySurfaceDigest::new(
            ReplaySurfaceAuthorityKind::History,
            &history,
        )),
        Some(crate::replay::data::stable_digest(&history.0.len())),
    )
}

fn surface_basis_for_snapshot(
    surface: &crate::replay::data::ReplaySnapshotSurface,
) -> ReplaySurfaceComparisonBasis {
    ReplaySurfaceComparisonBasis::new(
        ReplaySurfaceAuthorityKind::Snapshot,
        Some(VerifiedReplaySurfaceDigest::new(
            ReplaySurfaceAuthorityKind::Snapshot,
            surface,
        )),
        Some(crate::replay::data::stable_digest(&(
            surface.version_id,
            surface.entities.len(),
            surface.relations.len(),
        ))),
    )
}

fn surface_basis_for_branch_head(
    commit: Option<&crate::history::data::CommitReference>,
) -> ReplaySurfaceComparisonBasis {
    let basis = commit.cloned();
    ReplaySurfaceComparisonBasis::new(
        ReplaySurfaceAuthorityKind::BranchHead,
        Some(VerifiedReplaySurfaceDigest::new(
            ReplaySurfaceAuthorityKind::BranchHead,
            &basis,
        )),
        Some(crate::replay::data::stable_digest(
            &basis.as_ref().map(|commit| (commit.commit_id, commit.version_id)),
        )),
    )
}

fn surface_basis_for_published_lineage(
    published_lineage: &PublishedLineageArtifact,
) -> ReplaySurfaceComparisonBasis {
    ReplaySurfaceComparisonBasis::new(
        ReplaySurfaceAuthorityKind::Lineage,
        Some(VerifiedReplaySurfaceDigest::new(
            ReplaySurfaceAuthorityKind::Lineage,
            &(
                lineage_event_batch_comparison_basis(published_lineage),
                lineage_decision_log_comparison_basis(published_lineage),
                published_lineage.observed_event_batch_digest_basis(),
                published_lineage.observed_decision_log_digest_basis(),
                published_lineage.digest_basis(),
                published_lineage.counters(),
            ),
        )),
        Some(crate::replay::data::stable_digest(&(
            published_lineage.digest_basis().lineage_event_count(),
            published_lineage.digest_basis().lineage_decision_count(),
        ))),
    )
}

fn published_lineage_artifacts_match(
    expected: &PublishedLineageArtifact,
    observed: &PublishedLineageArtifact,
) -> bool {
    if expected.branch_id() != observed.branch_id()
        || expected.lineage_event_ids() != observed.lineage_event_ids()
        || expected.lineage_events() != observed.lineage_events()
        || expected.digest_basis() != observed.digest_basis()
        || expected.observed_event_batch_digest_basis() != observed.observed_event_batch_digest_basis()
        || expected.observed_decision_log_digest_basis() != observed.observed_decision_log_digest_basis()
        || expected.counters() != observed.counters()
        || expected.lineage_decision_log() != observed.lineage_decision_log()
    {
        return false;
    }

    let candidate_ids = expected
        .lineage_decision_log()
        .iter()
        .chain(observed.lineage_decision_log().iter())
        .filter_map(|decision| decision.candidate_id)
        .collect::<std::collections::BTreeSet<CorrespondenceCandidateId>>();
    for candidate_id in candidate_ids {
        if expected
            .decisions_for_candidate(candidate_id)
            .collect::<Vec<_>>()
            != observed
                .decisions_for_candidate(candidate_id)
                .collect::<Vec<_>>()
        {
            return false;
        }
    }

    let event_ids = expected
        .lineage_event_ids()
        .iter()
        .copied()
        .chain(
            expected
                .lineage_decision_log()
                .iter()
                .chain(observed.lineage_decision_log().iter())
                .filter_map(|decision| decision.event_id),
        )
        .collect::<std::collections::BTreeSet<u64>>();
    for event_id in event_ids {
        if expected.decisions_for_event_id(event_id).collect::<Vec<_>>()
            != observed.decisions_for_event_id(event_id).collect::<Vec<_>>()
        {
            return false;
        }
    }

    let mut rejection_classes = expected
        .lineage_decision_log()
        .iter()
        .chain(observed.lineage_decision_log().iter())
        .filter_map(|decision| decision.rejection_class)
        .collect::<Vec<CorrespondencePromotionRejectionClass>>();
    rejection_classes.sort_by_key(|class| format!("{class:?}"));
    rejection_classes.dedup();
    for rejection_class in rejection_classes {
        if expected
            .decisions_for_rejection_class(rejection_class)
            .collect::<Vec<_>>()
            != observed
                .decisions_for_rejection_class(rejection_class)
                .collect::<Vec<_>>()
        {
            return false;
        }
    }

    true
}

fn lineage_event_batch_comparison_basis(
    published_lineage: &PublishedLineageArtifact,
) -> CertifiedLineageSurfaceComparisonBasis {
    CertifiedLineageSurfaceComparisonBasis::new(
        LineageCertifiedSurfaceKind::EventBatch,
        Some(CertifiedLineageSurfaceDigest::new(
            LineageCertifiedSurfaceKind::EventBatch,
            &published_lineage.observed_event_batch_digest_basis(),
        )),
        Some(crate::replay::data::stable_digest(&(
            published_lineage
                .observed_event_batch_digest_basis()
                .canonical_event_ids()
                .len(),
            published_lineage.event_batch_digest_basis().branch_id(),
        ))),
    )
}

fn lineage_decision_log_comparison_basis(
    published_lineage: &PublishedLineageArtifact,
) -> CertifiedLineageSurfaceComparisonBasis {
    CertifiedLineageSurfaceComparisonBasis::new(
        LineageCertifiedSurfaceKind::DecisionLog,
        Some(CertifiedLineageSurfaceDigest::new(
            LineageCertifiedSurfaceKind::DecisionLog,
            &published_lineage.observed_decision_log_digest_basis(),
        )),
        Some(crate::replay::data::stable_digest(&(
            published_lineage
                .observed_decision_log_digest_basis()
                .canonical_decision_kinds()
                .len(),
            published_lineage.decision_log_digest_basis().branch_id(),
        ))),
    )
}

fn select_published_lineage_authority<'a>(
    runtime: &'a RelationalRuntime,
    envelope: &'a CanonicalCommitEnvelope,
) -> SelectedPublishedLineageAuthority<'a> {
    if let Some(artifact) = runtime
        .durability
        .durable_log_envelope(envelope.commit.commit_id)
        .map(|candidate| candidate.published_lineage())
    {
        SelectedPublishedLineageAuthority {
            kind: ReplayAuthorityBasisKind::DurableLogCanonical,
            indexed_source: Some(ReplayLineageAuthorityIndexedSource::DurableLog),
            artifact,
        }
    } else if let Some(artifact) = runtime
        .durability
        .checkpoint_envelope(envelope.commit.commit_id)
        .map(|candidate| candidate.published_lineage())
    {
        SelectedPublishedLineageAuthority {
            kind: ReplayAuthorityBasisKind::DurableLogCanonical,
            indexed_source: Some(ReplayLineageAuthorityIndexedSource::Checkpoint),
            artifact,
        }
    } else {
        SelectedPublishedLineageAuthority {
            kind: ReplayAuthorityBasisKind::HistoryEnvelopeFallback,
            indexed_source: None,
            artifact: envelope.published_lineage(),
        }
    }
}

fn surface_basis_for_derived_indexes<T: serde::Serialize + ?Sized>(
    index_generations: &T,
) -> ReplaySurfaceComparisonBasis {
    ReplaySurfaceComparisonBasis::new(
        ReplaySurfaceAuthorityKind::DerivedIndexes,
        Some(VerifiedReplaySurfaceDigest::new(
            ReplaySurfaceAuthorityKind::DerivedIndexes,
            index_generations,
        )),
        Some(crate::replay::data::stable_digest(index_generations)),
    )
}

impl RelationalRuntime {
    pub fn replay_authority(&mut self) -> ReplayAuthority<'_> {
        ReplayAuthority::new(self)
    }
}
