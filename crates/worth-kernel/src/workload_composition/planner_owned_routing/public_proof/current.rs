use crate::workload_composition::compiled_product_consumer_cutover::{
    current_kernel_compiled_product_consumer_dependency_matrix_with_targets_loader,
    KernelCompiledProductConsumerCoverageTarget,
};
use crate::workload_composition::performance_trace::trace_scope;
use crate::workload_composition::planner_owned_routing::admit_worth_touched_graph_conflict_public_proof_input;
use crate::workload_composition::planner_owned_routing::current_worth_workload_ordinary_consumer_cutover;
use crate::workload_composition::{
    current_kernel_compiled_product_consumer_dependency_matrix,
    current_worth_touched_graph_conflict_deletion_closeout,
    current_worth_touched_graph_conflict_selected_route_packet,
    current_worth_touched_graph_conflict_source_firewall_report,
    KernelCompiledProductConsumerDependencyError, KernelCompiledProductConsumerDependencyMatrix,
    PlannerOwnedRoutingError, WorthTouchedGraphConflictAdmittedPublicProofInput,
    WorthTouchedGraphConflictSelectedRoutePacket,
};

use super::assembly::assemble_public_closeout_from_parts;
use super::assembly_types::CurrentWorthTouchedGraphConflictPublicProofAssemblyComponents;
use super::types::{
    WorthTouchedGraphConflictPublicCloseout, WorthTouchedGraphConflictPublicCloseoutError,
    WorthTouchedGraphConflictPublicCloseoutErrorKind,
};

pub fn current_worth_touched_graph_conflict_public_closeout(
) -> Result<WorthTouchedGraphConflictPublicCloseout, WorthTouchedGraphConflictPublicCloseoutError> {
    trace_scope(
        "current_worth_touched_graph_conflict_public_closeout",
        || {
            let components = current_public_closeout_components()?;
            assemble_public_closeout_from_parts(
                components.input()?,
                components.cutover(),
                components.selected_route_packet(),
                components.admitted_public_proof_input(),
            )
        },
    )
}

pub fn current_worth_touched_graph_conflict_milestone_fifteen_seed() -> Result<
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

pub(crate) fn current_public_closeout_components() -> Result<
    CurrentWorthTouchedGraphConflictPublicProofAssemblyComponents,
    WorthTouchedGraphConflictPublicCloseoutError,
> {
    current_public_closeout_components_with_loaders(
        current_worth_touched_graph_conflict_selected_route_packet,
        admit_worth_touched_graph_conflict_public_proof_input,
    )
}

fn current_public_closeout_components_with_loaders<R, A>(
    load_selected_route_packet: R,
    admit_public_proof_input: A,
) -> Result<
    CurrentWorthTouchedGraphConflictPublicProofAssemblyComponents,
    WorthTouchedGraphConflictPublicCloseoutError,
>
where
    R: FnOnce() -> Result<WorthTouchedGraphConflictSelectedRoutePacket, PlannerOwnedRoutingError>,
    A: FnOnce(
        &WorthTouchedGraphConflictSelectedRoutePacket,
    ) -> Result<
        WorthTouchedGraphConflictAdmittedPublicProofInput,
        PlannerOwnedRoutingError,
    >,
{
    trace_scope("current_public_closeout_components", || {
        let cutover = trace_scope("public_closeout_current_cutover", || {
            current_worth_workload_ordinary_consumer_cutover().map_err(|error| {
                WorthTouchedGraphConflictPublicCloseoutError::new(
                    WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable,
                    format!("phase 13 ordinary-consumer cutover did not assemble: {error:?}"),
                )
            })
        })?;
        let deletion_closeout = trace_scope("public_closeout_deletion_closeout", || {
            current_worth_touched_graph_conflict_deletion_closeout().map_err(|error| {
                WorthTouchedGraphConflictPublicCloseoutError::new(
                    WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable,
                    format!("phase 13 deletion closeout did not assemble: {error:?}"),
                )
            })
        })?;
        let source_firewall_report = trace_scope("public_closeout_source_firewall_report", || {
            current_worth_touched_graph_conflict_source_firewall_report().map_err(|error| {
                WorthTouchedGraphConflictPublicCloseoutError::new(
                    WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable,
                    format!("phase 13 source firewall report did not assemble: {error:?}"),
                )
            })
        })?;
        let selected_route_packet = trace_scope("public_closeout_selected_route_packet", || {
            load_selected_route_packet().map_err(map_planner_owned_routing_error)
        })?;
        let admitted_public_proof_input =
            trace_scope("public_closeout_admitted_public_proof_input", || {
                admit_public_proof_input(&selected_route_packet)
                    .map_err(map_planner_owned_routing_error)
            })?;
        Ok(
            CurrentWorthTouchedGraphConflictPublicProofAssemblyComponents::new(
                cutover,
                deletion_closeout,
                source_firewall_report,
                selected_route_packet,
                admitted_public_proof_input,
            ),
        )
    })
}

pub(crate) fn current_public_closeout_components_with_matrix_loader<F>(
    load_matrix: F,
) -> Result<
    CurrentWorthTouchedGraphConflictPublicProofAssemblyComponents,
    WorthTouchedGraphConflictPublicCloseoutError,
>
where
    F: FnOnce() -> Result<
        KernelCompiledProductConsumerDependencyMatrix,
        KernelCompiledProductConsumerDependencyError,
    >,
{
    trace_scope("public_closeout_dependency_matrix_probe", || {
        load_matrix().map_err(|error| {
            WorthTouchedGraphConflictPublicCloseoutError::new(
                WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable,
                format!("phase 14 kernel consumer dependency matrix did not assemble: {error:?}"),
            )
        })
    })?;
    current_public_closeout_components_with_loaders(
        current_worth_touched_graph_conflict_selected_route_packet,
        admit_worth_touched_graph_conflict_public_proof_input,
    )
}

#[cfg(test)]
pub(crate) fn current_public_closeout_components_with_matrix_targets_loader<F>(
    load_targets: F,
) -> Result<
    CurrentWorthTouchedGraphConflictPublicProofAssemblyComponents,
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

#[cfg(test)]
pub(crate) fn current_public_closeout_components_with_route_loader<R>(
    load_selected_route_packet: R,
) -> Result<
    CurrentWorthTouchedGraphConflictPublicProofAssemblyComponents,
    WorthTouchedGraphConflictPublicCloseoutError,
>
where
    R: FnOnce() -> Result<WorthTouchedGraphConflictSelectedRoutePacket, PlannerOwnedRoutingError>,
{
    current_public_closeout_components_with_loaders(
        load_selected_route_packet,
        admit_worth_touched_graph_conflict_public_proof_input,
    )
}

#[cfg(test)]
pub(crate) fn current_worth_touched_graph_conflict_public_closeout_with_route_loader<R>(
    load_selected_route_packet: R,
) -> Result<WorthTouchedGraphConflictPublicCloseout, WorthTouchedGraphConflictPublicCloseoutError>
where
    R: FnOnce() -> Result<WorthTouchedGraphConflictSelectedRoutePacket, PlannerOwnedRoutingError>,
{
    let components =
        current_public_closeout_components_with_route_loader(load_selected_route_packet)?;
    assemble_public_closeout_from_parts(
        components.input()?,
        components.cutover(),
        components.selected_route_packet(),
        components.admitted_public_proof_input(),
    )
}
