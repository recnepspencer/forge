use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::facade::WorthUiHostSessionIdentity;
use crate::graph::{UiGraphAuthority, UiGraphNodeIdentity};
use worth_ui_host_contract::{
    UiHostSurfaceRegistrationRequest, UiMountIncarnation, UiMountedFrameIdentity,
    UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity, UiMountedProjectionAudience,
    UiSemanticSurfaceIdentity,
};

use super::{
    UiMountedFrameIdentityView, UiMountedGraphNodeHandle, UiMountedGraphWorldIdentity,
    UiMountedIdentityBasis, UiMountedIdentityDenial, UiSurfaceBindingIdentityView,
};

static NEXT_WORLD: AtomicU64 = AtomicU64::new(1);
const SEMANTIC_SURFACE_LIMIT: usize = 256;
const MOUNTED_CLOSURE_LIMIT: usize = 2_048;
const GRAPH_NODE_MOUNT_LIMIT: usize = 1_024;
const RETIRED_INSTANCE_LIMIT: usize = 256;

mod frame_lifecycle;
pub(crate) mod surface_lifecycle;

#[derive(Clone, Debug)]
struct MountedInstanceRecord {
    basis: UiMountedIdentityBasis,
}

#[derive(Clone, Copy, Debug)]
struct SurfaceBindingRecord {
    view: UiSurfaceBindingIdentityView,
    request: UiHostSurfaceRegistrationRequest,
}

#[derive(Clone)]
pub(crate) struct UiMountedIdentityFrameCandidate {
    frame: UiMountedFrameIdentity,
    receipts: BTreeMap<UiMountedInstanceIdentity, UiMountedFrameIdentityView>,
}

pub(crate) struct UiMountedIdentityState {
    world_identity: UiMountedGraphWorldIdentity,
    host_session_identity: WorthUiHostSessionIdentity,
    semantic_surfaces: BTreeMap<UiSemanticSurfaceIdentity, UiMountedProjectionAudience>,
    bindings: BTreeMap<UiSemanticSurfaceIdentity, SurfaceBindingRecord>,
    instances: BTreeMap<UiMountedInstanceIdentity, MountedInstanceRecord>,
    retired_instances: BTreeSet<UiMountedInstanceIdentity>,
    retirement_order: VecDeque<UiMountedInstanceIdentity>,
    by_graph: BTreeMap<UiGraphNodeIdentity, BTreeSet<UiMountedInstanceIdentity>>,
    visible_order: Vec<UiMountedInstanceIdentity>,
    current_frame: Option<UiMountedFrameIdentity>,
    current_receipts: BTreeMap<UiMountedInstanceIdentity, UiMountedFrameIdentityView>,
    current_projection: Option<super::UiMountedProjectionFrame>,
    current_manifest: Option<worth_ui_host_contract::UiMountedFrameManifest>,
    current_core: Option<worth_ui_host_contract::UiMountedFrameCanonicalCore>,
    current_publication: Option<super::UiMountedFramePublicationReceipt>,
    presented_frames: super::retention::UiMountedPresentedFrameRetention,
}

impl UiMountedIdentityState {
    pub(crate) fn world_identity(&self) -> UiMountedGraphWorldIdentity {
        self.world_identity
    }

    pub(crate) fn new(
        host_session_identity: WorthUiHostSessionIdentity,
    ) -> Result<Self, UiMountedIdentityDenial> {
        Ok(Self {
            world_identity: UiMountedGraphWorldIdentity::new(next(&NEXT_WORLD)?),
            host_session_identity,
            semantic_surfaces: BTreeMap::new(),
            bindings: BTreeMap::new(),
            instances: BTreeMap::new(),
            retired_instances: BTreeSet::new(),
            retirement_order: VecDeque::new(),
            by_graph: BTreeMap::new(),
            visible_order: Vec::new(),
            current_frame: None,
            current_receipts: BTreeMap::new(),
            current_projection: None,
            current_manifest: None,
            current_core: None,
            current_publication: None,
            presented_frames: Default::default(),
        })
    }

