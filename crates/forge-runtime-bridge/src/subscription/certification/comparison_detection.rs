use super::{
    precedence_stage_for_boundary, BridgeSubscriptionCertificationBundleSealed,
    BridgeSubscriptionCertificationFailureBoundary,
};

pub(crate) fn detect_failures(
    left: &BridgeSubscriptionCertificationBundleSealed,
    right: &BridgeSubscriptionCertificationBundleSealed,
) -> Vec<BridgeSubscriptionCertificationFailureBoundary> {
    let mut failures = Vec::new();
    if left.schema_version() != right.schema_version()
        || left.digest_algorithm() != right.digest_algorithm()
    {
        return vec![
            BridgeSubscriptionCertificationFailureBoundary::BundleSchemaOrDigestIncompatibility,
        ];
    }
    if left.completeness_report().digest() != right.completeness_report().digest() {
        failures.push(BridgeSubscriptionCertificationFailureBoundary::BundleInsufficiency);
    }
    if left.semantic_digests().subscription_digest()
        != right.semantic_digests().subscription_digest()
    {
        failures.push(BridgeSubscriptionCertificationFailureBoundary::DeclarationEquivalenceDrift);
    }
    if left.semantic_digests().subscription_registry_digest()
        != right.semantic_digests().subscription_registry_digest()
    {
        failures.push(BridgeSubscriptionCertificationFailureBoundary::RegistryDrift);
    }
    if left.semantic_digests().subscription_basis_digest()
        != right.semantic_digests().subscription_basis_digest()
    {
        failures.push(BridgeSubscriptionCertificationFailureBoundary::BasisDrift);
    }
    if left.semantic_digests().strategy_lowering_digest()
        != right.semantic_digests().strategy_lowering_digest()
    {
        failures.push(
            BridgeSubscriptionCertificationFailureBoundary::StrategyLoweringProvenanceMismatch,
        );
    }
    if left.semantic_digests().subscription_lifecycle_digest()
        != right.semantic_digests().subscription_lifecycle_digest()
    {
        failures.push(BridgeSubscriptionCertificationFailureBoundary::LifecycleTransitionMismatch);
    }
    if left.semantic_digests().consumer_contract_digest()
        != right.semantic_digests().consumer_contract_digest()
    {
        failures.push(BridgeSubscriptionCertificationFailureBoundary::ConsumerContractMismatch);
    }
    if left.semantic_digests().subscription_share_digest()
        != right.semantic_digests().subscription_share_digest()
    {
        failures.push(BridgeSubscriptionCertificationFailureBoundary::IllegalSharingReuse);
    }
    if left.semantic_digests().subscription_delivery_digest()
        != right.semantic_digests().subscription_delivery_digest()
    {
        failures.push(BridgeSubscriptionCertificationFailureBoundary::DeliveryDigestDrift);
    }
    if left.semantic_digests().routing_digest() != right.semantic_digests().routing_digest() {
        failures.push(BridgeSubscriptionCertificationFailureBoundary::DeliveryFamilyMismatch);
    }
    if left.semantic_digests().subscription_continuation_digest()
        != right.semantic_digests().subscription_continuation_digest()
    {
        failures
            .push(BridgeSubscriptionCertificationFailureBoundary::ContinuationDenialOrAmbiguity);
    }
    if left.semantic_digests().checkpoint_digest() != right.semantic_digests().checkpoint_digest() {
        failures.push(BridgeSubscriptionCertificationFailureBoundary::CheckpointIncompatibility);
    }
    if left.semantic_digests().replay_digest() != right.semantic_digests().replay_digest() {
        failures.push(BridgeSubscriptionCertificationFailureBoundary::ReplayMismatch);
    }
    if left.semantic_digests().residue_digest() != right.semantic_digests().residue_digest() {
        failures.push(BridgeSubscriptionCertificationFailureBoundary::PreviewResidueMismatch);
    }
    if left.semantic_digests().diagnostics_digest() != right.semantic_digests().diagnostics_digest()
    {
        failures.push(BridgeSubscriptionCertificationFailureBoundary::DiagnosticsInfluence);
    }
    if left.semantic_digests().failure_digest() != right.semantic_digests().failure_digest() {
        failures
            .push(BridgeSubscriptionCertificationFailureBoundary::MissingRequiredRetainedArtifact);
    }
    if left.counters().digest() != right.counters().digest() {
        failures.push(BridgeSubscriptionCertificationFailureBoundary::CounterContractViolation);
    }
    failures
}

pub(crate) fn primary_failure_boundary(
    failures: &[BridgeSubscriptionCertificationFailureBoundary],
) -> Option<BridgeSubscriptionCertificationFailureBoundary> {
    failures
        .iter()
        .copied()
        .min_by_key(|boundary| precedence_stage_for_boundary(*boundary).rank())
}
