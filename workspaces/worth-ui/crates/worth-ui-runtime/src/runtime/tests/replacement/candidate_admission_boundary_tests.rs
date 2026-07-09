use std::{collections::BTreeMap, path::Path};

use worth_query::facade::{
    discover_basis_lifecycle_support, BasisFamily, QuerySubscriptionFamily,
    QuerySubscriptionSupportPosture, ResultShapeFamily, ViewShapeDescriptor,
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily,
};

use crate::capability::{
    QueryBasisPostureReference, QueryDenialPresentation, QueryLiveCompatibility,
    QueryResultShapeReference, QueryViewCapabilityReference, ViewBindingDescriptor,
    ViewBindingFamily, ViewBindingId,
};
use crate::facade::{WorthUi, WorthUiApp};
use crate::runtime::admission::{
    WorthUiCandidateAdmission, WorthUiCandidateAdmissionDenial, WorthUiQuerySupportReceipt,
    WorthUiQuerySupportStatus, WorthUiRuntimeReplacementPosture,
};
use crate::runtime::candidate::rust_authored_replacement_candidate;
use crate::runtime::{
    WorthUiCandidateArtifactBundle, WorthUiCandidateDependencyMetadata,
    WorthUiCandidateLoweringBasis, WorthUiReplacementCandidate, WorthUiReplacementCause,
    WorthUiRuntimeHost, WorthUiRuntimeLaunch,
};
use crate::source::{
    WorthUiArtifact, WorthUiArtifactDigestor, WorthUiArtifactEquivalenceBasis,
    WorthUiArtifactHandle, WorthUiArtifactIdentitySeed, WorthUiArtifactImportHandle,
    WorthUiArtifactImportNode, WorthUiArtifactInputReference, WorthUiArtifactModule,
    WorthUiArtifactNode, WorthUiBindingSemanticsLowerer, WorthUiCanonicalArtifactAssembler,
    WorthUiDurableStateEligibility, WorthUiDurableStateIneligibilityReason,
    WorthUiIdentitySeedLowerer, WorthUiRustAuthoredArtifactInput,
    WorthUiRustAuthoredArtifactInputModule, WorthUiRustAuthoredToArtifactInputLowerer,
    WorthUiSourceModuleId, WorthUiStructuralLegalityLowerer,
};

#[test]
fn same_candidate_and_same_active_basis_admit_equivalently() {
    let app = WorthUi::app().freeze();
    let left = replacement_candidate(&app, ["app/panels/inspector.wui"]);
    let right = replacement_candidate(&app, ["app/panels/inspector.wui"]);
    let runtime = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let active_basis = runtime.replacement_admission_basis();

    let left_admitted = WorthUiCandidateAdmission::for_active_basis(active_basis)
        .admit(left)
        .expect("left candidate admits");
    let right_admitted = WorthUiCandidateAdmission::for_active_basis(active_basis)
        .admit(right)
        .expect("right candidate admits");

    assert_eq!(left_admitted.report(), right_admitted.report());
    assert_eq!(left_admitted.active_basis(), active_basis);
    assert_eq!(left_admitted.verify_receipts_unchanged(), Ok(()));
}

#[test]
fn snapshot_mismatch_rejected_before_equivalence_comparison() {
    let app = WorthUi::app().freeze();
    let runtime = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let active_basis = runtime.replacement_admission_basis();
    let candidate = candidate_with_lowering_basis(
        ["app/panels/inspector.wui"],
        WorthUiCandidateLoweringBasis::from_raw_parts_for_test(
            active_basis.snapshot_digest() ^ 0x55aa,
            WorthUiQuerySupportReceipt::for_test(WorthUiQuerySupportStatus::Supported, 10),
        ),
    );

    let report = WorthUiCandidateAdmission::for_active_basis(active_basis)
        .admit(candidate)
        .expect_err("mismatched snapshot denies");

    assert_eq!(
        report.denial(),
        Some(WorthUiCandidateAdmissionDenial::SnapshotMismatch {
            candidate_snapshot_digest: active_basis.snapshot_digest() ^ 0x55aa,
            active_snapshot_digest: active_basis.snapshot_digest(),
        })
    );
    assert_eq!(report.counters().snapshot_compatibility_checks(), 1);
    assert_eq!(report.counters().runtime_posture_checks(), 0);
    assert_eq!(report.counters().query_support_checks(), 0);
    assert_eq!(report.counters().artifact_comparisons(), 0);
    assert_eq!(report.counters().plan_lowering_attempts(), 0);
}

