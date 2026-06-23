use crate::runtime::{
    WorthUiComponentCompatibility, WorthUiComponentReloadReceipt, WorthUiProjectionFamily,
    WorthUiProjectionIdentity,
};

use super::WorthUiProjectionRebindStatus;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiProjectionRebindRowReceipt {
    projection_identity: WorthUiProjectionIdentity,
    projection_family: WorthUiProjectionFamily,
    status: WorthUiProjectionRebindStatus,
    rebuild_attempted: bool,
    previous_frame_digest: u64,
    rebound_frame_digest: u64,
    component_reload_receipt: Option<WorthUiComponentReloadReceipt>,
}

impl WorthUiProjectionRebindRowReceipt {
    #[cfg(test)]
    pub(crate) fn new(
        projection_identity: WorthUiProjectionIdentity,
        projection_family: WorthUiProjectionFamily,
        status: WorthUiProjectionRebindStatus,
        previous_frame_digest: u64,
        rebound_frame_digest: u64,
    ) -> Self {
        Self::new_with_component_compatibility(
            projection_identity,
            projection_family,
            status,
            false,
            previous_frame_digest,
            rebound_frame_digest,
            None,
        )
    }

    pub(crate) fn new_with_component_compatibility(
        projection_identity: WorthUiProjectionIdentity,
        projection_family: WorthUiProjectionFamily,
        status: WorthUiProjectionRebindStatus,
        rebuild_attempted: bool,
        previous_frame_digest: u64,
        rebound_frame_digest: u64,
        component_compatibility: Option<WorthUiComponentCompatibility>,
    ) -> Self {
        Self {
            projection_identity,
            projection_family,
            status,
            rebuild_attempted,
            previous_frame_digest,
            rebound_frame_digest,
            component_reload_receipt: component_compatibility
                .map(|compatibility| WorthUiComponentReloadReceipt::new(Vec::new(), compatibility)),
        }
    }

    pub fn projection_identity(&self) -> &WorthUiProjectionIdentity {
        &self.projection_identity
    }

    pub fn projection_family(&self) -> WorthUiProjectionFamily {
        self.projection_family
    }

    pub fn status(&self) -> WorthUiProjectionRebindStatus {
        self.status
    }

    pub fn previous_frame_digest(&self) -> u64 {
        self.previous_frame_digest
    }

    pub fn rebuild_attempted(&self) -> bool {
        self.rebuild_attempted
    }

    pub fn rebound_frame_digest(&self) -> u64 {
        self.rebound_frame_digest
    }

    pub fn component_compatibility(&self) -> Option<&WorthUiComponentCompatibility> {
        self.component_reload_receipt
            .as_ref()
            .map(WorthUiComponentReloadReceipt::compatibility)
    }

    pub fn component_reload_receipt(&self) -> Option<&WorthUiComponentReloadReceipt> {
        self.component_reload_receipt.as_ref()
    }
}
