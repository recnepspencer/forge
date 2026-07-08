//! Graph-owned layout operator planning contract construction.

use crate::evidence::{
    UiAllocationNeighborhoodClass, UiLayoutOperatorContainmentKind, UiLayoutOperatorFamily,
    UiLayoutOperatorPlanningContract, UiLayoutOperatorSlotParticipationKind, UiMeasurementBasis,
};
use crate::graph::{
    UiGraphContainmentClaim, UiGraphNodeRecord, UiGraphNodeTopology,
};

use super::membership_rule::classify_allocation_neighborhood_membership_rule;

pub(crate) fn construct_allocation_neighborhood_operator_contract(
    basis: &UiMeasurementBasis,
    root_record: &UiGraphNodeRecord,
    topology: &UiGraphNodeTopology,
) -> UiLayoutOperatorPlanningContract {
    let policy = basis.declared_measurement_policy();
    let neighborhood_class =
        UiAllocationNeighborhoodClass::from_measurement_hint(basis.neighborhood_class_hint());
    let membership_rule =
        classify_allocation_neighborhood_membership_rule(basis, root_record.operator_kind());
    UiLayoutOperatorPlanningContract::new(
        root_record.operator_kind(),
        UiLayoutOperatorFamily::from_structural_role(root_record.structural_role()),
        containment_kind(topology.containment_claim()),
        topology
            .containment_claim()
            .mosaic_sizing_contract_id()
            .cloned(),
        slot_participation_kind(topology.slot_topology().is_some()),
        topology.ordering_guarantee(),
        root_record.repetition_posture(),
        neighborhood_class,
        membership_rule,
        policy.mode(),
        policy.constraint_modifier(),
        policy.basis_source(),
        policy.ownership_posture(),
        policy.evidence_requirements().to_vec(),
    )
}

fn containment_kind(containment_claim: &UiGraphContainmentClaim) -> UiLayoutOperatorContainmentKind {
    match containment_claim {
        UiGraphContainmentClaim::RootPage => UiLayoutOperatorContainmentKind::RootPage,
        UiGraphContainmentClaim::PageSet { .. } => UiLayoutOperatorContainmentKind::PageSet,
        UiGraphContainmentClaim::Region { .. } => UiLayoutOperatorContainmentKind::Region,
        UiGraphContainmentClaim::Mosaic { .. } => UiLayoutOperatorContainmentKind::Mosaic,
        UiGraphContainmentClaim::LocalComposition { .. } => {
            UiLayoutOperatorContainmentKind::LocalComposition
        }
        UiGraphContainmentClaim::Control { .. } => UiLayoutOperatorContainmentKind::Control,
        UiGraphContainmentClaim::DiagnosticSurface { .. } => {
            UiLayoutOperatorContainmentKind::DiagnosticSurface
        }
    }
}

fn slot_participation_kind(has_slot_topology: bool) -> UiLayoutOperatorSlotParticipationKind {
    if has_slot_topology {
        UiLayoutOperatorSlotParticipationKind::DeclaredParticipant
    } else {
        UiLayoutOperatorSlotParticipationKind::None
    }
}