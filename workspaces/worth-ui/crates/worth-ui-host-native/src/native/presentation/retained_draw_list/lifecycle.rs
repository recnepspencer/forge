use worth_ui_host_contract::{UiMountedPaintCommand, UiMountedPresentationUnchanged};

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
