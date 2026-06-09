use forge_query::facade::{
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalValue,
    ForgeQueryDeclarationEntryReadinessStatus, ForgeQueryOrdinaryOutcome,
};
use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_spatial::facade::bindings::{
    author_primitive_anchor_binding_declaration, author_primitive_binding_declaration,
    author_primitive_rebinding_declaration, primitive_rebinding_retained_fact_source,
    AnchorCarrierOwnership, AuthorPrimitiveAnchorBindingIntent, AuthorPrimitiveBindingIntent,
    AuthorPrimitiveRebindingIntent, CarrierOwnedParameterPointAnchorSpec, FaceBindingSite,
    FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily,
    RebindingOutcomeClass, ReplacementCandidateSet, VertexBindingSite, VertexGeometryBindingSpec,
    VertexGeometryProvenanceKind, VertexToleranceRegime,
};

use super::super::support::{
    admitted_anchor_binding_handle, admitted_rebinding_handle, anchor_binding_workflow_artifacts,
    assert_workflow_artifact_parity, canonical_geometry, canonical_text_entries_for_anchor_binding,
    canonical_text_entries_for_rebinding, orthotope_contract, rebind_surface_on_face,
    rebinding_candidate_from_anchor_declaration, rebinding_candidate_from_binding_declaration,
    rebinding_ordinary_outcome_for_entry, rebinding_prior_fact_from_anchor_declaration,
    rebinding_prior_fact_from_binding_declaration, rebinding_receipt_for_entry,
    rebinding_workflow_artifacts,
};

#[test]
fn kernel_binding_workflow_consumes_spatial_authority_without_local_rebinding_logic() {
    let prior_declaration = author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
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
        ),
    );
    let successor_declaration = author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
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
        ),
    );
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-old",
        ReplacementCandidateSet::new(vec![rebinding_candidate_from_anchor_declaration(
            "successor",
            &successor_declaration,
            "workflow-boundary-successor",
        )
        .expect("successor candidate")])
        .expect("candidate set"),
    )
    .expect("neighborhood");
    let entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            rebinding_prior_fact_from_anchor_declaration(
                &prior_declaration,
                "workflow-boundary-prior",
            ),
            neighborhood.clone(),
        ),
    );
    let handle = admitted_rebinding_handle("phase-five-rebinding");
    let ergonomic = rebinding_workflow_artifacts(&entry, &handle);
    let kernel_receipt = primitive_rebinding_retained_fact_source(&entry, &handle)
        .expect("retained fact source")
        .receipt()
        .clone();
    let ordinary_outcome = rebinding_ordinary_outcome_for_entry(&entry, &handle);
    let generic_progression = handle
        .declare_review_and_progress(entry.clone())
        .unwrap_or_else(|_| panic!("generic progression"));

    let direct_receipt = rebind_surface_on_face(
        rebinding_prior_fact_from_anchor_declaration(
            &prior_declaration,
            "workflow-boundary-direct-prior",
        ),
        neighborhood,
    )
    .expect("direct receipt");

    assert_eq!(
        kernel_receipt.outcome_class(),
        direct_receipt.outcome_class()
    );
    assert_eq!(
        kernel_receipt.selected_candidate_identity(),
        direct_receipt.selected_candidate_identity()
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
    assert!(matches!(
        ordinary_outcome,
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
    let generic_progression = handle
        .declare_review_and_progress(entry.clone())
        .unwrap_or_else(|_| panic!("generic progression"));

    assert_eq!(
        canonical_text_entries_for_anchor_binding(&entry),
        canonical_text_map(ergonomic.canonical_entries())
    );
    assert_workflow_artifact_parity(&ergonomic, &handle, generic_progression, entry.clone());
}

#[test]
fn kernel_rebinding_dx_lane_and_generic_query_lane_converge_to_same_artifacts() {
    let prior_declaration = author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
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
        ),
    );
    let successor_declaration = author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
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
        ),
    );
    let entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            rebinding_prior_fact_from_anchor_declaration(
                &prior_declaration,
                "workflow-boundary-dx-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_anchor_declaration(
                    "successor",
                    &successor_declaration,
                    "workflow-boundary-dx-successor",
                )
                .expect("successor candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let handle = admitted_rebinding_handle("phase-twelve-rebinding-parity");
    let ergonomic = rebinding_workflow_artifacts(&entry, &handle);
    let generic_progression = handle
        .declare_review_and_progress(entry.clone())
        .unwrap_or_else(|_| panic!("generic progression"));

    assert_eq!(
        canonical_text_entries_for_rebinding(&entry),
        canonical_text_map(ergonomic.canonical_entries())
    );
    assert_workflow_artifact_parity(&ergonomic, &handle, generic_progression, entry.clone());
}

#[test]
fn kernel_cannot_reinterpret_spatial_denials_or_continuity_classes_for_convenience() {
    let ambiguous_prior_declaration = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-old").with_persistent_name("surface-alpha"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        )),
    );
    let ambiguous_a_declaration = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-a").with_persistent_name("surface-beta"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
        )),
    );
    let ambiguous_b_declaration = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-b").with_persistent_name("surface-gamma"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
        )),
    );
    let ambiguous_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &ambiguous_prior_declaration,
                "workflow-boundary-ambiguous-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-old",
                ReplacementCandidateSet::new(vec![
                    rebinding_candidate_from_binding_declaration(
                        "a",
                        &ambiguous_a_declaration,
                        "workflow-boundary-ambiguous-a",
                    )
                    .expect("candidate a"),
                    rebinding_candidate_from_binding_declaration(
                        "b",
                        &ambiguous_b_declaration,
                        "workflow-boundary-ambiguous-b",
                    )
                    .expect("candidate b"),
                ])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let ambiguous_handle = admitted_rebinding_handle("phase-twelve-ambiguous");
    let ambiguous_receipt =
        rebinding_receipt_for_entry(&ambiguous_entry, "workflow-boundary-ambiguous")
            .expect("ambiguous receipt");
    let ambiguous_outcome =
        rebinding_ordinary_outcome_for_entry(&ambiguous_entry, &ambiguous_handle);

    let unsupported_prior_declaration = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_vertex_geometry(VertexGeometryBindingSpec::new(
            VertexBindingSite::new("vertex-old"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        )),
    );
    let unsupported_successor_declaration = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_vertex_geometry(VertexGeometryBindingSpec::new(
            VertexBindingSite::new("vertex-new"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        )),
    );
    let unsupported_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &unsupported_prior_declaration,
                "workflow-boundary-unsupported-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "vertex-successor",
                    &unsupported_successor_declaration,
                    "workflow-boundary-unsupported-successor",
                )
                .expect("vertex successor candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let unsupported_handle = admitted_rebinding_handle("phase-twelve-unsupported");
    let unsupported_receipt =
        rebinding_receipt_for_entry(&unsupported_entry, "workflow-boundary-unsupported")
            .expect("unsupported receipt");
    let unsupported_outcome =
        rebinding_ordinary_outcome_for_entry(&unsupported_entry, &unsupported_handle);

    assert_eq!(
        ambiguous_receipt.outcome_class(),
        RebindingOutcomeClass::Ambiguous
    );
    assert!(matches!(
        ambiguous_outcome,
        ForgeQueryOrdinaryOutcome::Ambiguous(_)
    ));
    assert_eq!(
        unsupported_receipt.outcome_class(),
        RebindingOutcomeClass::Unsupported
    );
    assert!(matches!(
        unsupported_outcome,
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