#[test]
fn deferred_runtime_posture_rejected_before_plan_lowering() {
    let app = WorthUi::app().freeze();
    let runtime = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let active_basis = runtime
        .replacement_admission_basis()
        .with_replacement_posture_for_test(WorthUiRuntimeReplacementPosture::Deferred);
    let candidate = replacement_candidate(&app, ["app/panels/inspector.wui"]);

    let report = WorthUiCandidateAdmission::for_active_basis(active_basis)
        .admit(candidate)
        .expect_err("deferred runtime posture denies");

    assert_eq!(
        report.denial(),
        Some(WorthUiCandidateAdmissionDenial::DeferredRuntimePosture {
            posture: WorthUiRuntimeReplacementPosture::Deferred,
        })
    );
    assert_eq!(report.counters().runtime_posture_checks(), 1);
    assert_eq!(report.counters().query_support_checks(), 0);
    assert_eq!(report.counters().artifact_comparisons(), 0);
    assert_eq!(report.counters().plan_lowering_attempts(), 0);
}

#[test]
fn unsupported_runtime_posture_rejected_before_plan_lowering() {
    let app = WorthUi::app().freeze();
    let runtime = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let active_basis = runtime
        .replacement_admission_basis()
        .with_replacement_posture_for_test(WorthUiRuntimeReplacementPosture::Unsupported);
    let candidate = replacement_candidate(&app, ["app/panels/inspector.wui"]);

    let report = WorthUiCandidateAdmission::for_active_basis(active_basis)
        .admit(candidate)
        .expect_err("unsupported runtime posture denies");

    assert_eq!(
        report.denial(),
        Some(WorthUiCandidateAdmissionDenial::UnsupportedRuntimePosture {
            posture: WorthUiRuntimeReplacementPosture::Unsupported,
        })
    );
    assert_eq!(report.counters().runtime_posture_checks(), 1);
    assert_eq!(report.counters().query_support_checks(), 0);
    assert_eq!(report.counters().artifact_comparisons(), 0);
    assert_eq!(report.counters().plan_lowering_attempts(), 0);
}

#[test]
fn deferred_query_support_rejected_before_plan_lowering() {
    let app = WorthUi::app().freeze();
    let runtime = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let active_basis = runtime.replacement_admission_basis();
    let query_receipt =
        WorthUiQuerySupportReceipt::for_test(WorthUiQuerySupportStatus::Deferred, 123);
    let candidate = candidate_with_lowering_basis(
        ["app/panels/inspector.wui"],
        WorthUiCandidateLoweringBasis::from_raw_parts_for_test(
            active_basis.snapshot_digest(),
            query_receipt,
        ),
    );

    let report = WorthUiCandidateAdmission::for_active_basis(active_basis)
        .admit(candidate)
        .expect_err("deferred query support denies");

    assert_eq!(
        report.denial(),
        Some(WorthUiCandidateAdmissionDenial::DeferredQuerySupport {
            receipt: query_receipt,
        })
    );
    assert_eq!(report.counters().query_support_checks(), 1);
    assert_eq!(report.counters().artifact_comparisons(), 0);
    assert_eq!(report.counters().plan_lowering_attempts(), 0);
}

#[test]
fn unsupported_query_support_rejected_before_plan_lowering() {
    let app = WorthUi::app().freeze();
    let runtime = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let active_basis = runtime.replacement_admission_basis();
    let query_receipt =
        WorthUiQuerySupportReceipt::for_test(WorthUiQuerySupportStatus::Unsupported, 456);
    let candidate = candidate_with_lowering_basis(
        ["app/panels/inspector.wui"],
        WorthUiCandidateLoweringBasis::from_raw_parts_for_test(
            active_basis.snapshot_digest(),
            query_receipt,
        ),
    );

    let report = WorthUiCandidateAdmission::for_active_basis(active_basis)
        .admit(candidate)
        .expect_err("unsupported query support denies");

    assert_eq!(
        report.denial(),
        Some(WorthUiCandidateAdmissionDenial::UnsupportedQuerySupport {
            receipt: query_receipt,
        })
    );
    assert_eq!(report.counters().query_support_checks(), 1);
    assert_eq!(report.counters().artifact_comparisons(), 0);
    assert_eq!(report.counters().plan_lowering_attempts(), 0);
}