    pub(crate) fn create_semantic_surface(
        &mut self,
    ) -> Result<UiSemanticSurfaceIdentity, UiMountedIdentityDenial> {
        self.create_semantic_surface_for(UiMountedProjectionAudience::full())
    }

    pub(crate) fn create_semantic_surface_for(
        &mut self,
        audience: UiMountedProjectionAudience,
    ) -> Result<UiSemanticSurfaceIdentity, UiMountedIdentityDenial> {
        if self.semantic_surfaces.len() >= SEMANTIC_SURFACE_LIMIT {
            return Err(UiMountedIdentityDenial::SemanticSurfaceCapacityExceeded);
        }
        let identity = UiSemanticSurfaceIdentity::mint_unbound()
            .map_err(|_| UiMountedIdentityDenial::IdentityExhausted)?;
        self.semantic_surfaces.insert(identity, audience);
        Ok(identity)
    }

    pub(crate) fn graph_node_handle(
        &self,
        graph: UiGraphAuthority<'_>,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Result<UiMountedGraphNodeHandle, UiMountedIdentityDenial> {
        graph
            .lookup()
            .graph_node(graph_node_identity)
            .ok_or(UiMountedIdentityDenial::UnknownGraphNode)?;
        Ok(UiMountedGraphNodeHandle::new(
            self.world_identity,
            graph_node_identity,
        ))
    }

    pub(crate) fn mount(
        &mut self,
        graph: UiGraphAuthority<'_>,
        handle: UiMountedGraphNodeHandle,
        surface: UiSemanticSurfaceIdentity,
    ) -> Result<UiMountedInstanceIdentity, UiMountedIdentityDenial> {
        self.require_handle(handle)?;
        self.require_surface(surface)?;
        if self.instances.len() >= MOUNTED_CLOSURE_LIMIT {
            return Err(UiMountedIdentityDenial::MountedClosureCapacityExceeded);
        }
        if self
            .by_graph
            .get(&handle.graph_node_identity())
            .is_some_and(|instances| instances.len() >= GRAPH_NODE_MOUNT_LIMIT)
        {
            return Err(UiMountedIdentityDenial::GraphNodeMountCapacityExceeded);
        }
        let graph_node = graph
            .lookup()
            .graph_node(handle.graph_node_identity())
            .ok_or(UiMountedIdentityDenial::UnknownGraphNode)?;
        let incarnation = UiMountIncarnation::mint_unbound()
            .map_err(|_| UiMountedIdentityDenial::IdentityExhausted)?;
        let identity = UiMountedInstanceIdentity::mint_unbound()
            .map_err(|_| UiMountedIdentityDenial::IdentityExhausted)?;
        let basis = UiMountedIdentityBasis::new(
            handle.graph_node_identity(),
            graph_node.value().repeated_instance_basis().clone(),
            surface,
            incarnation,
        );
        self.instances
            .insert(identity, MountedInstanceRecord { basis });
        self.by_graph
            .entry(handle.graph_node_identity())
            .or_default()
            .insert(identity);
        self.visible_order.push(identity);
        Ok(identity)
    }

    pub(crate) fn unmount(
        &mut self,
        identity: UiMountedInstanceIdentity,
    ) -> Result<(), UiMountedIdentityDenial> {
        let record = self.instances.remove(&identity).ok_or_else(|| {
            if self.retired_instances.contains(&identity) {
                UiMountedIdentityDenial::RetiredMountedInstance
            } else {
                UiMountedIdentityDenial::UnknownMountedInstance
            }
        })?;
        if let Some(instances) = self.by_graph.get_mut(&record.basis.graph_node_identity()) {
            instances.remove(&identity);
        }
        self.visible_order
            .retain(|candidate| *candidate != identity);
        self.current_receipts.remove(&identity);
        self.remember_retirement(identity);
        Ok(())
    }

    pub(crate) fn prepare_graph_replacement(
        &self,
    ) -> Result<UiMountedGraphWorldIdentity, UiMountedIdentityDenial> {
        Ok(UiMountedGraphWorldIdentity::new(next(&NEXT_WORLD)?))
    }

