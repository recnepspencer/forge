use worth_ui::facade::WorthUiApp;
use worth_ui_harness::facade::{HarnessRunReceipt, HarnessRunner};

use crate::honesty::{
    ValidationAppEvidenceGate, ValidationAppEvidenceGateDenial, ValidationAppPublicFacadeLaunch,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationAppHonestyBoundary;

impl ValidationAppHonestyBoundary {
    pub fn prepare_public_facade_launch(
        app: &WorthUiApp,
    ) -> Result<
        worth_ui::facade::WorthUiRuntimeLaunch,
        worth_ui::facade::WorthUiRuntimeLaunchPreparationDenial,
    > {
        ValidationAppPublicFacadeLaunch::DEFAULT.prepare_for(app)
    }

    pub fn runner_for_public_app(app: WorthUiApp) -> HarnessRunner {
        HarnessRunner::for_app(app)
    }

    pub fn require_runtime_backed_receipt(
        receipt: &HarnessRunReceipt,
    ) -> Result<(), ValidationAppEvidenceGateDenial> {
        ValidationAppEvidenceGate::require_runtime_backed_receipt(receipt)
    }
}
