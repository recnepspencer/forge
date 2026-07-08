//! Graph-owned membership rule classification from measurement basis inputs.

use crate::declaration::UiDeclarationPlanningOperatorKind;
use crate::evidence::{
    UiAllocationNeighborhoodClass, UiAllocationNeighborhoodMembershipRule, UiMeasurementBasis,
    UiMeasurementDependencyLineageKind,
};

pub(crate) fn classify_allocation_neighborhood_membership_rule(
    basis: &UiMeasurementBasis,
    operator_kind: UiDeclarationPlanningOperatorKind,
) -> UiAllocationNeighborhoodMembershipRule {
    let neighborhood_class =
        UiAllocationNeighborhoodClass::from_measurement_hint(basis.neighborhood_class_hint());
    let dependency_entries = basis.dependency_map().entries();
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
        return classify_special_scope_membership_rule(neighborhood_class, operator_kind);
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

fn classify_special_scope_membership_rule(
    neighborhood_class: UiAllocationNeighborhoodClass,
    operator_kind: UiDeclarationPlanningOperatorKind,
) -> UiAllocationNeighborhoodMembershipRule {
    match neighborhood_class {
        UiAllocationNeighborhoodClass::Viewport | UiAllocationNeighborhoodClass::ScrollContainer
            if operator_supports_child_intrinsic_return(operator_kind) =>
        {
            UiAllocationNeighborhoodMembershipRule::ParentSlotPeerGroup
        }
        UiAllocationNeighborhoodClass::Viewport | UiAllocationNeighborhoodClass::ScrollContainer => {
            UiAllocationNeighborhoodMembershipRule::RootOnly
        }
        UiAllocationNeighborhoodClass::LocalIntrinsicContent
        | UiAllocationNeighborhoodClass::ContainerPeerGroup
        | UiAllocationNeighborhoodClass::PortalAnchor => {
            UiAllocationNeighborhoodMembershipRule::RootOnly
        }
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