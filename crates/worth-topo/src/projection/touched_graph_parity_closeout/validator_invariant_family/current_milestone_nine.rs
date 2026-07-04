use crate::projection::touched_graph_parity_closeout::TopologyTouchedGraphParityCoverageError;
use crate::validator_invariant_catalog::{
    current_topology_validator_invariant_milestone_nine_closeout,
    WorthTopologyMilestoneNineCloseout,
};

pub(super) fn current_projection_validator_invariant_milestone_nine_closeout(
) -> Result<WorthTopologyMilestoneNineCloseout, TopologyTouchedGraphParityCoverageError> {
    current_topology_validator_invariant_milestone_nine_closeout()
        .map_err(|error| TopologyTouchedGraphParityCoverageError::new(format!("{error:?}")))
}
