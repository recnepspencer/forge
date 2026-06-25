use super::super::hit_plan::WorthUiPrimitiveEventDispatchPlan;
use super::super::region_receipt::WorthUiPrimitiveEventHitTestPoint;
use crate::runtime::{
    target_denial_graph_execution, WorthUiEventDispatchTargetBinding, WorthUiRuntimeHost,
    WorthUiUserIntentOperationFamily, WorthUiUserIntentTargetDenial,
    WorthUiUserIntentTargetPosture,
};

impl WorthUiRuntimeHost {
    pub fn bind_primitive_event_dispatch_target(
        &self,
        plan: &WorthUiPrimitiveEventDispatchPlan,
        point: WorthUiPrimitiveEventHitTestPoint,
    ) -> Result<WorthUiEventDispatchTargetBinding, WorthUiUserIntentTargetDenial> {
        let Some(region) = plan.primary_region_at(point) else {
            return Err(WorthUiUserIntentTargetDenial::MissingSurface {
                slot_name: "host-pointer:no-hit".to_owned(),
                surface_id: "worth.ui.event.no-hit".to_owned(),
                operation_family: WorthUiUserIntentOperationFamily::EventDispatch,
                graph_execution: target_denial_graph_execution(
                    self,
                    "host-pointer:no-hit",
                    None,
                    None,
                    WorthUiUserIntentOperationFamily::EventDispatch,
                    WorthUiUserIntentTargetPosture::Unmounted,
                ),
            });
        };
        let surface_id = crate::capability::SurfaceId::new(region.surface_id()).map_err(|_| {
            WorthUiUserIntentTargetDenial::InvalidSurfaceId {
                slot_name: format!("host-pointer:{}", region.surface_id()),
                surface_id: region.surface_id().to_owned(),
                operation_family: WorthUiUserIntentOperationFamily::EventDispatch,
                graph_execution: target_denial_graph_execution(
                    self,
                    &format!("host-pointer:{}", region.surface_id()),
                    None,
                    None,
                    WorthUiUserIntentOperationFamily::EventDispatch,
                    WorthUiUserIntentTargetPosture::Denied,
                ),
            }
        })?;
        Ok(self
            .bind_authored_primitive_proof_target(&surface_id)?
            .for_event_dispatch(self.graph_authority()))
    }
}
