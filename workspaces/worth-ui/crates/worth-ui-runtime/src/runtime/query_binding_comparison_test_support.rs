use forge_query::facade::{
    discover_basis_lifecycle_support, BasisFamily, ForgeQueryApplicationFacade,
    ForgeQueryCapabilityFamily, QuerySubscriptionFamily, QuerySubscriptionSupportPosture,
    ResultShapeFamily, ViewShapeDescriptor,
};

use crate::capability::{
    QueryBasisPostureReference, QueryDenialPresentation, QueryLiveCompatibility,
    QueryResultShapeReference, QueryViewCapabilityReference, ViewBindingDescriptor,
    ViewBindingFamily, ViewBindingId,
};
use crate::facade::{WorthUi, WorthUiApp};
use crate::runtime::dependency_impact_narrowing_test_support::lower_rust_authored_artifact;
use crate::runtime::replacement_impact_test_support::{admitted_candidate, launch_runtime};
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiNodeReplacementPlan, WorthUiRuntimeHost,
    WorthUiRuntimeImpactNarrowing,
};
use crate::source::{WorthUiArtifact, WorthUiRustAuthoredArtifactInputModule};

pub(super) fn standard_query_app() -> WorthUiApp {
    query_app(
        "subscription_declaration",
        QueryDenialPresentation::structured_status(),
        ResultShapeFamily::Collection,
        QuerySubscriptionFamily::CollectionMembership,
        QuerySubscriptionSupportPosture::RuntimeBackedCertified,
    )
}

pub(super) fn basis_drift_query_app() -> WorthUiApp {
    query_app(
        "observation",
        QueryDenialPresentation::structured_status(),
        ResultShapeFamily::Collection,
        QuerySubscriptionFamily::CollectionMembership,
        QuerySubscriptionSupportPosture::RuntimeBackedCertified,
    )
}

pub(super) fn denial_presentation_drift_query_app() -> WorthUiApp {
    query_app(
        "subscription_declaration",
        QueryDenialPresentation::advisory_text(),
        ResultShapeFamily::Collection,
        QuerySubscriptionFamily::CollectionMembership,
        QuerySubscriptionSupportPosture::RuntimeBackedCertified,
    )
}

pub(super) fn result_shape_drift_query_app() -> WorthUiApp {
    query_app(
        "subscription_declaration",
        QueryDenialPresentation::structured_status(),
        ResultShapeFamily::Detail,
        QuerySubscriptionFamily::DetailExact,
        QuerySubscriptionSupportPosture::RuntimeBackedCertified,
    )
}

pub(super) fn query_artifact(app: &WorthUiApp, binding_id: &str) -> WorthUiArtifact {
    lower_rust_authored_artifact(
        app,
        [WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_binding(binding_id)],
    )
}

pub(super) fn phase11_pipeline(
    active_app: &WorthUiApp,
    active_artifact: WorthUiArtifact,
    candidate_artifact: WorthUiArtifact,
) -> (
    WorthUiRuntimeHost,
    WorthUiAdmittedReplacementCandidate,
    WorthUiRuntimeImpactNarrowing,
    WorthUiNodeReplacementPlan,
) {
    let runtime = launch_runtime(active_app, active_artifact);
    let admitted = admitted_candidate(active_app, &runtime, candidate_artifact);
    let artifact_comparison = runtime
        .compare_admitted_replacement(&admitted)
        .expect("runtime comparison succeeds");
    let impact = runtime
        .classify_replacement_impact(&artifact_comparison, &admitted)
        .expect("impact classification succeeds");
    let narrowing = runtime
        .narrow_replacement_impact(&impact, &admitted)
        .expect("impact narrowing succeeds");
    let identity_report = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect("identity report succeeds");
    let plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("node replacement plan succeeds");
    (runtime, admitted, narrowing, plan)
}

fn query_app(
    operation_lane: &'static str,
    denial: QueryDenialPresentation,
    result_shape: ResultShapeFamily,
    subscription_family: QuerySubscriptionFamily,
    subscription_posture: QuerySubscriptionSupportPosture,
) -> WorthUiApp {
    WorthUi::app()
        .register_view_binding(query_binding(
            "workspace.view_binding.selection",
            operation_lane,
            denial.clone(),
            result_shape.clone(),
            subscription_family.clone(),
            subscription_posture,
        ))
        .register_view_binding(query_binding(
            "workspace.view_binding.detail",
            operation_lane,
            denial,
            result_shape,
            subscription_family,
            subscription_posture,
        ))
        .freeze()
}

fn query_binding(
    id: &str,
    operation_lane: &'static str,
    denial: QueryDenialPresentation,
    result_shape: ResultShapeFamily,
    subscription_family: QuerySubscriptionFamily,
    subscription_posture: QuerySubscriptionSupportPosture,
) -> ViewBindingDescriptor {
    let support_report = ForgeQueryApplicationFacade::runtime_backed_default().support_report();
    let query_capability = support_report
        .support_matrix()
        .descriptor(ForgeQueryCapabilityFamily::QueryComposition)
        .expect("query composition support posture");
    let query_composition = support_report
        .query_composition_support_profile()
        .expect("query composition profile");
    let basis_support = discover_basis_lifecycle_support(BasisFamily::CurrentHead, operation_lane);

    ViewBindingDescriptor::query_owned(
        ViewBindingId::new(id).expect("valid view binding id"),
        ViewBindingFamily::collection(),
    )
    .with_query_capability_posture(
        QueryViewCapabilityReference::from_query_capability_descriptor(query_capability),
    )
    .with_query_composition_support(query_composition)
    .with_view_shape(ViewShapeDescriptor::table())
    .with_result_shape(QueryResultShapeReference::from_result_shape_family(
        result_shape,
    ))
    .with_basis_posture(QueryBasisPostureReference::from_basis_support_discovery(
        &basis_support,
    ))
    .with_live_compatibility(QueryLiveCompatibility::from_subscription_posture(
        subscription_family,
        subscription_posture,
    ))
    .with_denial_presentation(denial)
}
