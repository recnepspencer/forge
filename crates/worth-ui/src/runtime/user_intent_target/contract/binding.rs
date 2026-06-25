use std::marker::PhantomData;

use crate::capability::{ComponentId, SurfaceId};
use crate::runtime::WorthUiQueryGraphExecutionReceipt;

use super::counters::WorthUiUserIntentTargetCounters;
use super::operation_family::WorthUiUserIntentOperationFamily;
use super::target_family::{
    WorthUiAppearanceTarget, WorthUiContentAnatomyTarget, WorthUiEventDispatchTarget,
    WorthUiEvidenceTarget, WorthUiFlowLayoutTarget, WorthUiLiveViewTarget,
    WorthUiMountedInteractionTarget, WorthUiPrimitiveProofTarget,
};
use super::target_posture::WorthUiUserIntentTargetPosture;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiUserIntentTargetBinding<Family> {
    slot_name: String,
    surface_id: SurfaceId,
    component_id: ComponentId,
    operation_family: WorthUiUserIntentOperationFamily,
    posture: WorthUiUserIntentTargetPosture,
    graph_execution: WorthUiQueryGraphExecutionReceipt,
    counters: WorthUiUserIntentTargetCounters,
    binding_digest: u64,
    _family: PhantomData<Family>,
}

pub type WorthUiPrimitiveProofTargetBinding =
    WorthUiUserIntentTargetBinding<WorthUiPrimitiveProofTarget>;
pub type WorthUiLiveViewTargetBinding = WorthUiUserIntentTargetBinding<WorthUiLiveViewTarget>;
pub type WorthUiMountedInteractionTargetBinding =
    WorthUiUserIntentTargetBinding<WorthUiMountedInteractionTarget>;
pub type WorthUiEventDispatchTargetBinding =
    WorthUiUserIntentTargetBinding<WorthUiEventDispatchTarget>;
pub type WorthUiContentAnatomyTargetBinding =
    WorthUiUserIntentTargetBinding<WorthUiContentAnatomyTarget>;
pub type WorthUiAppearanceTargetBinding = WorthUiUserIntentTargetBinding<WorthUiAppearanceTarget>;
pub type WorthUiFlowLayoutTargetBinding = WorthUiUserIntentTargetBinding<WorthUiFlowLayoutTarget>;
pub type WorthUiEvidenceTargetBinding = WorthUiUserIntentTargetBinding<WorthUiEvidenceTarget>;

impl<Family> WorthUiUserIntentTargetBinding<Family> {
    pub(in crate::runtime::user_intent_target) fn new_for_bound_target(
        slot_name: impl Into<String>,
        surface_id: SurfaceId,
        component_id: ComponentId,
        operation_family: WorthUiUserIntentOperationFamily,
        graph_execution: WorthUiQueryGraphExecutionReceipt,
        counters: WorthUiUserIntentTargetCounters,
        binding_digest: u64,
    ) -> Self {
        Self {
            slot_name: slot_name.into(),
            surface_id,
            component_id,
            operation_family,
            posture: WorthUiUserIntentTargetPosture::Bound,
            graph_execution,
            counters,
            binding_digest,
            _family: PhantomData,
        }
    }

    pub fn slot_name(&self) -> &str {
        &self.slot_name
    }

    pub fn surface_id(&self) -> &SurfaceId {
        &self.surface_id
    }

    pub fn component_id(&self) -> &ComponentId {
        &self.component_id
    }

    pub fn operation_family(&self) -> WorthUiUserIntentOperationFamily {
        self.operation_family
    }

    pub fn posture(&self) -> WorthUiUserIntentTargetPosture {
        self.posture
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.graph_execution
    }

    pub fn counters(&self) -> WorthUiUserIntentTargetCounters {
        self.counters
    }

    pub fn binding_digest(&self) -> u64 {
        self.binding_digest
    }
}
