use crate::certification::TopologyBranchAuthoringBoundary;
use crate::topology_operators::{TopologyMutationDigest, TopologyMutationFamily};
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MilestoneThreeScalePressureSweep {
    HighCardinalityLoops,
    HighFaceCountShells,
    LargeBranchLocalHistories,
    WireMembershipDetach,
    RadialAdjacencySplice,
    RadialAdjacencyDetach,
}

impl MilestoneThreeScalePressureSweep {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighCardinalityLoops => "high_cardinality_loops",
            Self::HighFaceCountShells => "high_face_count_shells",
            Self::LargeBranchLocalHistories => "large_branch_local_histories",
            Self::WireMembershipDetach => "wire_membership_detach",
            Self::RadialAdjacencySplice => "radial_adjacency_splice",
            Self::RadialAdjacencyDetach => "radial_adjacency_detach",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeScalePressureRow {
    pub(crate) sweep: MilestoneThreeScalePressureSweep,
    pub(crate) primitive_family: String,
    pub(crate) primitive: MilestoneOnePrimitiveCase,
    pub(crate) workload_size: usize,
    pub(crate) mutation_step_count: usize,
    pub(crate) mutation_families: Vec<TopologyMutationFamily>,
    pub(crate) branch_local: bool,
    pub(crate) branch_authoring_boundary: Option<TopologyBranchAuthoringBoundary>,
    pub(crate) topology_mutation_digest: TopologyMutationDigest,
    pub(crate) replay_verified: bool,
    pub(crate) final_state_digest: String,
    pub(crate) replay_final_state_digest: String,
    pub(crate) derived_validation_row_count: usize,
    pub(crate) row_digest: String,
}
