use crate::admission::{
    UiAdmissionHostCapability, UiAdmissionQueryBasis, UiAdmissionSelectionBudget,
};
use crate::graph::UiGraphNodeIdentity;
use crate::graph::UiGraphWorldProfile;
use worth_ui_host_contract::{WorthUiHostCapabilityPosture, WorthUiHostCapabilityReport};

use super::UiAdmissionWorld;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAdmissionTarget {
    graph_node_identity: UiGraphNodeIdentity,
    world: UiAdmissionWorld,
    query_basis: UiAdmissionQueryBasis,
    host_capability: UiAdmissionHostCapability,
    selection_budget: UiAdmissionSelectionBudget,
    host_capability_report: Option<WorthUiHostCapabilityReport>,
}

impl UiAdmissionTarget {
    pub fn graph_node(graph_node_identity: UiGraphNodeIdentity, world: UiAdmissionWorld) -> Self {
        Self {
            graph_node_identity,
            world,
            query_basis: UiAdmissionQueryBasis::graph_aligned(),
            host_capability: UiAdmissionHostCapability::available(),
            selection_budget: UiAdmissionSelectionBudget::unbounded(),
            host_capability_report: None,
        }
    }

    pub fn graph_node_identity(&self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }

    pub fn world(&self) -> &UiAdmissionWorld {
        &self.world
    }

    pub fn query_basis(&self) -> UiAdmissionQueryBasis {
        if matches!(
            self.world.graph_world_profile(),
            UiGraphWorldProfile::SettledQueryBinding { .. }
        ) {
            UiAdmissionQueryBasis::GraphAligned
        } else {
            self.query_basis
        }
    }

    pub fn host_capability(&self) -> UiAdmissionHostCapability {
        self.host_capability_report
            .as_ref()
            .map(host_capability_from_report)
            .unwrap_or(self.host_capability)
    }

    pub fn selection_budget(&self) -> UiAdmissionSelectionBudget {
        self.selection_budget
    }

    pub fn host_capability_report(&self) -> Option<&WorthUiHostCapabilityReport> {
        self.host_capability_report.as_ref()
    }

    pub fn with_selection_budget(mut self, selection_budget: UiAdmissionSelectionBudget) -> Self {
        self.selection_budget = selection_budget;
        self
    }

    pub fn with_host_capability_report(
        mut self,
        host_capability_report: WorthUiHostCapabilityReport,
    ) -> Self {
        self.host_capability_report = Some(host_capability_report);
        self
    }
}

fn host_capability_from_report(report: &WorthUiHostCapabilityReport) -> UiAdmissionHostCapability {
    match report.posture() {
        WorthUiHostCapabilityPosture::Available => UiAdmissionHostCapability::Available,
        WorthUiHostCapabilityPosture::Missing => UiAdmissionHostCapability::Missing,
        WorthUiHostCapabilityPosture::Ambiguous | WorthUiHostCapabilityPosture::DiagnosticOnly => {
            UiAdmissionHostCapability::Ambiguous
        }
    }
}
