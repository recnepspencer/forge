use super::{
    WorthQueryOperationBindingCounters, WorthQueryOperationBindingDenial,
    WorthQueryOperationBindingDenialKind,
};
use crate::domain_installation::{
    PublishedAftermathPosture, WorthQueryGraphBudgetPosture, WorthQueryGraphFailureTopology,
    WorthQueryGraphIdentityPosture, WorthQueryGraphLocalityPosture, WorthQueryGraphMutationPosture,
    WorthQueryGraphObservationPosture, WorthQueryGraphProjectionPosture,
    WorthQueryOperationCostClass, WorthQueryOperationFailureClass, WorthQueryOperationGraphAccess,
    WorthQueryOperationLineageContract, WorthQueryOperationResultState,
    WorthQueryOperationTouchContract,
};

pub(super) fn admit_graph_contract<D, O, F>(
    operation: &crate::domain_installation::WorthQueryInstalledDomainOperation<D, O, F>,
    role: &str,
    record: &crate::domain_installation::WorthQueryInstalledGraphParticipationRecord,
    counters: &mut WorthQueryOperationBindingCounters,
) -> Result<(), WorthQueryOperationBindingDenial> {
    counters.graph_contract_checks += 1;
    let contract = &record.definition.contract;
    admit_access_contract(operation, role, record, counters)?;
    let semantics = operation.definition().semantics();
    if !lineage_is_admitted(semantics.lineage, contract.identity) {
        return Err(denied(
            role,
            "cannot satisfy the operation lineage contract",
            counters,
        ));
    }
    if !cost_is_admitted(semantics.cost.execution, contract.locality, contract.budget) {
        return Err(denied(
            role,
            "exceeds the installed operation cost contract",
            counters,
        ));
    }
    if !failure_is_admitted(semantics, contract.failure) {
        return Err(denied(
            role,
            "exceeds the installed operation failure contract",
            counters,
        ));
    }
    Ok(())
}

fn admit_access_contract<D, O, F>(
    operation: &crate::domain_installation::WorthQueryInstalledDomainOperation<D, O, F>,
    role: &str,
    record: &crate::domain_installation::WorthQueryInstalledGraphParticipationRecord,
    counters: &mut WorthQueryOperationBindingCounters,
) -> Result<(), WorthQueryOperationBindingDenial> {
    let contract = &record.definition.contract;
    if let Some(read) = operation
        .definition()
        .semantics()
        .graph_reads
        .roles()
        .iter()
        .find(|read| {
            counters.graph_read_role_checks += 1;
            read.role == role
        })
    {
        let admitted = match read.access {
            WorthQueryOperationGraphAccess::Observe => {
                contract.observation != WorthQueryGraphObservationPosture::NotRequired
            }
            WorthQueryOperationGraphAccess::Project => {
                contract.projection == WorthQueryGraphProjectionPosture::NativeProjection
            }
        };
        if !admitted {
            return Err(WorthQueryOperationBindingDenial::new(
                WorthQueryOperationBindingDenialKind::GraphAuthorityInsufficient,
                format!(
                    "graph role `{role}` cannot satisfy declared {:?} access",
                    read.access
                ),
                *counters,
            ));
        }
    }
    let touched = operation_touches_role(operation, role, counters);
    if touched && contract.mutation != WorthQueryGraphMutationPosture::TouchAndEffect {
        return Err(WorthQueryOperationBindingDenial::new(
            WorthQueryOperationBindingDenialKind::GraphAuthorityInsufficient,
            format!("graph role `{role}` cannot satisfy declared touch/effect access"),
            *counters,
        ));
    }
    Ok(())
}

fn lineage_is_admitted(
    lineage: WorthQueryOperationLineageContract,
    identity: WorthQueryGraphIdentityPosture,
) -> bool {
    matches!(
        (lineage, identity),
        (WorthQueryOperationLineageContract::NotRequired, _)
            | (
                WorthQueryOperationLineageContract::Preserve,
                WorthQueryGraphIdentityPosture::PreservedLineage
            )
            | (
                WorthQueryOperationLineageContract::Evolve,
                WorthQueryGraphIdentityPosture::EvolvingLineage
            )
    )
}

fn cost_is_admitted(
    execution_cost: WorthQueryOperationCostClass,
    locality: WorthQueryGraphLocalityPosture,
    budget: WorthQueryGraphBudgetPosture,
) -> bool {
    match (locality, budget) {
        (WorthQueryGraphLocalityPosture::ExternalBoundary, _)
        | (_, WorthQueryGraphBudgetPosture::ExternalBoundary) => {
            execution_cost == WorthQueryOperationCostClass::ExternalBoundary
        }
        (_, WorthQueryGraphBudgetPosture::DeclaredBreadth) => matches!(
            execution_cost,
            WorthQueryOperationCostClass::GraphBreadth
                | WorthQueryOperationCostClass::ExternalBoundary
        ),
        (_, WorthQueryGraphBudgetPosture::ConstantAdmission) => true,
    }
}

fn failure_is_admitted(
    semantics: &worth_query_installation::facade::WorthQueryDomainOperationSemanticClosure,
    failure: WorthQueryGraphFailureTopology,
) -> bool {
    let declares_boundary_failure = semantics.terminal.failure_classes.iter().any(|class| {
        matches!(
            class,
            WorthQueryOperationFailureClass::Dependency
                | WorthQueryOperationFailureClass::Indeterminate
        )
    });
    match failure {
        WorthQueryGraphFailureTopology::Local => true,
        WorthQueryGraphFailureTopology::BoundaryFailure => declares_boundary_failure,
        WorthQueryGraphFailureTopology::PartialCommitPossible => {
            declares_boundary_failure
                && semantics
                    .terminal
                    .result_states
                    .contains(&WorthQueryOperationResultState::Partial)
                && semantics.aftermath.as_ref().is_some_and(|contract| {
                    matches!(
                        contract.published_posture(),
                        PublishedAftermathPosture::Compensatable
                            | PublishedAftermathPosture::Reconcilable
                    )
                })
        }
    }
}

fn denied(
    role: &str,
    reason: &str,
    counters: &WorthQueryOperationBindingCounters,
) -> WorthQueryOperationBindingDenial {
    WorthQueryOperationBindingDenial::new(
        WorthQueryOperationBindingDenialKind::GraphAuthorityInsufficient,
        format!("graph role `{role}` {reason}"),
        *counters,
    )
}

fn operation_touches_role<D, O, F>(
    operation: &crate::domain_installation::WorthQueryInstalledDomainOperation<D, O, F>,
    role: &str,
    counters: &mut WorthQueryOperationBindingCounters,
) -> bool {
    match &operation.definition().semantics().touches {
        WorthQueryOperationTouchContract::Declared { graph_roles, .. } => {
            graph_roles.iter().any(|declared| {
                counters.touched_graph_role_checks += 1;
                declared == role
            })
        }
        WorthQueryOperationTouchContract::NotRequired => false,
    }
}
