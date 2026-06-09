#![cfg(test)]

use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};

use crate::bindings::anchors::{AnchorCarrierOwnership, CarrierOwnedParameterPointAnchorSpec};
use crate::bindings::authority::{
    FaceBindingSite, FaceSurfaceBindingSpec, VertexBindingSite, VertexGeometryBindingSpec,
    VertexGeometryProvenanceKind, VertexToleranceRegime,
};
use crate::bindings::query_native_anchor_binding_authoring::{
    author_primitive_anchor_binding_declaration, AuthorPrimitiveAnchorBindingIntent,
};
use crate::bindings::query_native_binding_authoring::{
    author_primitive_binding_declaration, AuthorPrimitiveBindingIntent,
};
use crate::bindings::rebinding::{
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily, RebindingOutcomeClass,
    ReplacementCandidateSet, UnsupportedRebindingReason,
};
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
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

fn point_binding_declaration(
    face_id: &str,
    persistent_name: &str,
    vertices: [[f64; 3]; 2],
    point: [f64; 2],
) -> crate::bindings::query_native_anchor_binding_authoring::PrimitiveAnchorBindingDeclarationEntry
{
    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new(face_id).with_persistent_name(persistent_name),
                PrimitiveConstructionFamilyContractRegistry::contract_for(
                    &PrimitiveWitnessDescriptor::Orthotope,
                ),
                plane_geometry(vertices),
            ),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_face_surface(face_id, ParameterDomain::plane())
                    .expect("ownership"),
                ParameterSpacePoint::try_new(point).expect("parameter"),
            )
            .expect("anchor spec"),
        ),
    )
}

fn surface_binding_declaration(
    face_id: &str,
    persistent_name: &str,
    vertices: [[f64; 3]; 2],
) -> crate::bindings::query_native_binding_authoring::PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_surface_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new(face_id).with_persistent_name(persistent_name),
            PrimitiveConstructionFamilyContractRegistry::contract_for(
                &PrimitiveWitnessDescriptor::Orthotope,
            ),
            plane_geometry(vertices),
        ),
    ))
}

#[test]
fn rebinding_outcome_classes_keep_success_and_unsupported_posture_distinct() {
    let prior = surface_binding_declaration(
        "face-old",
        "surface-alpha",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
    );
    let exact = surface_binding_declaration(
        "face-new",
        "surface-beta",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
    );
    let successor = surface_binding_declaration(
        "face-successor",
        "surface-gamma",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
    );
    let prior_point = point_binding_declaration(
        "face-anchor-old",
        "surface-anchor-alpha",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        [0.25, 0.5],
    );
    let correspondence = point_binding_declaration(
        "face-correspondence",
        "surface-delta",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        [0.5, 0.5],
    );

    let preserved = super::rebind_surface_on_face_from_fact(
        super::rebinding_prior_fact_from_binding_declaration(&prior, "outcomes-preserved-prior"),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::FaceSurface,
            "face-old",
            ReplacementCandidateSet::new(vec![
                super::rebinding_candidate_from_binding_declaration(
                    "preserved",
                    &prior,
                    "outcomes-preserved-candidate",
                )
                .expect("candidate"),
            ])
            .expect("candidate set"),
        )
        .expect("neighborhood"),
    )
    .expect("preserved");
    let exact = super::rebind_surface_on_face_from_fact(
        super::rebinding_prior_fact_from_binding_declaration(&prior, "outcomes-exact-prior"),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::FaceSurface,
            "face-old",
            ReplacementCandidateSet::new(vec![
                super::rebinding_candidate_from_binding_declaration(
                    "exact",
                    &exact,
                    "outcomes-exact-candidate",
                )
                .expect("candidate"),
            ])
            .expect("candidate set"),
        )
        .expect("neighborhood"),
    )
    .expect("exact");
    let successor = super::rebind_surface_on_face_from_fact(
        super::rebinding_prior_fact_from_binding_declaration(&prior, "outcomes-successor-prior"),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::FaceSurface,
            "face-old",
            ReplacementCandidateSet::new(vec![
                super::rebinding_candidate_from_binding_declaration(
                    "successor",
                    &successor,
                    "outcomes-successor-candidate",
                )
                .expect("candidate"),
            ])
            .expect("candidate set"),
        )
        .expect("neighborhood"),
    )
    .expect("successor");
    let correspondence = super::rebind_surface_on_face_from_fact(
        super::rebinding_prior_fact_from_anchor_declaration(
            &prior_point,
            "outcomes-correspondence-prior",
        ),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::FaceSurfacePointAnchor,
            "face-anchor-old",
            ReplacementCandidateSet::new(vec![super::rebinding_candidate_from_anchor_declaration(
                "correspondence",
                &correspondence,
                "outcomes-correspondence-candidate",
            )
            .expect("candidate")])
            .expect("candidate set"),
        )
        .expect("neighborhood"),
    )
    .expect("correspondence");

    assert_eq!(preserved.outcome_class(), RebindingOutcomeClass::Preserved);
    assert_eq!(
        exact.outcome_class(),
        RebindingOutcomeClass::ExactReattachment
    );
    assert_eq!(
        successor.outcome_class(),
        RebindingOutcomeClass::ContinuityJustifiedReattachment
    );
    assert_eq!(
        correspondence.outcome_class(),
        RebindingOutcomeClass::CorrespondenceOnly
    );
}

#[test]
fn rebinding_unsupported_is_typed_outcome_not_error_fallback() {
    let vertex_prior = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_vertex_geometry(VertexGeometryBindingSpec::new(
            VertexBindingSite::new("vertex-old"),
            PrimitiveConstructionFamilyContractRegistry::contract_for(
                &PrimitiveWitnessDescriptor::Orthotope,
            ),
            plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        )),
    );
    let decision = super::rebind_surface_on_face_from_fact(
        super::rebinding_prior_fact_from_binding_declaration(
            &vertex_prior,
            "outcomes-unsupported-prior",
        ),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::VertexGeometry,
            "vertex-old",
            ReplacementCandidateSet::new(vec![
                super::rebinding_candidate_from_binding_declaration(
                    "vertex-successor",
                    &vertex_prior,
                    "outcomes-unsupported-candidate",
                )
                .expect("candidate"),
            ])
            .expect("candidate set"),
        )
        .expect("neighborhood"),
    )
    .expect("unsupported decision");

    assert_eq!(decision.outcome_class(), RebindingOutcomeClass::Unsupported);
    assert_eq!(
        decision.unsupported_reason(),
        Some(
            UnsupportedRebindingReason::RequestedRebindingFamilyDoesNotAdmitBindingFamily {
                requested: NeighborhoodBindingFamily::FaceSurface,
                actual: NeighborhoodBindingFamily::VertexGeometry,
            },
        )
    );
}
