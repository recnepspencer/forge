use crate::runtime::{
    WorthUiNodeLifecycleTransition, WorthUiPlanNodeInput, WorthUiPlanNodeInputFamily,
    WorthUiQueryRebindRequiredSurface, WorthUiQuerySupportStatus,
};

pub(super) fn plan_input_fingerprint(node_inputs: &[WorthUiPlanNodeInput]) -> u64 {
    let mut digest = 0x6861_6e64_6c65_0001u64;
    for (index, node_input) in node_inputs.iter().enumerate() {
        digest ^= (index as u64).rotate_left(3);
        digest = fold_plan_node_input(digest, node_input);
    }
    digest
}

fn fold_plan_node_input(mut digest: u64, node_input: &WorthUiPlanNodeInput) -> u64 {
    digest ^= family_tag(node_input.family()).rotate_left(5);
    digest = fold_text(digest, node_input.identity_basis());
    digest ^= option_transition_tag(node_input.transition()).rotate_left(11);
    digest = fold_query_identity(digest, node_input);
    digest = fold_query_posture(digest, node_input);
    digest = fold_query_required_surfaces(digest, node_input);
    digest ^= option_egui_boundary_tag(node_input.egui_boundary_input()).rotate_left(23);
    digest = fold_topology_input(digest, node_input.topology_input());
    if let Some(receipt) = node_input.query_preservation_receipt() {
        digest = fold_text(digest ^ 0x7072_6573_6572_7665, receipt);
    }
    digest.rotate_left(17)
}

fn fold_query_identity(mut digest: u64, node_input: &WorthUiPlanNodeInput) -> u64 {
    let Some(identity) = node_input.query_binding_identity() else {
        return digest ^ 0x7175_6572_795f_0000;
    };
    digest = fold_text(digest ^ 0x7175_6572_795f_0001, identity.view_binding_id());
    digest = fold_text(digest, identity.query_capability_digest());
    digest = fold_text(digest, identity.query_composition_profile_digest());
    fold_text(digest, identity.result_shape_digest())
}

fn fold_query_posture(mut digest: u64, node_input: &WorthUiPlanNodeInput) -> u64 {
    let Some(posture) = node_input.query_binding_posture() else {
        return digest ^ 0x7175_6572_795f_0002;
    };
    digest ^= query_support_status_tag(posture.query_support_status()).rotate_left(13);
    digest = fold_text(digest, posture.support_admission_digest());
    digest = fold_text(digest, posture.basis_capability_digest());
    digest = fold_text(digest, posture.live_compatibility_digest());
    digest = fold_text(digest, posture.async_result_state_digest());
    digest = fold_text(digest, posture.recovery_digest());
    digest = fold_text(digest, posture.inspection_digest());
    digest = fold_text(digest, posture.projection_consumption_digest());
    fold_text(digest, posture.denial_presentation_digest())
}

fn fold_query_required_surfaces(mut digest: u64, node_input: &WorthUiPlanNodeInput) -> u64 {
    for surface in node_input.query_required_surfaces() {
        digest ^= query_required_surface_tag(*surface).rotate_left(7);
        digest = digest.rotate_left(19);
    }
    digest
}

fn family_tag(family: WorthUiPlanNodeInputFamily) -> u64 {
    match family {
        WorthUiPlanNodeInputFamily::ComponentInvocation => 1,
        WorthUiPlanNodeInputFamily::ChildRange => 2,
        WorthUiPlanNodeInputFamily::Command => 3,
        WorthUiPlanNodeInputFamily::TokenStyle => 4,
        WorthUiPlanNodeInputFamily::LayoutRegion => 5,
        WorthUiPlanNodeInputFamily::QueryViewBinding => 6,
        WorthUiPlanNodeInputFamily::Accessibility => 7,
        WorthUiPlanNodeInputFamily::DiagnosticsRef => 8,
        WorthUiPlanNodeInputFamily::LanePartitionRef => 9,
        WorthUiPlanNodeInputFamily::EguiBoundaryRef => 10,
        WorthUiPlanNodeInputFamily::RenderResourceRef => 11,
    }
}

fn option_transition_tag(transition: Option<WorthUiNodeLifecycleTransition>) -> u64 {
    match transition {
        Some(WorthUiNodeLifecycleTransition::Preserve) => 1,
        Some(WorthUiNodeLifecycleTransition::Replace) => 2,
        Some(WorthUiNodeLifecycleTransition::Drop) => 3,
        Some(WorthUiNodeLifecycleTransition::Create) => 4,
        Some(WorthUiNodeLifecycleTransition::Move) => 5,
        Some(WorthUiNodeLifecycleTransition::Rebind) => 6,
        Some(WorthUiNodeLifecycleTransition::LaneChange) => 7,
        None => 0,
    }
}

fn query_support_status_tag(status: WorthUiQuerySupportStatus) -> u64 {
    match status {
        WorthUiQuerySupportStatus::Supported => 1,
        WorthUiQuerySupportStatus::Deferred => 2,
        WorthUiQuerySupportStatus::Unsupported => 3,
    }
}

fn query_required_surface_tag(surface: WorthUiQueryRebindRequiredSurface) -> u64 {
    match surface {
        WorthUiQueryRebindRequiredSurface::LiveViewsAndLivePromotion => 1,
        WorthUiQueryRebindRequiredSurface::SubscriptionSelectionAndDiagnostics => 2,
        WorthUiQueryRebindRequiredSurface::BasisCapabilityLifecycle => 3,
        WorthUiQueryRebindRequiredSurface::AsyncResourcesAndResultState => 4,
        WorthUiQueryRebindRequiredSurface::Recovery => 5,
        WorthUiQueryRebindRequiredSurface::Inspection => 6,
        WorthUiQueryRebindRequiredSurface::ProjectionConsumption => 7,
        WorthUiQueryRebindRequiredSurface::ContinuationPipeline => 8,
    }
}

fn option_egui_boundary_tag(boundary: Option<crate::runtime::WorthUiEguiBoundaryInput>) -> u64 {
    match boundary {
        Some(crate::runtime::WorthUiEguiBoundaryInput::Component) => 1,
        Some(crate::runtime::WorthUiEguiBoundaryInput::Surface) => 2,
        Some(crate::runtime::WorthUiEguiBoundaryInput::QueryBinding) => 3,
        Some(crate::runtime::WorthUiEguiBoundaryInput::Token) => 4,
        Some(crate::runtime::WorthUiEguiBoundaryInput::Diagnostics) => 5,
        None => 0,
    }
}

fn fold_topology_input(
    mut digest: u64,
    topology: crate::runtime::WorthUiPlanNodeTopologyInput,
) -> u64 {
    digest ^= u64::from(topology.structure_declared()).rotate_left(29);
    digest ^= (topology.root_region_count() as u64).rotate_left(2);
    digest ^= (topology.region_count() as u64).rotate_left(7);
    digest ^= (topology.mount_count() as u64).rotate_left(13);
    digest ^ (topology.max_region_depth() as u64).rotate_left(19)
}

fn fold_text(mut digest: u64, text: &str) -> u64 {
    for byte in text.as_bytes() {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x100_0000_01b3);
    }
    digest
}
