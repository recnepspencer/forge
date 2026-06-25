use crate::capability::{ComponentId, SurfaceId};

use super::super::{WorthUiPageHostPlan, WorthUiRuntimeHost};
use super::digest::target_binding_digest;
use super::graph_execution::target_graph_execution_with_authority;
use super::{
    WorthUiLiveViewTargetBinding, WorthUiMountedInteractionTargetBinding,
    WorthUiPrimitiveProofTargetBinding, WorthUiUserIntentOperationFamily,
    WorthUiUserIntentTargetBinding, WorthUiUserIntentTargetCounters, WorthUiUserIntentTargetDenial,
    WorthUiUserIntentTargetPosture,
};

impl WorthUiRuntimeHost {
    pub fn bind_visible_primitive_proof_target(
        &self,
        page_host_plan: &WorthUiPageHostPlan,
        slot_name: &str,
    ) -> Result<WorthUiPrimitiveProofTargetBinding, WorthUiUserIntentTargetDenial> {
        bind_visible_slot_target(
            self,
            page_host_plan,
            slot_name,
            WorthUiUserIntentOperationFamily::PrimitiveProof,
        )
    }

    pub fn bind_visible_live_view_target(
        &self,
        page_host_plan: &WorthUiPageHostPlan,
        slot_name: &str,
    ) -> Result<WorthUiLiveViewTargetBinding, WorthUiUserIntentTargetDenial> {
        bind_visible_slot_target(
            self,
            page_host_plan,
            slot_name,
            WorthUiUserIntentOperationFamily::LiveViewStateBinding,
        )
    }

    pub fn bind_authored_primitive_proof_target(
        &self,
        surface_id: &SurfaceId,
    ) -> Result<WorthUiPrimitiveProofTargetBinding, WorthUiUserIntentTargetDenial> {
        bind_authored_surface_target(
            self,
            surface_id,
            WorthUiUserIntentOperationFamily::PrimitiveProof,
        )
    }

    pub fn bind_authored_mounted_interaction_target(
        &self,
        surface_id: &SurfaceId,
    ) -> Result<WorthUiMountedInteractionTargetBinding, WorthUiUserIntentTargetDenial> {
        bind_authored_surface_target(
            self,
            surface_id,
            WorthUiUserIntentOperationFamily::MountedInteraction,
        )
    }

    pub fn bind_authored_live_view_target(
        &self,
        surface_id: &SurfaceId,
    ) -> Result<WorthUiLiveViewTargetBinding, WorthUiUserIntentTargetDenial> {
        bind_authored_surface_target(
            self,
            surface_id,
            WorthUiUserIntentOperationFamily::LiveViewStateBinding,
        )
    }
}

fn bind_visible_slot_target<Family>(
    runtime: &WorthUiRuntimeHost,
    page_host_plan: &WorthUiPageHostPlan,
    slot_name: &str,
    operation_family: WorthUiUserIntentOperationFamily,
) -> Result<WorthUiUserIntentTargetBinding<Family>, WorthUiUserIntentTargetDenial> {
    let Some(slot_mount) = page_host_plan.resolve_slot_mount(slot_name) else {
        return Err(WorthUiUserIntentTargetDenial::MissingSlot {
            page_name: page_host_plan.page_name().to_owned(),
            slot_name: slot_name.to_owned(),
            operation_family,
            graph_execution: target_denial_graph_execution(
                runtime,
                slot_name,
                None,
                None,
                operation_family,
                WorthUiUserIntentTargetPosture::Unmounted,
            ),
        });
    };
    let surface_id = parse_slot_surface_id(
        runtime,
        slot_name,
        slot_mount.surface_id(),
        operation_family,
    )?;
    let component_id =
        resolve_surface_component_id(runtime, slot_name, &surface_id, operation_family)?;
    Ok(bound_target(
        runtime,
        slot_name,
        surface_id,
        component_id,
        operation_family,
        WorthUiUserIntentTargetCounters::bound_with_page_slot_lookups(1),
    ))
}

fn bind_authored_surface_target<Family>(
    runtime: &WorthUiRuntimeHost,
    surface_id: &SurfaceId,
    operation_family: WorthUiUserIntentOperationFamily,
) -> Result<WorthUiUserIntentTargetBinding<Family>, WorthUiUserIntentTargetDenial> {
    let subject = authored_surface_subject(surface_id);
    let component_id =
        resolve_surface_component_id(runtime, &subject, surface_id, operation_family)?;
    Ok(bound_target(
        runtime,
        subject,
        surface_id.clone(),
        component_id,
        operation_family,
        WorthUiUserIntentTargetCounters::bound_with_page_slot_lookups(0),
    ))
}

