use crate::replay_undo_semantic_graph::{
    current_topology_invalidation_proof, CurrentTopologyInvalidationProofError,
};

use super::route_input::{admit_topology_invalidation_route_input, TopologyInvalidationRouteInput};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyInvalidationRouteInputCurrentError {
    detail: String,
}

pub fn current_topology_invalidation_route_input(
) -> Result<TopologyInvalidationRouteInput, TopologyInvalidationRouteInputCurrentError> {
    let proof = current_topology_invalidation_proof().map_err(current_runtime_error)?;
    admit_topology_invalidation_route_input(proof.touched_closure(), proof.selected_plan())
        .map_err(TopologyInvalidationRouteInputCurrentError::from_admission)
}

impl TopologyInvalidationRouteInputCurrentError {
    fn from_admission(
        error: super::admission_error::TopologyInvalidationRouteInputAdmissionError,
    ) -> Self {
        Self {
            detail: error.detail().to_string(),
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

fn current_runtime_error(
    error: CurrentTopologyInvalidationProofError,
) -> TopologyInvalidationRouteInputCurrentError {
    TopologyInvalidationRouteInputCurrentError {
        detail: format!(
            "current topology invalidation route input did not assemble: {}",
            error.detail()
        ),
    }
}
