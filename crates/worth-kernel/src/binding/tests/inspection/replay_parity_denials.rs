use forge_query::facade::ForgeQueryDeclarationEntryInspectionInput;
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::bindings::{
    attach_vertex_geometry, NeighborhoodBindingFamily, ReplacementCandidate,
    SpatialAdmittedPrimitiveBinding, VertexBindingSite, VertexGeometryBindingSpec,
    VertexGeometryProvenanceKind, VertexToleranceRegime,
};

use crate::{
    binding::rebinding::PrimitiveRebindingReplaySource,
    facade::authoring::binding::{
        author_primitive_rebinding_declaration, AuthorPrimitiveRebindingIntent,
    },
};

use super::super::support::{
    admitted_rebinding_handle, canonical_geometry, progress_rebinding_entry,
    replacement_neighborhood,
};

#[test]
fn replay_parity_preserves_equivalent_unsupported_histories_without_upgrading_denial_shape() {
    let prior = vertex_binding("vertex-old");
    let unsupported_a = vertex_binding("vertex-new-a");
    let unsupported_b = vertex_binding("vertex-new-b");
    let left = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            prior.clone(),
            replacement_neighborhood(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                vec![
                    ReplacementCandidate::new("a", unsupported_a).expect("a candidate"),
                    ReplacementCandidate::new("b", unsupported_b.clone()).expect("b candidate"),
                ],
            ),
        ),
    );
    let right = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            prior,
            replacement_neighborhood(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                vec![
                    ReplacementCandidate::new("b", unsupported_b).expect("b candidate"),
                    ReplacementCandidate::new("a", vertex_binding("vertex-new-a"))
                        .expect("a candidate"),
                ],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("phase-fifteen-denied-parity");
    let left_historical = left
        .historical_inspection_with_query(
            &handle,
            ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
                handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                    &left, &handle,
                )),
            ),
        )
        .expect("left historical inspection");
    let right_historical = right
        .historical_inspection_with_query(
            &handle,
            ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
                handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                    &right, &handle,
                )),
            ),
        )
        .expect("right historical inspection");
    let left_decision = left_historical.decision().clone();
    let right_decision = right_historical.decision().clone();
    let parity = left
        .replay_parity_with_query(
            &handle,
            PrimitiveRebindingReplaySource::Historical(left_historical),
            &right,
            PrimitiveRebindingReplaySource::Historical(right_historical),
        )
        .expect("replay parity");

    assert_eq!(
        parity.binding_identity(),
        left_decision.explanation().prior_identity()
    );
    assert_eq!(
        parity.anchor_identity(),
        left_decision.explanation().prior_site_identity()
    );
    assert_eq!(parity.outcome_class(), left_decision.outcome_class());
    assert_eq!(
        parity.continuity_class(),
        left_decision.explanation().continuity_class()
    );
    assert_eq!(
        parity.selected_candidate_identity(),
        left_decision.explanation().selected_candidate_identity()
    );
    assert_eq!(
        parity.selected_candidate_label(),
        left_decision.explanation().selected_candidate_label()
    );
    assert_eq!(
        parity.unsupported_reason(),
        format!("{:?}", left_decision.explanation().unsupported_reason())
    );
    assert_eq!(
        parity.binding_identity(),
        right_decision.explanation().prior_identity()
    );
    assert_eq!(parity.ordinary_kind(), "unsupported");
    assert_eq!(
        parity.next_step(),
        Some(forge_query::facade::ForgeQueryOrdinaryNextStep::CheckSupport)
    );
    assert_eq!(
        parity.outcome_class(),
        worth_spatial::facade::bindings::RebindingOutcomeClass::Unsupported
    );
    assert!(!parity.replay_digest().is_empty());
}

fn vertex_binding(vertex_id: &str) -> SpatialAdmittedPrimitiveBinding {
    SpatialAdmittedPrimitiveBinding::VertexGeometry(
        attach_vertex_geometry(VertexGeometryBindingSpec::new(
            VertexBindingSite::new(vertex_id),
            PrimitiveConstructionFamilyContractRegistry::contract_for(
                &PrimitiveWitnessDescriptor::Orthotope,
            ),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        ))
        .expect("vertex geometry binding"),
    )
}
