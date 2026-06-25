use super::digest::target_binding_digest;
use super::graph_execution::target_graph_execution_with_authority;
use super::{
    WorthUiMountedInteractionTargetBinding, WorthUiPrimitiveProofTargetBinding,
    WorthUiUserIntentOperationFamily, WorthUiUserIntentTargetBinding,
    WorthUiUserIntentTargetPosture,
};
use crate::runtime::WorthUiRuntimeGraphAuthority;

impl WorthUiPrimitiveProofTargetBinding {
    pub fn for_mounted_interaction(
        &self,
        graph_authority: &WorthUiRuntimeGraphAuthority,
    ) -> WorthUiMountedInteractionTargetBinding {
        self.rebind_family(
            graph_authority,
            WorthUiUserIntentOperationFamily::MountedInteraction,
        )
    }

    pub fn for_event_dispatch(
        &self,
        graph_authority: &WorthUiRuntimeGraphAuthority,
    ) -> super::WorthUiEventDispatchTargetBinding {
        self.rebind_family(
            graph_authority,
            WorthUiUserIntentOperationFamily::EventDispatch,
        )
    }

    fn rebind_family<Next>(
        &self,
        graph_authority: &WorthUiRuntimeGraphAuthority,
        family: WorthUiUserIntentOperationFamily,
    ) -> WorthUiUserIntentTargetBinding<Next> {
        let graph_execution = target_graph_execution_with_authority(
            graph_authority,
            self.slot_name(),
            self.surface_id(),
            self.component_id(),
            family,
            WorthUiUserIntentTargetPosture::Bound,
        );
        let binding_digest = target_binding_digest(
            self.slot_name(),
            self.surface_id(),
            self.component_id(),
            family,
            &graph_execution,
        );
        WorthUiUserIntentTargetBinding::new_for_bound_target(
            self.slot_name().to_owned(),
            self.surface_id().clone(),
            self.component_id().clone(),
            family,
            graph_execution,
            self.counters(),
            binding_digest,
        )
    }
}
