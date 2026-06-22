use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::bindings::{
    author_primitive_anchor_binding_declaration, AnchorCarrierOwnership,
    AuthorPrimitiveAnchorBindingIntent, CarrierOwnedParameterPointAnchorSpec, FaceBindingSite,
    FaceSurfaceBindingSpec, PrimitiveAnchorBindingDeclarationEntry, VertexBindingSite,
    VertexGeometryBindingSpec, VertexGeometryProvenanceKind, VertexToleranceRegime,
};

use crate::binding::tests::support::{
    anchored_surface_prior_fact_from_declaration, canonical_geometry, orthotope_contract,
};

pub(super) fn anchored_planar_surface(
    face_id: &str,
    point: [f64; 2],
    width: f64,
) -> PrimitiveAnchorBindingDeclarationEntry {
    anchored_curved_surface_declaration(
        face_id,
        ParameterDomain::plane(),
        point,
        canonical_geometry([[0.0, 0.0, 0.0], [width, 0.0, 0.0]]),
    )
}

pub(super) fn anchored_curved_surface_declaration(
    face_id: &str,
    domain: ParameterDomain,
    point: [f64; 2],
    geometry: worth_primitives::PrimitiveGeometryIdentityBundle,
) -> PrimitiveAnchorBindingDeclarationEntry {
    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new(face_id),
                orthotope_contract(),
                geometry,
            ),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_face_surface(face_id, domain).expect("ownership"),
                ParameterSpacePoint::try_new(point).expect("parameter"),
            )
            .expect("anchor"),
        ),
    )
}

pub(super) fn anchored_curved_surface(
    face_id: &str,
    domain: ParameterDomain,
    point: [f64; 2],
    geometry: worth_primitives::PrimitiveGeometryIdentityBundle,
) -> String {
    anchored_curved_surface_runtime(face_id, domain, point, geometry)
}

fn anchored_curved_surface_runtime(
    face_id: &str,
    domain: ParameterDomain,
    point: [f64; 2],
    geometry: worth_primitives::PrimitiveGeometryIdentityBundle,
) -> String {
    anchored_surface_prior_fact_from_declaration(
        &anchored_curved_surface_declaration(face_id, domain, point, geometry),
        "binding-layer-closeout-curved-identity",
    )
    .prior_binding_identity()
    .to_string()
}

pub(super) fn anchored_ellipsoid_surface(
    face_id: &str,
    geometry: worth_primitives::PrimitiveGeometryIdentityBundle,
) -> String {
    anchored_curved_surface_runtime(
        face_id,
        ParameterDomain::triaxial_ellipsoid(),
        [0.25, 0.4],
        geometry,
    )
}

pub(super) fn anchored_ellipsoid_surface_declaration(
    face_id: &str,
    geometry: worth_primitives::PrimitiveGeometryIdentityBundle,
) -> PrimitiveAnchorBindingDeclarationEntry {
    anchored_curved_surface_declaration(
        face_id,
        ParameterDomain::triaxial_ellipsoid(),
        [0.25, 0.4],
        geometry,
    )
}

pub(super) fn vertex_binding_declaration(
    vertex_id: &str,
) -> worth_spatial::facade::bindings::PrimitiveBindingDeclarationEntry {
    worth_spatial::facade::bindings::author_primitive_binding_declaration(
        worth_spatial::facade::bindings::AuthorPrimitiveBindingIntent::attach_vertex_geometry(
            VertexGeometryBindingSpec::new(
                VertexBindingSite::new(vertex_id),
                PrimitiveConstructionFamilyContractRegistry::contract_for(
                    &PrimitiveWitnessDescriptor::Orthotope,
                ),
                canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                VertexGeometryProvenanceKind::CanonicalWitness,
                VertexToleranceRegime::ExactBits,
            ),
        ),
    )
}
