use crate::admission::{UiAdmissionTarget, UiAdmissionWorld};
use crate::declaration::{UiDeclarationIdentity, UiDeclarationSupportMilestoneExpectation};
use crate::evidence::UiEvidenceAuthorityGeneration;
use crate::graph::UiGraphNodeIdentity;
use worth_ui_host_contract::{
    WorthUiHostCapabilityObservationGeneration, WorthUiHostCapabilityReport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiMeasurementUnsupportedReason {
    Support(crate::admission::UiSupportReason),
    SelectionDidNotYieldMeasurementRequirement,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiMeasurementCapabilityGateReason {
    MissingHostCapabilityReport,
    MissingHostCapability,
    AmbiguousHostCapability,
    DiagnosticOnlyHostCapability,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiMeasurementAdmissionPosture {
    Admitted {
        world: UiAdmissionWorld,
        host_capability: WorthUiHostCapabilityReport,
    },
    Unsupported {
        world: UiAdmissionWorld,
        reason: UiMeasurementUnsupportedReason,
    },
    WrongWorld {
        expected: UiAdmissionWorld,
        observed: UiAdmissionWorld,
    },
    Deferred {
        world: UiAdmissionWorld,
        expected_in: UiDeclarationSupportMilestoneExpectation,
    },
    DiagnosticOnly {
        world: UiAdmissionWorld,
    },
    CapabilityGated {
        world: UiAdmissionWorld,
        reason: UiMeasurementCapabilityGateReason,
    },
    StaleSupportPosture {
        world: UiAdmissionWorld,
        selected_generation: UiEvidenceAuthorityGeneration,
        boundary_generation: UiEvidenceAuthorityGeneration,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMeasurementAdmission {
    target: UiAdmissionTarget,
    graph_node_identity: UiGraphNodeIdentity,
    declaration_identity: Option<UiDeclarationIdentity>,
    touch_identity_digest: u64,
    selected_measurement_obligation_identity_digest: Option<u64>,
    selected_support_authority_generation: UiEvidenceAuthorityGeneration,
    boundary_support_authority_generation: UiEvidenceAuthorityGeneration,
    host_capability_profile_digest: Option<u64>,
    host_capability_observation_generation: Option<WorthUiHostCapabilityObservationGeneration>,
    posture: UiMeasurementAdmissionPosture,
}

impl UiMeasurementAdmission {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        target: UiAdmissionTarget,
        graph_node_identity: UiGraphNodeIdentity,
        declaration_identity: Option<UiDeclarationIdentity>,
        touch_identity_digest: u64,
        selected_measurement_obligation_identity_digest: Option<u64>,
        selected_support_authority_generation: UiEvidenceAuthorityGeneration,
        boundary_support_authority_generation: UiEvidenceAuthorityGeneration,
        host_capability_profile_digest: Option<u64>,
        host_capability_observation_generation: Option<WorthUiHostCapabilityObservationGeneration>,
        posture: UiMeasurementAdmissionPosture,
    ) -> Self {
        Self {
            target,
            graph_node_identity,
            declaration_identity,
            touch_identity_digest,
            selected_measurement_obligation_identity_digest,
            selected_support_authority_generation,
            boundary_support_authority_generation,
            host_capability_profile_digest,
            host_capability_observation_generation,
            posture,
        }
    }

    pub fn target(&self) -> &UiAdmissionTarget {
        &self.target
    }

    pub fn graph_node_identity(&self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }

    pub fn declaration_identity(&self) -> Option<&UiDeclarationIdentity> {
        self.declaration_identity.as_ref()
    }

    pub fn touch_identity_digest(&self) -> u64 {
        self.touch_identity_digest
    }

    pub fn selected_measurement_obligation_identity_digest(&self) -> Option<u64> {
        self.selected_measurement_obligation_identity_digest
    }

    pub fn selected_support_authority_generation(&self) -> UiEvidenceAuthorityGeneration {
        self.selected_support_authority_generation
    }

    pub fn boundary_support_authority_generation(&self) -> UiEvidenceAuthorityGeneration {
        self.boundary_support_authority_generation
    }

    pub fn host_capability_profile_digest(&self) -> Option<u64> {
        self.host_capability_profile_digest
    }

    pub fn host_capability_observation_generation(
        &self,
    ) -> Option<WorthUiHostCapabilityObservationGeneration> {
        self.host_capability_observation_generation
    }

    pub fn posture(&self) -> &UiMeasurementAdmissionPosture {
        &self.posture
    }
}
