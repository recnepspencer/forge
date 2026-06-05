use forge_query::facade::{
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalValue,
    ForgeQueryDeclarationEntryReadinessStatus, ForgeQueryOrdinaryOutcome,
};
use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_spatial::facade::bindings::{
    attach_parameter_space_point_to_face, attach_surface_to_face, attach_vertex_geometry,
    rebind_surface_on_face, AnchorCarrierOwnership, CarrierOwnedParameterPointAnchorSpec,
    FaceBindingSite, FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood,
    NeighborhoodBindingFamily, RebindingOutcomeClass, ReplacementCandidate,
    ReplacementCandidateSet, SpatialAdmittedPrimitiveBinding, VertexBindingSite,
    VertexGeometryBindingSpec, VertexGeometryProvenanceKind, VertexToleranceRegime,
};

use crate::facade::authoring::anchoring::{
    author_primitive_anchor_binding_declaration, AuthorPrimitiveAnchorBindingIntent,
};
use crate::facade::authoring::binding::author_primitive_rebinding_declaration;
use crate::facade::authoring::binding::AuthorPrimitiveRebindingIntent;

use super::super::support::{
    admitted_anchor_binding_handle, admitted_rebinding_handle, anchor_binding_workflow_artifacts,
    assert_workflow_artifact_parity, canonical_geometry, canonical_text_entries_for_anchor_binding,
    canonical_text_entries_for_rebinding, orthotope_contract, rebinding_workflow_artifacts,
    rebinding_workflow_transport,
};

#[test]
fn kernel_binding_workflow_consumes_spatial_authority_without_local_rebinding_logic() {
    let prior = attach_parameter_space_point_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-old").with_persistent_name("surface-alpha"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        ),
        CarrierOwnedParameterPointAnchorSpec::new(
            AnchorCarrierOwnership::for_face_surface("face-old", ParameterDomain::plane())
                .expect("ownership"),
            ParameterSpacePoint::try_new([0.25, 0.5]).expect("parameter"),
        )
        .expect("anchor spec"),
    )
    .expect("prior");
    let successor = attach_parameter_space_point_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-new").with_persistent_name("surface-beta"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        ),
        CarrierOwnedParameterPointAnchorSpec::new(
            AnchorCarrierOwnership::for_face_surface("face-new", ParameterDomain::plane())
                .expect("ownership"),
            ParameterSpacePoint::try_new([0.25, 0.5]).expect("parameter"),
        )
        .expect("anchor spec"),
    )
    .expect("successor");
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-old",
        ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
            "successor",
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(successor.clone()),
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("neighborhood");
    let entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior.clone()),
            neighborhood.clone(),
        ),
    );
    let handle = admitted_rebinding_handle("phase-five-rebinding");
    let ergonomic = rebinding_workflow_artifacts(&entry, &handle);
    let transport = rebinding_workflow_transport(&entry, &handle);
    let generic_progression = entry
        .progress_with_query(&handle)
        .unwrap_or_else(|_| panic!("generic progression"));

    let kernel_decision = transport.decision().clone();
    let direct_decision = rebind_surface_on_face(
        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior),
        neighborhood,
    )
    .expect("direct decision");

    assert_eq!(
        kernel_decision.outcome_class(),
        direct_decision.outcome_class()
    );
    assert_eq!(
        kernel_decision.explanation().selected_candidate_identity(),
        direct_decision.explanation().selected_candidate_identity()
    );
    assert_eq!(
        canonical_text_entries_for_rebinding(&entry),
        canonical_text_map(ergonomic.canonical_entries())
    );
    assert_eq!(
        ergonomic.readiness().rows()[0].status(),
        ForgeQueryDeclarationEntryReadinessStatus::Admitted
    );
    assert_workflow_artifact_parity(&ergonomic, &handle, generic_progression, entry.clone());
    assert_eq!(
        transport.artifacts().progression().progression_digest(),
        ergonomic.progression().progression_digest()
    );
    assert!(matches!(
        transport.ordinary_outcome(),
        ForgeQueryOrdinaryOutcome::Bound(_)
    ));
}

#[test]
fn kernel_authoring_lane_and_generic_query_lane_share_canonical_declaration_and_progression_truth()
{
    let entry = author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new("face-1").with_persistent_name("surface-alpha"),
                orthotope_contract(),
                canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            ),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_face_surface("face-1", ParameterDomain::plane())
                    .expect("ownership"),
                ParameterSpacePoint::try_new([0.25, 0.5]).expect("parameter"),
            )
            .expect("anchor spec"),
        ),
    );
    let handle = admitted_anchor_binding_handle("phase-five-binding");
    let ergonomic = anchor_binding_workflow_artifacts(&entry, &handle);
    let generic_progression = entry
        .progress_with_query(&handle)
        .unwrap_or_else(|_| panic!("generic progression"));

    assert_eq!(
        canonical_text_entries_for_anchor_binding(&entry),
        canonical_text_map(ergonomic.canonical_entries())
    );
    assert_workflow_artifact_parity(&ergonomic, &handle, generic_progression, entry.clone());
}