#[test]
fn query_support_receipt_is_derived_from_runtime_dependency_metadata() {
    let app = query_bound_app(QuerySubscriptionSupportPosture::RuntimeBackedCertified);
    let artifact = query_bound_artifact(&app);
    let runtime = launch_runtime(&app, artifact.clone());
    let active_basis = runtime.replacement_admission_basis();
    let candidate = rust_authored_replacement_candidate(
        artifact,
        app.capabilities().digest(),
        WorthUiReplacementCause::manual_refresh(3),
    )
    .expect("query-bound candidate seals through dependency metadata");

    let admitted = WorthUiCandidateAdmission::for_active_basis(active_basis)
        .admit(candidate)
        .expect("query-supported candidate admits");

    let receipt = admitted.report().query_support_receipt();
    assert_eq!(receipt.status(), WorthUiQuerySupportStatus::Supported);
    assert_eq!(receipt.runtime_hook_count(), 4);
    assert_eq!(
        admitted.report().counters().snapshot_compatibility_checks(),
        1
    );
    assert_eq!(admitted.report().counters().runtime_posture_checks(), 1);
    assert_eq!(admitted.report().counters().query_support_checks(), 1);
    assert_eq!(admitted.report().counters().artifact_comparisons(), 0);
    assert_eq!(admitted.report().counters().plan_lowering_attempts(), 0);
}

#[test]
fn admitted_candidate_cannot_swap_query_support_receipts_after_admission() {
    let app = WorthUi::app().freeze();
    let runtime = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let active_basis = runtime.replacement_admission_basis();
    let candidate = replacement_candidate(&app, ["app/panels/inspector.wui"]);

    let admitted = WorthUiCandidateAdmission::for_active_basis(active_basis)
        .admit(candidate)
        .expect("candidate admits before receipt tampering");
    let admitted_digest = admitted.report().query_support_receipt().receipt_digest();

    assert_eq!(
        admitted.verify_test_receipt_digest(admitted_digest ^ 0x99),
        Err(
            WorthUiCandidateAdmissionDenial::QuerySupportReceiptChanged {
                admitted_receipt_digest: admitted_digest,
                current_receipt_digest: admitted_digest ^ 0x99,
            }
        )
    );
    assert_eq!(admitted.report().counters().artifact_comparisons(), 0);
    assert_eq!(admitted.report().counters().plan_lowering_attempts(), 0);
}

fn replacement_candidate<const N: usize>(
    app: &WorthUiApp,
    targets: [&str; N],
) -> WorthUiReplacementCandidate {
    rust_authored_replacement_candidate(
        import_artifact(targets),
        app.capabilities().digest(),
        WorthUiReplacementCause::manual_refresh(1),
    )
    .expect("candidate seals")
}

fn candidate_with_lowering_basis<const N: usize>(
    targets: [&str; N],
    lowering_basis: WorthUiCandidateLoweringBasis,
) -> WorthUiReplacementCandidate {
    let artifact = import_artifact(targets);
    let artifact_digest =
        WorthUiArtifactDigestor::digest(&artifact, WorthUiArtifactEquivalenceBasis::semantic());
    let dependency_metadata = WorthUiCandidateDependencyMetadata::derive_for_artifact(&artifact);
    let bundle = WorthUiCandidateArtifactBundle::from_optional_parts_for_test(
        artifact,
        Some(artifact_digest),
        Some(dependency_metadata),
        Some(lowering_basis),
    )
    .expect("test candidate bundle seals");
    WorthUiReplacementCandidate::from_artifact_bundle(
        bundle,
        WorthUiReplacementCause::manual_refresh(2),
        crate::runtime::WorthUiCandidateAuthoringLane::rust_authored(),
    )
    .expect("test candidate seals")
}

fn launch_runtime(app: &WorthUiApp, artifact: WorthUiArtifact) -> WorthUiRuntimeHost {
    app.launch_runtime(WorthUiRuntimeLaunch::from_canonical_artifact(artifact))
        .expect("runtime launches from canonical artifact")
}

