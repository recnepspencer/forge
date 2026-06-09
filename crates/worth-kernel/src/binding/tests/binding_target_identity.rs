use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    author_primitive_binding_declaration, AuthorPrimitiveBindingIntent, EdgeBindingSite,
    EdgeCurveBindingSpec,
};
use worth_spatial::facade::bindings::{
    primitive_binding_geometry_target_identity, GeometryTargetIdentityFactReceipt,
    GeometryTargetKind, GeometryTargetSourceAuthority,
};

use super::support::admitted_binding_handle;

#[test]
fn binding_target_identity_tracks_family_owned_target_truth() {
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
    let handle = admitted_binding_handle("binding-target-identity");
    let receipt: GeometryTargetIdentityFactReceipt =
        primitive_binding_geometry_target_identity(&declaration, &handle).expect("receipt");

    assert_eq!(receipt.target_identity(), "edge-1");
    assert_eq!(receipt.target_kind(), GeometryTargetKind::EdgeCurve);
    assert_eq!(
        receipt.source_authority(),
        GeometryTargetSourceAuthority::PrimitiveBindingDeclarationEnvelope
    );
    assert_eq!(receipt.alias_identities().len(), 1);
}
