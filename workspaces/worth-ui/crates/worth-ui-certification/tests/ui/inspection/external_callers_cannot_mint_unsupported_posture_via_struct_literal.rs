use worth_ui::facade::inspection::{
    UiInspectionMilestoneExpectation, UiInspectionSupportReason, UiInspectionUnsupportedPosture,
};

fn main() {
    let _ = UiInspectionUnsupportedPosture {
        reason: UiInspectionSupportReason::BelongsArchitecturallyNotYetAdmitted,
        expected_in: Some(UiInspectionMilestoneExpectation::Milestone31),
    };
}
