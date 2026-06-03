use super::{
    BridgeSubscriptionCertificationAssemblyPlan, BridgeSubscriptionCertificationBundleDraft,
    BridgeSubscriptionCertificationBundleSealed, BridgeSubscriptionCertificationCostProfile,
    BridgeSubscriptionCertificationDensityPosture, BridgeSubscriptionCertificationFieldExpectation,
    BridgeSubscriptionCertificationScratch, BridgeSubscriptionReferenceWorkloadComponentIdSet,
    BridgeSubscriptionReferenceWorkloadLaneIdSet, BridgeSubscriptionReferenceWorkloadManifestDraft,
    BridgeSubscriptionReferenceWorkloadManifestSealed,
    BridgeSubscriptionReferenceWorkloadProductIdSet, BridgeSubscriptionSourceArtifactIndex,
    BridgeSubscriptionSourceArtifactInput, BridgeSubscriptionSourceArtifactKind,
    BridgeSubscriptionSourceArtifactRole, BridgeSubscriptionSourceArtifactScenario,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BridgeSubscriptionCertificationReportBundleScenario {
    StableAdmitted,
    LatestUnretainedBasis,
    DivergentFanout,
    DeniedContinuation,
    CollectionMembershipStrategyLowering,
    BundleInsufficiency,
}

impl BridgeSubscriptionCertificationReportBundleScenario {
    const fn source_artifact_roles(self) -> BridgeSubscriptionCertificationReportArtifactRoles {
        match self {
            Self::StableAdmitted => BridgeSubscriptionCertificationReportArtifactRoles::stable(),
            Self::LatestUnretainedBasis => {
                BridgeSubscriptionCertificationReportArtifactRoles::latest_unretained_basis()
            }
            Self::DivergentFanout => {
                BridgeSubscriptionCertificationReportArtifactRoles::divergent_fanout()
            }
            Self::DeniedContinuation => {
                BridgeSubscriptionCertificationReportArtifactRoles::denied_continuation()
            }
            Self::CollectionMembershipStrategyLowering => {
                BridgeSubscriptionCertificationReportArtifactRoles::collection_membership_strategy_lowering()
            }
            Self::BundleInsufficiency => BridgeSubscriptionCertificationReportArtifactRoles::stable(),
        }
    }

    const fn field_expectation(self) -> BridgeSubscriptionCertificationFieldExpectation {
        match self {
            Self::BundleInsufficiency => {
                BridgeSubscriptionCertificationFieldExpectation::RetainedArtifactCompletenessRequirement
            }
            Self::StableAdmitted
            | Self::LatestUnretainedBasis
            | Self::DivergentFanout
            | Self::DeniedContinuation
            | Self::CollectionMembershipStrategyLowering => {
                BridgeSubscriptionCertificationFieldExpectation::CompleteReferenceBundle
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct BridgeSubscriptionCertificationReportArtifactRoles {
    declaration_role: BridgeSubscriptionSourceArtifactRole,
    basis_role: BridgeSubscriptionSourceArtifactRole,
    lifecycle_role: BridgeSubscriptionSourceArtifactRole,
    active_delivery_role: BridgeSubscriptionSourceArtifactRole,
    fanout_role: BridgeSubscriptionSourceArtifactRole,
    checkpoint_role: BridgeSubscriptionSourceArtifactRole,
    replay_role: BridgeSubscriptionSourceArtifactRole,
    continuation_role: BridgeSubscriptionSourceArtifactRole,
    preview_role: BridgeSubscriptionSourceArtifactRole,
    strategy_role: BridgeSubscriptionSourceArtifactRole,
    failure_role: Option<BridgeSubscriptionSourceArtifactRole>,
    rich_diagnostics: bool,
}

impl BridgeSubscriptionCertificationReportArtifactRoles {
    const fn stable() -> Self {
        Self {
            declaration_role: BridgeSubscriptionSourceArtifactRole::Stable,
            basis_role: BridgeSubscriptionSourceArtifactRole::Stable,
            lifecycle_role: BridgeSubscriptionSourceArtifactRole::Stable,
            active_delivery_role: BridgeSubscriptionSourceArtifactRole::Stable,
            fanout_role: BridgeSubscriptionSourceArtifactRole::Stable,
            checkpoint_role: BridgeSubscriptionSourceArtifactRole::Stable,
            replay_role: BridgeSubscriptionSourceArtifactRole::Stable,
            continuation_role: BridgeSubscriptionSourceArtifactRole::Stable,
            preview_role: BridgeSubscriptionSourceArtifactRole::Stable,
            strategy_role: BridgeSubscriptionSourceArtifactRole::Stable,
            failure_role: None,
            rich_diagnostics: false,
        }
    }

    const fn latest_unretained_basis() -> Self {
        Self {
            basis_role: BridgeSubscriptionSourceArtifactRole::Divergent,
            ..Self::stable()
        }
    }

    const fn divergent_fanout() -> Self {
        Self {
            fanout_role: BridgeSubscriptionSourceArtifactRole::DivergentFanout,
            ..Self::stable()
        }
    }

    const fn denied_continuation() -> Self {
        Self {
            continuation_role: BridgeSubscriptionSourceArtifactRole::DeniedContinuation,
            ..Self::stable()
        }
    }

    const fn collection_membership_strategy_lowering() -> Self {
        Self {
            strategy_role: BridgeSubscriptionSourceArtifactRole::CollectionMembershipIndex,
            ..Self::stable()
        }
    }
}

pub(crate) fn reference_manifest() -> BridgeSubscriptionReferenceWorkloadManifestSealed {
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
            "shared-fanout",
            "divergent-sharing-rejection",
            "denied-continuation",
            "bundle-insufficiency",
            "strategy-lowering-provenance",
        ]),
    )
    .seal()
    .expect("certification report fixture manifest should seal")
}

pub(crate) fn assemble_reference_bundle(
    manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
    scenario: BridgeSubscriptionCertificationReportBundleScenario,
) -> BridgeSubscriptionCertificationBundleSealed {
    let artifact_roles = scenario.source_artifact_roles();
    let mut inputs = vec![
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::LaneIdentity,
            BridgeSubscriptionSourceArtifactRole::Stable,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::Declaration,
            artifact_roles.declaration_role,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::BasisBinding,
            artifact_roles.basis_role,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::Lifecycle,
            artifact_roles.lifecycle_role,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::AdmittedSubscription,
            BridgeSubscriptionSourceArtifactRole::Stable,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::ActiveDelivery,
            artifact_roles.active_delivery_role,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::Fanout,
            artifact_roles.fanout_role,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::Checkpoint,
            artifact_roles.checkpoint_role,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::RetainedReplay,
            artifact_roles.replay_role,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::Continuation,
            artifact_roles.continuation_role,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::Preview,
            artifact_roles.preview_role,
        ),
        source_artifact(
            BridgeSubscriptionSourceArtifactKind::StrategyLowering,
            artifact_roles.strategy_role,
        ),
    ];
    if let Some(failure_role) = artifact_roles.failure_role {
        inputs.push(source_artifact(
            BridgeSubscriptionSourceArtifactKind::Failure,
            failure_role,
        ));
    }
    let index = BridgeSubscriptionSourceArtifactIndex::build(inputs);
    let plan = BridgeSubscriptionCertificationAssemblyPlan::plan_with_field_expectation(
        manifest,
        &index,
        scenario.field_expectation(),
    );
    let cost_profile = BridgeSubscriptionCertificationCostProfile::admit(
        BridgeSubscriptionCertificationDensityPosture::SparseCertificationWindow,
        16,
        16,
        32,
        artifact_roles.rich_diagnostics,
    )
    .expect("certification report sparse cost profile should admit");
    let scratch = BridgeSubscriptionCertificationScratch::prepare(&cost_profile);
    BridgeSubscriptionCertificationBundleDraft::assemble(plan, cost_profile, scratch)
        .expect("certification report bundle should assemble")
        .seal()
}

fn source_artifact(
    artifact_kind: BridgeSubscriptionSourceArtifactKind,
    role: BridgeSubscriptionSourceArtifactRole,
) -> BridgeSubscriptionSourceArtifactInput {
    BridgeSubscriptionSourceArtifactInput::from_evidence(
        super::BridgeSubscriptionSourceArtifactEvidence::scenario(
            artifact_kind,
            BridgeSubscriptionSourceArtifactScenario::CertificationReport,
            role,
        ),
    )
}
