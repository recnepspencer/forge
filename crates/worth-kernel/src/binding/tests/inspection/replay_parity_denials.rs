use super::super::support::{
    admitted_rebinding_handle, canonical_geometry, progress_rebinding_entry,
    rebinding_candidate_from_binding_declaration, rebinding_prior_fact_from_binding_declaration,
    replacement_neighborhood, PrimitiveRebindingKernelQueryExt,
};
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::bindings::{
    author_primitive_binding_declaration, author_primitive_rebinding_declaration,
    AuthorPrimitiveBindingIntent, NeighborhoodBindingFamily, VertexBindingSite,
    VertexGeometryBindingSpec, VertexGeometryProvenanceKind, VertexToleranceRegime,
};
use worth_spatial::facade::inspection::PrimitiveRebindingReplaySource;

#[test]
fn replay_parity_preserves_equivalent_unsupported_histories_without_upgrading_denial_shape() {
    let prior = vertex_binding_declaration("vertex-old");
    let unsupported_a = vertex_binding_declaration("vertex-new-a");
    let unsupported_b = vertex_binding_declaration("vertex-new-b");
    let left = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(&prior, "denial-parity-left-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                vec![
                    rebinding_candidate_from_binding_declaration(
                        "a",
                        &unsupported_a,
                        "denial-parity-left-a",
                    )
                    .expect("a candidate"),
                    rebinding_candidate_from_binding_declaration(
                        "b",
                        &unsupported_b,
                        "denial-parity-left-b",
                    )
                    .expect("b candidate"),
                ],
            ),
        ),
    );
    let right = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(&prior, "denial-parity-right-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                vec![
                    rebinding_candidate_from_binding_declaration(
                        "b",
                        &unsupported_b,
                        "denial-parity-right-b",
                    )
                    .expect("b candidate"),
                    rebinding_candidate_from_binding_declaration(
                        "a",
                        &unsupported_a,
                        "denial-parity-right-a",
                    )
                    .expect("a candidate"),
                ],
            ),
        ),
    );
    let handle = admitted_rebinding_handle("replay-parity-denied");
    let left_historical = left
        .historical_inspection_with_query(
            &handle,
            handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                &left, &handle,
            )),
        )
        .expect("left historical inspection");
    let right_historical = right
        .historical_inspection_with_query(
            &handle,
            handle.orchestrate_envelope_from_progressed_checked(progress_rebinding_entry(
                &right, &handle,
            )),
        )
        .expect("right historical inspection");
    let left_receipt = left_historical.receipt().clone();
    let right_receipt = right_historical.receipt().clone();
    let parity = left
        .replay_parity_with_query(
            &handle,
            PrimitiveRebindingReplaySource::Historical(left_historical.retained_fact_receipt()),
            PrimitiveRebindingReplaySource::Historical(right_historical.retained_fact_receipt()),
        )
        .expect("replay parity");

    assert_eq!(
        parity.binding_identity(),
        left_receipt.prior_binding_identity()
    );
    assert_eq!(parity.anchor_identity(), left_receipt.prior_site_identity());
    assert_eq!(parity.outcome_class(), left_receipt.outcome_class());
    assert_eq!(parity.continuity_class(), left_receipt.continuity_class());
    assert_eq!(
        parity.selected_candidate_identity(),
        left_receipt.selected_candidate_identity()
    );
    assert_eq!(
        parity.selected_candidate_label(),
        left_receipt.selected_candidate_label()
    );
    assert_eq!(
        parity.unsupported_reason(),
        format!("{:?}", left_receipt.unsupported_reason())
    );
    assert_eq!(
        parity.binding_identity(),
        right_receipt.prior_binding_identity()
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

fn vertex_binding_declaration(
    vertex_id: &str,
) -> worth_spatial::facade::bindings::PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_vertex_geometry(
        VertexGeometryBindingSpec::new(
            VertexBindingSite::new(vertex_id),
            PrimitiveConstructionFamilyContractRegistry::contract_for(
                &PrimitiveWitnessDescriptor::Orthotope,
            ),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        ),
    ))
}
