use crate::capability::{MosaicResizePermission, MosaicSizingContractId};
use crate::declaration::stable_text_digest;
use crate::source::{WorthUiArtifact, WorthUiArtifactNode};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactDurableResizeDefinition {
    identity_basis: String,
    authored_provenance_digest: u64,
    resize_contract_id: MosaicSizingContractId,
    resize_permission: MosaicResizePermission,
    resize_shape_digest: u64,
}

pub(crate) struct WorthUiArtifactDurableResizeDefinitionScan {
    definitions: Vec<WorthUiArtifactDurableResizeDefinition>,
    node_visits: usize,
}

pub(crate) fn durable_resize_definitions(
    artifact: &WorthUiArtifact,
) -> WorthUiArtifactDurableResizeDefinitionScan {
    let mut definitions = Vec::new();
    let mut node_visits = 0;
    for module in artifact
        .module_ids()
        .iter()
        .filter_map(|module_id| artifact.module(module_id))
    {
        for node in module.nodes() {
            node_visits += 1;
            if let Some(definition) = durable_resize_definition(node) {
                definitions.push(definition);
            }
        }
    }
    definitions.sort_by(|left, right| left.identity_basis.cmp(&right.identity_basis));
    WorthUiArtifactDurableResizeDefinitionScan {
        definitions,
        node_visits,
    }
}

pub(crate) fn node_resize_contract_id(
    node: &WorthUiArtifactNode,
) -> Option<MosaicSizingContractId> {
    node_structure(node)?
        .unique_root_sizing_contract_id()
        .ok()
        .flatten()
}

pub(crate) fn node_resize_permission(node: &WorthUiArtifactNode) -> Option<MosaicResizePermission> {
    let root_region = node_structure(node)?.root_regions().first()?;
    let (_, descriptor) = root_region.sizing_contract()?;
    descriptor.resize_permission().cloned()
}

pub(crate) fn node_resize_shape_digest(node: &WorthUiArtifactNode) -> Option<u64> {
    Some(structure_shape_digest(node_structure(node)?))
}

pub(crate) fn node_has_restorable_splitter_state(node: &WorthUiArtifactNode) -> bool {
    node_structure(node).is_some_and(|structure| {
        structure
            .root_regions()
            .iter()
            .any(region_has_restorable_splitter_state)
    })
}

impl WorthUiArtifactDurableResizeDefinition {
    pub(crate) fn identity_basis(&self) -> &str {
        &self.identity_basis
    }

    pub(crate) fn authored_provenance_digest(&self) -> u64 {
        self.authored_provenance_digest
    }

    pub(crate) fn resize_contract_id(&self) -> &MosaicSizingContractId {
        &self.resize_contract_id
    }

    pub(crate) fn resize_permission(&self) -> &MosaicResizePermission {
        &self.resize_permission
    }

    pub(crate) fn resize_shape_digest(&self) -> u64 {
        self.resize_shape_digest
    }
}

impl WorthUiArtifactDurableResizeDefinitionScan {
    pub(crate) fn definitions(&self) -> &[WorthUiArtifactDurableResizeDefinition] {
        &self.definitions
    }

    pub(crate) fn node_visits(&self) -> usize {
        self.node_visits
    }
}

fn durable_resize_definition(
    node: &WorthUiArtifactNode,
) -> Option<WorthUiArtifactDurableResizeDefinition> {
    let WorthUiArtifactNode::Surface(surface) = node else {
        return None;
    };
    if !surface.identity_seed().is_stable() || !node_has_restorable_splitter_state(node) {
        return None;
    }
    let resize_contract_id = node_resize_contract_id(node)?;
    let resize_permission = node_resize_permission(node)?;
    if resize_permission != MosaicResizePermission::UserResizable {
        return None;
    }
    Some(WorthUiArtifactDurableResizeDefinition {
        identity_basis: surface.identity_seed().basis().to_owned(),
        authored_provenance_digest: surface.authored_provenance_digest(),
        resize_contract_id,
        resize_permission,
        resize_shape_digest: node_resize_shape_digest(node)?,
    })
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

fn region_has_restorable_splitter_state(region: &crate::source::WorthUiMosaicRegionFacts) -> bool {
    slot_is_restorable_splitter(region.state_slot().map(|(_, descriptor)| descriptor))
        || region.mounts().iter().any(|mount| {
            slot_is_restorable_splitter(mount.state_slot().map(|(_, descriptor)| descriptor))
        })
        || region
            .child_regions()
            .iter()
            .any(region_has_restorable_splitter_state)
}

fn slot_is_restorable_splitter(
    slot: Option<&crate::capability::MosaicStateSlotDescriptor>,
) -> bool {
    slot.is_some_and(|descriptor| {
        descriptor.kind() == &crate::capability::MosaicStateSlotKind::splitter_position()
            && matches!(
                descriptor.persistence_policy(),
                Some(
                    crate::capability::MosaicStatePersistencePolicy::RestoreAcrossHotReload
                        | crate::capability::MosaicStatePersistencePolicy::PersistAcrossRuntimeRestart
                )
            )
    })
}

fn resize_permission_digest(permission: Option<&MosaicResizePermission>) -> u64 {
    stable_text_digest(match permission {
        Some(MosaicResizePermission::FixedByRuntime) => "fixed_by_runtime",
        Some(MosaicResizePermission::UserResizable) => "user_resizable",
        Some(MosaicResizePermission::ContentDriven) => "content_driven",
        Some(MosaicResizePermission::MissingForDiagnostics) => "missing",
        None => "none",
    })
}

fn state_slot_kind_digest(kind: &crate::capability::MosaicStateSlotKind) -> u64 {
    stable_text_digest(match kind {
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
    })
}
