use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionCertificationBundleDraft, BridgeSubscriptionCertificationBundleSealed,
    BridgeSubscriptionCertificationComparisonOutcome,
    BridgeSubscriptionCertificationComparisonPlan,
    BridgeSubscriptionCertificationComparisonRelationship,
    BridgeSubscriptionCertificationComparisonReport, BridgeSubscriptionCertificationCostProfile,
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationDensityPosture,
    BridgeSubscriptionCertificationScratch, BridgeSubscriptionReferenceWorkloadManifestDraft,
    BridgeSubscriptionSourceArtifactIndex, BridgeSubscriptionSourceArtifactInput,
    BridgeSubscriptionSourceArtifactKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationOrderingHostilityReport {
    control_source_artifact_index_digest: Arc<str>,
    hostile_source_artifact_index_digest: Arc<str>,
    control_bundle_digest: Arc<str>,
    hostile_bundle_digest: Arc<str>,
    comparison_report_digest: Arc<str>,
    comparison_outcome: BridgeSubscriptionCertificationComparisonOutcome,
    canonical_source_order_preserved: bool,
    semantic_digest_preserved: bool,
    sealed_bundle_digest_preserved: bool,
    field_order_preserved: bool,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationOrderingHostilityReport {
    pub(crate) fn certify() -> Self {
        let manifest = BridgeSubscriptionReferenceWorkloadManifestDraft::new(
            ordering_product_ids(),
            vec!["steel", "rubber", "copper", "glass", "labor"],
            vec!["canonical-ordering-hostility"],
        )
        .seal()
        .expect("ordering hostility manifest is valid by construction");
        let control_inputs = ordering_hostility_source_inputs();
        let hostile_inputs = {
            let mut inputs = control_inputs.clone();
            inputs.reverse();
            inputs
        };
        let control_index = BridgeSubscriptionSourceArtifactIndex::build(control_inputs);
        let hostile_index = BridgeSubscriptionSourceArtifactIndex::build(hostile_inputs);
        let control = assemble_bundle(&manifest, &control_index);
        let hostile = assemble_bundle(&manifest, &hostile_index);
        let comparison_plan = BridgeSubscriptionCertificationComparisonPlan::admit(
            BridgeSubscriptionCertificationComparisonRelationship::SemanticEquivalence,
            None,
            None,
        )
        .expect("ordering hostility semantic equivalence plan is valid by construction");
        let comparison = BridgeSubscriptionCertificationComparisonReport::compare(
            comparison_plan,
            &control,
            &hostile,
        );
        Self::from_certified_parts(control_index, hostile_index, control, hostile, comparison)
    }

    fn from_certified_parts(
        control_index: BridgeSubscriptionSourceArtifactIndex,
        hostile_index: BridgeSubscriptionSourceArtifactIndex,
        control: BridgeSubscriptionCertificationBundleSealed,
        hostile: BridgeSubscriptionCertificationBundleSealed,
        comparison: BridgeSubscriptionCertificationComparisonReport,
    ) -> Self {
        let canonical_source_order_preserved = control_index.digest() == hostile_index.digest()
            && control_index
                .records()
                .iter()
                .map(|record| record.digest())
                .eq(hostile_index.records().iter().map(|record| record.digest()));
        let semantic_digest_preserved = control.semantic_digests() == hostile.semantic_digests();
        let sealed_bundle_digest_preserved = control.digest() == hostile.digest();
        let field_order_preserved = control
            .fields()
            .iter()
            .map(|field| field.digest())
            .eq(hostile.fields().iter().map(|field| field.digest()));
        let counters = BridgeSubscriptionCertificationCounterSnapshot::combine([
            *control.counters(),
            *hostile.counters(),
            *comparison.counters(),
            BridgeSubscriptionCertificationCounterSnapshot::from_ordering_hostility_report(),
        ]);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-ordering-hostility-report|control-index={}|hostile-index={}|control-bundle={}|hostile-bundle={}|comparison={}|outcome={}|source-order={canonical_source_order_preserved}|semantic={semantic_digest_preserved}|bundle={sealed_bundle_digest_preserved}|fields={field_order_preserved}|counters={}",
            control_index.digest(),
            hostile_index.digest(),
            control.digest(),
            hostile.digest(),
            comparison.digest(),
            comparison.outcome().as_str(),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            control_source_artifact_index_digest: Arc::from(control_index.digest()),
            hostile_source_artifact_index_digest: Arc::from(hostile_index.digest()),
            control_bundle_digest: Arc::from(control.digest()),
            hostile_bundle_digest: Arc::from(hostile.digest()),
            comparison_report_digest: Arc::from(comparison.digest()),
            comparison_outcome: comparison.outcome(),
            canonical_source_order_preserved,
            semantic_digest_preserved,
            sealed_bundle_digest_preserved,
            field_order_preserved,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-ordering-hostility-report:sha256:{digest:x}"
            )),
        }
    }

    pub fn control_source_artifact_index_digest(&self) -> &str {
        self.control_source_artifact_index_digest.as_ref()
    }

    pub fn hostile_source_artifact_index_digest(&self) -> &str {
        self.hostile_source_artifact_index_digest.as_ref()
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

    pub fn comparison_outcome(&self) -> BridgeSubscriptionCertificationComparisonOutcome {
        self.comparison_outcome
    }

    pub fn canonical_source_order_preserved(&self) -> bool {
        self.canonical_source_order_preserved
    }

    pub fn semantic_digest_preserved(&self) -> bool {
        self.semantic_digest_preserved
    }

    pub fn sealed_bundle_digest_preserved(&self) -> bool {
        self.sealed_bundle_digest_preserved
    }

    pub fn field_order_preserved(&self) -> bool {
        self.field_order_preserved
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

fn assemble_bundle(
    manifest: &super::BridgeSubscriptionReferenceWorkloadManifestSealed,
    source_artifact_index: &BridgeSubscriptionSourceArtifactIndex,
) -> BridgeSubscriptionCertificationBundleSealed {
    let plan =
        super::BridgeSubscriptionCertificationAssemblyPlan::plan(manifest, source_artifact_index);
    let cost_profile = BridgeSubscriptionCertificationCostProfile::admit(
        BridgeSubscriptionCertificationDensityPosture::SparseCertificationWindow,
        16,
        16,
        32,
        false,
    )
    .expect("ordering hostility sparse cost profile is admitted by construction");
    let scratch = BridgeSubscriptionCertificationScratch::prepare(&cost_profile);
    BridgeSubscriptionCertificationBundleDraft::assemble(plan, cost_profile, scratch)
        .expect("ordering hostility bundle is admitted by construction")
        .seal()
}

fn ordering_hostility_source_inputs() -> Vec<BridgeSubscriptionSourceArtifactInput> {
    [
        (
            BridgeSubscriptionSourceArtifactKind::Declaration,
            "ordering:declaration",
            "digest:ordering:declaration",
        ),
        (
            BridgeSubscriptionSourceArtifactKind::BasisBinding,
            "ordering:basis",
            "digest:ordering:basis",
        ),
        (
            BridgeSubscriptionSourceArtifactKind::AdmittedSubscription,
            "ordering:admitted",
            "digest:ordering:admitted",
        ),
        (
            BridgeSubscriptionSourceArtifactKind::Lifecycle,
            "ordering:lifecycle",
            "digest:ordering:lifecycle",
        ),
        (
            BridgeSubscriptionSourceArtifactKind::ActiveDelivery,
            "ordering:active-delivery",
            "digest:ordering:active-delivery",
        ),
        (
            BridgeSubscriptionSourceArtifactKind::DeliveryWindow,
            "ordering:delivery-window",
            "digest:ordering:delivery-window",
        ),
        (
            BridgeSubscriptionSourceArtifactKind::Fanout,
            "ordering:fanout",
            "digest:ordering:fanout",
        ),
        (
            BridgeSubscriptionSourceArtifactKind::Checkpoint,
            "ordering:checkpoint",
            "digest:ordering:checkpoint",
        ),
        (
            BridgeSubscriptionSourceArtifactKind::Resume,
            "ordering:resume",
            "digest:ordering:resume",
        ),
        (
            BridgeSubscriptionSourceArtifactKind::Continuation,
            "ordering:continuation",
            "digest:ordering:continuation",
        ),
        (
            BridgeSubscriptionSourceArtifactKind::Preview,
            "ordering:preview",
            "digest:ordering:preview",
        ),
        (
            BridgeSubscriptionSourceArtifactKind::RetainedReplay,
            "ordering:retained-replay",
            "digest:ordering:retained-replay",
        ),
        (
            BridgeSubscriptionSourceArtifactKind::StrategyLowering,
            "ordering:strategy-lowering",
            "digest:ordering:strategy-lowering",
        ),
        (
            BridgeSubscriptionSourceArtifactKind::Failure,
            "ordering:failure",
            "digest:ordering:failure",
        ),
    ]
    .into_iter()
    .map(|(artifact_kind, identity, digest)| {
        BridgeSubscriptionSourceArtifactInput::new(artifact_kind, identity, digest)
    })
    .collect()
}

fn ordering_product_ids() -> Vec<String> {
    (0..128)
        .map(|slot| format!("ordering-product-{slot:03}"))
        .collect()
}
