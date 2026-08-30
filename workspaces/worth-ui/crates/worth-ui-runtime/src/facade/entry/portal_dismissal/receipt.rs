use crate::facade::entry::focus_placement::UiSemanticFocusPublicationReceipt;

pub struct UiPortalDismissalPublicationReceipt {
    mounted: crate::mounting::UiMountedFramePublicationReceipt,
    focus: UiSemanticFocusPublicationReceipt,
}

impl UiPortalDismissalPublicationReceipt {
    pub(super) const fn new(
        mounted: crate::mounting::UiMountedFramePublicationReceipt,
        focus: UiSemanticFocusPublicationReceipt,
    ) -> Self {
        Self { mounted, focus }
    }

    pub const fn mounted(&self) -> &crate::mounting::UiMountedFramePublicationReceipt {
        &self.mounted
    }

    pub const fn focus_publication(&self) -> UiSemanticFocusPublicationReceipt {
        self.focus
    }
}
