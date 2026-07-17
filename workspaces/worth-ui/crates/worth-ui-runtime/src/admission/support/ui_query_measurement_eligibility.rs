use crate::admission::{UiAdmissionTarget, UiAdmissionWorld};
use crate::declaration::UiDeclarationIdentity;
use crate::evidence::{
    query_measurement_fact_family_set_digest, UiEvidenceAuthorityGeneration,
    UiProjectionFactReceipt,
};
use crate::graph::UiGraphNodeIdentity;
use worth_ui_query_binding::WorthUiQueryMeasurementFactFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiQueryMeasurementUnsupportedQueryReason {
    MissingQueryPrerequisites,
    WrongWorldProjection,
    RebindRequired,
    AmbiguousSources,
    ProjectionConsumptionUnavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiQueryMeasurementBasisAuthority {
    AdmittedPrerequisites {
        prerequisites: Box<worth_ui_query_binding::WorthUiQueryPrerequisiteEvidence>,
    },
    ProjectionConsumption {
        authority: Box<worth_ui_query_binding::WorthUiQueryAuthorityHandle>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiQueryMeasurementEligibilityPosture {
    Eligible {
        world: UiAdmissionWorld,
        available_families: Box<[WorthUiQueryMeasurementFactFamily]>,
        available_fact_family_set_digest: u64,
    },
    UnsupportedQueryPosture {
        world: UiAdmissionWorld,
        reason: UiQueryMeasurementUnsupportedQueryReason,
    },
    StaleBasisGeneration {
        world: UiAdmissionWorld,
        expected: UiQueryMeasurementBasisAuthority,
        observed: UiQueryMeasurementBasisAuthority,
    },
    UnavailableFactFamilies {
        world: UiAdmissionWorld,
        available_families: Box<[WorthUiQueryMeasurementFactFamily]>,
        missing_families: Box<[WorthUiQueryMeasurementFactFamily]>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiQueryMeasurementEligibility {
    target: UiAdmissionTarget,
    graph_node_identity: UiGraphNodeIdentity,
    declaration_identity: Option<UiDeclarationIdentity>,
    touch_identity_digest: u64,
    selected_measurement_obligation_identity_digest: Option<u64>,
    selected_support_authority_generation: UiEvidenceAuthorityGeneration,
    boundary_support_authority_generation: UiEvidenceAuthorityGeneration,
    required_fact_family_set_digest: u64,
    required_families: Box<[WorthUiQueryMeasurementFactFamily]>,
    projection_fact_receipt: Option<UiProjectionFactReceipt>,
    posture: UiQueryMeasurementEligibilityPosture,
}

pub(crate) struct UiQueryMeasurementEligibilityInput {
    pub target: UiAdmissionTarget,
    pub graph_node_identity: UiGraphNodeIdentity,
    pub declaration_identity: Option<UiDeclarationIdentity>,
    pub touch_identity_digest: u64,
    pub selected_measurement_obligation_identity_digest: Option<u64>,
    pub selected_support_authority_generation: UiEvidenceAuthorityGeneration,
    pub boundary_support_authority_generation: UiEvidenceAuthorityGeneration,
    pub required_families: Box<[WorthUiQueryMeasurementFactFamily]>,
    pub projection_fact_receipt: Option<UiProjectionFactReceipt>,
    pub posture: UiQueryMeasurementEligibilityPosture,
}

impl UiQueryMeasurementEligibility {
    pub(crate) fn new(input: UiQueryMeasurementEligibilityInput) -> Self {
        let UiQueryMeasurementEligibilityInput {
            target,
            graph_node_identity,
            declaration_identity,
            touch_identity_digest,
            selected_measurement_obligation_identity_digest,
            selected_support_authority_generation,
            boundary_support_authority_generation,
            required_families,
            projection_fact_receipt,
            posture,
        } = input;
        let required_fact_family_set_digest =
            query_measurement_fact_family_set_digest(&required_families);
        Self {
            target,
            graph_node_identity,
            declaration_identity,
            touch_identity_digest,
            selected_measurement_obligation_identity_digest,
            selected_support_authority_generation,
            boundary_support_authority_generation,
            required_fact_family_set_digest,
            required_families,
            projection_fact_receipt,
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

    pub fn query_basis_digest_for_diagnostics(&self) -> Option<&str> {
        self.target
            .query_prerequisites()
            .map(|evidence| evidence.basis_digest_for_diagnostics())
    }

    pub fn query_resolution_mode(
        &self,
    ) -> Option<worth_ui_query_binding::WorthUiQueryResolutionMode> {
        self.target
            .query_prerequisites()
            .map(|evidence| evidence.resolution_mode())
    }

    pub fn query_projection_contract_identity(
        &self,
    ) -> Option<worth_ui_query_binding::WorthUiQueryProjectionContractIdentity> {
        self.target
            .query_prerequisites()
            .and_then(|evidence| evidence.projection_contract_identity())
    }

    pub fn required_fact_family_set_digest(&self) -> u64 {
        self.required_fact_family_set_digest
    }

    pub fn required_families(&self) -> &[WorthUiQueryMeasurementFactFamily] {
        &self.required_families
    }

    pub fn projection_fact_receipt(&self) -> Option<&UiProjectionFactReceipt> {
        self.projection_fact_receipt.as_ref()
    }

    pub fn posture(&self) -> &UiQueryMeasurementEligibilityPosture {
        &self.posture
    }
}
