use crate::capability::{MosaicRegionRole, SurfacePlacementClass};

use super::super::{
    MosaicPlacementAction, MosaicPlacementPolicyDescriptor, MosaicPlacementSource,
    MosaicPlacementTarget,
};

pub(super) fn is_illegal_mosaic_placement_policy(
    descriptor: &MosaicPlacementPolicyDescriptor,
) -> bool {
    let Some(source) = descriptor.source() else {
        return false;
    };
    let Some(target) = descriptor.target() else {
        return false;
    };
    if source.is_missing() || target.is_missing() {
        return false;
    }
    if descriptor.action().is_imperative_mutation()
        || source.is_imperative_mutation()
        || target.is_imperative_mutation()
        || is_cyclic_mosaic_containment_policy(descriptor.source(), descriptor.target())
    {
        return false;
    }
    if source.is_unsupported_surface_class()
        || source.is_product_domain_region_role()
        || target.is_product_domain_region_role()
    {
        return true;
    }
    !is_legal_source_target(descriptor.action(), source, target)
}

pub(super) fn is_cyclic_mosaic_containment_policy(
    source: Option<&MosaicPlacementSource>,
    target: Option<&MosaicPlacementTarget>,
) -> bool {
    matches!(
        (source, target),
        (
            Some(MosaicPlacementSource::RegionRole(source_role)),
            Some(
                MosaicPlacementTarget::RegionRole(target_role)
                    | MosaicPlacementTarget::RegionStack(target_role),
            ),
        ) if source_role == target_role
    )
}

fn is_legal_source_target(
    action: &MosaicPlacementAction,
    source: &MosaicPlacementSource,
    target: &MosaicPlacementTarget,
) -> bool {
    match (source, target) {
        (
            MosaicPlacementSource::SurfaceClass(surface_class),
            MosaicPlacementTarget::RegionRole(region_role)
            | MosaicPlacementTarget::RegionStack(region_role),
        ) => {
            surface_class_matches_region(surface_class, region_role)
                && action_matches_target_region(action, region_role)
        }
        (
            MosaicPlacementSource::RegionRole(source_role),
            MosaicPlacementTarget::RegionRole(target_role)
            | MosaicPlacementTarget::RegionStack(target_role),
        ) => source_role != target_role && action_matches_target_region(action, target_role),
        _ => false,
    }
}

fn surface_class_matches_region(
    surface_class: &SurfacePlacementClass,
    region_role: &MosaicRegionRole,
) -> bool {
    matches!(
        (surface_class, region_role),
        (
            SurfacePlacementClass::PrimaryRegion,
            MosaicRegionRole::Primary
        ) | (
            SurfacePlacementClass::PrimaryRegion,
            MosaicRegionRole::Viewport
        ) | (
            SurfacePlacementClass::AuxiliaryRegion,
            MosaicRegionRole::Auxiliary
        ) | (
            SurfacePlacementClass::AuxiliaryRegion,
            MosaicRegionRole::Side
        ) | (
            SurfacePlacementClass::AuxiliaryRegion,
            MosaicRegionRole::Bottom
        ) | (
            SurfacePlacementClass::AuxiliaryRegion,
            MosaicRegionRole::Stack
        ) | (
            SurfacePlacementClass::TransientLayer,
            MosaicRegionRole::Floating
        ) | (SurfacePlacementClass::ModalLayer, MosaicRegionRole::Modal)
            | (
                SurfacePlacementClass::OverlayLayer,
                MosaicRegionRole::Overlay
            )
            | (
                SurfacePlacementClass::StatusRegion,
                MosaicRegionRole::Status
            )
    )
}

fn action_matches_target_region(
    action: &MosaicPlacementAction,
    target_role: &MosaicRegionRole,
) -> bool {
    match action {
        MosaicPlacementAction::Dock | MosaicPlacementAction::Pin => {
            matches!(
                target_role,
                MosaicRegionRole::Primary
                    | MosaicRegionRole::Auxiliary
                    | MosaicRegionRole::Side
                    | MosaicRegionRole::Bottom
                    | MosaicRegionRole::Viewport
            )
        }
        MosaicPlacementAction::Tab | MosaicPlacementAction::Split => {
            matches!(
                target_role,
                MosaicRegionRole::Stack | MosaicRegionRole::Split
            )
        }
        MosaicPlacementAction::Collapse => matches!(target_role, MosaicRegionRole::Auxiliary),
        MosaicPlacementAction::Overlay => matches!(target_role, MosaicRegionRole::Overlay),
        MosaicPlacementAction::Float => matches!(target_role, MosaicRegionRole::Floating),
        MosaicPlacementAction::Modal => matches!(target_role, MosaicRegionRole::Modal),
        MosaicPlacementAction::StatusProjection => matches!(target_role, MosaicRegionRole::Status),
        MosaicPlacementAction::ToolbarProjection => {
            matches!(target_role, MosaicRegionRole::Toolbar)
        }
        MosaicPlacementAction::ImperativeMutationForDiagnostics => false,
    }
}
