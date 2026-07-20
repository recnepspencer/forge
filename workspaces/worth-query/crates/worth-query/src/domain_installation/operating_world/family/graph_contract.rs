use super::{
    WorthQueryOperationBindingCounters, WorthQueryOperationBindingDenial,
    WorthQueryOperationBindingDenialKind,
};
use crate::domain_installation::{
    WorthQueryGraphBudgetPosture, WorthQueryGraphFailureTopology, WorthQueryGraphIdentityPosture,
    WorthQueryGraphLocalityPosture, WorthQueryGraphMutationPosture,
    WorthQueryGraphObservationPosture, WorthQueryGraphProjectionPosture,
    WorthQueryOperationCostClass, WorthQueryOperationFailureClass, WorthQueryOperationGraphAccess,
    WorthQueryOperationLineageContract, WorthQueryOperationResultState,
    WorthQueryOperationReversalContract, WorthQueryOperationTouchContract,
};

pub(super) fn admit_graph_contract<D, O, F>(
    operation: &crate::domain_installation::WorthQueryInstalledDomainOperation<D, O, F>,
    role: &str,
    record: &crate::domain_installation::WorthQueryInstalledGraphParticipationRecord,
    counters: WorthQueryOperationBindingCounters,
) -> Result<(), WorthQueryOperationBindingDenial> {
    let contract = &record.definition.contract;
    if let Some(read) = operation
        .definition()
        .semantics()
        .graph_reads
        .roles()
        .iter()
        .find(|read| read.role == role)
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
                counters,
            ));
        }
    }
    let touched = matches!(
        &operation.definition().semantics().touches,
        WorthQueryOperationTouchContract::Declared { graph_roles, .. }
            if graph_roles.iter().any(|declared| declared == role)
    );
    if touched && contract.mutation != WorthQueryGraphMutationPosture::TouchAndEffect {
        return Err(WorthQueryOperationBindingDenial::new(
            WorthQueryOperationBindingDenialKind::GraphAuthorityInsufficient,
            format!("graph role `{role}` cannot satisfy declared touch/effect access"),
            counters,
        ));
    }
    let semantics = operation.definition().semantics();
    let identity_admitted = matches!(
        (semantics.lineage, contract.identity),
        (WorthQueryOperationLineageContract::NotRequired, _)
            | (
                WorthQueryOperationLineageContract::Preserve,
                WorthQueryGraphIdentityPosture::PreservedLineage
            )
            | (
                WorthQueryOperationLineageContract::Evolve,
                WorthQueryGraphIdentityPosture::EvolvingLineage
            )
    );
    if !identity_admitted {
        return Err(WorthQueryOperationBindingDenial::new(
            WorthQueryOperationBindingDenialKind::GraphAuthorityInsufficient,
            format!("graph role `{role}` cannot satisfy the operation lineage contract"),
            counters,
        ));
    }
    let execution_cost = semantics.cost.execution;
    let cost_admitted = match (contract.locality, contract.budget) {
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
    };
    if !cost_admitted {
        return Err(WorthQueryOperationBindingDenial::new(
            WorthQueryOperationBindingDenialKind::GraphAuthorityInsufficient,
            format!("graph role `{role}` exceeds the installed operation cost contract"),
            counters,
        ));
    }
    let declares_boundary_failure = semantics.terminal.failure_classes.iter().any(|class| {
        matches!(
            class,
            WorthQueryOperationFailureClass::Dependency
                | WorthQueryOperationFailureClass::Indeterminate
        )
    });
    let failure_admitted = match contract.failure {
        WorthQueryGraphFailureTopology::Local => true,
        WorthQueryGraphFailureTopology::BoundaryFailure => declares_boundary_failure,
        WorthQueryGraphFailureTopology::PartialCommitPossible => {
            declares_boundary_failure
                && semantics
                    .terminal
                    .result_states
                    .contains(&WorthQueryOperationResultState::Partial)
                && matches!(
                    &semantics.reversal,
                    WorthQueryOperationReversalContract::Compensation { .. }
                        | WorthQueryOperationReversalContract::RebuildRequired { .. }
                )
        }
    };
    if !failure_admitted {
        return Err(WorthQueryOperationBindingDenial::new(
            WorthQueryOperationBindingDenialKind::GraphAuthorityInsufficient,
            format!("graph role `{role}` exceeds the installed operation failure contract"),
            counters,
        ));
    }
    Ok(())
}
