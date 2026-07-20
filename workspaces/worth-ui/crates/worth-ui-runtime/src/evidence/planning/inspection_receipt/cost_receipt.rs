use crate::evidence::{
    MeasurementEvidenceInput, UiAllocationNeighborhoodClass, UiAllocationNeighborhoodIdentity,
    UiMeasurementBasis,
};
use crate::runtime::{
    WorthUiAllocationPlanning, WorthUiAllocationPlanningCounters,
    WorthUiAllocationPlanningDenialReason, WorthUiAllocationPlanningInspection,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationPlanningCostClass {
    Local,
    Container,
    Viewport,
    ScrollContainer,
    PortalAnchor,
    DurableResizeGroup,
    DeniedUnbounded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationPlanningDeniedBroadeningReason {
    MeasurementBasisDenied,
    ConstraintSetDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationPlanningCostReceipt {
    neighborhood_identity: UiAllocationNeighborhoodIdentity,
    nodes_considered: usize,
    nodes_admitted: usize,
    edges_emitted: usize,
    propagation_passes: usize,
    special_inputs_loaded: usize,
    query_fact_refs_consumed: usize,
    host_evidence_refs_consumed: usize,
    denied_broadening_reason: Option<UiAllocationPlanningDeniedBroadeningReason>,
    cost_class: UiAllocationPlanningCostClass,
    counters: WorthUiAllocationPlanningCounters,
}

impl UiAllocationPlanningCostReceipt {
    pub(crate) fn new(
        planning: &WorthUiAllocationPlanning,
        inspection: &WorthUiAllocationPlanningInspection,
    ) -> Self {
        let measurement_basis = planning.measurement_basis();
        let neighborhood = inspection.neighborhood();
        let nodes_considered = neighborhood.members().len();
        let nodes_admitted = if inspection.denial().is_some() {
            0
        } else {
            nodes_considered
        };
        let special_inputs_loaded = usize::from(inspection.viewport_planning_input().is_some())
            + usize::from(inspection.scroll_owner_planning_input().is_some())
            + usize::from(inspection.portal_anchor_planning_input().is_some());
        let cost_class = cost_class_for(inspection);
        Self {
            neighborhood_identity: neighborhood.identity().clone(),
            nodes_considered,
            nodes_admitted,
            edges_emitted: inspection.propagation_edges().len(),
            propagation_passes: inspection.solve_trace().pass_order().len(),
            special_inputs_loaded,
            query_fact_refs_consumed: query_fact_ref_count(measurement_basis),
            host_evidence_refs_consumed: host_evidence_ref_count(measurement_basis),
            denied_broadening_reason: inspection
                .denial()
                .map(|denial| denial.reason())
                .map(denied_broadening_reason_for),
            cost_class,
            counters: inspection.counters(),
        }
    }

    pub fn neighborhood_identity_digest(&self) -> u64 {
        self.neighborhood_identity.identity_digest()
    }

    pub fn nodes_considered(&self) -> usize {
        self.nodes_considered
    }

    pub fn nodes_admitted(&self) -> usize {
        self.nodes_admitted
    }

    pub fn edges_emitted(&self) -> usize {
        self.edges_emitted
    }

    pub fn propagation_passes(&self) -> usize {
        self.propagation_passes
    }

    pub fn special_inputs_loaded(&self) -> usize {
        self.special_inputs_loaded
    }

    pub fn query_fact_refs_consumed(&self) -> usize {
        self.query_fact_refs_consumed
    }

    pub fn host_evidence_refs_consumed(&self) -> usize {
        self.host_evidence_refs_consumed
    }

    pub fn denied_broadening_reason(&self) -> Option<UiAllocationPlanningDeniedBroadeningReason> {
        self.denied_broadening_reason
    }

    pub fn cost_class(&self) -> UiAllocationPlanningCostClass {
        self.cost_class
    }

    pub fn counters(&self) -> WorthUiAllocationPlanningCounters {
        self.counters
    }
}

fn cost_class_for(
    inspection: &WorthUiAllocationPlanningInspection,
) -> UiAllocationPlanningCostClass {
    if inspection.denial().is_some() {
        return UiAllocationPlanningCostClass::DeniedUnbounded;
    }
    if inspection.constraint_summary().is_some_and(|summary| {
        summary.resize_permission_posture()
            == crate::evidence::UiConstraintResizePermissionPosture::DurableAuthorityLane
    }) {
        return UiAllocationPlanningCostClass::DurableResizeGroup;
    }
    match inspection.neighborhood().neighborhood_class() {
        UiAllocationNeighborhoodClass::LocalIntrinsicContent => {
            UiAllocationPlanningCostClass::Local
        }
        UiAllocationNeighborhoodClass::ContainerPeerGroup => {
            UiAllocationPlanningCostClass::Container
        }
        UiAllocationNeighborhoodClass::Viewport => UiAllocationPlanningCostClass::Viewport,
        UiAllocationNeighborhoodClass::ScrollContainer => {
            UiAllocationPlanningCostClass::ScrollContainer
        }
        UiAllocationNeighborhoodClass::PortalAnchor => UiAllocationPlanningCostClass::PortalAnchor,
    }
}

fn denied_broadening_reason_for(
    denial_reason: WorthUiAllocationPlanningDenialReason,
) -> UiAllocationPlanningDeniedBroadeningReason {
    match denial_reason {
        WorthUiAllocationPlanningDenialReason::MeasurementBasisDenied => {
            UiAllocationPlanningDeniedBroadeningReason::MeasurementBasisDenied
        }
        WorthUiAllocationPlanningDenialReason::ConstraintSetDenied => {
            UiAllocationPlanningDeniedBroadeningReason::ConstraintSetDenied
        }
    }
}

fn query_fact_ref_count(measurement_basis: &UiMeasurementBasis) -> usize {
    measurement_basis
        .evidence_inputs()
        .iter()
        .filter(|input| match input {
            MeasurementEvidenceInput::QueryProjectionFact(_) => true,
            MeasurementEvidenceInput::ChildIntrinsicMeasurement(evidence) => {
                evidence.query_projection_fact().is_some()
            }
            MeasurementEvidenceInput::HostMeasurementResult(_)
            | MeasurementEvidenceInput::HostCapabilityReport(_)
            | MeasurementEvidenceInput::SiblingResizeSupport(_) => false,
        })
        .count()
}

fn host_evidence_ref_count(measurement_basis: &UiMeasurementBasis) -> usize {
    measurement_basis
        .evidence_inputs()
        .iter()
        .filter(|input| match input {
            MeasurementEvidenceInput::HostMeasurementResult(_)
            | MeasurementEvidenceInput::HostCapabilityReport(_)
            | MeasurementEvidenceInput::SiblingResizeSupport(_) => true,
            MeasurementEvidenceInput::ChildIntrinsicMeasurement(evidence) => {
                evidence.host_measurement_result().is_some()
            }
            MeasurementEvidenceInput::QueryProjectionFact(_) => false,
        })
        .count()
}
