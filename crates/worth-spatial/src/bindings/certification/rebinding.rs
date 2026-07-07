#![cfg(test)]

mod ambiguity;
mod continuity_evidence;
mod ordering_stability;
mod preserved_prior;
mod vertex_neighborhood;

use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};

use crate::bindings::authority::{
    EdgeBindingSite, EdgeCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    VertexBindingSite, VertexGeometryBindingSpec, VertexGeometryProvenanceKind,
    VertexToleranceRegime,
};
use crate::bindings::query_native_binding_authoring::{
    author_primitive_binding_declaration, AuthorPrimitiveBindingIntent,
};

fn plane_geometry(vertices: [[f64; 3]; 2]) -> PrimitiveGeometryIdentityBundle {
    PrimitiveGeometryIdentityBundle::new(
        vec![PrimitiveSupportPlaneIdentity::new(
            "0".to_string(),
            "0".to_string(),
            "1".to_string(),
            "0".to_string(),
        )],
        vertices
            .into_iter()
            .map(PrimitiveVertexIdentity::from_position)
            .collect(),
    )
}

fn surface_binding_declaration(
    face_id: &str,
    geometry: PrimitiveGeometryIdentityBundle,
) -> crate::bindings::query_native_binding_authoring::PrimitiveBindingDeclarationEntry {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_surface_to_face(
        FaceSurfaceBindingSpec::new(FaceBindingSite::new(face_id), contract, geometry),
    ))
}

fn edge_binding_declaration(
    edge_id: &str,
    geometry: PrimitiveGeometryIdentityBundle,
) -> crate::bindings::query_native_binding_authoring::PrimitiveBindingDeclarationEntry {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_curve_to_edge(
        EdgeCurveBindingSpec::new(EdgeBindingSite::new(edge_id), contract, geometry),
    ))
}

fn vertex_binding_declaration(
    vertex_id: &str,
    geometry: PrimitiveGeometryIdentityBundle,
) -> crate::bindings::query_native_binding_authoring::PrimitiveBindingDeclarationEntry {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_vertex_geometry(
        VertexGeometryBindingSpec::new(
            VertexBindingSite::new(vertex_id),
            contract,
            geometry,
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        ),
    ))
}
