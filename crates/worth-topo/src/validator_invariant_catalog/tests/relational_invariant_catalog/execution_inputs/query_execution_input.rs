use forge_query::facade::consumer_kit::{
    graph_obligation_consumer_kit, ForgeQueryBoundaryAuditSourceSet,
    ForgeQueryGraphObligationConsumerRegistrationDeclaration,
    ForgeQueryGraphObligationExecutionBackedAdoptionProof, ForgeQueryGraphObligationExecutionProof,
    ForgeQueryGraphObligationInMemoryTestWorkspace, ForgeQueryGraphObligationLocalCeremonyAudit,
    ForgeQueryGraphObligationResidueManifest, ForgeQueryGraphObligationSelectorCoverageDeclaration,
    ForgeQueryGraphObligationSupportPin,
};
use forge_query::facade::{
    ForgeQueryGraphObligationExecutionInput, ForgeQueryGraphObligationExecutionResultEnvelope,
    ForgeQueryGraphObligationExecutionResultRow, ForgeQueryGraphObligationIndex,
    ForgeQueryGraphObligationMaterializedDispatch, ForgeQueryGraphObligationRegistrationCatalog,
    ForgeQueryGraphObligationSupportMatrix, ForgeQueryGraphObligationSupportMatrixRow,
};

use crate::validator_invariant_catalog::{
    WorthTopologyLegalitySelectionCloseout, WorthTopologyRelationalInvariantCatalogCloseout,
    WorthTopologySelectedGraphObligationExecutionInput,
};

use super::super::super::production_phase_two_closeout;
use super::routing_closures::{
    routing_closure_for_loop_successor_program, routing_closure_for_rewire_operator,
};
use super::validator_seed::validator_phase_five_seed;

pub(in crate::validator_invariant_catalog::tests) fn relational_invariant_query_execution_input(
) -> (
    WorthTopologyRelationalInvariantCatalogCloseout,
    WorthTopologySelectedGraphObligationExecutionInput,
) {
    relational_invariant_query_execution_input_for_rewire_slot(30)
}

pub(in crate::validator_invariant_catalog::tests) fn relational_invariant_query_execution_input_for_rewire_slot(
    relation_slot: u64,
) -> (
    WorthTopologyRelationalInvariantCatalogCloseout,
    WorthTopologySelectedGraphObligationExecutionInput,
) {
    relational_invariant_query_execution_input_for_rewire_slot_with_rows(
        relation_slot,
        |dispatch| dispatch.selected_result_envelope(),
    )
}

pub(in crate::validator_invariant_catalog::tests) fn relational_invariant_query_execution_input_for_rewire_slot_with_rows(
    relation_slot: u64,
    row_builder: impl FnOnce(
        &ForgeQueryGraphObligationMaterializedDispatch,
    ) -> ForgeQueryGraphObligationExecutionResultEnvelope,
) -> (
    WorthTopologyRelationalInvariantCatalogCloseout,
    WorthTopologySelectedGraphObligationExecutionInput,
) {
    let routing_closure = routing_closure_for_rewire_operator(relation_slot);
    relational_invariant_query_execution_input_from_routing_closure(routing_closure, row_builder)
}

pub(in crate::validator_invariant_catalog::tests) fn relational_invariant_query_execution_input_for_loop_successor_program_slot(
    relation_slot: u64,
) -> (
    WorthTopologyRelationalInvariantCatalogCloseout,
    WorthTopologySelectedGraphObligationExecutionInput,
) {
    relational_invariant_query_execution_input_from_routing_closure(
        routing_closure_for_loop_successor_program(relation_slot),
        |dispatch| dispatch.selected_result_envelope(),
    )
}

