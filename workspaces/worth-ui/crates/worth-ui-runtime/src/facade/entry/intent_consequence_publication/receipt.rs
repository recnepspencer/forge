use crate::facade::entry::focus_placement::UiSemanticFocusPublicationReceipt;

pub struct UiIntentConsequencePublicationReceipt {
    rebind: crate::runtime::rebind::UiRebindReceipt,
    focus: Option<UiSemanticFocusPublicationReceipt>,
}

impl UiIntentConsequencePublicationReceipt {
    pub(super) const fn new(
        rebind: crate::runtime::rebind::UiRebindReceipt,
        focus: Option<UiSemanticFocusPublicationReceipt>,
    ) -> Self {
        Self { rebind, focus }
    }

    pub const fn rebind(&self) -> &crate::runtime::rebind::UiRebindReceipt {
        &self.rebind
    }

    pub fn into_rebind(self) -> crate::runtime::rebind::UiRebindReceipt {
        self.rebind
    }

    pub const fn focus_publication(&self) -> Option<UiSemanticFocusPublicationReceipt> {
        self.focus
    }
}
