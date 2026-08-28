#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiScrollRuntimeCertificationSnapshot {
    owners: usize,
    ownership_instances: usize,
    revision: u64,
    admitted_requests: u64,
    rejected_requests: u64,
    owners_visited: u64,
    owners_changed: u64,
    ownership_resolutions: u64,
    ownership_graph_nodes_visited: u64,
    ownership_plan_nodes_visited: u64,
    owner_geometry: Box<[UiScrollOwnerGeometryCertificationRow]>,
    ownership_incarnations: Box<[u64]>,
    ownership_mounted_instances: Box<[u64]>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiScrollOwnerGeometryCertificationRow {
    graph_node_digest: Option<u64>,
    plan_region_index: Option<u32>,
    inline_offset_subpixels: i64,
    block_offset_subpixels: i64,
    max_inline_subpixels: i64,
    max_block_subpixels: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiSelectionRuntimeCertificationSnapshot {
    owners: usize,
    available_catalog_owners: usize,
    selected_keys: usize,
    revision: u64,
    requests: u64,
    keys_visited: u64,
    catalog_keys_reconciled: u64,
    selected_application_item_keys: Box<[core::num::NonZeroU64]>,
}

pub trait WorthUiServiceStateCertificationExt {
    fn inspect_scroll_runtime_for_certification(&self) -> UiScrollRuntimeCertificationSnapshot;
    fn inspect_selection_runtime_for_certification(
        &self,
    ) -> UiSelectionRuntimeCertificationSnapshot;
}

impl WorthUiServiceStateCertificationExt for crate::facade::WorthUiActiveApplicationSession {
    fn inspect_scroll_runtime_for_certification(&self) -> UiScrollRuntimeCertificationSnapshot {
        crate::facade::WorthUiActiveApplicationSession::inspect_scroll_runtime_for_certification(
            self,
        )
    }

    fn inspect_selection_runtime_for_certification(
        &self,
    ) -> UiSelectionRuntimeCertificationSnapshot {
        crate::facade::WorthUiActiveApplicationSession::inspect_selection_runtime_for_certification(
            self,
        )
    }
}

impl UiScrollRuntimeCertificationSnapshot {
    pub(crate) fn new(
        owners: usize,
        ownership_instances: usize,
        revision: u64,
        admitted_requests: u64,
        rejected_requests: u64,
        owners_visited: u64,
        owners_changed: u64,
        ownership_resolutions: u64,
        ownership_graph_nodes_visited: u64,
        ownership_plan_nodes_visited: u64,
        owner_geometry: Box<[UiScrollOwnerGeometryCertificationRow]>,
        ownership_incarnations: Box<[u64]>,
        ownership_mounted_instances: Box<[u64]>,
    ) -> Self {
        Self {
            owners,
            ownership_instances,
            revision,
            admitted_requests,
            rejected_requests,
            owners_visited,
            owners_changed,
            ownership_resolutions,
            ownership_graph_nodes_visited,
            ownership_plan_nodes_visited,
            owner_geometry,
            ownership_incarnations,
            ownership_mounted_instances,
        }
    }

    pub const fn owners(&self) -> usize {
        self.owners
    }
    pub const fn ownership_instances(&self) -> usize {
        self.ownership_instances
    }
    pub const fn revision(&self) -> u64 {
        self.revision
    }
    pub const fn admitted_requests(&self) -> u64 {
        self.admitted_requests
    }
    pub const fn rejected_requests(&self) -> u64 {
        self.rejected_requests
    }
    pub const fn owners_visited(&self) -> u64 {
        self.owners_visited
    }
    pub const fn owners_changed(&self) -> u64 {
        self.owners_changed
    }
    pub const fn ownership_resolutions(&self) -> u64 {
        self.ownership_resolutions
    }
    pub const fn ownership_graph_nodes_visited(&self) -> u64 {
        self.ownership_graph_nodes_visited
    }
    pub const fn ownership_plan_nodes_visited(&self) -> u64 {
        self.ownership_plan_nodes_visited
    }
    pub fn owner_geometry(&self) -> &[UiScrollOwnerGeometryCertificationRow] {
        &self.owner_geometry
    }
    pub fn ownership_incarnations(&self) -> &[u64] {
        &self.ownership_incarnations
    }
    pub fn ownership_mounted_instances(&self) -> &[u64] {
        &self.ownership_mounted_instances
    }
}

impl UiScrollOwnerGeometryCertificationRow {
    pub(crate) fn new(
        owner: crate::runtime::scroll::UiScrollOwnerIdentity,
        offset: crate::runtime::scroll::UiScrollOffset,
        bounds: crate::runtime::scroll::UiScrollBounds,
    ) -> Self {
        let (graph_node_digest, plan_region_index) = match owner {
            crate::runtime::scroll::UiScrollOwnerIdentity::Region {
                region,
                plan_region_index,
                ..
            } => (Some(region.digest()), Some(plan_region_index)),
            crate::runtime::scroll::UiScrollOwnerIdentity::Surface(_)
            | crate::runtime::scroll::UiScrollOwnerIdentity::Viewport(_) => (None, None),
        };
        Self {
            graph_node_digest,
            plan_region_index,
            inline_offset_subpixels: offset.inline_subpixels(),
            block_offset_subpixels: offset.block_subpixels(),
            max_inline_subpixels: bounds.max_inline_subpixels(),
            max_block_subpixels: bounds.max_block_subpixels(),
        }
    }

    pub const fn graph_node_digest(self) -> Option<u64> {
        self.graph_node_digest
    }
    pub const fn plan_region_index(self) -> Option<u32> {
        self.plan_region_index
    }
    pub const fn inline_offset_subpixels(self) -> i64 {
        self.inline_offset_subpixels
    }
    pub const fn block_offset_subpixels(self) -> i64 {
        self.block_offset_subpixels
    }
    pub const fn max_inline_subpixels(self) -> i64 {
        self.max_inline_subpixels
    }
    pub const fn max_block_subpixels(self) -> i64 {
        self.max_block_subpixels
    }
}

impl UiSelectionRuntimeCertificationSnapshot {
    pub(crate) const fn new(
        owners: usize,
        available_catalog_owners: usize,
        selected_keys: usize,
        revision: u64,
        requests: u64,
        keys_visited: u64,
        catalog_keys_reconciled: u64,
        selected_application_item_keys: Box<[core::num::NonZeroU64]>,
    ) -> Self {
        Self {
            owners,
            available_catalog_owners,
            selected_keys,
            revision,
            requests,
            keys_visited,
            catalog_keys_reconciled,
            selected_application_item_keys,
        }
    }

    pub const fn owners(&self) -> usize {
        self.owners
    }
    pub const fn available_catalog_owners(&self) -> usize {
        self.available_catalog_owners
    }
    pub const fn selected_keys(&self) -> usize {
        self.selected_keys
    }
    pub const fn revision(&self) -> u64 {
        self.revision
    }
    pub const fn requests(&self) -> u64 {
        self.requests
    }
    pub const fn keys_visited(&self) -> u64 {
        self.keys_visited
    }
    pub const fn catalog_keys_reconciled(&self) -> u64 {
        self.catalog_keys_reconciled
    }

    pub fn selected_application_item_keys(&self) -> &[core::num::NonZeroU64] {
        &self.selected_application_item_keys
    }
}
