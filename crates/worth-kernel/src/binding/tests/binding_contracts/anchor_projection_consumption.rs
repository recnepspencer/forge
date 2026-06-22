use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    author_primitive_anchor_binding_declaration, primitive_anchor_binding_projection_facts,
    AnchorCarrierOwnership, AuthorPrimitiveAnchorBindingIntent,
    CarrierOwnedParameterPointAnchorSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    PrimitiveAnchorBindingFactProvenance, PrimitiveAnchorBindingFactReadSurface,
    PrimitiveAnchorBindingProjectionFactReceipt, SpatialBindingCompleteness, SpatialBindingKind,
};

use crate::binding::tests::support::admitted_anchor_binding_handle;

#[test]
fn anchor_binding_projection_facts_preserve_family_owned_binding_truth() {
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
    let handle = admitted_anchor_binding_handle("anchor-binding-projection-facts");
    let facts: PrimitiveAnchorBindingProjectionFactReceipt =
        primitive_anchor_binding_projection_facts(&declaration, &handle).expect("facts");

    assert_eq!(facts.binding_kind(), SpatialBindingKind::FaceSurface);
    assert!(!facts.binding_identity().is_empty());
    assert_eq!(facts.site_identity(), "face-1");
    assert_eq!(facts.completeness(), SpatialBindingCompleteness::Complete);
    assert_eq!(
        facts.read_surface(),
        PrimitiveAnchorBindingFactReadSurface::ProjectionConsumptionFromDeclarationEnvelope
    );
    assert_eq!(
        facts.fact_provenance(),
        PrimitiveAnchorBindingFactProvenance::DeclarationEnvelopeBackedProjectionConsumption
    );
    assert!(facts.progression_digest().is_some());
    assert!(facts.route_plan_digest().is_some());
    assert!(!facts.receipt_digest().is_empty());
    assert!(!facts.envelope_digest().is_empty());
    assert!(!facts.fact_digest().is_empty());
}
