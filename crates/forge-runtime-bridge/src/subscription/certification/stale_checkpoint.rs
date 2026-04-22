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
pub struct BridgeSubscriptionCertificationStaleCheckpointReport {
    fresh_bundle_digest: Arc<str>,
    stale_bundle_digest: Arc<str>,
    fresh_checkpoint_digest: Arc<str>,
    stale_checkpoint_digest: Arc<str>,
    comparison_report_digest: Arc<str>,
    primary_failure_boundary: BridgeSubscriptionCertificationFailureBoundary,
    primary_failure_precedence_stage: BridgeSubscriptionCertificationFailurePrecedenceStage,
    checkpoint_drift_is_primary_without_replay_mismatch: bool,
    suppressed_failure_boundary_count: usize,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationStaleCheckpointReport {
    pub(crate) fn certify() -> Self {
        let manifest = reference_manifest();
        let fresh = assemble_bundle(&manifest, "checkpoint-digest-fresh");
        let stale = assemble_bundle(&manifest, "checkpoint-digest-stale");
        let plan = BridgeSubscriptionCertificationComparisonPlan::admit(
            BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(BridgeSubscriptionCertificationFailureBoundary::CheckpointIncompatibility),
            None,
        )
        .expect("stale checkpoint report names its expected checkpoint boundary");
        let comparison =
            BridgeSubscriptionCertificationComparisonReport::compare(plan, &fresh, &stale);
        Self::from_certified_parts(fresh, stale, comparison)
    }

    fn from_certified_parts(
        fresh: BridgeSubscriptionCertificationBundleSealed,
        stale: BridgeSubscriptionCertificationBundleSealed,
        comparison: BridgeSubscriptionCertificationComparisonReport,
    ) -> Self {
        let primary_failure_boundary = comparison
            .primary_failure_boundary()
            .expect("stale checkpoint comparison must localize a primary failure");
        let primary_failure_precedence_stage = comparison
            .primary_failure_precedence_stage()
            .expect("stale checkpoint comparison must expose primary precedence");
        let suppressed_failure_boundary_count = comparison.suppressed_failure_boundaries().len();
        let checkpoint_drift_is_primary_without_replay_mismatch = primary_failure_boundary
            == BridgeSubscriptionCertificationFailureBoundary::CheckpointIncompatibility
            && primary_failure_precedence_stage
                == BridgeSubscriptionCertificationFailurePrecedenceStage::CheckpointResumeOrReplay
            && fresh.semantic_digests().checkpoint_digest()
                != stale.semantic_digests().checkpoint_digest()
            && fresh.semantic_digests().replay_digest() == stale.semantic_digests().replay_digest()
            && !comparison
                .suppressed_failure_boundaries()
                .contains(&BridgeSubscriptionCertificationFailureBoundary::ReplayMismatch)
            && comparison.mismatch_count() == 1;
        let counters = BridgeSubscriptionCertificationCounterSnapshot::combine([
            *comparison.counters(),
            BridgeSubscriptionCertificationCounterSnapshot::from_stale_checkpoint_report(),
        ]);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-stale-checkpoint-report|fresh={}|stale={}|fresh-checkpoint={}|stale-checkpoint={}|comparison={}|primary={}|stage={}|checkpoint-primary-without-replay={checkpoint_drift_is_primary_without_replay_mismatch}|suppressed={suppressed_failure_boundary_count}|counters={}",
            fresh.digest(),
            stale.digest(),
            fresh.semantic_digests().checkpoint_digest(),
            stale.semantic_digests().checkpoint_digest(),
            comparison.digest(),
            primary_failure_boundary.as_str(),
            primary_failure_precedence_stage.as_str(),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            fresh_bundle_digest: Arc::from(fresh.digest()),
            stale_bundle_digest: Arc::from(stale.digest()),
            fresh_checkpoint_digest: Arc::from(fresh.semantic_digests().checkpoint_digest()),
            stale_checkpoint_digest: Arc::from(stale.semantic_digests().checkpoint_digest()),
            comparison_report_digest: Arc::from(comparison.digest()),
            primary_failure_boundary,
            primary_failure_precedence_stage,
            checkpoint_drift_is_primary_without_replay_mismatch,
            suppressed_failure_boundary_count,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-stale-checkpoint-report:sha256:{digest:x}"
            )),
        }
    }

    pub fn fresh_bundle_digest(&self) -> &str {
        self.fresh_bundle_digest.as_ref()
    }

    pub fn stale_bundle_digest(&self) -> &str {
        self.stale_bundle_digest.as_ref()
    }

    pub fn fresh_checkpoint_digest(&self) -> &str {
        self.fresh_checkpoint_digest.as_ref()
    }

    pub fn stale_checkpoint_digest(&self) -> &str {
        self.stale_checkpoint_digest.as_ref()
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

    pub fn checkpoint_drift_is_primary_without_replay_mismatch(&self) -> bool {
        self.checkpoint_drift_is_primary_without_replay_mismatch
    }

    pub fn suppressed_failure_boundary_count(&self) -> usize {
        self.suppressed_failure_boundary_count
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
            "stale-checkpoint-rejection",
            "historical-replay",
            "branch-local",
        ]
        .to_vec(),
    )
    .seal()
    .expect("stale checkpoint fixture manifest should seal")
}

fn assemble_bundle(
    manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
    checkpoint_digest: &str,
) -> BridgeSubscriptionCertificationBundleSealed {
    let index = BridgeSubscriptionSourceArtifactIndex::build(vec![
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::Declaration,
            "stale-checkpoint-declaration",
            "declaration-digest-stable",
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::AdmittedSubscription,
            "stale-checkpoint-admitted-subscription",
            "admitted-digest-stable",
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::ActiveDelivery,
            "stale-checkpoint-active-delivery",
            "active-delivery-digest-stable",
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::Checkpoint,
            "stale-checkpoint-boundary",
            checkpoint_digest,
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::StrategyLowering,
            "stale-checkpoint-strategy",
            "strategy-digest-stable",
        ),
    ]);
    let plan = BridgeSubscriptionCertificationAssemblyPlan::plan(manifest, &index);
    let cost_profile = BridgeSubscriptionCertificationCostProfile::admit(
        BridgeSubscriptionCertificationDensityPosture::SparseCertificationWindow,
        8,
        16,
        32,
        false,
    )
    .expect("stale checkpoint sparse cost profile should admit");
    let scratch = BridgeSubscriptionCertificationScratch::prepare(&cost_profile);
    BridgeSubscriptionCertificationBundleDraft::assemble(plan, cost_profile, scratch)
        .expect("stale checkpoint bundle should assemble")
        .seal()
}
