use crate::admission::{
    UiAdmissionHostCapability, UiAdmissionQueryBasis, UiAdmissionSelectionBudget,
};
use crate::graph::UiGraphNodeIdentity;
use crate::graph::UiGraphWorldProfile;
use worth_ui_host_contract::{WorthUiHostCapabilityPosture, WorthUiHostCapabilityReport};
use worth_ui_query_binding::{
    WorthUiQueryBasisPosture, WorthUiQueryBindingSubsystem,
    WorthUiQueryMeasurementFactEligibilityError, WorthUiQueryPrerequisiteEvidence,
};

use super::UiAdmissionWorld;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAdmissionTarget {
    graph_node_identity: UiGraphNodeIdentity,
    world: UiAdmissionWorld,
    query_basis: UiAdmissionQueryBasis,
    host_capability: UiAdmissionHostCapability,
    selection_budget: UiAdmissionSelectionBudget,
    query_prerequisites: Option<WorthUiQueryPrerequisiteEvidence>,
    host_capability_report: Option<WorthUiHostCapabilityReport>,
}

impl UiAdmissionTarget {
    pub fn graph_node(graph_node_identity: UiGraphNodeIdentity, world: UiAdmissionWorld) -> Self {
        let query_prerequisites = query_prerequisites_for_world(&world);
        Self {
            graph_node_identity,
            world,
            query_basis: UiAdmissionQueryBasis::graph_aligned(),
            host_capability: UiAdmissionHostCapability::available(),
            selection_budget: UiAdmissionSelectionBudget::unbounded(),
            query_prerequisites,
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
        self.query_prerequisites
            .as_ref()
            .map(query_basis_from_evidence)
            .unwrap_or(self.query_basis)
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

    pub fn query_prerequisites(&self) -> Option<&WorthUiQueryPrerequisiteEvidence> {
        self.query_prerequisites.as_ref()
    }

    pub fn host_capability_report(&self) -> Option<&WorthUiHostCapabilityReport> {
        self.host_capability_report.as_ref()
    }

    pub fn with_selection_budget(mut self, selection_budget: UiAdmissionSelectionBudget) -> Self {
        self.selection_budget = selection_budget;
        self
    }

    pub fn with_query_prerequisites(
        mut self,
        query_prerequisites: WorthUiQueryPrerequisiteEvidence,
    ) -> Self {
        self.query_prerequisites = Some(query_prerequisites);
        self
    }

    pub fn with_query_prerequisites_from_projection_consumption(
        mut self,
        consumption: &forge_query::facade::ProjectionFactConsumptionAttempt,
    ) -> Result<Self, WorthUiQueryMeasurementFactEligibilityError> {
        let Some(query_prerequisites) = self.query_prerequisites.take() else {
            return Ok(self);
        };
        self.query_prerequisites = Some(
            WorthUiQueryBindingSubsystem::bootstrap()
                .prerequisites()
                .bind_projection_consumption(query_prerequisites, consumption)?,
        );
        Ok(self)
    }

    pub fn with_host_capability_report(
        mut self,
        host_capability_report: WorthUiHostCapabilityReport,
    ) -> Self {
        self.host_capability_report = Some(host_capability_report);
        self
    }
}

fn query_prerequisites_for_world(
    world: &UiAdmissionWorld,
) -> Option<WorthUiQueryPrerequisiteEvidence> {
    let UiGraphWorldProfile::QuerySnapshotBasis {
        basis,
        resolution_report,
    } = world.graph_world_profile()
    else {
        return None;
    };

    WorthUiQueryBindingSubsystem::bootstrap()
        .prerequisites()
        .graph_aligned(basis.clone(), resolution_report.clone())
        .ok()
}

fn query_basis_from_evidence(evidence: &WorthUiQueryPrerequisiteEvidence) -> UiAdmissionQueryBasis {
    match evidence.basis_posture() {
        WorthUiQueryBasisPosture::GraphAligned => UiAdmissionQueryBasis::GraphAligned,
        WorthUiQueryBasisPosture::WrongWorldProjection => {
            UiAdmissionQueryBasis::WrongWorldProjection
        }
        WorthUiQueryBasisPosture::RebindRequired => UiAdmissionQueryBasis::RebindRequired,
        WorthUiQueryBasisPosture::StaleReceipt => UiAdmissionQueryBasis::StaleReceipt,
        WorthUiQueryBasisPosture::AmbiguousSources => UiAdmissionQueryBasis::AmbiguousSources,
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
