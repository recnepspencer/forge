use worth_ui_host_contract::{
    UiMountedFrameIdentity, UiMountedPaintCommand, UiMountedPresentationUnchanged,
};

use super::{UiNativeRetainedDrawList, UiNativeRetainedDrawListDenial};

impl UiNativeRetainedDrawList {
    pub(crate) fn apply_unchanged(
        &mut self,
        unchanged: &UiMountedPresentationUnchanged,
    ) -> Result<(), UiNativeRetainedDrawListDenial> {
        let affinity = unchanged.affinity();
        if affinity.predecessor() != Some(self.frame)
            || affinity.surface() != self.surface
            || affinity.binding() != self.binding
            || affinity.baseline() != self.baseline
        {
            return Err(UiNativeRetainedDrawListDenial::AffinityMismatch);
        }
        self.frame = affinity.successor();
        Ok(())
    }

    pub(crate) fn stage_unchanged(
        &mut self,
        unchanged: &UiMountedPresentationUnchanged,
    ) -> Result<UiMountedFrameIdentity, UiNativeRetainedDrawListDenial> {
        let predecessor = self.frame;
        self.apply_unchanged(unchanged)?;
        Ok(predecessor)
    }

    pub(crate) fn rollback_unchanged(&mut self, predecessor: UiMountedFrameIdentity) {
        self.frame = predecessor;
    }

    pub(in crate::native::presentation) fn reconstruction_commands(
        &self,
    ) -> Result<Vec<&UiMountedPaintCommand>, UiNativeRetainedDrawListDenial> {
        self.order
            .ordered()
            .map(|identity| {
                self.commands
                    .get(&identity.command())
                    .ok_or(UiNativeRetainedDrawListDenial::OrderMismatch)
            })
            .collect()
    }
}
