use crate::capability::CommandProjectionSelectionMode;
use crate::runtime::{
    WorthUiDropdownFrameReceipt, WorthUiDropdownSelectionState,
    WorthUiDropdownSelectionStateReconciliationReceipt,
};

use super::WorthUiHeaderMenuCommand;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderMenuGroup {
    title: String,
    dropdown_frame: WorthUiDropdownFrameReceipt,
}

impl WorthUiHeaderMenuGroup {
    pub(crate) fn new(
        title: impl Into<String>,
        dropdown_frame: WorthUiDropdownFrameReceipt,
    ) -> Self {
        Self {
            title: title.into(),
            dropdown_frame,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn projection_id(&self) -> &str {
        self.dropdown_frame.projection_id()
    }

    pub fn selection_mode(&self) -> CommandProjectionSelectionMode {
        self.dropdown_frame.selection_mode()
    }

    pub fn commands(&self) -> &[WorthUiHeaderMenuCommand] {
        self.dropdown_frame.commands()
    }

    pub fn dropdown_frame(&self) -> &WorthUiDropdownFrameReceipt {
        &self.dropdown_frame
    }

    pub fn selection_state(&self) -> &WorthUiDropdownSelectionState {
        self.dropdown_frame.selection_state()
    }

    pub fn selection_reconciliation(&self) -> &WorthUiDropdownSelectionStateReconciliationReceipt {
        self.dropdown_frame.reconciliation()
    }
}
