use crate::declaration::UiDeclarationPlanningOperatorKind;
use crate::evidence::{
    UiAllocationNeighborhood, UiAllocationNeighborhoodClass,
    UiAllocationNeighborhoodMembershipRule, UiLayoutOperatorContainmentKind,
    UiLayoutOperatorFamily, UiLayoutOperatorPlanningContract,
    UiLayoutOperatorSlotParticipationKind, UiMeasurementBasis, UiMeasurementDependencyLineageKind,
};
use crate::graph::{
    allocation_neighborhood::authority::{
        admit_allocation_neighborhood_for_basis, admit_allocation_neighborhood_for_selected,
    },
    UiAllocationNeighborhoodDenial, UiGraphContainmentClaim, UiGraphNodeRecord,
    UiGraphNodeTopology, UiGraphSnapshot,
};
use crate::obligations::selection::UiSelectedObligationSet;

impl UiMeasurementBasis {
    pub(crate) fn allocation_neighborhood_membership_rule(
        &self,
        operator_kind: UiDeclarationPlanningOperatorKind,
    ) -> UiAllocationNeighborhoodMembershipRule {
        let neighborhood_class =
            UiAllocationNeighborhoodClass::from_measurement_hint(self.neighborhood_class_hint());
        let dependency_entries = self.dependency_map().entries();
        if dependency_entries.is_empty() {
            return UiAllocationNeighborhoodMembershipRule::default_for_class(neighborhood_class);
        }

        let contains_special_scope_lineage = dependency_entries.iter().any(|entry| {
            matches!(
                entry.lineage().kind(),
                UiMeasurementDependencyLineageKind::HostViewportExtent
                    | UiMeasurementDependencyLineageKind::HostPortalAnchorRect
                    | UiMeasurementDependencyLineageKind::HostScrollContainerViewport
            )
        });
        if contains_special_scope_lineage {
            return match neighborhood_class {
                UiAllocationNeighborhoodClass::Viewport
                | UiAllocationNeighborhoodClass::ScrollContainer
                    if operator_supports_child_intrinsic_return(operator_kind) =>
                {
                    UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup
                }
                UiAllocationNeighborhoodClass::Viewport
                | UiAllocationNeighborhoodClass::ScrollContainer => {
                    UiAllocationNeighborhoodMembershipRule::RootOnly
                }
                UiAllocationNeighborhoodClass::LocalIntrinsicContent
                | UiAllocationNeighborhoodClass::ContainerPeerGroup
                | UiAllocationNeighborhoodClass::PortalAnchor => {
                    UiAllocationNeighborhoodMembershipRule::RootOnly
                }
            };
        }

        if matches!(
            neighborhood_class,
            UiAllocationNeighborhoodClass::LocalIntrinsicContent
        ) && operator_supports_child_intrinsic_return(operator_kind)
        {
            return UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup;
        }

        if matches!(
            neighborhood_class,
            UiAllocationNeighborhoodClass::ContainerPeerGroup
        ) {
            return UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup;
        }

        UiAllocationNeighborhoodMembershipRule::RootOnly
    }

    pub(crate) fn allocation_neighborhood_operator_contract(
        &self,
        root_record: &UiGraphNodeRecord,
        topology: &UiGraphNodeTopology,
    ) -> UiLayoutOperatorPlanningContract {
        let policy = self.declared_measurement_policy();
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
            UiAllocationNeighborhoodClass::from_measurement_hint(self.neighborhood_class_hint()),
            self.allocation_neighborhood_membership_rule(root_record.operator_kind()),
            policy.mode(),
            policy.constraint_modifier(),
            policy.basis_source(),
            policy.ownership_posture(),
            policy.evidence_requirements().to_vec(),
        )
    }

    pub(crate) fn admit_allocation_neighborhood(
        &self,
        snapshot: &UiGraphSnapshot,
        selected: &UiSelectedObligationSet,
    ) -> Result<UiAllocationNeighborhood, UiAllocationNeighborhoodDenial> {
        admit_allocation_neighborhood_for_selected(snapshot, selected, self)
    }

    pub(crate) fn admit_allocation_neighborhood_from_graph(
        &self,
        snapshot: &UiGraphSnapshot,
    ) -> Result<UiAllocationNeighborhood, UiAllocationNeighborhoodDenial> {
        admit_allocation_neighborhood_for_basis(snapshot, self)
    }
}

fn containment_kind(
    containment_claim: &UiGraphContainmentClaim,
) -> UiLayoutOperatorContainmentKind {
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

#[allow(dead_code)]
fn slot_participation_kind(has_slot_topology: bool) -> UiLayoutOperatorSlotParticipationKind {
    if has_slot_topology {
        UiLayoutOperatorSlotParticipationKind::DeclaredParticipant
    } else {
        UiLayoutOperatorSlotParticipationKind::None
    }
}

fn operator_supports_child_intrinsic_return(
    operator_kind: UiDeclarationPlanningOperatorKind,
) -> bool {
    matches!(
        operator_kind,
        UiDeclarationPlanningOperatorKind::PageSet
            | UiDeclarationPlanningOperatorKind::Region
            | UiDeclarationPlanningOperatorKind::LocalComposition
            | UiDeclarationPlanningOperatorKind::Control
            | UiDeclarationPlanningOperatorKind::Stack
            | UiDeclarationPlanningOperatorKind::Row
            | UiDeclarationPlanningOperatorKind::Grid
            | UiDeclarationPlanningOperatorKind::Split
            | UiDeclarationPlanningOperatorKind::Mosaic
            | UiDeclarationPlanningOperatorKind::Overlay
            | UiDeclarationPlanningOperatorKind::Scroll
    )
}
