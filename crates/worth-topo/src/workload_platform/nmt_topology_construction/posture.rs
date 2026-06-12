use schema::facade::platform::authority::ShellInterpretationClass;

use crate::derived_topology::traversal_views::types::InterpretedTopologyView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NmtTopologyPosture {
    OpenWire,
    OpenSheet,
    OpenNonManifold,
    LayeredOpen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyPostureReceipt {
    posture: NmtTopologyPosture,
    shell_count: usize,
    wire_count: usize,
    boundary_half_edge_count: usize,
    non_manifold_edge_count: usize,
}

impl TopologyPostureReceipt {
    pub(crate) fn from_interpreted(
        interpreted: &InterpretedTopologyView,
        wire_count: usize,
        layer_count: usize,
    ) -> Self {
        let boundary_half_edge_count = interpreted
            .interpretations()
            .shells
            .iter()
            .map(|shell| shell.boundary_half_edge_count)
            .sum();
        let non_manifold_edge_count = interpreted
            .interpretations()
            .shells
            .iter()
            .map(|shell| shell.non_manifold_edge_ids.len())
            .sum();
        let shell_count = interpreted.interpretations().shells.len();
        let open_non_manifold = interpreted
            .interpretations()
            .shells
            .iter()
            .any(|shell| shell.class == ShellInterpretationClass::OpenNonManifold);

        let posture = if layer_count > 1 {
            NmtTopologyPosture::LayeredOpen
        } else if open_non_manifold {
            NmtTopologyPosture::OpenNonManifold
        } else if shell_count > 0 {
            NmtTopologyPosture::OpenSheet
        } else {
            NmtTopologyPosture::OpenWire
        };

        Self {
            posture,
            shell_count,
            wire_count,
            boundary_half_edge_count,
            non_manifold_edge_count,
        }
    }

    pub fn posture(&self) -> NmtTopologyPosture {
        self.posture
    }

    pub fn shell_count(&self) -> usize {
        self.shell_count
    }

    pub fn wire_count(&self) -> usize {
        self.wire_count
    }

    pub fn boundary_half_edge_count(&self) -> usize {
        self.boundary_half_edge_count
    }

    pub fn non_manifold_edge_count(&self) -> usize {
        self.non_manifold_edge_count
    }
}
