use crate::runtime::{
    WorthUiAdmittedRuntimeChangeEvidence, WorthUiDropdownAppearanceRequest,
    WorthUiDropdownProjectionPlan, WorthUiHeaderFramePlan, WorthUiHeaderFrameRebindDenial,
    WorthUiHeaderMenuGroup, WorthUiHeaderMenuPlanDenial, WorthUiHeaderMenuProjectionRequest,
    WorthUiProjectionRebindBatchReceipt, WorthUiProjectionRebindPlan,
};

use super::frame_rebind_support::map_rebind_denial;

pub(super) struct WorthUiHeaderDropdownRebindOutcome {
    pub(super) groups: Vec<WorthUiHeaderMenuGroup>,
    pub(super) dropdown_plans: Vec<WorthUiDropdownProjectionPlan>,
    pub(super) receipts: Vec<WorthUiProjectionRebindBatchReceipt>,
}

pub(super) fn rebind_header_dropdowns(
    runtime: &mut crate::runtime::WorthUiRuntimeHost,
    current_plan: &WorthUiHeaderFramePlan,
    menu_requests: Vec<WorthUiHeaderMenuProjectionRequest>,
    dropdown_appearance: WorthUiDropdownAppearanceRequest,
    evidence: &WorthUiAdmittedRuntimeChangeEvidence,
    capability_path: bool,
) -> Result<WorthUiHeaderDropdownRebindOutcome, WorthUiHeaderFrameRebindDenial> {
    let snapshot = runtime
        .active_state_for_read()
        .capability_snapshot()
        .clone();
    let mut groups = Vec::with_capacity(menu_requests.len());
    let mut dropdown_plans = Vec::with_capacity(menu_requests.len());
    let mut receipts = Vec::with_capacity(menu_requests.len());
    for request in menu_requests {
        let current_dropdown = current_plan
            .menu_plan()
            .dropdown_plans()
            .iter()
            .find(|candidate| {
                candidate.execute_frame().projection_id() == request.projection_id().as_str()
            })
            .expect("header menu requests stay aligned with current plan topology");
        let admitted_current = runtime
            .admit_projection_plan(current_dropdown.clone())
            .map_err(WorthUiHeaderFrameRebindDenial::ProjectionAdmission)?;
        let rebind = runtime
            .prepare_projection_rebind(evidence, admitted_current)
            .map_err(|denial| map_rebind_denial(denial, capability_path))?;
        match rebind {
            WorthUiProjectionRebindPlan::Preserve(plan) => {
                let (_, receipt) = plan.complete_preserved();
                runtime
                    .active_state_for_swap_mut()
                    .record_dropdown_selection_state(
                        request.projection_id(),
                        current_dropdown.execute_frame().selection_state(),
                    );
                receipts.push(receipt);
                groups.push(WorthUiHeaderMenuGroup::new(
                    request.title(),
                    current_dropdown.execute_frame().clone(),
                ));
                dropdown_plans.push(current_dropdown.clone());
            }
            WorthUiProjectionRebindPlan::Rebuild(plan) => {
                let rebound = WorthUiDropdownProjectionPlan::rebuild_from_snapshot(
                    &snapshot,
                    request.to_dropdown_request(dropdown_appearance.clone()),
                    runtime
                        .active_state_for_read()
                        .dropdown_selection_state(request.projection_id()),
                )
                .map_err(dropdown_plan_denial)?;
                let admitted_rebound = runtime
                    .admit_projection_plan(rebound.clone())
                    .map_err(WorthUiHeaderFrameRebindDenial::ProjectionAdmission)?;
                let (_, receipt) = plan.complete_rebuild(admitted_rebound);
                runtime
                    .active_state_for_swap_mut()
                    .record_dropdown_selection_state(
                        request.projection_id(),
                        rebound.execute_frame().selection_state(),
                    );
                receipts.push(receipt);
                groups.push(WorthUiHeaderMenuGroup::new(
                    request.title(),
                    rebound.execute_frame().clone(),
                ));
                dropdown_plans.push(rebound);
            }
        }
    }

    Ok(WorthUiHeaderDropdownRebindOutcome {
        groups,
        dropdown_plans,
        receipts,
    })
}

fn dropdown_plan_denial(
    denial: crate::runtime::WorthUiDropdownProjectionPlanDenial,
) -> WorthUiHeaderFrameRebindDenial {
    WorthUiHeaderFrameRebindDenial::FramePlan(crate::runtime::WorthUiHeaderFramePlanDenial::Menu(
        WorthUiHeaderMenuPlanDenial::Dropdown(denial),
    ))
}
