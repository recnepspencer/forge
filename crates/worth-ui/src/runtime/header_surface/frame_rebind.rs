use crate::capability::CapabilitySnapshot;
use crate::runtime::{
    WorthUiCapabilityReloadEvidence, WorthUiCapabilityReloadStatus, WorthUiRuntimeHost,
    WorthUiValidationReloadEvidence, WorthUiValidationReloadStatus,
};

use super::{
    WorthUiHeaderFramePlan, WorthUiHeaderFramePlanDenial, WorthUiHeaderMenuProjectionRequest,
    WorthUiHeaderThemeTokenRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderFrameRebindRequest {
    menu_requests: Vec<WorthUiHeaderMenuProjectionRequest>,
    theme_request: WorthUiHeaderThemeTokenRequest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderFrameRebindReceipt {
    status: WorthUiHeaderFrameRebindStatus,
    previous_frame_digest: u64,
    rebound_frame_digest: u64,
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
    FramePlan(WorthUiHeaderFramePlanDenial),
}

impl WorthUiHeaderFrameRebindRequest {
    pub fn new(
        menu_requests: Vec<WorthUiHeaderMenuProjectionRequest>,
        theme_request: WorthUiHeaderThemeTokenRequest,
    ) -> Self {
        Self {
            menu_requests,
            theme_request,
        }
    }

    fn into_parts(
        self,
    ) -> (
        Vec<WorthUiHeaderMenuProjectionRequest>,
        WorthUiHeaderThemeTokenRequest,
    ) {
        (self.menu_requests, self.theme_request)
    }
}

impl WorthUiHeaderFrameRebindReceipt {
    fn preserved(status: WorthUiHeaderFrameRebindStatus, frame_digest: u64) -> Self {
        Self {
            status,
            previous_frame_digest: frame_digest,
            rebound_frame_digest: frame_digest,
            source_parse_count: 0,
            registry_lookup_count: 0,
            artifact_tree_scan_count: 0,
            projection_rebuild_count: 0,
        }
    }

    fn rebound(
        status: WorthUiHeaderFrameRebindStatus,
        previous_frame_digest: u64,
        rebound_frame_digest: u64,
    ) -> Self {
        Self {
            status,
            previous_frame_digest,
            rebound_frame_digest,
            source_parse_count: 0,
            registry_lookup_count: 0,
            artifact_tree_scan_count: 0,
            projection_rebuild_count: 1,
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
        &self,
        current_plan: &WorthUiHeaderFramePlan,
        request: WorthUiHeaderFrameRebindRequest,
        evidence: &WorthUiCapabilityReloadEvidence,
    ) -> Result<
        (WorthUiHeaderFramePlan, WorthUiHeaderFrameRebindReceipt),
        WorthUiHeaderFrameRebindDenial,
    > {
        self.verify_capability_rebind_evidence(evidence)?;
        match evidence.status() {
            WorthUiCapabilityReloadStatus::EquivalentNoOp => Ok((
                current_plan.clone(),
                WorthUiHeaderFrameRebindReceipt::preserved(
                    WorthUiHeaderFrameRebindStatus::PreservedEquivalentReload,
                    current_plan.frame_digest(),
                ),
            )),
            WorthUiCapabilityReloadStatus::Denied(_) => Ok((
                current_plan.clone(),
                WorthUiHeaderFrameRebindReceipt::preserved(
                    WorthUiHeaderFrameRebindStatus::PreservedDeniedReload,
                    current_plan.frame_digest(),
                ),
            )),
            WorthUiCapabilityReloadStatus::ReadyForFrameBoundary => {
                Err(WorthUiHeaderFrameRebindDenial::CapabilityReloadNotActivated)
            }
            WorthUiCapabilityReloadStatus::Activated => {
                let (menu_requests, theme_request) = request.into_parts();
                let snapshot = self.active_state_for_read().capability_snapshot();
                let rebound =
                    WorthUiHeaderFramePlan::from_snapshot(snapshot, menu_requests, theme_request)
                        .map_err(WorthUiHeaderFrameRebindDenial::FramePlan)?;
                let status = if rebound.frame_digest() == current_plan.frame_digest() {
                    WorthUiHeaderFrameRebindStatus::EquivalentAfterActivation
                } else {
                    WorthUiHeaderFrameRebindStatus::ReboundAfterActivation
                };
                let receipt = WorthUiHeaderFrameRebindReceipt::rebound(
                    status,
                    current_plan.frame_digest(),
                    rebound.frame_digest(),
                );
                Ok((rebound, receipt))
            }
        }
    }

    pub fn rebind_header_frame_after_reload(
        &self,
        snapshot: &CapabilitySnapshot,
        current_plan: &WorthUiHeaderFramePlan,
        request: WorthUiHeaderFrameRebindRequest,
        evidence: &WorthUiValidationReloadEvidence,
    ) -> Result<
        (WorthUiHeaderFramePlan, WorthUiHeaderFrameRebindReceipt),
        WorthUiHeaderFrameRebindDenial,
    > {
        self.verify_rebind_evidence(evidence)?;
        self.verify_rebind_snapshot(snapshot)?;
        match evidence.status() {
            WorthUiValidationReloadStatus::EquivalentNoOp => Ok((
                current_plan.clone(),
                WorthUiHeaderFrameRebindReceipt::preserved(
                    WorthUiHeaderFrameRebindStatus::PreservedEquivalentReload,
                    current_plan.frame_digest(),
                ),
            )),
            WorthUiValidationReloadStatus::Denied(_) => Ok((
                current_plan.clone(),
                WorthUiHeaderFrameRebindReceipt::preserved(
                    WorthUiHeaderFrameRebindStatus::PreservedDeniedReload,
                    current_plan.frame_digest(),
                ),
            )),
            WorthUiValidationReloadStatus::ReadyForFrameBoundary => {
                Err(WorthUiHeaderFrameRebindDenial::ReloadNotActivated)
            }
            WorthUiValidationReloadStatus::Activated => {
                let (menu_requests, theme_request) = request.into_parts();
                let rebound =
                    WorthUiHeaderFramePlan::from_snapshot(snapshot, menu_requests, theme_request)
                        .map_err(WorthUiHeaderFrameRebindDenial::FramePlan)?;
                let status = if rebound.frame_digest() == current_plan.frame_digest() {
                    WorthUiHeaderFrameRebindStatus::EquivalentAfterActivation
                } else {
                    WorthUiHeaderFrameRebindStatus::ReboundAfterActivation
                };
                let receipt = WorthUiHeaderFrameRebindReceipt::rebound(
                    status,
                    current_plan.frame_digest(),
                    rebound.frame_digest(),
                );
                Ok((rebound, receipt))
            }
        }
    }

    fn verify_rebind_evidence(
        &self,
        evidence: &WorthUiValidationReloadEvidence,
    ) -> Result<(), WorthUiHeaderFrameRebindDenial> {
        if evidence.runtime_instance_witness() != self.instance_id().raw() {
            return Err(WorthUiHeaderFrameRebindDenial::RuntimeEvidenceMismatch);
        }
        let active = self.inspect_active();
        if evidence.active_artifact_digest_after() != active.artifact_digest()
            || evidence.active_plan_digest_after() != active.active_plan_digest()
        {
            return Err(WorthUiHeaderFrameRebindDenial::RuntimeEvidenceMismatch);
        }
        Ok(())
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

    fn verify_capability_rebind_evidence(
        &self,
        evidence: &WorthUiCapabilityReloadEvidence,
    ) -> Result<(), WorthUiHeaderFrameRebindDenial> {
        if evidence.runtime_instance_witness() != self.instance_id().raw() {
            return Err(WorthUiHeaderFrameRebindDenial::RuntimeEvidenceMismatch);
        }
        if evidence.active_snapshot_digest_after() != self.inspect_active().snapshot_digest() {
            return Err(WorthUiHeaderFrameRebindDenial::RuntimeEvidenceMismatch);
        }
        Ok(())
    }
}
