use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::facade::WorthUiHostSessionIdentity;
use crate::graph::UiGraphNodeIdentity;
use worth_ui_host_contract::{
    UiHostSurfaceRegistrationRequest, UiMountedFrameIdentity, UiMountedInstanceIdentity,
    UiMountedProjectionAudience, UiSemanticSurfaceIdentity,
};

use super::{
    UiMountedGraphWorldIdentity, UiMountedIdentityBasis, UiMountedIdentityDenial,
    UiSurfaceBindingIdentityView,
};

static NEXT_WORLD: AtomicU64 = AtomicU64::new(1);
static NEXT_STATE_REVISION: AtomicU64 = AtomicU64::new(1);
const SEMANTIC_SURFACE_LIMIT: usize = 256;
const RETIRED_INSTANCE_LIMIT: usize = 256;

mod frame_lifecycle;
mod graph_replacement;
mod instance_lifecycle;
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
    receipt_basis: super::UiMountedNodeReceiptBasis,
}

pub(crate) struct UiAuthorityAdmittedMountedFrame {
    frame: super::UiPreparedMountedFrame,
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
    mounted_instance_membership:
        crate::runtime::persistent_index::UiPersistentOrdSet<UiMountedInstanceIdentity>,
    current_frame: Option<UiMountedFrameIdentity>,
    current_receipt_basis: Option<super::UiMountedNodeReceiptBasis>,
    current_projection: Option<super::UiMountedProjectionFrame>,
    current_manifest: Option<worth_ui_host_contract::UiMountedFrameManifest>,
    current_core: Option<worth_ui_host_contract::UiMountedFrameCanonicalCore>,
    current_publication: Option<super::UiMountedFramePublicationReceipt>,
    current_reuse_contract: Option<super::UiMountedFrameReuseContract>,
    presented_frames: super::retention::UiMountedPresentedFrameRetention,
    pending_projection_changes: super::UiMountedProjectionChanges,
    semantic_revision: u64,
    binding_revision: u64,
}

impl UiMountedIdentityState {
    pub(crate) fn world_identity(&self) -> UiMountedGraphWorldIdentity {
        self.world_identity
    }

    pub(crate) fn new(
        host_session_identity: WorthUiHostSessionIdentity,
    ) -> Result<Self, UiMountedIdentityDenial> {
        let semantic_revision = next(&NEXT_STATE_REVISION)?;
        let binding_revision = next(&NEXT_STATE_REVISION)?;
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
            mounted_instance_membership: Default::default(),
            current_frame: None,
            current_receipt_basis: None,
            current_projection: None,
            current_manifest: None,
            current_core: None,
            current_publication: None,
            current_reuse_contract: None,
            presented_frames: Default::default(),
            pending_projection_changes: Default::default(),
            semantic_revision,
            binding_revision,
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
        let semantic_revision = next(&NEXT_STATE_REVISION)?;
        self.semantic_surfaces.insert(identity, audience);
        self.semantic_revision = semantic_revision;
        Ok(identity)
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

    pub(crate) fn seal_reuse_contract(
        &self,
        basis: super::UiMountedFrameReuseExternalBasis,
    ) -> super::UiMountedFrameReuseContract {
        super::UiMountedFrameReuseContract::seal(
            basis,
            self.world_identity.diagnostic_value(),
            self.semantic_revision,
            self.binding_revision,
        )
    }

    pub(crate) fn projection_change_snapshot(&self) -> super::UiMountedProjectionChangeSnapshot {
        self.pending_projection_changes
            .snapshot(self.semantic_revision, self.binding_revision)
    }

    fn commit_projection_changes(
        &mut self,
        snapshot: &super::UiMountedProjectionChangeSnapshot,
    ) -> bool {
        if !snapshot.matches(self.semantic_revision, self.binding_revision) {
            return false;
        }
        self.pending_projection_changes = Default::default();
        true
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
        self.receipt_basis.frame()
    }

    pub(super) fn receipt_basis(&self) -> &super::UiMountedNodeReceiptBasis {
        &self.receipt_basis
    }
}

impl UiAuthorityAdmittedMountedFrame {
    fn new(frame: super::UiPreparedMountedFrame) -> Self {
        Self { frame }
    }

    pub(in crate::mounting) fn into_frame(self) -> super::UiPreparedMountedFrame {
        self.frame
    }
}
