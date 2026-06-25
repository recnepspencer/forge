use super::{WorthUiCapabilityReloadFamilyKind, WorthUiCapabilityReloadStage};

pub(super) fn family_for_stage(
    stage: WorthUiCapabilityReloadStage,
) -> WorthUiCapabilityReloadFamilyKind {
    match stage {
        WorthUiCapabilityReloadStage::ThemeTokenSourceParse
        | WorthUiCapabilityReloadStage::ThemeTokenAdmission => {
            WorthUiCapabilityReloadFamilyKind::ThemeTokens
        }
        WorthUiCapabilityReloadStage::CommandSourceParse
        | WorthUiCapabilityReloadStage::CommandAdmission => {
            WorthUiCapabilityReloadFamilyKind::Commands
        }
        WorthUiCapabilityReloadStage::CommandProjectionSourceParse
        | WorthUiCapabilityReloadStage::CommandProjectionAdmission => {
            WorthUiCapabilityReloadFamilyKind::CommandProjections
        }
        WorthUiCapabilityReloadStage::ComponentSourceParse
        | WorthUiCapabilityReloadStage::ComponentAdmission => {
            WorthUiCapabilityReloadFamilyKind::Components
        }
        WorthUiCapabilityReloadStage::AppearanceSourceParse
        | WorthUiCapabilityReloadStage::AppearanceAdmission => {
            WorthUiCapabilityReloadFamilyKind::Appearance
        }
        WorthUiCapabilityReloadStage::DensitySourceParse
        | WorthUiCapabilityReloadStage::DensityAdmission => {
            WorthUiCapabilityReloadFamilyKind::Density
        }
        WorthUiCapabilityReloadStage::DuplicateCapabilityFamily
        | WorthUiCapabilityReloadStage::ActiveSnapshotDrift
        | WorthUiCapabilityReloadStage::RuntimeInstanceMismatch
        | WorthUiCapabilityReloadStage::MissingReadyActivation => {
            panic!("test-only denied constructor requires a single capability family stage")
        }
    }
}
