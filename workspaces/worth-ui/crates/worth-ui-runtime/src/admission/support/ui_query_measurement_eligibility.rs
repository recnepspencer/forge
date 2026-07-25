use crate::admission::{UiAdmissionTarget, UiAdmissionWorld};
use crate::declaration::UiDeclarationIdentity;
use crate::evidence::{
    query_measurement_fact_family_set_digest, UiEvidenceAuthorityGeneration,
    UiSettledQueryFactReceipt,
};
use crate::graph::UiGraphNodeIdentity;
use worth_ui_query_binding::WorthUiQueryMeasurementFactFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiQueryMeasurementUnsupportedQueryReason {
    MissingSettledQueryFact,
    WrongWorldProjection,
    RebindRequired,
    AmbiguousSources,
    ProjectionConsumptionUnavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiQueryMeasurementSourceIdentity {
    view_binding_id: crate::capability::ViewBindingId,
    binding_reference: worth_ui_query_binding::WorthUiAdmittedQueryBindingReference,
    settlement_reference: worth_ui_query_binding::WorthUiAdmittedQuerySettlementReference,
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
    StaleSettlement {
        world: UiAdmissionWorld,
        expected_view_binding_id: crate::capability::ViewBindingId,
        expected_binding_reference: worth_ui_query_binding::WorthUiAdmittedQueryBindingReference,
        observed: Box<UiQueryMeasurementSourceIdentity>,
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
    projection_fact_receipt: Option<UiSettledQueryFactReceipt>,
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
    pub projection_fact_receipt: Option<UiSettledQueryFactReceipt>,
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

    pub fn required_fact_family_set_digest(&self) -> u64 {
        self.required_fact_family_set_digest
    }

    pub fn required_families(&self) -> &[WorthUiQueryMeasurementFactFamily] {
        &self.required_families
    }

    pub fn projection_fact_receipt(&self) -> Option<&UiSettledQueryFactReceipt> {
        self.projection_fact_receipt.as_ref()
    }

    pub fn posture(&self) -> &UiQueryMeasurementEligibilityPosture {
        &self.posture
    }
}

impl UiQueryMeasurementSourceIdentity {
    pub(crate) fn from_settled_fact(
        view_binding_id: crate::capability::ViewBindingId,
        fact: &worth_ui_query_binding::WorthUiSettledSnapshotFact,
    ) -> Self {
        Self {
            view_binding_id,
            binding_reference: fact.binding_reference().clone(),
            settlement_reference: fact.settlement_reference().clone(),
        }
    }

    pub fn view_binding_id(&self) -> &crate::capability::ViewBindingId {
        &self.view_binding_id
    }

    pub fn binding_reference(
        &self,
    ) -> &worth_ui_query_binding::WorthUiAdmittedQueryBindingReference {
        &self.binding_reference
    }

    pub fn settlement_reference(
        &self,
    ) -> &worth_ui_query_binding::WorthUiAdmittedQuerySettlementReference {
        &self.settlement_reference
    }
}
