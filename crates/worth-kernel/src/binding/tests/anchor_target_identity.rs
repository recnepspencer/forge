use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    author_primitive_anchor_binding_declaration, AnchorCarrierOwnership,
    AuthorPrimitiveAnchorBindingIntent, CarrierOwnedParameterPointAnchorSpec, FaceBindingSite,
    FaceSurfaceBindingSpec,
};
use worth_spatial::facade::bindings::{
    primitive_anchor_binding_geometry_target_identity, GeometryTargetIdentityFactReceipt,
    GeometryTargetKind, GeometryTargetSourceAuthority,
};

use super::support::admitted_anchor_binding_handle;

#[test]
fn anchor_target_identity_tracks_family_owned_target_truth() {
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
    let declaration = author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new("face-1").with_persistent_name("surface-alpha"),
                contract,
                geometry,
            ),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_face_surface("face-1", ParameterDomain::plane())
                    .expect("carrier ownership"),
                ParameterSpacePoint::try_new([0.25, 0.5]).expect("parameter point"),
            )
            .expect("anchor spec"),
        ),
    );
    let handle = admitted_anchor_binding_handle("anchor-target-identity");
    let receipt: GeometryTargetIdentityFactReceipt =
        primitive_anchor_binding_geometry_target_identity(&declaration, &handle).expect("receipt");

    assert_eq!(receipt.target_identity(), "face-1");
    assert_eq!(
        receipt.target_kind(),
        GeometryTargetKind::FaceSurfacePointAnchor
    );
    assert_eq!(
        receipt.source_authority(),
        GeometryTargetSourceAuthority::PrimitiveAnchorBindingDeclarationEnvelope
    );
    assert_eq!(receipt.alias_identities().len(), 1);
}
