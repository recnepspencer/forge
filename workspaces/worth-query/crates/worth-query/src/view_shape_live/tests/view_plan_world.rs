use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, OrderingSelector,
    RawAuthoredQuery, RawAuthoredResultShape, RootEntityKey,
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
use crate::view_shape::{
    admit_view_shape, plan_admitted_view_shape, validate_canonical_bundle_for_admitted_view_shape,
    ViewShapeDescriptor,
};

pub(super) fn detail_canonical() -> crate::canonicalization::CanonicalQueryBundle {
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

pub(super) fn collection_canonical() -> crate::canonicalization::CanonicalQueryBundle {
    GuidedAuthoringPath::canonicalize_collection(
        RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .project(AspectFieldSelector::new("status", "lane").unwrap())
            .order_by(OrderingSelector::ascending("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        RawAuthoredResultShape::collection_builder()
            .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
            .field(
                AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap(),
            )
            .field(AuthoredResultShapeField::new("status", "lane", "lane").unwrap())
            .build()
            .unwrap(),
    )
    .unwrap()
}

pub(super) fn runtime_basis(
    schema_basis: SchemaBasisDigest,
) -> crate::basis::ResolvedSnapshotBasis {
    resolve_snapshot_basis(
        ExecutionBasisIntent::new(
            BasisAuthorityFamily::Runtime,
            SnapshotLineageClass::CurrentHead,
            false,
        ),
        ResolvedSnapshotIdentity::new(
            BasisAuthorityFamily::Runtime,
            None,
            crate::basis::bridge_snapshot_evidence_identity(
                &super::grouped_truth_world::grouped_snapshot_identity(),
            )
            .expect("grouped snapshot identity should lower to query evidence identity"),
            schema_basis,
            SnapshotLineageClass::CurrentHead,
        ),
        BasisResolutionMode::RuntimeDirect,
    )
    .unwrap()
}

fn identity_query_digest(label: &str) -> CanonicalQueryDigest {
    CanonicalQueryDigest::from_parts(&[format!("view-shape-live:{label}")])
}

fn identity_basis_digest(label: &str) -> BasisDigest {
    BasisDigest::from_parts(&[format!("view-shape-live:{label}")])
}

pub(super) fn inspector_identity_artifact(
    classification: InspectorIdentityClassification,
) -> InspectorIdentityArtifact {
    let (context, scenario) = match classification {
        InspectorIdentityClassification::AuthoritativeContinuity => (
            IdentityEvolutionQueryContext::lineage_traversal_for_test(
                identity_query_digest("authoritative"),
                identity_basis_digest("authoritative-basis"),
                LineageTraversalDescriptor::direct_replacement("anchor"),
            ),
            IdentityEvolutionSyntheticScenario::Standard,
        ),
        InspectorIdentityClassification::AdvisoryCandidates => (
            IdentityEvolutionQueryContext::correspondence_identity_comparison_for_test(
                identity_query_digest("advisory"),
                IdentityEvolutionComparisonBasisFamily::BranchToBranch,
                identity_basis_digest("left"),
                identity_basis_digest("right"),
                CorrespondenceIdentityComparison::advisory_between("left-id", "right-id"),
            ),
            IdentityEvolutionSyntheticScenario::Standard,
        ),
        InspectorIdentityClassification::IdentityBreak => (
            IdentityEvolutionQueryContext::lineage_traversal_for_test(
                identity_query_digest("identity-break"),
                identity_basis_digest("identity-break-basis"),
                LineageTraversalDescriptor::branch_local_direct_evolution("anchor"),
            ),
            IdentityEvolutionSyntheticScenario::IdentityBreak,
        ),
        other => panic!("test helper does not support inspector classification '{other:?}'"),
    };
    let admitted = admit_identity_evolution_query_for_scenario(context, scenario)
        .expect("identity evolution request should admit");
    let artifact = execute_admitted_identity_evolution_query(&admitted)
        .expect("identity evolution request should execute");
    let evidence = IdentityEvolutionCertificationResultEvidence::from_execution_artifact(&artifact);
    InspectorIdentityArtifact::from_result_evidence(&evidence)
}

pub(super) fn planned_view(
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

fn schema_view() -> crate::schema_view::QuerySchemaView {
    crate::schema_view::QuerySchemaView::new(
        "view-shape-live",
        [
            crate::schema_view::SchemaFieldView::new(
                crate::authoring::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("id").expect("schema field literal must be valid"),
                crate::schema_view::ScalarAspectType::String,
            ),
            crate::schema_view::SchemaFieldView::new(
                crate::authoring::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("display_name")
                    .expect("schema field literal must be valid"),
                crate::schema_view::ScalarAspectType::String,
            ),
            crate::schema_view::SchemaFieldView::new(
                crate::authoring::AspectName::new("status")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("lane")
                    .expect("schema field literal must be valid"),
                crate::schema_view::ScalarAspectType::String,
            ),
        ],
        [],
    )
}
