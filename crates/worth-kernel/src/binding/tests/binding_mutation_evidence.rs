use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    author_primitive_binding_declaration, primitive_binding_mutation_evidence,
    AuthorPrimitiveBindingIntent, EdgeBindingSite, EdgeCurveBindingSpec, GeometryTargetKind,
    PrimitiveBindingMutationEvidence, SpatialBindingCompleteness, SpatialBindingKind,
};

use super::support::admitted_binding_handle;

#[test]
fn binding_mutation_evidence_preserves_family_owned_binding_truth() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let geometry = PrimitiveGeometryIdentityBundle::new(
        vec![PrimitiveSupportPlaneIdentity::new(
            "0".to_string(),
            "0".to_string(),
            "1".to_string(),
            "0".to_string(),
        )],
        vec![
            PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0]),
            PrimitiveVertexIdentity::from_position([1.0, 0.0, 0.0]),
        ],
    );
    let declaration = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_curve_to_edge(EdgeCurveBindingSpec::new(
            EdgeBindingSite::new("edge-1").with_persistent_name("curve-alpha"),
            contract,
            geometry,
        )),
    );
    let handle = admitted_binding_handle("binding-mutation-evidence");
    let evidence: PrimitiveBindingMutationEvidence =
        primitive_binding_mutation_evidence(&declaration, &handle).expect("evidence");

    assert_eq!(evidence.binding_kind(), SpatialBindingKind::EdgeCurve);
    assert_eq!(evidence.site_identity(), "edge-1");
    assert_eq!(evidence.target_identity().target_identity(), "edge-1");
    assert_eq!(
        evidence.target_identity().target_kind(),
        GeometryTargetKind::EdgeCurve
    );
    assert_eq!(
        evidence.completeness(),
        SpatialBindingCompleteness::Complete
    );
    assert!(evidence.progression_digest().is_some());
    assert!(evidence.route_plan_digest().is_some());
}
