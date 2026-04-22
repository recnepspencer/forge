use super::{
    BridgeSubscriptionCertificationAssemblyPlan, BridgeSubscriptionCertificationBundleDraft,
    BridgeSubscriptionCertificationBundleSealed, BridgeSubscriptionCertificationCostProfile,
    BridgeSubscriptionCertificationDensityPosture, BridgeSubscriptionCertificationScratch,
    BridgeSubscriptionReferenceWorkloadManifestDraft,
    BridgeSubscriptionReferenceWorkloadManifestSealed, BridgeSubscriptionSourceArtifactIndex,
    BridgeSubscriptionSourceArtifactInput, BridgeSubscriptionSourceArtifactKind,
};

#[derive(Debug, Clone)]
pub(crate) struct BridgeSubscriptionCertificationReportBundleInput {
    pub(crate) declaration_digest: &'static str,
    pub(crate) basis_digest: &'static str,
    pub(crate) lifecycle_digest: &'static str,
    pub(crate) active_delivery_digest: &'static str,
    pub(crate) fanout_digest: &'static str,
    pub(crate) checkpoint_digest: &'static str,
    pub(crate) replay_digest: &'static str,
    pub(crate) continuation_digest: &'static str,
    pub(crate) preview_digest: &'static str,
    pub(crate) strategy_digest: &'static str,
    pub(crate) failure_digest: Option<&'static str>,
    pub(crate) rich_diagnostics: bool,
}

impl BridgeSubscriptionCertificationReportBundleInput {
    pub(crate) const fn stable() -> Self {
        Self {
            declaration_digest: "report-declaration-digest-stable",
            basis_digest: "report-basis-digest-stable",
            lifecycle_digest: "report-lifecycle-digest-stable",
            active_delivery_digest: "report-active-delivery-digest-stable",
            fanout_digest: "report-fanout-digest-stable",
            checkpoint_digest: "report-checkpoint-digest-stable",
            replay_digest: "report-replay-digest-stable",
            continuation_digest: "report-continuation-digest-stable",
            preview_digest: "report-preview-digest-stable",
            strategy_digest: "report-strategy-digest-stable",
            failure_digest: None,
            rich_diagnostics: false,
        }
    }
}

pub(crate) fn reference_manifest() -> BridgeSubscriptionReferenceWorkloadManifestSealed {
    BridgeSubscriptionReferenceWorkloadManifestDraft::new(
        (0..128)
            .map(|slot| format!("product-{slot:03}"))
            .collect::<Vec<_>>(),
        ["steel", "rubber", "copper", "glass", "labor"].to_vec(),
        [
            "authoritative-live",
            "historical-replay",
            "branch-local",
            "shared-fanout",
            "incompatible-sharing-rejection",
            "denied-continuation",
            "bundle-insufficiency",
            "strategy-lowering-provenance",
        ]
        .to_vec(),
    )
    .seal()
    .expect("certification report fixture manifest should seal")
}

pub(crate) fn assemble_reference_bundle(
    manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
    input: BridgeSubscriptionCertificationReportBundleInput,
) -> BridgeSubscriptionCertificationBundleSealed {
    let mut inputs = vec![
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::LaneIdentity,
            "report-lane",
            "report-lane-digest-stable",
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::Declaration,
            "report-declaration",
            input.declaration_digest,
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::BasisBinding,
            "report-basis",
            input.basis_digest,
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::Lifecycle,
            "report-lifecycle",
            input.lifecycle_digest,
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::AdmittedSubscription,
            "report-admitted-subscription",
            "report-admitted-digest-stable",
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::ActiveDelivery,
            "report-active-delivery",
            input.active_delivery_digest,
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::Fanout,
            "report-fanout",
            input.fanout_digest,
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::Checkpoint,
            "report-checkpoint",
            input.checkpoint_digest,
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::RetainedReplay,
            "report-replay",
            input.replay_digest,
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::Continuation,
            "report-continuation",
            input.continuation_digest,
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::Preview,
            "report-preview",
            input.preview_digest,
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::StrategyLowering,
            "report-strategy",
            input.strategy_digest,
        ),
    ];
    if let Some(failure_digest) = input.failure_digest {
        inputs.push(BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::Failure,
            "report-failure",
            failure_digest,
        ));
    }
    let index = BridgeSubscriptionSourceArtifactIndex::build(inputs);
    let plan = BridgeSubscriptionCertificationAssemblyPlan::plan(manifest, &index);
    let cost_profile = BridgeSubscriptionCertificationCostProfile::admit(
        BridgeSubscriptionCertificationDensityPosture::SparseCertificationWindow,
        16,
        16,
        32,
        input.rich_diagnostics,
    )
    .expect("certification report sparse cost profile should admit");
    let scratch = BridgeSubscriptionCertificationScratch::prepare(&cost_profile);
    BridgeSubscriptionCertificationBundleDraft::assemble(plan, cost_profile, scratch)
        .expect("certification report bundle should assemble")
        .seal()
}
