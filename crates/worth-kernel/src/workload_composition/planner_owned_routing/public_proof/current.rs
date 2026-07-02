use crate::workload_composition::planner_owned_routing::admit_worth_touched_graph_conflict_public_proof_input;
use crate::workload_composition::compiled_product_consumer_cutover::{
    current_kernel_compiled_product_consumer_dependency_matrix_with_targets_loader,
    KernelCompiledProductConsumerCoverageTarget,
};
use crate::workload_composition::public_closeout::{
    CurrentWorthTouchedGraphConflictPublicCloseoutComponents,
    WorthTouchedGraphConflictPublicCloseout, WorthTouchedGraphConflictPublicCloseoutError,
    WorthTouchedGraphConflictPublicCloseoutErrorKind,
};
use crate::workload_composition::worth_workload::current_worth_workload_ordinary_consumer_cutover;
use crate::workload_composition::{
    current_kernel_compiled_product_consumer_dependency_matrix,
    current_worth_touched_graph_conflict_deletion_closeout,
    current_worth_touched_graph_conflict_selected_route_packet,
    current_worth_touched_graph_conflict_source_firewall_report,
    KernelCompiledProductConsumerDependencyError, KernelCompiledProductConsumerDependencyMatrix,
    PlannerOwnedRoutingError,
};

use super::assembly::assemble_public_closeout_from_parts;

pub fn current_worth_touched_graph_conflict_public_closeout(
) -> Result<WorthTouchedGraphConflictPublicCloseout, WorthTouchedGraphConflictPublicCloseoutError> {
    let components = current_public_closeout_components()?;
    assemble_public_closeout_from_parts(
        components.input()?,
        components.cutover(),
        components.selected_route_packet(),
        components.admitted_public_proof_input(),
    )
}

pub fn current_worth_touched_graph_conflict_milestone_fifteen_seed(
) -> Result<
    crate::workload_composition::WorthTouchedGraphConflictMilestoneFifteenSeed,
    WorthTouchedGraphConflictPublicCloseoutError,
> {
    Ok(current_worth_touched_graph_conflict_public_closeout()?
        .milestone_fifteen_seed()
        .clone())
}

fn map_planner_owned_routing_error(
    error: PlannerOwnedRoutingError,
) -> WorthTouchedGraphConflictPublicCloseoutError {
    WorthTouchedGraphConflictPublicCloseoutError::new(
        WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable,
        error.detail(),
    )
}

pub(crate) fn current_public_closeout_components(
) -> Result<
    CurrentWorthTouchedGraphConflictPublicCloseoutComponents,
    WorthTouchedGraphConflictPublicCloseoutError,
> {
    current_public_closeout_components_with_matrix_loader(
        current_kernel_compiled_product_consumer_dependency_matrix,
    )
}

pub(crate) fn current_public_closeout_components_with_matrix_loader<F>(
    load_matrix: F,
) -> Result<
    CurrentWorthTouchedGraphConflictPublicCloseoutComponents,
    WorthTouchedGraphConflictPublicCloseoutError,
>
where
    F: FnOnce() -> Result<
        KernelCompiledProductConsumerDependencyMatrix,
        KernelCompiledProductConsumerDependencyError,
    >,
{
    let cutover = current_worth_workload_ordinary_consumer_cutover().map_err(|error| {
        WorthTouchedGraphConflictPublicCloseoutError::new(
            WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable,
            format!("phase 13 ordinary-consumer cutover did not assemble: {error:?}"),
        )
    })?;
    load_matrix().map_err(|error| {
        WorthTouchedGraphConflictPublicCloseoutError::new(
            WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable,
            format!("phase 14 kernel consumer dependency matrix did not assemble: {error:?}"),
        )
    })?;
    let deletion_closeout =
        current_worth_touched_graph_conflict_deletion_closeout().map_err(|error| {
            WorthTouchedGraphConflictPublicCloseoutError::new(
                WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable,
                format!("phase 13 deletion closeout did not assemble: {error:?}"),
            )
        })?;
    let source_firewall_report = current_worth_touched_graph_conflict_source_firewall_report()
        .map_err(|error| {
            WorthTouchedGraphConflictPublicCloseoutError::new(
                WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable,
                format!("phase 13 source firewall report did not assemble: {error:?}"),
            )
        })?;
    let selected_route_packet =
        current_worth_touched_graph_conflict_selected_route_packet()
            .map_err(map_planner_owned_routing_error)?;
    let admitted_public_proof_input =
        admit_worth_touched_graph_conflict_public_proof_input(&selected_route_packet)
            .map_err(map_planner_owned_routing_error)?;
    Ok(
        CurrentWorthTouchedGraphConflictPublicCloseoutComponents::new(
            cutover,
            deletion_closeout,
            source_firewall_report,
            selected_route_packet,
            admitted_public_proof_input,
        ),
    )
}

#[cfg(test)]
pub(crate) fn current_public_closeout_components_with_matrix_targets_loader<F>(
    load_targets: F,
) -> Result<
    CurrentWorthTouchedGraphConflictPublicCloseoutComponents,
    WorthTouchedGraphConflictPublicCloseoutError,
>
where
    F: FnOnce() -> Result<
        Vec<KernelCompiledProductConsumerCoverageTarget>,
        KernelCompiledProductConsumerDependencyError,
    >,
{
    current_public_closeout_components_with_matrix_loader(|| {
        current_kernel_compiled_product_consumer_dependency_matrix_with_targets_loader(load_targets)
    })
}