fn relational_invariant_query_execution_input_from_routing_closure(
    routing_closure: crate::validator_invariant_catalog::selection_from_touched_closure::WorthTopologyValidatorRoutingClosure,
    row_builder: impl FnOnce(
        &ForgeQueryGraphObligationMaterializedDispatch,
    ) -> ForgeQueryGraphObligationExecutionResultEnvelope,
) -> (
    WorthTopologyRelationalInvariantCatalogCloseout,
    WorthTopologySelectedGraphObligationExecutionInput,
) {
    let phase_two_closeout = production_phase_two_closeout();
    let selection_closeout =
        WorthTopologyLegalitySelectionCloseout::from_phase_two_closeout_and_routing_closure(
            &phase_two_closeout,
            &routing_closure,
        )
        .expect("routing closure should select validator and invariant obligations");
    let validator_seed = validator_phase_five_seed(&selection_closeout);
    let closeout =
        WorthTopologyRelationalInvariantCatalogCloseout::from_catalog_selected_plan_and_validator_seed(
            phase_two_closeout.catalog(),
            selection_closeout.selected_plan(),
            &validator_seed,
        )
        .expect("relational invariant catalog should close from selected plan");
    let registrations = phase_two_closeout
        .catalog()
        .query_projection()
        .query_catalog()
        .registrations()
        .to_vec();
    let registration_catalog =
        ForgeQueryGraphObligationRegistrationCatalog::from_registrations(registrations.clone())
            .expect("relational invariant fixture registrations should form a Query catalog");
    let selection = ForgeQueryGraphObligationIndex::from_catalog(&registration_catalog)
        .select_for_touch(
            routing_closure.touch_descriptor(),
            routing_closure.query_operating_world_descriptor(),
        );
    let execution_envelope =
        row_builder(&ForgeQueryGraphObligationMaterializedDispatch::from_selection(selection));
    let execution_proof = ForgeQueryGraphObligationExecutionProof::from_envelope(
        ForgeQueryGraphObligationInMemoryTestWorkspace::from_registrations(registrations.clone())
            .expect("Consumer Kit test workspace should build from Query registrations")
            .prove_selection(
                routing_closure.touch_descriptor(),
                routing_closure.query_operating_world_descriptor(),
            ),
        execution_envelope.clone(),
    );
    let execution_backed_proof = execution_backed_adoption_proof(registrations, execution_proof);
    (
        closeout,
        WorthTopologySelectedGraphObligationExecutionInput::from_query_authority(
            execution_envelope,
            execution_backed_proof,
        ),
    )
}

pub(in crate::validator_invariant_catalog::tests) fn envelope_from_input_rows(
    dispatch: &ForgeQueryGraphObligationMaterializedDispatch,
    row_builder: impl FnMut(
        ForgeQueryGraphObligationExecutionInput,
    ) -> ForgeQueryGraphObligationExecutionResultRow,
) -> ForgeQueryGraphObligationExecutionResultEnvelope {
    ForgeQueryGraphObligationExecutionResultEnvelope::new(
        dispatch.inputs().iter().cloned().map(row_builder).collect(),
    )
}

fn execution_backed_adoption_proof(
    registrations: Vec<forge_query::facade::ForgeQueryGraphObligationRegistration>,
    execution_proof: ForgeQueryGraphObligationExecutionProof,
) -> ForgeQueryGraphObligationExecutionBackedAdoptionProof {
    graph_obligation_consumer_kit("worth-topo.validator-invariant-catalog")
        .register_obligations(
            ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                "worth-topo-validator-invariant-catalog",
                registrations.clone(),
            )
            .expect("registration declaration should build from Query registrations"),
        )
        .declare_selector_coverage(selector_coverage_for_registrations(&registrations))
        .pin_support(support_pin_for_registrations(&registrations))
        .against_support_matrix(support_matrix_for_registrations(&registrations))
        .audit_local_ceremony(evaluated_clean_audit())
        .account_for_residue(ForgeQueryGraphObligationResidueManifest::empty())
        .prove_execution(execution_proof)
        .prove_adoption_with_execution()
        .expect("Consumer Kit should produce execution-backed adoption proof")
}

fn selector_coverage_for_registrations(
    registrations: &[forge_query::facade::ForgeQueryGraphObligationRegistration],
) -> ForgeQueryGraphObligationSelectorCoverageDeclaration {
    ForgeQueryGraphObligationSelectorCoverageDeclaration::required(
        registrations
            .iter()
            .enumerate()
            .map(|(index, registration)| {
                (
                    format!("worth-topo validator invariant selector {index}"),
                    registration.touch_selector().clone(),
                )
            }),
    )
}

fn support_pin_for_registrations(
    registrations: &[forge_query::facade::ForgeQueryGraphObligationRegistration],
) -> ForgeQueryGraphObligationSupportPin {
    ForgeQueryGraphObligationSupportPin::new_with_budget(registrations.iter().map(|registration| {
        (
            registration.kind(),
            registration.support_posture().lane(),
            registration.support_posture().status(),
            registration.execution_budget().clone(),
        )
    }))
}

fn support_matrix_for_registrations(
    registrations: &[forge_query::facade::ForgeQueryGraphObligationRegistration],
) -> ForgeQueryGraphObligationSupportMatrix {
    ForgeQueryGraphObligationSupportMatrix::new(
        registrations
            .iter()
            .map(|registration| {
                ForgeQueryGraphObligationSupportMatrixRow::new(
                    registration.kind(),
                    registration.support_posture().lane(),
                    registration.support_posture().status(),
                )
            })
            .collect(),
    )
}

fn evaluated_clean_audit() -> ForgeQueryGraphObligationLocalCeremonyAudit {
    ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(
        &ForgeQueryBoundaryAuditSourceSet::new("worth-topo")
            .source(
                "selected_graph_obligation_enforcement/closeout.rs",
                include_str!("../../../selected_graph_obligation_enforcement/closeout.rs"),
            )
            .source(
                "selected_graph_obligation_enforcement/query_execution/execution_receipt.rs",
                include_str!("../../../selected_graph_obligation_enforcement/execution_receipt.rs"),
            ),
    )
}
