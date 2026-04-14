use forge_relational::facade::identity::EntityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthWireInterpretationClass {
    OpenChain,
    ClosedCycle,
    ConnectedBranch,
    Disconnected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthShellInterpretationClass {
    OpenSheet,
    ClosedSolid,
    OpenNonManifold,
    ClosedNonManifold,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthWireInterpretationRecord {
    pub wire_id: EntityId,
    pub class: WorthWireInterpretationClass,
    pub connected_component_count: usize,
    pub terminal_vertex_ids: Vec<EntityId>,
    pub branch_vertex_ids: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthShellInterpretationRecord {
    pub shell_id: EntityId,
    pub class: WorthShellInterpretationClass,
    pub boundary_half_edge_count: usize,
    pub non_manifold_edge_ids: Vec<EntityId>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyInterpretationRecordSet {
    pub wires: Vec<WorthWireInterpretationRecord>,
    pub shells: Vec<WorthShellInterpretationRecord>,
}
