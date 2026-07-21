use crate::capability::{MosaicResizePermission, MosaicSizingContractId};
use crate::declaration::stable_text_digest;
use crate::source::{WorthUiArtifactNode, WorthUiDurableStateEligibility};

pub(super) fn node_identity_seed(
    node: &WorthUiArtifactNode,
) -> &crate::source::WorthUiArtifactIdentitySeed {
    match node {
        WorthUiArtifactNode::Import(node) => node.identity_seed(),
        WorthUiArtifactNode::Component(node) => node.identity_seed(),
        WorthUiArtifactNode::Surface(node) => node.identity_seed(),
        WorthUiArtifactNode::Binding(node) => node.identity_seed(),
        WorthUiArtifactNode::Token(node) => node.identity_seed(),
    }
}

pub(super) fn node_durable_state_eligibility(
    node: &WorthUiArtifactNode,
) -> &WorthUiDurableStateEligibility {
    match node {
        WorthUiArtifactNode::Import(node) => node.durable_state_eligibility(),
        WorthUiArtifactNode::Component(node) => node.durable_state_eligibility(),
        WorthUiArtifactNode::Surface(node) => node.durable_state_eligibility(),
        WorthUiArtifactNode::Binding(node) => node.durable_state_eligibility(),
        WorthUiArtifactNode::Token(node) => node.durable_state_eligibility(),
    }
}

pub(super) fn durable_state_is_eligible(eligibility: &WorthUiDurableStateEligibility) -> bool {
    matches!(eligibility, WorthUiDurableStateEligibility::Eligible { .. })
}

pub(super) fn node_resize_contract_id(
    node: &WorthUiArtifactNode,
) -> Option<MosaicSizingContractId> {
    node_structure(node)
        .and_then(|structure| structure.unique_root_sizing_contract_id().ok())
        .flatten()
}

pub(super) fn node_resize_permission(node: &WorthUiArtifactNode) -> Option<MosaicResizePermission> {
    let structure = node_structure(node)?;
    let root_region = structure.root_regions().first()?;
    let (_, descriptor) = root_region.sizing_contract()?;
    descriptor.resize_permission().cloned()
}

pub(super) fn node_resize_shape_digest(node: &WorthUiArtifactNode) -> Option<u64> {
    let structure = node_structure(node)?;
    Some(structure_shape_digest(structure))
}

fn node_structure(
    node: &WorthUiArtifactNode,
) -> Option<&crate::source::WorthUiMosaicStructureFacts> {
    match node {
        WorthUiArtifactNode::Component(node) => Some(node.structure()),
        WorthUiArtifactNode::Surface(node) => Some(node.structure()),
        WorthUiArtifactNode::Binding(node) => Some(node.structure()),
        WorthUiArtifactNode::Import(_) | WorthUiArtifactNode::Token(_) => None,
    }
}

fn structure_shape_digest(structure: &crate::source::WorthUiMosaicStructureFacts) -> u64 {
    let mut digest = stable_text_digest("worth-ui.runtime.resize-shape");
    let mut region_count = 0_u64;
    let mut mount_count = 0_u64;
    let mut max_depth = 0_u64;
    for root_region in structure.root_regions() {
        fold_region_shape(
            root_region,
            1,
            &mut digest,
            &mut region_count,
            &mut mount_count,
            &mut max_depth,
        );
    }
    digest ^= (structure.root_regions().len() as u64).rotate_left(7);
    digest ^= region_count.rotate_left(13);
    digest ^= mount_count.rotate_left(19);
    digest ^ max_depth.rotate_left(23)
}

fn fold_region_shape(
    region: &crate::source::WorthUiMosaicRegionFacts,
    depth: u64,
    digest: &mut u64,
    region_count: &mut u64,
    mount_count: &mut u64,
    max_depth: &mut u64,
) {
    *digest ^= stable_text_digest(region.region().id().as_str()).rotate_left(3);
    *region_count += 1;
    *mount_count += region.mounts().len() as u64;
    *max_depth = (*max_depth).max(depth);
    if let Some((contract, descriptor)) = region.sizing_contract() {
        *digest ^= stable_text_digest(contract.id().as_str()).rotate_left(5);
        *digest ^= resize_permission_digest(descriptor.resize_permission()).rotate_left(7);
    }
    if let Some((slot, descriptor)) = region.state_slot() {
        *digest ^= stable_text_digest(slot.id().as_str()).rotate_left(11);
        *digest ^= state_slot_kind_digest(descriptor.kind()).rotate_left(13);
    }
    for mount in region.mounts() {
        *digest ^= stable_text_digest(mount.surface().id().as_str()).rotate_left(17);
        if let Some((slot, descriptor)) = mount.state_slot() {
            *digest ^= stable_text_digest(slot.id().as_str()).rotate_left(19);
            *digest ^= state_slot_kind_digest(descriptor.kind()).rotate_left(23);
        }
    }
    for child in region.child_regions() {
        fold_region_shape(
            child,
            depth + 1,
            digest,
            region_count,
            mount_count,
            max_depth,
        );
    }
}

fn resize_permission_digest(permission: Option<&MosaicResizePermission>) -> u64 {
    let basis = match permission {
        Some(MosaicResizePermission::FixedByRuntime) => "fixed_by_runtime",
        Some(MosaicResizePermission::UserResizable) => "user_resizable",
        Some(MosaicResizePermission::ContentDriven) => "content_driven",
        Some(MosaicResizePermission::MissingForDiagnostics) => "missing",
        None => "none",
    };
    stable_text_digest(basis)
}

fn state_slot_kind_digest(kind: &crate::capability::MosaicStateSlotKind) -> u64 {
    let basis = match kind {
        crate::capability::MosaicStateSlotKind::SplitterPosition => "splitter_position",
        crate::capability::MosaicStateSlotKind::ActiveStackItem => "active_stack_item",
        crate::capability::MosaicStateSlotKind::RegionVisibility => "region_visibility",
        crate::capability::MosaicStateSlotKind::CollapsedPosture => "collapsed_posture",
        crate::capability::MosaicStateSlotKind::PinnedPosture => "pinned_posture",
        crate::capability::MosaicStateSlotKind::ScrollPosition => "scroll_position",
        crate::capability::MosaicStateSlotKind::FocusedRegion => "focused_region",
        crate::capability::MosaicStateSlotKind::ActivePrimarySurface => "active_primary_surface",
        crate::capability::MosaicStateSlotKind::ActiveAuxiliarySurface => {
            "active_auxiliary_surface"
        }
        crate::capability::MosaicStateSlotKind::SelectionToken => "selection_token",
        crate::capability::MosaicStateSlotKind::DraftInputState => "draft_input_state",
    };
    stable_text_digest(basis)
}
