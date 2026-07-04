use worth_ui_inspection::UiInspectionCloseoutReport;

use crate::facade::WorthUiApp;

impl WorthUiApp {
    pub fn inspection_closeout_report(&self) -> UiInspectionCloseoutReport {
        UiInspectionCloseoutReport::milestone35()
    }
}
