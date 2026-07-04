use forge_query::facade::consumer_kit::{
    graph_obligation_consumer_kit, ForgeQueryBoundaryAuditSourceSet,
    ForgeQueryGraphObligationConsumerRegistrationDeclaration,
    ForgeQueryGraphObligationExecutionProof, ForgeQueryGraphObligationInMemoryTestWorkspace,
    ForgeQueryGraphObligationLocalCeremonyAudit, ForgeQueryGraphObligationResidueManifest,
    ForgeQueryGraphObligationSelectorCoverageDeclaration, ForgeQueryGraphObligationSupportPin,
};
use forge_query::facade::{
    ForgeQueryGraphObligationExecutionResultEnvelope, ForgeQueryGraphObligationIndex,
    ForgeQueryGraphObligationMaterializedDispatch, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationRegistrationCatalog, ForgeQueryGraphObligationSupportMatrix,
    ForgeQueryGraphObligationSupportMatrixRow,
};
use forge_relational::facade::identity::{EntityId, PartitionId};

use crate::replay_undo_semantic_graph::current_topology_invalidation_declared_touch_proof;
use crate::validation::loop_wiring_rule;
use crate::validation_authority_inventory::WorthValidationAuthorityMilestoneEightSeedSummary;
use crate::validator_invariant_catalog::{
    current_worth_topology_legality_catalog_closeout, WorthTopologyLegalityCatalogError,
    WorthTopologyLegalitySelectionCloseout, WorthTopologyLoopWiringAdmittedLocalFacts,
    WorthTopologyLoopWiringHalfEdgeWitnessRow, WorthTopologyLoopWiringLoopWitnessRow,
    WorthTopologyOperatorCertificationCutoverCloseout,
    WorthTopologyRelationalInvariantCatalogCloseout,
    WorthTopologySelectedGraphObligationEnforcementCloseout,
    WorthTopologySelectedGraphObligationExecutionInput, WorthTopologySelectedLegalityObligationRow,
    WorthTopologySelectedValidatorEnforcementCloseout,
    WorthTopologySelectedValidatorEnforcementPhaseFiveSeed, WorthTopologyValidatorFamilyIdentity,
    WorthTopologyValidatorRoutingClosure,
};

use super::WorthTopologyMilestoneNineCloseout;

pub fn current_topology_validator_invariant_milestone_nine_closeout(
) -> Result<WorthTopologyMilestoneNineCloseout, WorthTopologyLegalityCatalogError> {
    let routing_closure = current_validator_routing_closure()?;
    let phase_two_closeout = current_worth_topology_legality_catalog_closeout()?;
    let selection_closeout =
        WorthTopologyLegalitySelectionCloseout::from_phase_two_closeout_and_routing_closure(
            &phase_two_closeout,
            &routing_closure,
        )?;
    let validator_seed = current_validator_phase_five_seed(&selection_closeout)?;
    let relational_closeout =
        WorthTopologyRelationalInvariantCatalogCloseout::from_catalog_selected_plan_and_validator_seed(
            phase_two_closeout.catalog(),
            selection_closeout.selected_plan(),
            &validator_seed,
        )?;
    let execution_input = current_query_execution_input(
        &routing_closure,
        phase_two_closeout
            .catalog()
            .query_projection()
            .query_catalog()
            .registrations(),
    )?;
    let enforcement =
        WorthTopologySelectedGraphObligationEnforcementCloseout::execute_from_relational_invariant_closeout(
            &relational_closeout,
            execution_input,
        )?;
    let operator_cutover =
        WorthTopologyOperatorCertificationCutoverCloseout::from_selected_graph_obligation_enforcement(
            &enforcement,
        )?;
    WorthTopologyMilestoneNineCloseout::from_operator_cutover(
        operator_cutover.phase_eight_seed(),
        &operator_cutover,
    )
}

fn current_validator_routing_closure(
) -> Result<WorthTopologyValidatorRoutingClosure, WorthTopologyLegalityCatalogError> {
    let proof = current_topology_invalidation_declared_touch_proof().map_err(|error| {
        WorthTopologyLegalityCatalogError::SourceFirewall(format!(
            "current validator milestone-nine closeout requires current declared touch proof: {}",
            error.detail()
        ))
    })?;
    WorthTopologyValidatorRoutingClosure::from_declared_touch(
        &proof,
        &WorthValidationAuthorityMilestoneEightSeedSummary::current_imported_public_closeout(),
    )
}

