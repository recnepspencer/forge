mod comparison_reports;
mod manifest_cost_and_schema;
mod precedence_ordering_and_bundle;

use super::support::*;
use crate::facade::{
    BridgeSubscriptionReferenceWorkloadComponentIdSet,
    BridgeSubscriptionReferenceWorkloadLaneIdSet, BridgeSubscriptionReferenceWorkloadProductIdSet,
    RuntimeBridge,
};

fn product_ids() -> BridgeSubscriptionReferenceWorkloadProductIdSet {
    BridgeSubscriptionReferenceWorkloadProductIdSet::from_declared_product_labels(
        (0..128).map(|slot| format!("product-{slot:03}")),
    )
}

fn component_ids() -> BridgeSubscriptionReferenceWorkloadComponentIdSet {
    BridgeSubscriptionReferenceWorkloadComponentIdSet::from_declared_component_labels([
        "steel", "rubber", "copper", "glass", "labor",
    ])
}

fn lane_ids() -> BridgeSubscriptionReferenceWorkloadLaneIdSet {
    BridgeSubscriptionReferenceWorkloadLaneIdSet::from_declared_lane_labels([
        "authoritative-live",
        "historical-replay",
        "branch-local",
        "preview-discard",
    ])
}

fn sealed_certification_bundle(
    runtime: &RuntimeBridge,
    source_inputs: Vec<crate::facade::BridgeSubscriptionSourceArtifactInput>,
    rich_diagnostics_admitted: bool,
) -> crate::facade::BridgeSubscriptionCertificationBundleSealed {
    let manifest = runtime
        .declare_subscription_reference_workload_manifest(
            product_ids(),
            component_ids(),
            lane_ids(),
        )
        .expect("manifest should seal");
    let index = runtime.build_subscription_certification_source_index(source_inputs);
    let plan = runtime.plan_subscription_certification_bundle(&manifest, &index);
    let cost_profile = runtime
        .admit_subscription_certification_cost_profile(
            crate::facade::BridgeSubscriptionCertificationDensityPosture::SparseCertificationWindow,
            16,
            16,
            32,
            rich_diagnostics_admitted,
        )
        .expect("sparse certification profile should admit");
    let scratch = runtime.prepare_subscription_certification_scratch(&cost_profile);
    let draft = runtime
        .assemble_subscription_certification_bundle(plan, cost_profile, scratch)
        .expect("admitted certification bundle should assemble");
    runtime.seal_subscription_certification_bundle(draft)
}

fn active_source_inputs(
    declaration_role: crate::facade::BridgeSubscriptionSourceArtifactRole,
    strategy_role: crate::facade::BridgeSubscriptionSourceArtifactRole,
) -> Vec<crate::facade::BridgeSubscriptionSourceArtifactInput> {
    vec![
        source_artifact(
            crate::facade::BridgeSubscriptionSourceArtifactKind::Declaration,
            declaration_role,
        ),
        source_artifact(
            crate::facade::BridgeSubscriptionSourceArtifactKind::AdmittedSubscription,
            crate::facade::BridgeSubscriptionSourceArtifactRole::Stable,
        ),
        source_artifact(
            crate::facade::BridgeSubscriptionSourceArtifactKind::ActiveDelivery,
            crate::facade::BridgeSubscriptionSourceArtifactRole::Stable,
        ),
        source_artifact(
            crate::facade::BridgeSubscriptionSourceArtifactKind::StrategyLowering,
            strategy_role,
        ),
    ]
}

fn source_artifact(
    artifact_kind: crate::facade::BridgeSubscriptionSourceArtifactKind,
    role: crate::facade::BridgeSubscriptionSourceArtifactRole,
) -> crate::facade::BridgeSubscriptionSourceArtifactInput {
    crate::facade::BridgeSubscriptionSourceArtifactInput::from_evidence(
        crate::facade::BridgeSubscriptionSourceArtifactEvidence::scenario(
            artifact_kind,
            crate::facade::BridgeSubscriptionSourceArtifactScenario::CertificationBundle,
            role,
        ),
    )
}
