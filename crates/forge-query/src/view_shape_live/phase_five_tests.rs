use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, RawAuthoredQuery,
    RawAuthoredResultShape, RootEntityKey,
};
use crate::basis::{
    resolve_snapshot_basis, BasisAuthorityFamily, BasisResolutionMode, ExecutionBasisIntent,
    ResolvedSnapshotIdentity, SnapshotLineageClass,
};
use crate::identity::{BasisDigest, CanonicalQueryDigest, SchemaBasisDigest};
use crate::identity_evolution::{
    admit_identity_evolution_query_for_scenario, execute_admitted_identity_evolution_query,
    CorrespondenceIdentityComparison, IdentityEvolutionCertificationResultEvidence,
    IdentityEvolutionComparisonBasisFamily, IdentityEvolutionQueryContext,
    IdentityEvolutionSyntheticScenario, InspectorIdentityArtifact, InspectorIdentityClassification,
    LineageTraversalDescriptor,
};
use crate::live::{BridgeChangeSummary, BridgeFieldDelta};
use crate::view_shape::{
    admit_view_shape, plan_admitted_view_shape, validate_canonical_bundle_for_admitted_view_shape,
    ViewShapeComplexityStatus, ViewShapeDescriptor,
};

use super::{
    execute_live_view_shape_change, lower_view_shape_plan_to_live, ViewShapeLiveFailureClass,
    ViewShapePatchFamily, ViewShapePatchPayload,
};

fn schema_view() -> crate::schema_view::QuerySchemaView {
    crate::schema_view::QuerySchemaView::new(
        "phase-five-inspector-view-closure",
        [
            crate::schema_view::SchemaFieldView::new(
                "identity",
                "id",
                crate::schema_view::SchemaFieldKind::String,
            ),
            crate::schema_view::SchemaFieldView::new(
                "profile",
                "display_name",
                crate::schema_view::SchemaFieldKind::String,
            ),
        ],
        [],
    )
}

fn detail_canonical() -> crate::canonicalization::CanonicalQueryBundle {
    GuidedAuthoringPath::canonicalize_detail(
        RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        RawAuthoredResultShape::detail_builder()
            .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
            .field(
                AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap(),
            )
            .build()
            .unwrap(),
    )
    .unwrap()
}

fn runtime_basis(schema_basis: SchemaBasisDigest) -> crate::basis::ResolvedSnapshotBasis {
    resolve_snapshot_basis(
        ExecutionBasisIntent::new(
            BasisAuthorityFamily::Runtime,
            SnapshotLineageClass::CurrentHead,
            false,
        ),
        ResolvedSnapshotIdentity::new(
            BasisAuthorityFamily::Runtime,
            None,
            "phase-five-snapshot",
            schema_basis,
            SnapshotLineageClass::CurrentHead,
        ),
        BasisResolutionMode::RuntimeDirect,
    )
    .unwrap()
}

fn planned_view(
    canonical: &crate::canonicalization::CanonicalQueryBundle,
    descriptor: ViewShapeDescriptor,
) -> crate::view_shape::ViewShapePlanArtifact {
    let admitted = admit_view_shape(canonical, descriptor).unwrap();
    let validated =
        validate_canonical_bundle_for_admitted_view_shape(canonical, schema_view(), admitted)
            .unwrap();
    plan_admitted_view_shape(
        validated,
        ExecutionBasisIntent::new(
            BasisAuthorityFamily::Runtime,
            SnapshotLineageClass::CurrentHead,
            false,
        ),
    )
    .unwrap()
}

fn identity_query_digest(label: &str) -> CanonicalQueryDigest {
    CanonicalQueryDigest::from_parts(&[format!("phase-five-inspector:{label}")])
}

fn identity_basis_digest(label: &str) -> BasisDigest {
    BasisDigest::from_parts(&[format!("phase-five-inspector:{label}")])
}

fn inspector_identity_artifact(
    classification: InspectorIdentityClassification,
) -> InspectorIdentityArtifact {
    let (context, scenario) = match classification {
        InspectorIdentityClassification::AuthoritativeContinuity => (
            IdentityEvolutionQueryContext::lineage_traversal(
                identity_query_digest("authoritative"),
                identity_basis_digest("authoritative-basis"),
                LineageTraversalDescriptor::direct_replacement("anchor"),
            ),
            IdentityEvolutionSyntheticScenario::Standard,
        ),
        InspectorIdentityClassification::IdentityBreak => (
            IdentityEvolutionQueryContext::lineage_traversal(
                identity_query_digest("identity-break"),
                identity_basis_digest("identity-break-basis"),
                LineageTraversalDescriptor::branch_local_direct_evolution("anchor"),
            ),
            IdentityEvolutionSyntheticScenario::IdentityBreak,
        ),
        InspectorIdentityClassification::AdvisoryCandidates => (
            IdentityEvolutionQueryContext::correspondence_identity_comparison(
                identity_query_digest("advisory"),
                IdentityEvolutionComparisonBasisFamily::BranchToBranch,
                identity_basis_digest("left"),
                identity_basis_digest("right"),
                CorrespondenceIdentityComparison::advisory_between("left-id", "right-id"),
            ),
            IdentityEvolutionSyntheticScenario::Standard,
        ),
        other => panic!("unsupported test identity classification: {other:?}"),
    };
    let admitted = admit_identity_evolution_query_for_scenario(context, scenario).unwrap();
    let artifact = execute_admitted_identity_evolution_query(&admitted).unwrap();
    let evidence = IdentityEvolutionCertificationResultEvidence::from_execution_artifact(&artifact);
    InspectorIdentityArtifact::from_result_evidence(&evidence)
}

