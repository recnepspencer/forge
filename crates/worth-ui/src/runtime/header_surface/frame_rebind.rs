use crate::capability::CapabilitySnapshot;
use crate::runtime::{
    WorthUiCapabilityReloadEvidence, WorthUiHeaderFramePlan, WorthUiHeaderFramePlanDenial,
    WorthUiProjectionPlanAdmissionDenial, WorthUiProjectionRebindBatchReceipt,
    WorthUiProjectionRebindPlan, WorthUiRuntimeChangeAdmissionDenial, WorthUiRuntimeHost,
    WorthUiValidationReloadEvidence,
};

use super::{
    dropdown_rebind::rebind_header_dropdowns,
    frame_plan::dropdown_appearance_request,
    frame_rebind_support::{header_status, map_rebind_denial, map_runtime_change_denial},
    WorthUiHeaderAppearancePlan, WorthUiHeaderAppearanceRequest, WorthUiHeaderMenuPlan,
    WorthUiHeaderMenuProjectionRequest, WorthUiHeaderThemePlan, WorthUiHeaderThemeTokenRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderFrameRebindRequest {
    menu_requests: Vec<WorthUiHeaderMenuProjectionRequest>,
    theme_request: WorthUiHeaderThemeTokenRequest,
    appearance_request: WorthUiHeaderAppearanceRequest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderFrameRebindReceipt {
    status: WorthUiHeaderFrameRebindStatus,
    previous_frame_digest: u64,
    rebound_frame_digest: u64,
    projection_batch: WorthUiProjectionRebindBatchReceipt,
    source_parse_count: usize,
    registry_lookup_count: usize,
    artifact_tree_scan_count: usize,
    projection_rebuild_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiHeaderFrameRebindStatus {
    PreservedEquivalentReload,
    PreservedDeniedReload,
    ReboundAfterActivation,
    EquivalentAfterActivation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiHeaderFrameRebindDenial {
    ReloadNotActivated,
    RuntimeEvidenceMismatch,
    CapabilityReloadNotActivated,
    CapabilitySnapshotMismatch {
        active_snapshot_digest: u64,
        provided_snapshot_digest: u64,
    },
    RuntimeChange(WorthUiRuntimeChangeAdmissionDenial),
    ProjectionAdmission(WorthUiProjectionPlanAdmissionDenial),
    FramePlan(WorthUiHeaderFramePlanDenial),
}

impl WorthUiHeaderFrameRebindRequest {
    pub fn new(
        menu_requests: Vec<WorthUiHeaderMenuProjectionRequest>,
        theme_request: WorthUiHeaderThemeTokenRequest,
        appearance_request: WorthUiHeaderAppearanceRequest,
    ) -> Self {
        Self {
            menu_requests,
            theme_request,
            appearance_request,
        }
    }

    fn clone_parts(
        &self,
    ) -> (
        Vec<WorthUiHeaderMenuProjectionRequest>,
        WorthUiHeaderThemeTokenRequest,
        WorthUiHeaderAppearanceRequest,
    ) {
        (
            self.menu_requests.clone(),
            self.theme_request.clone(),
            self.appearance_request.clone(),
        )
    }
}

impl WorthUiHeaderFrameRebindReceipt {
    pub(crate) fn new(
        status: WorthUiHeaderFrameRebindStatus,
        previous_frame_digest: u64,
        rebound_frame_digest: u64,
        projection_batch: WorthUiProjectionRebindBatchReceipt,
        projection_rebuild_count: usize,
    ) -> Self {
        Self {
            status,
            previous_frame_digest,
            rebound_frame_digest,
            projection_batch,
            source_parse_count: 0,
            registry_lookup_count: 0,
            artifact_tree_scan_count: 0,
            projection_rebuild_count,
        }
    }

    pub fn status(&self) -> WorthUiHeaderFrameRebindStatus {
        self.status
    }

    pub fn previous_frame_digest(&self) -> u64 {
        self.previous_frame_digest
    }

    pub fn rebound_frame_digest(&self) -> u64 {
        self.rebound_frame_digest
    }

    pub fn projection_rebind_batch(&self) -> &WorthUiProjectionRebindBatchReceipt {
        &self.projection_batch
    }

    pub fn source_parse_count(&self) -> usize {
        self.source_parse_count
    }

    pub fn registry_lookup_count(&self) -> usize {
        self.registry_lookup_count
    }

    pub fn artifact_tree_scan_count(&self) -> usize {
        self.artifact_tree_scan_count
    }

    pub fn projection_rebuild_count(&self) -> usize {
        self.projection_rebuild_count
    }
}

impl WorthUiRuntimeHost {
    pub fn rebind_header_frame_after_capability_reload(
        &mut self,
        current_plan: &WorthUiHeaderFramePlan,
        request: WorthUiHeaderFrameRebindRequest,
        evidence: &WorthUiCapabilityReloadEvidence,
    ) -> Result<
        (WorthUiHeaderFramePlan, WorthUiHeaderFrameRebindReceipt),
        WorthUiHeaderFrameRebindDenial,
    > {
        let admitted_change = self
            .admit_capability_runtime_change(evidence)
            .map_err(map_runtime_change_denial)?;
        self.rebind_header_frame_after_admitted_change(
            current_plan,
            request,
            &admitted_change,
            true,
        )
    }

    pub fn rebind_header_frame_after_reload(
        &mut self,
        snapshot: &CapabilitySnapshot,
        current_plan: &WorthUiHeaderFramePlan,
        request: WorthUiHeaderFrameRebindRequest,
        evidence: &WorthUiValidationReloadEvidence,
    ) -> Result<
        (WorthUiHeaderFramePlan, WorthUiHeaderFrameRebindReceipt),
        WorthUiHeaderFrameRebindDenial,
    > {
        self.verify_rebind_snapshot(snapshot)?;
        self.verify_validation_evidence_active_digests(evidence)?;
        let admitted_change = self
            .admit_validation_runtime_change(evidence)
            .map_err(map_runtime_change_denial)?;
        self.rebind_header_frame_after_admitted_change(
            current_plan,
            request,
            &admitted_change,
            false,
        )
    }

    pub fn rebind_header_frame_after_runtime_change(
        &mut self,
        current_plan: &WorthUiHeaderFramePlan,
        request: WorthUiHeaderFrameRebindRequest,
        evidence: &crate::runtime::WorthUiAdmittedRuntimeChangeEvidence,
    ) -> Result<
        (WorthUiHeaderFramePlan, WorthUiHeaderFrameRebindReceipt),
        WorthUiHeaderFrameRebindDenial,
    > {
        self.rebind_header_frame_after_admitted_change(current_plan, request, evidence, false)
    }

    fn rebind_header_frame_after_admitted_change(
        &mut self,
        current_plan: &WorthUiHeaderFramePlan,
        request: WorthUiHeaderFrameRebindRequest,
        evidence: &crate::runtime::WorthUiAdmittedRuntimeChangeEvidence,
        capability_path: bool,
    ) -> Result<
        (WorthUiHeaderFramePlan, WorthUiHeaderFrameRebindReceipt),
        WorthUiHeaderFrameRebindDenial,
    > {
        let previous_frame_digest = current_plan.frame_digest();
        let (menu_requests, theme_request, appearance_request) = request.clone_parts();
        let dropdown_appearance = dropdown_appearance_request(&appearance_request);
        let (theme_plan, mut receipts) =
            self.rebind_header_theme(current_plan, theme_request, evidence, capability_path)?;
        let (appearance_plan, appearance_receipt) = self.rebind_header_appearance(
            current_plan,
            appearance_request.clone(),
            evidence,
            capability_path,
        )?;
        receipts.push(appearance_receipt);

        let dropdowns = rebind_header_dropdowns(
            self,
            current_plan,
            menu_requests,
            dropdown_appearance,
            evidence,
            capability_path,
        )?;
        receipts.extend(dropdowns.receipts);
        let menu_plan =
            WorthUiHeaderMenuPlan::from_dropdown_plans(dropdowns.groups, dropdowns.dropdown_plans);
        let next_plan =
            WorthUiHeaderFramePlan::from_composed_plans(menu_plan, theme_plan, appearance_plan);
        let batch = WorthUiProjectionRebindBatchReceipt::aggregate(receipts)
            .expect("nested header projection rebinds share one runtime evidence digest");
        let rebound_frame_digest = next_plan.frame_digest();
        let status = header_status(
            evidence.posture(),
            previous_frame_digest,
            rebound_frame_digest,
        );
        let receipt = WorthUiHeaderFrameRebindReceipt::new(
            status,
            previous_frame_digest,
            rebound_frame_digest,
            batch.clone(),
            batch.counters().rebuild_attempt_count(),
        );
        Ok((next_plan, receipt))
    }

    fn rebind_header_theme(
        &mut self,
        current_plan: &WorthUiHeaderFramePlan,
        theme_request: WorthUiHeaderThemeTokenRequest,
        evidence: &crate::runtime::WorthUiAdmittedRuntimeChangeEvidence,
        capability_path: bool,
    ) -> Result<
        (
            WorthUiHeaderThemePlan,
            Vec<WorthUiProjectionRebindBatchReceipt>,
        ),
        WorthUiHeaderFrameRebindDenial,
    > {
        let admitted_current = self
            .admit_projection_plan(current_plan.theme_plan().clone())
            .map_err(WorthUiHeaderFrameRebindDenial::ProjectionAdmission)?;
        let rebind = self
            .prepare_projection_rebind(evidence, admitted_current)
            .map_err(|denial| map_rebind_denial(denial, capability_path))?;
        match rebind {
            WorthUiProjectionRebindPlan::Preserve(plan) => {
                let (_, receipt) = plan.complete_preserved();
                Ok((current_plan.theme_plan().clone(), vec![receipt]))
            }
            WorthUiProjectionRebindPlan::Rebuild(plan) => {
                let rebound = WorthUiHeaderThemePlan::from_snapshot(
                    self.active_state_for_read().capability_snapshot(),
                    theme_request,
                )
                .map_err(|denial| {
                    WorthUiHeaderFrameRebindDenial::FramePlan(WorthUiHeaderFramePlanDenial::Theme(
                        denial,
                    ))
                })?;
                let admitted_rebound = self
                    .admit_projection_plan(rebound.clone())
                    .map_err(WorthUiHeaderFrameRebindDenial::ProjectionAdmission)?;
                let (_, receipt) = plan.complete_rebuild(admitted_rebound);
                Ok((rebound, vec![receipt]))
            }
        }
    }

    fn rebind_header_appearance(
        &mut self,
        current_plan: &WorthUiHeaderFramePlan,
        appearance_request: WorthUiHeaderAppearanceRequest,
        evidence: &crate::runtime::WorthUiAdmittedRuntimeChangeEvidence,
        capability_path: bool,
    ) -> Result<
        (
            WorthUiHeaderAppearancePlan,
            WorthUiProjectionRebindBatchReceipt,
        ),
        WorthUiHeaderFrameRebindDenial,
    > {
        let admitted_current = self
            .admit_projection_plan(current_plan.appearance_plan().clone())
            .map_err(WorthUiHeaderFrameRebindDenial::ProjectionAdmission)?;
        let rebind = self
            .prepare_projection_rebind(evidence, admitted_current)
            .map_err(|denial| map_rebind_denial(denial, capability_path))?;
        match rebind {
            WorthUiProjectionRebindPlan::Preserve(plan) => {
                let (_, receipt) = plan.complete_preserved();
                Ok((current_plan.appearance_plan().clone(), receipt))
            }
            WorthUiProjectionRebindPlan::Rebuild(plan) => {
                let rebound = WorthUiHeaderAppearancePlan::from_snapshot(
                    self.active_state_for_read().capability_snapshot(),
                    appearance_request,
                )
                .map_err(|denial| {
                    WorthUiHeaderFrameRebindDenial::FramePlan(
                        WorthUiHeaderFramePlanDenial::Appearance(denial),
                    )
                })?;
                let admitted_rebound = self
                    .admit_projection_plan(rebound.clone())
                    .map_err(WorthUiHeaderFrameRebindDenial::ProjectionAdmission)?;
                let (_, receipt) = plan.complete_rebuild(admitted_rebound);
                Ok((rebound, receipt))
            }
        }
    }

    fn verify_rebind_snapshot(
        &self,
        snapshot: &CapabilitySnapshot,
    ) -> Result<(), WorthUiHeaderFrameRebindDenial> {
        let active_snapshot_digest = self.inspect_active().snapshot_digest();
        let provided_snapshot_digest = snapshot.digest().as_u64();
        if provided_snapshot_digest != active_snapshot_digest {
            return Err(WorthUiHeaderFrameRebindDenial::CapabilitySnapshotMismatch {
                active_snapshot_digest,
                provided_snapshot_digest,
            });
        }
        Ok(())
    }

    fn verify_validation_evidence_active_digests(
        &self,
        evidence: &WorthUiValidationReloadEvidence,
    ) -> Result<(), WorthUiHeaderFrameRebindDenial> {
        let active = self.inspect_active();
        if evidence.active_artifact_digest_after() != active.artifact_digest()
            || evidence.active_plan_digest_after() != active.active_plan_digest()
        {
            return Err(WorthUiHeaderFrameRebindDenial::RuntimeEvidenceMismatch);
        }
        Ok(())
    }
}
