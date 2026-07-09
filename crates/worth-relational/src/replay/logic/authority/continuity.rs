use crate::capabilities::RuntimeConfigSource;
use crate::history::data::HistoryDriftClass;
use crate::logic::runtime::RelationalRuntime;
use crate::replay::data::{
    digest_schema_continuation_descriptor, digest_schema_continuation_summary,
    digest_schema_lineage_summary, digest_schema_reconciliation_descriptor,
    digest_schema_reconciliation_summary, digest_schema_transition_descriptor,
    digest_schema_transition_summary, CanonicalCommitEnvelope, DescriptorAuthorityKind,
    DescriptorComparisonBasis, ReplayMismatch, ReplayMismatchClass, ReplayObservableSurface,
    ReplayVerificationLayer, ReplayVerificationPlan, VerifiedDescriptorDigest,
};
use crate::schema::logic::{validate_schema_continuity_bundle, SchemaContinuityBundleIssue};

use super::ValidatedReplayContinuityEnvelope;

pub(super) fn validated_replay_continuity_envelope<'a>(
    runtime: &RelationalRuntime,
    envelope: &'a CanonicalCommitEnvelope,
    verification_plan: &ReplayVerificationPlan,
) -> Result<ValidatedReplayContinuityEnvelope<'a>, ReplayMismatch> {
    let validated_bundle = validate_schema_continuity_bundle(envelope)
        .map_err(|issue| replay_mismatch_for_continuity_issue(issue, verification_plan))?;
    let canonical_basis_policy = runtime
        .runtime_config()
        .schema
        .descriptor_canonical_basis_policy
        .clone();
    if let Some(found) = envelope
        .schema_continuation_descriptor
        .as_ref()
        .map(|descriptor| descriptor.bridge.canonical_basis_version)
        .into_iter()
        .chain(
            envelope
                .schema_reconciliation_descriptor
                .as_ref()
                .map(|descriptor| descriptor.canonical_basis_version),
        )
        .find(|version| !canonical_basis_policy.supports(*version))
    {
        return Err(replay_mismatch_for_continuity_issue(
            SchemaContinuityBundleIssue::DescriptorCanonicalBasisVersionMismatch {
                expected: canonical_basis_policy.current_write_version(),
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

pub(super) fn replay_mismatch_for_continuity_issue(
    issue: SchemaContinuityBundleIssue,
    verification_plan: &ReplayVerificationPlan,
) -> ReplayMismatch {
    let (class, layer) = match issue {
        SchemaContinuityBundleIssue::IncompleteBundle
        | SchemaContinuityBundleIssue::TargetSchemaVersionMismatch => (
            ReplayMismatchClass::SchemaTransitionDrift,
            replay_issue_layer(verification_plan, ReplayVerificationLayer::DigestParity),
        ),
        SchemaContinuityBundleIssue::ContinuationDescriptorDrift { .. }
        | SchemaContinuityBundleIssue::ContinuationBoundaryFingerprintMismatch { .. }
        | SchemaContinuityBundleIssue::VisibleBridgeProofMismatch
        | SchemaContinuityBundleIssue::HistoricalReinterpretationViolation => (
            ReplayMismatchClass::SchemaContinuationDescriptorDrift,
            replay_issue_layer(verification_plan, ReplayVerificationLayer::DigestParity),
        ),
        SchemaContinuityBundleIssue::ReconciliationDescriptorDrift => (
            ReplayMismatchClass::SchemaReconciliationDescriptorDrift,
            replay_issue_layer(verification_plan, ReplayVerificationLayer::DigestParity),
        ),
        SchemaContinuityBundleIssue::DescriptorSemanticsVersionMismatch { .. }
        | SchemaContinuityBundleIssue::DescriptorCanonicalBasisVersionMismatch { .. } => (
            ReplayMismatchClass::DescriptorVersionDrift,
            replay_issue_layer(verification_plan, ReplayVerificationLayer::DigestParity),
        ),
        SchemaContinuityBundleIssue::LineageSchemaVersionMismatch => (
            ReplayMismatchClass::SchemaLineageDrift,
            ReplayVerificationLayer::SummaryParity,
        ),
    };
    ReplayMismatch {
        class,
        history_drift_class: replay_history_drift_class(class),
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

pub(super) fn replay_history_drift_class(
    mismatch_class: ReplayMismatchClass,
) -> Option<HistoryDriftClass> {
    match mismatch_class {
        ReplayMismatchClass::HistoryDrift => Some(HistoryDriftClass::ReplayAuthorityDrift),
        _ => None,
    }
}

fn descriptor_basis_for_transition(
    envelope: &CanonicalCommitEnvelope,
) -> Option<DescriptorComparisonBasis> {
    let transition = envelope.schema_transition.as_ref()?;
    Some(DescriptorComparisonBasis::new(
        DescriptorAuthorityKind::SchemaTransitionArtifact,
        Some(VerifiedDescriptorDigest::from_digest(
            DescriptorAuthorityKind::SchemaTransitionArtifact,
            envelope.descriptor_semantics_version,
            None,
            digest_schema_transition_descriptor(transition, envelope.descriptor_semantics_version),
        )),
        Some(digest_schema_transition_summary(transition)),
    ))
}

fn descriptor_basis_for_continuation(
    envelope: &CanonicalCommitEnvelope,
) -> Option<DescriptorComparisonBasis> {
    let descriptor = envelope.schema_continuation_descriptor.as_ref()?;
    Some(DescriptorComparisonBasis::new(
        DescriptorAuthorityKind::SchemaContinuationDescriptor,
        Some(VerifiedDescriptorDigest::from_digest(
            DescriptorAuthorityKind::SchemaContinuationDescriptor,
            envelope.descriptor_semantics_version,
            Some(descriptor.bridge.canonical_basis_version),
            digest_schema_continuation_descriptor(
                descriptor,
                envelope.descriptor_semantics_version,
            ),
        )),
        Some(digest_schema_continuation_summary(descriptor)),
    ))
}

fn descriptor_basis_for_reconciliation(
    envelope: &CanonicalCommitEnvelope,
) -> Option<DescriptorComparisonBasis> {
    let descriptor = envelope.schema_reconciliation_descriptor.as_ref()?;
    Some(DescriptorComparisonBasis::new(
        DescriptorAuthorityKind::SchemaReconciliationDescriptor,
        Some(VerifiedDescriptorDigest::from_digest(
            DescriptorAuthorityKind::SchemaReconciliationDescriptor,
            envelope.descriptor_semantics_version,
            Some(descriptor.canonical_basis_version),
            digest_schema_reconciliation_descriptor(
                descriptor,
                envelope.descriptor_semantics_version,
            ),
        )),
        Some(digest_schema_reconciliation_summary(descriptor)),
    ))
}

fn descriptor_basis_for_lineage(
    envelope: &CanonicalCommitEnvelope,
) -> Option<DescriptorComparisonBasis> {
    let lineage = envelope
        .schema_reconciliation_descriptor
        .as_ref()
        .map(|descriptor| &descriptor.resulting_lineage)?;
    Some(DescriptorComparisonBasis::new(
        DescriptorAuthorityKind::SchemaLineageArtifact,
        None,
        Some(digest_schema_lineage_summary(lineage)),
    ))
}
