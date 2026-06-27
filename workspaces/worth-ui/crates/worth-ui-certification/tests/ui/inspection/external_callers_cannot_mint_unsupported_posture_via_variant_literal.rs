use worth_ui::facade::inspection::{
    UiInspectionMilestoneExpectation, UiInspectionPosture, UiInspectionSupportReason,
    UiInspectionUnsupportedPosture,
};

fn main() {
    let _ = UiInspectionPosture::Unsupported(UiInspectionUnsupportedPosture {
        reason: UiInspectionSupportReason::BelongsArchitecturallyNotYetAdmitted,
        expected_in: Some(UiInspectionMilestoneExpectation::Milestone31),
    });
}
