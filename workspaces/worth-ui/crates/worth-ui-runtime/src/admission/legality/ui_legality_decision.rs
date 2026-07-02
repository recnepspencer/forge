use crate::admission::{
    UiAdmissionHostCapability, UiAdmissionQueryBasis, UiAdmissionSelectionBudget,
    UiAdmissionStaleEvidence,
};
use crate::declaration::UiDeclarationIdentity;
use crate::graph::UiGraphNodeIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiLegalityReason {
    MissingDeclarationArtifact,
    MissingQueryPrerequisiteEvidence,
    MissingHostCapabilityReport,
    QueryBindingRequiresLaterRuntimeLane,
    ServiceUsageRequiresLaterRuntimeLane,
    WrongQueryBasis {
        required: UiAdmissionQueryBasis,
        observed: UiAdmissionQueryBasis,
    },
    WrongHostCapability {
        required: UiAdmissionHostCapability,
        observed: UiAdmissionHostCapability,
    },
    Stale {
        required: UiAdmissionQueryBasis,
        observed: UiAdmissionQueryBasis,
        evidence: UiAdmissionStaleEvidence,
    },
    Ambiguous {
        required_query_basis: Option<UiAdmissionQueryBasis>,
        observed_query_basis: Option<UiAdmissionQueryBasis>,
        required_host_capability: Option<UiAdmissionHostCapability>,
        observed_host_capability: Option<UiAdmissionHostCapability>,
    },
    RebindRequired {
        required: UiAdmissionQueryBasis,
        observed: UiAdmissionQueryBasis,
    },
    BudgetExceeded {
        budget: UiAdmissionSelectionBudget,
        attempted_lane_cost: u8,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiLegalityPosture {
    Denied(UiLegalityReason),
    Admitted,
    AdmittedWithAdvisory(UiLegalityReason),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiLegalityDecision {
    graph_node_identity: Option<UiGraphNodeIdentity>,
    declaration_identity: Option<UiDeclarationIdentity>,
    posture: UiLegalityPosture,
}

impl UiLegalityDecision {
    pub(crate) fn denied(
        graph_node_identity: Option<UiGraphNodeIdentity>,
        declaration_identity: Option<UiDeclarationIdentity>,
        reason: UiLegalityReason,
    ) -> Self {
        Self {
            graph_node_identity,
            declaration_identity,
            posture: UiLegalityPosture::Denied(reason),
        }
    }

    pub(crate) fn admitted(
        graph_node_identity: UiGraphNodeIdentity,
        declaration_identity: UiDeclarationIdentity,
    ) -> Self {
        Self {
            graph_node_identity: Some(graph_node_identity),
            declaration_identity: Some(declaration_identity),
            posture: UiLegalityPosture::Admitted,
        }
    }

    pub(crate) fn admitted_with_advisory(
        graph_node_identity: UiGraphNodeIdentity,
        declaration_identity: UiDeclarationIdentity,
        reason: UiLegalityReason,
    ) -> Self {
        Self {
            graph_node_identity: Some(graph_node_identity),
            declaration_identity: Some(declaration_identity),
            posture: UiLegalityPosture::AdmittedWithAdvisory(reason),
        }
    }

    pub fn graph_node_identity(&self) -> Option<UiGraphNodeIdentity> {
        self.graph_node_identity
    }

    pub fn declaration_identity(&self) -> Option<&UiDeclarationIdentity> {
        self.declaration_identity.as_ref()
    }

    pub fn posture(&self) -> UiLegalityPosture {
        self.posture
    }
}
