use crate::derived_invalidation_route_input::admit_topology_invalidation_route_input;
use crate::projection::planner_owned_routing::query_backed_read_family::current_topology_query_backed_read_family_artifacts;
use crate::replay_undo_semantic_graph::current_topology_invalidation_proof;

use super::diagnostic_input::{
    admit_topology_derived_read_diagnostic_input, TopologyDerivedReadDiagnosticInput,
};
use super::selected_route_authority::TopologyDerivedReadDiagnosticSelectedRouteAuthority;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyDerivedReadDiagnosticInputCurrentError {
    detail: String,
}

pub fn current_topology_derived_read_diagnostic_input_with_selected_route_authority(
    authority: &TopologyDerivedReadDiagnosticSelectedRouteAuthority,
) -> Result<TopologyDerivedReadDiagnosticInput, TopologyDerivedReadDiagnosticInputCurrentError> {
    let proof = current_topology_invalidation_proof().map_err(current_runtime_error)?;
    let invalidation_route_input =
        admit_topology_invalidation_route_input(proof.touched_closure(), proof.selected_plan())
            .map_err(|error| TopologyDerivedReadDiagnosticInputCurrentError {
                detail: error.detail().to_string(),
            })?;
    let artifacts =
        current_topology_query_backed_read_family_artifacts().map_err(current_runtime_error)?;

    admit_topology_derived_read_diagnostic_input(
        &invalidation_route_input,
        authority,
        artifacts.read_basis(),
        artifacts.materialized(),
        artifacts.interpreted(),
        artifacts.validation(),
    )
    .map_err(|error| TopologyDerivedReadDiagnosticInputCurrentError {
        detail: error.detail().to_string(),
    })
}

fn current_runtime_error(
    error: impl std::fmt::Debug,
) -> TopologyDerivedReadDiagnosticInputCurrentError {
    TopologyDerivedReadDiagnosticInputCurrentError {
        detail: format!(
            "current topology derived-read diagnostic input did not assemble: {error:?}"
        ),
    }
}

impl TopologyDerivedReadDiagnosticInputCurrentError {
    pub fn detail(&self) -> &str {
        &self.detail
    }
}
