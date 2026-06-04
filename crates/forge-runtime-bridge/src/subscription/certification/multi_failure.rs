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
    BridgeSubscriptionReferenceWorkloadComponentIdSet,
    BridgeSubscriptionReferenceWorkloadLaneIdSet, BridgeSubscriptionReferenceWorkloadManifestDraft,
    BridgeSubscriptionReferenceWorkloadManifestSealed,
    BridgeSubscriptionReferenceWorkloadProductIdSet, BridgeSubscriptionSourceArtifactEvidence,
    BridgeSubscriptionSourceArtifactIndex, BridgeSubscriptionSourceArtifactInput,
    BridgeSubscriptionSourceArtifactKind, BridgeSubscriptionSourceArtifactRole,
    BridgeSubscriptionSourceArtifactScenario,
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
            BridgeSubscriptionSourceArtifactRole::Control,
            BridgeSubscriptionSourceArtifactRole::Control,
            BridgeSubscriptionSourceArtifactRole::Control,
            false,
        );
        let hostile = assemble_bundle(
            &manifest,
            BridgeSubscriptionSourceArtifactRole::Hostile,
            BridgeSubscriptionSourceArtifactRole::Hostile,
            BridgeSubscriptionSourceArtifactRole::Hostile,
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
            .contains(&BridgeSubscriptionCertificationFailureBoundary::CheckpointDivergence)
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
        BridgeSubscriptionReferenceWorkloadProductIdSet::from_declared_product_labels(
            (0..128).map(|slot| format!("product-{slot:03}")),
        ),
        BridgeSubscriptionReferenceWorkloadComponentIdSet::from_declared_component_labels([
            "steel", "rubber", "copper", "glass", "labor",
        ]),
        BridgeSubscriptionReferenceWorkloadLaneIdSet::from_declared_lane_labels([
            "authoritative-live",
            "historical-replay",
            "branch-local",
            "preview-discard",
        ]),
    )
    .seal()
    .expect("multi-failure fixture manifest should seal")
}

fn assemble_bundle(
    manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
    basis_role: BridgeSubscriptionSourceArtifactRole,
    checkpoint_role: BridgeSubscriptionSourceArtifactRole,
    replay_role: BridgeSubscriptionSourceArtifactRole,
    rich_diagnostics: bool,
) -> BridgeSubscriptionCertificationBundleSealed {
    let index = BridgeSubscriptionSourceArtifactIndex::build(vec![
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::Declaration,
            BridgeSubscriptionSourceArtifactRole::Stable,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::BasisBinding,
            basis_role,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::AdmittedSubscription,
            BridgeSubscriptionSourceArtifactRole::Stable,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::ActiveDelivery,
            BridgeSubscriptionSourceArtifactRole::Stable,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::Checkpoint,
            checkpoint_role,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::RetainedReplay,
            replay_role,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::StrategyLowering,
            BridgeSubscriptionSourceArtifactRole::Stable,
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

fn source_artifact(
    artifact_kind: BridgeSubscriptionSourceArtifactKind,
    role: BridgeSubscriptionSourceArtifactRole,
) -> BridgeSubscriptionSourceArtifactInput {
    BridgeSubscriptionSourceArtifactInput::from_evidence(
        BridgeSubscriptionSourceArtifactEvidence::scenario(
            artifact_kind,
            BridgeSubscriptionSourceArtifactScenario::MultiFailurePrecedence,
            role,
        ),
    )
}
