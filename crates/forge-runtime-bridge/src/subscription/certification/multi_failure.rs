use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionCertificationAssemblyPlan, BridgeSubscriptionCertificationBundleDraft,
    BridgeSubscriptionCertificationBundleSealed, BridgeSubscriptionCertificationComparisonPlan,
    BridgeSubscriptionCertificationComparisonRelationship,
    BridgeSubscriptionCertificationComparisonReport, BridgeSubscriptionCertificationCostProfile,
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationDensityPosture,
    BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionCertificationFailurePrecedenceStage, BridgeSubscriptionCertificationScratch,
    BridgeSubscriptionReferenceWorkloadManifestDraft,
    BridgeSubscriptionReferenceWorkloadManifestSealed, BridgeSubscriptionSourceArtifactIndex,
    BridgeSubscriptionSourceArtifactInput, BridgeSubscriptionSourceArtifactKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationMultiFailurePrecedenceReport {
    control_bundle_digest: Arc<str>,
    hostile_bundle_digest: Arc<str>,
    comparison_report_digest: Arc<str>,
    primary_failure_boundary: BridgeSubscriptionCertificationFailureBoundary,
    primary_failure_precedence_stage: BridgeSubscriptionCertificationFailurePrecedenceStage,
    suppressed_failure_boundaries: Vec<BridgeSubscriptionCertificationFailureBoundary>,
    basis_drift_is_primary_without_registry_drift: bool,
    suppressed_checkpoint_replay_and_diagnostics: bool,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationMultiFailurePrecedenceReport {
    pub(crate) fn certify() -> Self {
        let manifest = reference_manifest();
        let control = assemble_bundle(
            &manifest,
            "basis-digest-v1",
            "checkpoint-digest-v1",
            "replay-digest-v1",
            false,
        );
        let hostile = assemble_bundle(
            &manifest,
            "basis-digest-v2",
            "checkpoint-digest-v2",
            "replay-digest-v2",
            true,
        );
        let plan = BridgeSubscriptionCertificationComparisonPlan::admit(
            BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(BridgeSubscriptionCertificationFailureBoundary::BasisDrift),
            None,
        )
        .expect("multi-failure precedence report names its expected primary boundary");
        let comparison =
            BridgeSubscriptionCertificationComparisonReport::compare(plan, &control, &hostile);
        Self::from_certified_parts(control, hostile, comparison)
    }

    fn from_certified_parts(
        control: BridgeSubscriptionCertificationBundleSealed,
        hostile: BridgeSubscriptionCertificationBundleSealed,
        comparison: BridgeSubscriptionCertificationComparisonReport,
    ) -> Self {
        let primary_failure_boundary = comparison
            .primary_failure_boundary()
            .expect("multi-failure comparison must localize a primary failure");
        let primary_failure_precedence_stage = comparison
            .primary_failure_precedence_stage()
            .expect("multi-failure comparison must expose primary precedence");
        let suppressed_failure_boundaries = comparison.suppressed_failure_boundaries().to_vec();
        let basis_drift_is_primary_without_registry_drift = primary_failure_boundary
            == BridgeSubscriptionCertificationFailureBoundary::BasisDrift
            && primary_failure_precedence_stage
                == BridgeSubscriptionCertificationFailurePrecedenceStage::BasisBinding
            && !suppressed_failure_boundaries
                .contains(&BridgeSubscriptionCertificationFailureBoundary::RegistryDrift)
            && !suppressed_failure_boundaries.contains(
                &BridgeSubscriptionCertificationFailureBoundary::DeclarationEquivalenceDrift,
            )
            && control.semantic_digests().subscription_registry_digest()
                == hostile.semantic_digests().subscription_registry_digest()
            && control.semantic_digests().subscription_digest()
                == hostile.semantic_digests().subscription_digest()
            && control.semantic_digests().subscription_basis_digest()
                != hostile.semantic_digests().subscription_basis_digest();
        let suppressed_checkpoint_replay_and_diagnostics = suppressed_failure_boundaries
            .contains(&BridgeSubscriptionCertificationFailureBoundary::CheckpointIncompatibility)
            && suppressed_failure_boundaries
                .contains(&BridgeSubscriptionCertificationFailureBoundary::ReplayMismatch)
            && suppressed_failure_boundaries
                .contains(&BridgeSubscriptionCertificationFailureBoundary::DiagnosticsInfluence);
        let counters = BridgeSubscriptionCertificationCounterSnapshot::combine([
            *comparison.counters(),
            BridgeSubscriptionCertificationCounterSnapshot::from_multi_failure_precedence_report(),
        ]);
        let suppressed_basis = suppressed_failure_boundaries
            .iter()
            .map(|boundary| boundary.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-multi-failure-precedence-report|control={}|hostile={}|comparison={}|primary={}|stage={}|suppressed={suppressed_basis}|basis-primary={basis_drift_is_primary_without_registry_drift}|checkpoint-replay-diagnostics-suppressed={suppressed_checkpoint_replay_and_diagnostics}|counters={}",
            control.digest(),
            hostile.digest(),
            comparison.digest(),
            primary_failure_boundary.as_str(),
            primary_failure_precedence_stage.as_str(),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            control_bundle_digest: Arc::from(control.digest()),
            hostile_bundle_digest: Arc::from(hostile.digest()),
            comparison_report_digest: Arc::from(comparison.digest()),
            primary_failure_boundary,
            primary_failure_precedence_stage,
            suppressed_failure_boundaries,
            basis_drift_is_primary_without_registry_drift,
            suppressed_checkpoint_replay_and_diagnostics,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-multi-failure-precedence-report:sha256:{digest:x}"
            )),
        }
    }

    pub fn control_bundle_digest(&self) -> &str {
        self.control_bundle_digest.as_ref()
    }

    pub fn hostile_bundle_digest(&self) -> &str {
        self.hostile_bundle_digest.as_ref()
    }

    pub fn comparison_report_digest(&self) -> &str {
        self.comparison_report_digest.as_ref()
    }

    pub fn primary_failure_boundary(&self) -> BridgeSubscriptionCertificationFailureBoundary {
        self.primary_failure_boundary
    }

    pub fn primary_failure_precedence_stage(
        &self,
    ) -> BridgeSubscriptionCertificationFailurePrecedenceStage {
        self.primary_failure_precedence_stage
    }

    pub fn suppressed_failure_boundaries(
        &self,
    ) -> &[BridgeSubscriptionCertificationFailureBoundary] {
        &self.suppressed_failure_boundaries
    }

    pub fn basis_drift_is_primary_without_registry_drift(&self) -> bool {
        self.basis_drift_is_primary_without_registry_drift
    }

    pub fn suppressed_checkpoint_replay_and_diagnostics(&self) -> bool {
        self.suppressed_checkpoint_replay_and_diagnostics
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

fn reference_manifest() -> BridgeSubscriptionReferenceWorkloadManifestSealed {
    BridgeSubscriptionReferenceWorkloadManifestDraft::new(
        (0..128)
            .map(|slot| format!("product-{slot:03}"))
            .collect::<Vec<_>>(),
        ["steel", "rubber", "copper", "glass", "labor"].to_vec(),
        [
            "authoritative-live",
            "historical-replay",
            "branch-local",
            "preview-discard",
        ]
        .to_vec(),
    )
    .seal()
    .expect("multi-failure fixture manifest should seal")
}

fn assemble_bundle(
    manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
    basis_digest: &str,
    checkpoint_digest: &str,
    replay_digest: &str,
    rich_diagnostics: bool,
) -> BridgeSubscriptionCertificationBundleSealed {
    let index = BridgeSubscriptionSourceArtifactIndex::build(vec![
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::Declaration,
            "multi-failure-declaration",
            "declaration-digest-stable",
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::BasisBinding,
            "multi-failure-basis",
            basis_digest,
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::AdmittedSubscription,
            "multi-failure-admitted-subscription",
            "admitted-digest-stable",
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::ActiveDelivery,
            "multi-failure-active-delivery",
            "active-delivery-digest-stable",
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::Checkpoint,
            "multi-failure-checkpoint",
            checkpoint_digest,
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::RetainedReplay,
            "multi-failure-replay",
            replay_digest,
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::StrategyLowering,
            "multi-failure-strategy",
            "strategy-digest-stable",
        ),
    ]);
    let plan = BridgeSubscriptionCertificationAssemblyPlan::plan(manifest, &index);
    let cost_profile = BridgeSubscriptionCertificationCostProfile::admit(
        BridgeSubscriptionCertificationDensityPosture::SparseCertificationWindow,
        8,
        16,
        32,
        rich_diagnostics,
    )
    .expect("multi-failure sparse cost profile should admit");
    let scratch = BridgeSubscriptionCertificationScratch::prepare(&cost_profile);
    BridgeSubscriptionCertificationBundleDraft::assemble(plan, cost_profile, scratch)
        .expect("multi-failure bundle should assemble")
        .seal()
}