fn parse_slot_surface_id(
    runtime: &WorthUiRuntimeHost,
    slot_name: &str,
    surface_id: &str,
    operation_family: WorthUiUserIntentOperationFamily,
) -> Result<SurfaceId, WorthUiUserIntentTargetDenial> {
    SurfaceId::new(surface_id).map_err(|_| WorthUiUserIntentTargetDenial::InvalidSurfaceId {
        slot_name: slot_name.to_owned(),
        surface_id: surface_id.to_owned(),
        operation_family,
        graph_execution: target_denial_graph_execution(
            runtime,
            slot_name,
            None,
            None,
            operation_family,
            WorthUiUserIntentTargetPosture::Denied,
        ),
    })
}

fn resolve_surface_component_id(
    runtime: &WorthUiRuntimeHost,
    subject: &str,
    surface_id: &SurfaceId,
    operation_family: WorthUiUserIntentOperationFamily,
) -> Result<ComponentId, WorthUiUserIntentTargetDenial> {
    let Some(surface) = runtime.inspect_active_surface_descriptor(surface_id) else {
        return Err(WorthUiUserIntentTargetDenial::MissingSurface {
            slot_name: subject.to_owned(),
            surface_id: surface_id.as_str().to_owned(),
            operation_family,
            graph_execution: target_denial_graph_execution(
                runtime,
                subject,
                Some(surface_id.clone()),
                None,
                operation_family,
                WorthUiUserIntentTargetPosture::Unmounted,
            ),
        });
    };
    let authored_component_id = runtime
        .inspect_active_authored_surface_component_id(surface_id)
        .unwrap_or_else(|| surface.component_id().as_str());
    ComponentId::new(authored_component_id).map_err(|_| {
        WorthUiUserIntentTargetDenial::InvalidComponentId {
            slot_name: subject.to_owned(),
            surface_id: surface_id.as_str().to_owned(),
            component_id: authored_component_id.to_owned(),
            operation_family,
            graph_execution: target_denial_graph_execution(
                runtime,
                subject,
                Some(surface_id.clone()),
                None,
                operation_family,
                WorthUiUserIntentTargetPosture::Denied,
            ),
        }
    })
}

fn bound_target<Family>(
    runtime: &WorthUiRuntimeHost,
    slot_name: impl Into<String>,
    surface_id: SurfaceId,
    component_id: ComponentId,
    operation_family: WorthUiUserIntentOperationFamily,
    counters: WorthUiUserIntentTargetCounters,
) -> WorthUiUserIntentTargetBinding<Family> {
    let slot_name = slot_name.into();
    let graph_execution = target_graph_execution_with_authority(
        runtime.graph_authority(),
        &slot_name,
        &surface_id,
        &component_id,
        operation_family,
        WorthUiUserIntentTargetPosture::Bound,
    );
    let binding_digest = target_binding_digest(
        &slot_name,
        &surface_id,
        &component_id,
        operation_family,
        &graph_execution,
    );
    WorthUiUserIntentTargetBinding::new_for_bound_target(
        slot_name,
        surface_id,
        component_id,
        operation_family,
        graph_execution,
        counters,
        binding_digest,
    )
}

fn authored_surface_subject(surface_id: &SurfaceId) -> String {
    format!("authored-surface:{}", surface_id.as_str())
}

pub(crate) fn target_denial_graph_execution(
    runtime: &WorthUiRuntimeHost,
    slot_name: &str,
    surface_id: Option<SurfaceId>,
    component_id: Option<ComponentId>,
    operation_family: WorthUiUserIntentOperationFamily,
    posture: WorthUiUserIntentTargetPosture,
) -> super::super::WorthUiQueryGraphExecutionReceipt {
    let surface_id = surface_id.unwrap_or_else(denial_surface_id);
    let component_id = component_id.unwrap_or_else(denial_component_id);
    target_graph_execution_with_authority(
        runtime.graph_authority(),
        slot_name,
        &surface_id,
        &component_id,
        operation_family,
        posture,
    )
}

fn denial_surface_id() -> SurfaceId {
    SurfaceId::new("worth.surface.target.denial")
        .expect("target denial surface identity is a valid Worth surface id")
}

fn denial_component_id() -> ComponentId {
    ComponentId::new("worth.component.target.denial")
        .expect("target denial component identity is a valid Worth component id")
}