    pub(crate) fn prepare_graph_replacement_successor(
        &self,
        graph: UiGraphAuthority<'_>,
    ) -> Result<Self, UiMountedIdentityDenial> {
        let next_world = self.prepare_graph_replacement()?;
        let instances = self
            .instances
            .iter()
            .filter(|(_, record)| {
                graph
                    .lookup()
                    .graph_node(record.basis.graph_node_identity())
                    .is_some_and(|candidate| {
                        candidate.value().repeated_instance_basis()
                            == record.basis.repeated_instance_basis()
                    })
            })
            .map(|(identity, record)| (*identity, record.clone()))
            .collect::<BTreeMap<_, _>>();
        let mut by_graph = BTreeMap::<_, BTreeSet<_>>::new();
        for (identity, record) in &instances {
            by_graph
                .entry(record.basis.graph_node_identity())
                .or_default()
                .insert(*identity);
        }
        let visible_order = self
            .visible_order
            .iter()
            .copied()
            .filter(|identity| instances.contains_key(identity))
            .collect();
        let mut successor = Self {
            world_identity: next_world,
            host_session_identity: self.host_session_identity,
            semantic_surfaces: self.semantic_surfaces.clone(),
            bindings: self.bindings.clone(),
            instances,
            retired_instances: self.retired_instances.clone(),
            retirement_order: self.retirement_order.clone(),
            by_graph,
            visible_order,
            current_frame: None,
            current_receipts: BTreeMap::new(),
            current_projection: None,
            current_manifest: None,
            current_core: None,
            current_publication: None,
            presented_frames: self.presented_frames.inherited_by_replacement(),
        };
        for identity in self
            .instances
            .keys()
            .filter(|identity| !successor.instances.contains_key(identity))
            .copied()
            .collect::<Vec<_>>()
        {
            successor.remember_retirement(identity);
        }
        Ok(successor)
    }

    pub(crate) fn reorder(
        &mut self,
        order: &[UiMountedInstanceIdentity],
    ) -> Result<(), UiMountedIdentityDenial> {
        let requested = order.iter().copied().collect::<BTreeSet<_>>();
        let current = self.visible_order.iter().copied().collect::<BTreeSet<_>>();
        if requested != current || requested.len() != order.len() {
            return Err(UiMountedIdentityDenial::ReorderMembershipMismatch);
        }
        self.visible_order.clear();
        self.visible_order.extend_from_slice(order);
        Ok(())
    }

    fn require_handle(
        &self,
        handle: UiMountedGraphNodeHandle,
    ) -> Result<(), UiMountedIdentityDenial> {
        if handle.world_identity() != self.world_identity {
            return Err(UiMountedIdentityDenial::ForeignGraphWorld);
        }
        Ok(())
    }

    fn require_surface(
        &self,
        surface: UiSemanticSurfaceIdentity,
    ) -> Result<(), UiMountedIdentityDenial> {
        self.semantic_surfaces
            .contains_key(&surface)
            .then_some(())
            .ok_or(UiMountedIdentityDenial::UnknownSemanticSurface)
    }

    fn remember_retirement(&mut self, identity: UiMountedInstanceIdentity) {
        if !self.retired_instances.insert(identity) {
            return;
        }
        self.retirement_order.push_back(identity);
        if self.retirement_order.len() > RETIRED_INSTANCE_LIMIT {
            let expired = self
                .retirement_order
                .pop_front()
                .expect("an over-limit retirement queue is non-empty");
            self.retired_instances.remove(&expired);
        }
    }
}

fn next(counter: &AtomicU64) -> Result<u64, UiMountedIdentityDenial> {
    counter
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
            current.checked_add(1)
        })
        .map_err(|_| UiMountedIdentityDenial::IdentityExhausted)
}

impl UiMountedIdentityFrameCandidate {
    pub(super) fn frame(&self) -> UiMountedFrameIdentity {
        self.frame
    }

    pub(super) fn presented_receipts(
        &self,
    ) -> impl Iterator<Item = (UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity)> + '_ {
        self.receipts.values().map(|receipt| {
            (
                receipt.mounted_instance_identity(),
                receipt.node_receipt_identity(),
            )
        })
    }
}