fn current_validator_phase_five_seed(
    selection_closeout: &WorthTopologyLegalitySelectionCloseout,
) -> Result<WorthTopologySelectedValidatorEnforcementPhaseFiveSeed, WorthTopologyLegalityCatalogError>
{
    let selected_obligation = selected_loop_wiring_validator_row(selection_closeout)?;
    let admitted_facts =
        WorthTopologyLoopWiringAdmittedLocalFacts::from_selected_obligation_and_rows(
            selected_obligation,
            "current-validator-milestone-nine:loop-wiring-validator-facts",
            [WorthTopologyLoopWiringLoopWitnessRow::new(
                entity_id(10),
                vec![entity_id(20), entity_id(21)],
            )],
            [
                WorthTopologyLoopWiringHalfEdgeWitnessRow::new(
                    entity_id(20),
                    Some(entity_id(10)),
                    Some(entity_id(21)),
                    Some(entity_id(21)),
                ),
                WorthTopologyLoopWiringHalfEdgeWitnessRow::new(
                    entity_id(21),
                    Some(entity_id(10)),
                    Some(entity_id(20)),
                    Some(entity_id(20)),
                ),
            ],
        );
    WorthTopologySelectedValidatorEnforcementCloseout::execute_loop_wiring_family_from_admitted_facts(
        selection_closeout,
        &admitted_facts,
    )
    .map(|closeout| closeout.phase_five_seed().clone())
}

fn selected_loop_wiring_validator_row<'a>(
    selection_closeout: &'a WorthTopologyLegalitySelectionCloseout,
) -> Result<&'a WorthTopologySelectedLegalityObligationRow, WorthTopologyLegalityCatalogError> {
    let loop_wiring_identity =
        WorthTopologyValidatorFamilyIdentity::from_registered_rule(loop_wiring_rule());
    selection_closeout
        .selected_plan()
        .selected_obligation_rows()
        .iter()
        .find(|row| {
            row.worth_family_identity_digest() == loop_wiring_identity.identity_digest()
                && row.query_obligation_kind()
                    == forge_query::facade::ForgeQueryGraphObligationKind::SchemaContractValidator
        })
        .ok_or_else(|| {
            WorthTopologyLegalityCatalogError::SourceFirewall(
                "current validator milestone-nine closeout requires the loop-wiring validator family in the live selected plan".to_string(),
            )
        })
}

fn current_query_execution_input(
    routing_closure: &WorthTopologyValidatorRoutingClosure,
    registrations: &[ForgeQueryGraphObligationRegistration],
) -> Result<WorthTopologySelectedGraphObligationExecutionInput, WorthTopologyLegalityCatalogError> {
    let registrations = registrations.to_vec();
    let registration_catalog =
        ForgeQueryGraphObligationRegistrationCatalog::from_registrations(registrations.clone())
            .map_err(|error| {
                WorthTopologyLegalityCatalogError::QueryRegistration(error.to_string())
            })?;
    let selection = ForgeQueryGraphObligationIndex::from_catalog(&registration_catalog)
        .select_for_touch(
            routing_closure.touch_descriptor(),
            routing_closure.query_operating_world_descriptor(),
        );
    let execution_envelope = selected_result_envelope(
        &ForgeQueryGraphObligationMaterializedDispatch::from_selection(selection),
    );
    let execution_proof = ForgeQueryGraphObligationExecutionProof::from_envelope(
        ForgeQueryGraphObligationInMemoryTestWorkspace::from_registrations(registrations.clone())
            .map_err(|error| {
                WorthTopologyLegalityCatalogError::QueryRegistration(error.to_string())
            })?
            .prove_selection(
                routing_closure.touch_descriptor(),
                routing_closure.query_operating_world_descriptor(),
            ),
        execution_envelope.clone(),
    );
    let execution_backed_proof =
        graph_obligation_consumer_kit("worth-topo.validator-invariant-catalog")
            .register_obligations(
                ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                    "worth-topo-validator-invariant-catalog",
                    registrations.clone(),
                )
                .map_err(|error| {
                    WorthTopologyLegalityCatalogError::QueryRegistration(error.to_string())
                })?,
            )
            .declare_selector_coverage(selector_coverage_for_registrations(&registrations))
            .pin_support(support_pin_for_registrations(&registrations))
            .against_support_matrix(support_matrix_for_registrations(&registrations))
            .audit_local_ceremony(evaluated_clean_audit())
            .account_for_residue(ForgeQueryGraphObligationResidueManifest::empty())
            .prove_execution(execution_proof)
            .prove_adoption_with_execution()
            .map_err(|error| {
                WorthTopologyLegalityCatalogError::QueryRegistration(error.to_string())
            })?;
    Ok(
        WorthTopologySelectedGraphObligationExecutionInput::from_query_authority(
            execution_envelope,
            execution_backed_proof,
        ),
    )
}

fn selected_result_envelope(
    dispatch: &ForgeQueryGraphObligationMaterializedDispatch,
) -> ForgeQueryGraphObligationExecutionResultEnvelope {
    dispatch.selected_result_envelope()
}

fn selector_coverage_for_registrations(
    registrations: &[ForgeQueryGraphObligationRegistration],
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
    registrations: &[ForgeQueryGraphObligationRegistration],
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
    registrations: &[ForgeQueryGraphObligationRegistration],
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
                include_str!("../selected_graph_obligation_enforcement/closeout.rs"),
            )
            .source(
                "selected_graph_obligation_enforcement/execution_receipt.rs",
                include_str!("../selected_graph_obligation_enforcement/execution_receipt.rs"),
            ),
    )
}

fn entity_id(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}
