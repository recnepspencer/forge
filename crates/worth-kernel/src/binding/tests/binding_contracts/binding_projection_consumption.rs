use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    author_primitive_binding_declaration, primitive_binding_projection_facts,
    AuthorPrimitiveBindingIntent, EdgeBindingSite, EdgeCurveBindingSpec,
    PrimitiveBindingFactProvenance, PrimitiveBindingFactReadSurface,
    PrimitiveBindingProjectionFactReceipt, SpatialBindingCompleteness, SpatialBindingKind,
};

use crate::binding::tests::support::admitted_binding_handle;

#[test]
fn binding_projection_facts_preserve_family_owned_binding_truth() {
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
    let handle = admitted_binding_handle("binding-projection-facts");
    let facts: PrimitiveBindingProjectionFactReceipt =
        primitive_binding_projection_facts(&declaration, &handle).expect("facts");

    assert_eq!(facts.binding_kind(), SpatialBindingKind::EdgeCurve);
    assert!(!facts.binding_identity().is_empty());
    assert_eq!(facts.site_identity(), "edge-1");
    assert_eq!(facts.completeness(), SpatialBindingCompleteness::Complete);
    assert_eq!(
        facts.read_surface(),
        PrimitiveBindingFactReadSurface::ProjectionConsumptionFromDeclarationEnvelope
    );
    assert_eq!(
        facts.fact_provenance(),
        PrimitiveBindingFactProvenance::DeclarationEnvelopeBackedProjectionConsumption
    );
    assert!(facts.progression_digest().is_some());
    assert!(facts.route_plan_digest().is_some());
    assert!(!facts.receipt_digest().is_empty());
    assert!(!facts.envelope_digest().is_empty());
    assert!(!facts.fact_digest().is_empty());
}
