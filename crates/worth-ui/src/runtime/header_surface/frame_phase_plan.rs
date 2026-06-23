use crate::runtime::{
    WorthUiAdmittedRuntimeChangeEvidence, WorthUiHeaderFramePlan, WorthUiHeaderFrameRebindDenial,
    WorthUiHeaderFrameRebindReceipt, WorthUiHeaderFrameRebindRequest,
    WorthUiProjectionPlanContract, WorthUiProjectionRebindBatchReceipt,
    WorthUiProjectionRebindCounters, WorthUiProjectionRebindPlan,
    WorthUiProjectionRebindRowReceipt, WorthUiProjectionRebindStatus, WorthUiRuntimeHost,
};

use super::frame_rebind_support::header_status;

impl WorthUiRuntimeHost {
    pub fn rebind_header_frame_from_phase_plan(
        &mut self,
        current_plan: &WorthUiHeaderFramePlan,
        request: WorthUiHeaderFrameRebindRequest,
        phase_plan: WorthUiProjectionRebindPlan<WorthUiHeaderFramePlan>,
    ) -> Result<
        (WorthUiHeaderFramePlan, WorthUiHeaderFrameRebindReceipt),
        WorthUiHeaderFrameRebindDenial,
    > {
        match phase_plan {
            WorthUiProjectionRebindPlan::Preserve(preserved) => {
                Ok(preserve_header_frame_after_runtime_change(
                    current_plan,
                    preserved.evidence(),
                    preserved.status(),
                ))
            }
            WorthUiProjectionRebindPlan::Rebuild(activated) => self
                .rebind_header_frame_after_runtime_change(
                    current_plan,
                    request,
                    activated.evidence(),
                ),
        }
    }
}

fn preserve_header_frame_after_runtime_change(
    current_plan: &WorthUiHeaderFramePlan,
    evidence: &WorthUiAdmittedRuntimeChangeEvidence,
    status: WorthUiProjectionRebindStatus,
) -> (WorthUiHeaderFramePlan, WorthUiHeaderFrameRebindReceipt) {
    let previous_frame_digest = current_plan.frame_digest();
    let receipts = preserved_header_projection_receipts(current_plan, evidence, status);
    let batch = WorthUiProjectionRebindBatchReceipt::aggregate(receipts)
        .expect("preserved header rows share one runtime evidence digest");
    let receipt = WorthUiHeaderFrameRebindReceipt::new(
        header_status(
            evidence.posture(),
            previous_frame_digest,
            previous_frame_digest,
        ),
        previous_frame_digest,
        previous_frame_digest,
        batch,
        0,
    );
    (current_plan.clone(), receipt)
}

fn preserved_header_projection_receipts(
    current_plan: &WorthUiHeaderFramePlan,
    evidence: &WorthUiAdmittedRuntimeChangeEvidence,
    status: WorthUiProjectionRebindStatus,
) -> Vec<WorthUiProjectionRebindBatchReceipt> {
    let runtime_instance = evidence.runtime_instance();
    let change_evidence_digest = evidence.digest();
    let counters = WorthUiProjectionRebindCounters::inspected_without_intersection(status);
    let mut receipts = Vec::new();
    let theme_digest = current_plan.theme_plan().projection_equivalence_digest();
    receipts.push(WorthUiProjectionRebindBatchReceipt::single_row(
        runtime_instance,
        change_evidence_digest,
        counters,
        WorthUiProjectionRebindRowReceipt::new_with_component_compatibility(
            current_plan.theme_plan().projection_identity(),
            current_plan.theme_plan().projection_family(),
            status,
            false,
            theme_digest,
            theme_digest,
            None,
        ),
    ));
    let appearance_digest = current_plan
        .appearance_plan()
        .projection_equivalence_digest();
    receipts.push(WorthUiProjectionRebindBatchReceipt::single_row(
        runtime_instance,
        change_evidence_digest,
        counters,
        WorthUiProjectionRebindRowReceipt::new_with_component_compatibility(
            current_plan.appearance_plan().projection_identity(),
            current_plan.appearance_plan().projection_family(),
            status,
            false,
            appearance_digest,
            appearance_digest,
            None,
        ),
    ));
    receipts.extend(
        current_plan
            .menu_plan()
            .dropdown_plans()
            .iter()
            .map(|plan| {
                let digest = plan.projection_equivalence_digest();
                WorthUiProjectionRebindBatchReceipt::single_row(
                    runtime_instance,
                    change_evidence_digest,
                    counters,
                    WorthUiProjectionRebindRowReceipt::new_with_component_compatibility(
                        plan.projection_identity(),
                        plan.projection_family(),
                        status,
                        false,
                        digest,
                        digest,
                        None,
                    ),
                )
            }),
    );
    receipts
}