fn change() -> BridgeChangeSummary {
    BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
        "profile",
        "display_name",
        Some("Ada"),
        Some("Ada Lovelace"),
    ))
}

#[test]
fn observed_inspector_runtime_backed_lane_is_verified_without_residual_debt() {
    let canonical = detail_canonical();
    let plan = planned_view(&canonical, ViewShapeDescriptor::inspector_detail_observed());
    let live = lower_view_shape_plan_to_live(
        &plan,
        runtime_basis(plan.validated().query().schema_basis().clone()),
        None,
        None,
    )
    .unwrap();
    let execution = execute_live_view_shape_change(&live, &change()).unwrap();

    assert_eq!(
        plan.complexity().status(),
        ViewShapeComplexityStatus::Verified
    );
    assert_eq!(
        execution.patch_envelope().patch_family(),
        Some(ViewShapePatchFamily::ObservedInspectorPatch)
    );
    assert_eq!(execution.counters().complexity_status_debt_count(), 0);
}

#[test]
fn identity_aware_observed_inspector_requires_bound_identity_and_emits_it() {
    let canonical = detail_canonical();
    let plan = planned_view(
        &canonical,
        ViewShapeDescriptor::identity_aware_inspector_detail_observed(),
    );
    let missing_error = lower_view_shape_plan_to_live(
        &plan,
        runtime_basis(plan.validated().query().schema_basis().clone()),
        None,
        None,
    )
    .unwrap_err();
    assert_eq!(
        missing_error.failure_class(),
        &ViewShapeLiveFailureClass::InspectorIdentityBindingRejected
    );

    let live = lower_view_shape_plan_to_live(
        &plan,
        runtime_basis(plan.validated().query().schema_basis().clone()),
        None,
        Some(inspector_identity_artifact(
            InspectorIdentityClassification::AuthoritativeContinuity,
        )),
    )
    .unwrap();
    let execution = execute_live_view_shape_change(&live, &change()).unwrap();

    let ViewShapePatchPayload::ObservedInspectorPatch(patch) = execution.patch_envelope().payload()
    else {
        panic!("expected observed inspector patch payload");
    };
    assert!(patch.inspector_identity().is_some());
    assert_eq!(execution.counters().complexity_status_debt_count(), 0);
}

#[test]
fn focused_inspector_runtime_backed_lane_is_verified_and_still_denies_widening() {
    let canonical = detail_canonical();
    let plan = planned_view(
        &canonical,
        ViewShapeDescriptor::inspector_detail_focused("profile"),
    );
    let live = lower_view_shape_plan_to_live(
        &plan,
        runtime_basis(plan.validated().query().schema_basis().clone()),
        None,
        None,
    )
    .unwrap();
    let execution = execute_live_view_shape_change(&live, &change()).unwrap();

    assert_eq!(
        plan.complexity().status(),
        ViewShapeComplexityStatus::Verified
    );
    assert_eq!(
        execution.patch_envelope().patch_family(),
        Some(ViewShapePatchFamily::FocusedInspectorAspectPatch)
    );
    assert_eq!(execution.counters().complexity_status_debt_count(), 0);

    let widening_error = execute_live_view_shape_change(
        &live,
        &BridgeChangeSummary::default()
            .with_field_delta(BridgeFieldDelta::new(
                "profile",
                "display_name",
                Some("Ada"),
                Some("Ada Lovelace"),
            ))
            .with_field_delta(BridgeFieldDelta::new(
                "identity",
                "id",
                Some("user-1"),
                Some("user-2"),
            )),
    )
    .unwrap_err();
    assert_eq!(
        widening_error.failure_class(),
        &ViewShapeLiveFailureClass::FocusedInspectorWideningDenied
    );
}

#[test]
fn identity_aware_focused_inspector_requires_matching_classification() {
    let canonical = detail_canonical();
    let plan = planned_view(
        &canonical,
        ViewShapeDescriptor::identity_aware_inspector_detail_focused(
            "profile",
            InspectorIdentityClassification::IdentityBreak,
        ),
    );
    let mismatch_error = lower_view_shape_plan_to_live(
        &plan,
        runtime_basis(plan.validated().query().schema_basis().clone()),
        None,
        Some(inspector_identity_artifact(
            InspectorIdentityClassification::AuthoritativeContinuity,
        )),
    )
    .unwrap_err();
    assert_eq!(
        mismatch_error.failure_class(),
        &ViewShapeLiveFailureClass::InspectorIdentityBindingRejected
    );

    let live = lower_view_shape_plan_to_live(
        &plan,
        runtime_basis(plan.validated().query().schema_basis().clone()),
        None,
        Some(inspector_identity_artifact(
            InspectorIdentityClassification::IdentityBreak,
        )),
    )
    .unwrap();
    let execution = execute_live_view_shape_change(&live, &change()).unwrap();

    let ViewShapePatchPayload::FocusedInspectorAspectPatch(patch) =
        execution.patch_envelope().payload()
    else {
        panic!("expected focused inspector patch payload");
    };
    assert_eq!(
        patch
            .inspector_identity()
            .expect("identity-aware focused inspector should retain identity evidence")
            .classification(),
        InspectorIdentityClassification::IdentityBreak
    );
    assert_eq!(execution.counters().complexity_status_debt_count(), 0);
}