fn query_bound_app(live_posture: QuerySubscriptionSupportPosture) -> WorthUiApp {
    WorthUi::app()
        .register_view_binding(query_bound_view_binding(live_posture))
        .freeze()
}

fn query_bound_view_binding(
    live_posture: QuerySubscriptionSupportPosture,
) -> ViewBindingDescriptor {
    let support_report = WorthQueryApplicationFacade::runtime_backed_default().support_report();
    let query_capability = support_report
        .support_matrix()
        .descriptor(WorthQueryCapabilityFamily::QueryComposition)
        .expect("query composition support posture");
    let query_composition = support_report
        .query_composition_support_profile()
        .expect("query composition profile");
    let basis_support =
        discover_basis_lifecycle_support(BasisFamily::CurrentHead, "subscription_declaration");

    ViewBindingDescriptor::query_owned(
        ViewBindingId::new("workspace.view_binding.selection").unwrap(),
        ViewBindingFamily::collection(),
    )
    .with_query_capability_posture(
        QueryViewCapabilityReference::from_query_capability_descriptor(query_capability),
    )
    .with_query_composition_support(query_composition)
    .with_view_shape(ViewShapeDescriptor::table())
    .with_result_shape(QueryResultShapeReference::from_result_shape_family(
        ResultShapeFamily::Collection,
    ))
    .with_basis_posture(QueryBasisPostureReference::from_basis_support_discovery(
        &basis_support,
    ))
    .with_live_compatibility(QueryLiveCompatibility::from_subscription_posture(
        QuerySubscriptionFamily::CollectionMembership,
        live_posture,
    ))
    .with_denial_presentation(QueryDenialPresentation::structured_status())
}

fn query_bound_artifact(app: &WorthUiApp) -> WorthUiArtifact {
    let artifact_input = WorthUiRustAuthoredToArtifactInputLowerer::lower(
        &WorthUiRustAuthoredArtifactInput::from_modules([query_bound_module()]),
    );
    let resolved =
        crate::source::WorthUiArtifactInputResolver::resolve(&artifact_input, app.capabilities())
            .expect("query-bound artifact resolves");
    let structured = WorthUiStructuralLegalityLowerer::lower(&resolved, app.capabilities())
        .expect("query-bound artifact is structurally legal");
    let bound = WorthUiBindingSemanticsLowerer::lower(&structured, app.capabilities())
        .expect("query-bound artifact preserves binding semantics");
    let identity_seeded = WorthUiIdentitySeedLowerer::lower(&bound)
        .expect("query-bound artifact gets identity seeds")
        .0;
    WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .expect("query-bound artifact assembles")
}

fn query_bound_module() -> WorthUiRustAuthoredArtifactInputModule {
    WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_binding("workspace.view_binding.selection")
}

fn import_artifact<const N: usize>(targets: [&str; N]) -> WorthUiArtifact {
    let module_id = module_id("app/main.wui");
    let nodes = targets
        .into_iter()
        .enumerate()
        .map(|(node_index, target)| import_node(&module_id, node_index, target))
        .collect::<Vec<_>>();
    let module = WorthUiArtifactModule::new(module_id.clone(), nodes);
    let mut modules = BTreeMap::new();
    modules.insert(module_id.clone(), module);

    WorthUiArtifact::new(modules, vec![module_id])
}

fn import_node(
    module_id: &WorthUiSourceModuleId,
    node_index: usize,
    target: &str,
) -> WorthUiArtifactNode {
    WorthUiArtifactNode::Import(WorthUiArtifactImportNode::new(
        WorthUiArtifactHandle::Import(WorthUiArtifactImportHandle::new(
            module_id.clone(),
            node_index,
        )),
        WorthUiArtifactInputReference::new(target),
        0,
        WorthUiArtifactIdentitySeed::structural_fallback(format!(
            "module:{}|import:{}",
            module_id.as_str(),
            target
        )),
        WorthUiDurableStateEligibility::Ineligible {
            reason: WorthUiDurableStateIneligibilityReason::NoDurableStateSurface,
        },
    ))
}

fn module_id(path: &str) -> WorthUiSourceModuleId {
    WorthUiSourceModuleId::from_relative_path(Path::new(path)).expect("valid module id")
}
