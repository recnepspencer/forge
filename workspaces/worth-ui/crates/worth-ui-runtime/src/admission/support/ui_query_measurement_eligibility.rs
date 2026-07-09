use crate::admission::{UiAdmissionTarget, UiAdmissionWorld};
use crate::declaration::UiDeclarationIdentity;
use crate::evidence::{
    query_measurement_fact_family_set_digest, UiEvidenceAuthorityGeneration,
    UiProjectionFactReceipt,
};
use crate::graph::UiGraphNodeIdentity;
use worth_query::facade::{BasisDigest, BasisResolutionMode};
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
        basis_digest: BasisDigest,
        resolution_mode: BasisResolutionMode,
        projection_contract_digest: Option<Box<str>>,
    },
    ProjectionConsumption {
        basis_digest: Box<str>,
        projection_contract_digest: Box<str>,
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
    query_basis_digest: Option<BasisDigest>,
    query_resolution_mode: Option<BasisResolutionMode>,
    query_projection_contract_digest: Option<Box<str>>,
    required_fact_family_set_digest: u64,
    required_families: Box<[WorthUiQueryMeasurementFactFamily]>,
    projection_fact_receipt: Option<UiProjectionFactReceipt>,
    posture: UiQueryMeasurementEligibilityPosture,
}

impl UiQueryMeasurementEligibility {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        target: UiAdmissionTarget,
        graph_node_identity: UiGraphNodeIdentity,
        declaration_identity: Option<UiDeclarationIdentity>,
        touch_identity_digest: u64,
        selected_measurement_obligation_identity_digest: Option<u64>,
        selected_support_authority_generation: UiEvidenceAuthorityGeneration,
        boundary_support_authority_generation: UiEvidenceAuthorityGeneration,
        query_basis_digest: Option<BasisDigest>,
        query_resolution_mode: Option<BasisResolutionMode>,
        query_projection_contract_digest: Option<Box<str>>,
        required_families: Box<[WorthUiQueryMeasurementFactFamily]>,
        projection_fact_receipt: Option<UiProjectionFactReceipt>,
        posture: UiQueryMeasurementEligibilityPosture,
    ) -> Self {
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
            query_basis_digest,
            query_resolution_mode,
            query_projection_contract_digest,
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

    pub fn query_basis_digest(&self) -> Option<&BasisDigest> {
        self.query_basis_digest.as_ref()
    }

    pub fn query_resolution_mode(&self) -> Option<&BasisResolutionMode> {
        self.query_resolution_mode.as_ref()
    }

    pub fn query_projection_contract_digest(&self) -> Option<&str> {
        self.query_projection_contract_digest.as_deref()
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
