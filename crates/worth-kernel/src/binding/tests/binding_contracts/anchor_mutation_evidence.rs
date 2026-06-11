use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    author_primitive_anchor_binding_declaration, primitive_anchor_binding_mutation_evidence,
    AnchorCarrierOwnership, AuthorPrimitiveAnchorBindingIntent,
    CarrierOwnedParameterPointAnchorSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    GeometryTargetKind, PrimitiveAnchorBindingMutationEvidence, SpatialBindingCompleteness,
    SpatialBindingKind,
};

use crate::binding::tests::support::admitted_anchor_binding_handle;

#[test]
fn anchor_binding_mutation_evidence_preserves_family_owned_binding_truth() {
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
                ParameterSpacePoint::try_new([0.25, 0.5]).expect("anchor point"),
            )
            .expect("anchor spec"),
        ),
    );
    let handle = admitted_anchor_binding_handle("anchor-mutation-evidence");
    let evidence: PrimitiveAnchorBindingMutationEvidence =
        primitive_anchor_binding_mutation_evidence(&declaration, &handle).expect("evidence");

    assert_eq!(evidence.binding_kind(), SpatialBindingKind::FaceSurface);
    assert_eq!(evidence.site_identity(), "face-1");
    assert_eq!(evidence.target_identity().target_identity(), "face-1");
    assert_eq!(
        evidence.target_identity().target_kind(),
        GeometryTargetKind::FaceSurfacePointAnchor
    );
    assert_eq!(
        evidence.completeness(),
        SpatialBindingCompleteness::Complete
    );
    assert!(evidence.progression_digest().is_some());
    assert!(evidence.route_plan_digest().is_some());
}
