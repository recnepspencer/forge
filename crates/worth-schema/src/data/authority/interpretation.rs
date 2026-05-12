use forge_relational::facade::identity::EntityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WireInterpretationClass {
    OpenChain,
    ClosedCycle,
    ConnectedBranch,
    Disconnected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ShellInterpretationClass {
    OpenSheet,
    ClosedSolid,
    OpenNonManifold,
    ClosedNonManifold,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireInterpretationRecord {
    pub wire_id: EntityId,
    pub class: WireInterpretationClass,
    pub connected_component_count: usize,
    pub terminal_vertex_ids: Vec<EntityId>,
    pub branch_vertex_ids: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellInterpretationRecord {
    pub shell_id: EntityId,
    pub class: ShellInterpretationClass,
    pub face_count: usize,
    pub boundary_component_count: usize,
    pub boundary_half_edge_count: usize,
    pub non_manifold_edge_ids: Vec<EntityId>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyInterpretationRecordSet {
    pub wires: Vec<WireInterpretationRecord>,
    pub shells: Vec<ShellInterpretationRecord>,
}