#[test]
fn kernel_rebinding_dx_lane_and_generic_query_lane_converge_to_same_artifacts() {
    let prior = attach_parameter_space_point_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-old").with_persistent_name("surface-alpha"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        ),
        CarrierOwnedParameterPointAnchorSpec::new(
            AnchorCarrierOwnership::for_face_surface("face-old", ParameterDomain::plane())
                .expect("ownership"),
            ParameterSpacePoint::try_new([0.25, 0.5]).expect("parameter"),
        )
        .expect("anchor spec"),
    )
    .expect("prior");
    let successor = attach_parameter_space_point_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-new").with_persistent_name("surface-beta"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        ),
        CarrierOwnedParameterPointAnchorSpec::new(
            AnchorCarrierOwnership::for_face_surface("face-new", ParameterDomain::plane())
                .expect("ownership"),
            ParameterSpacePoint::try_new([0.25, 0.5]).expect("parameter"),
        )
        .expect("anchor spec"),
    )
    .expect("successor");
    let entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "successor",
                    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(successor),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let handle = admitted_rebinding_handle("phase-twelve-rebinding-parity");
    let ergonomic = rebinding_workflow_artifacts(&entry, &handle);
    let generic_progression = entry
        .progress_with_query(&handle)
        .unwrap_or_else(|_| panic!("generic progression"));

    assert_eq!(
        canonical_text_entries_for_rebinding(&entry),
        canonical_text_map(ergonomic.canonical_entries())
    );
    assert_workflow_artifact_parity(&ergonomic, &handle, generic_progression, entry.clone());
}

#[test]
fn kernel_cannot_reinterpret_spatial_denials_or_continuity_classes_for_convenience() {
    let ambiguous_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurface(
                attach_surface_to_face(FaceSurfaceBindingSpec::new(
                    FaceBindingSite::new("face-old").with_persistent_name("surface-alpha"),
                    orthotope_contract(),
                    canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                ))
                .expect("prior"),
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-old",
                ReplacementCandidateSet::new(vec![
                    ReplacementCandidate::new(
                        "a",
                        SpatialAdmittedPrimitiveBinding::FaceSurface(
                            attach_surface_to_face(FaceSurfaceBindingSpec::new(
                                FaceBindingSite::new("face-a").with_persistent_name("surface-beta"),
                                orthotope_contract(),
                                canonical_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
                            ))
                            .expect("candidate a"),
                        ),
                    )
                    .expect("candidate a"),
                    ReplacementCandidate::new(
                        "b",
                        SpatialAdmittedPrimitiveBinding::FaceSurface(
                            attach_surface_to_face(FaceSurfaceBindingSpec::new(
                                FaceBindingSite::new("face-b")
                                    .with_persistent_name("surface-gamma"),
                                orthotope_contract(),
                                canonical_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
                            ))
                            .expect("candidate b"),
                        ),
                    )
                    .expect("candidate b"),
                ])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let ambiguous_transport = rebinding_workflow_transport(
        &ambiguous_entry,
        &admitted_rebinding_handle("phase-twelve-ambiguous"),
    );

    let unsupported_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::VertexGeometry(
                attach_vertex_geometry(VertexGeometryBindingSpec::new(
                    VertexBindingSite::new("vertex-old"),
                    orthotope_contract(),
                    canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                    VertexGeometryProvenanceKind::CanonicalWitness,
                    VertexToleranceRegime::ExactBits,
                ))
                .expect("vertex"),
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "vertex-successor",
                    SpatialAdmittedPrimitiveBinding::VertexGeometry(
                        attach_vertex_geometry(VertexGeometryBindingSpec::new(
                            VertexBindingSite::new("vertex-new"),
                            orthotope_contract(),
                            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                            VertexGeometryProvenanceKind::CanonicalWitness,
                            VertexToleranceRegime::ExactBits,
                        ))
                        .expect("vertex successor"),
                    ),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let unsupported_transport = rebinding_workflow_transport(
        &unsupported_entry,
        &admitted_rebinding_handle("phase-twelve-unsupported"),
    );

    assert_eq!(
        ambiguous_transport.decision().outcome_class(),
        RebindingOutcomeClass::Ambiguous
    );
    assert!(matches!(
        ambiguous_transport.ordinary_outcome(),
        ForgeQueryOrdinaryOutcome::Ambiguous(_)
    ));
    assert_eq!(
        unsupported_transport.decision().outcome_class(),
        RebindingOutcomeClass::Unsupported
    );
    assert!(matches!(
        unsupported_transport.ordinary_outcome(),
        ForgeQueryOrdinaryOutcome::Unsupported(_)
    ));
}

fn canonical_text_map(
    entries: &[ForgeQueryDeclarationCanonicalEntry],
) -> std::collections::BTreeMap<String, String> {
    entries
        .iter()
        .filter_map(|row| match row.value() {
            ForgeQueryDeclarationCanonicalValue::ExactText(value)
            | ForgeQueryDeclarationCanonicalValue::DecimalText(value) => {
                Some((row.locus().to_string(), value.clone()))
            }
            _ => None,
        })
        .collect()
}
