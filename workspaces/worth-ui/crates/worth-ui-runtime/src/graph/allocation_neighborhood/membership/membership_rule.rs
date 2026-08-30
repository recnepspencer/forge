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
    if is_independent_viewport_contract(basis.declared_measurement_policy().mode()) {
        return UiAllocationNeighborhoodMembershipRule::RootOnly;
    }
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

fn is_independent_viewport_contract(
    mode: Option<crate::declaration::UiDeclaredMeasurementMode>,
) -> bool {
    matches!(
        mode,
        Some(
            crate::declaration::UiDeclaredMeasurementMode::FillViewport
                | crate::declaration::UiDeclaredMeasurementMode::ViewportInset { .. }
                | crate::declaration::UiDeclaredMeasurementMode::ViewportRegion { .. }
                | crate::declaration::UiDeclaredMeasurementMode::FixedLogicalSize { .. }
        )
    )
}

fn classify_special_scope_membership_rule(
    neighborhood_class: UiAllocationNeighborhoodClass,
    operator_kind: UiDeclarationPlanningOperatorKind,
) -> UiAllocationNeighborhoodMembershipRule {
    match neighborhood_class {
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

#[cfg(test)]
mod tests {
    use super::is_independent_viewport_contract;
    use crate::declaration::UiDeclaredMeasurementMode;

    #[test]
    fn absolute_viewport_modes_own_independent_allocation_neighborhoods() {
        assert!(is_independent_viewport_contract(Some(
            UiDeclaredMeasurementMode::FillViewport
        )));
        assert!(is_independent_viewport_contract(Some(
            UiDeclaredMeasurementMode::ViewportInset {
                horizontal_logical_points: 48,
                vertical_logical_points: 24,
            }
        )));
        assert!(is_independent_viewport_contract(Some(
            UiDeclaredMeasurementMode::ViewportRegion {
                horizontal: crate::capability::ComponentViewportAxisPlacement::stretch_between(
                    24, 24,
                ),
                vertical: crate::capability::ComponentViewportAxisPlacement::fixed_from_start(
                    24, 56,
                )
                .unwrap(),
            }
        )));
        assert!(!is_independent_viewport_contract(Some(
            UiDeclaredMeasurementMode::HugHeight
        )));
        assert!(!is_independent_viewport_contract(None));
    }
}
