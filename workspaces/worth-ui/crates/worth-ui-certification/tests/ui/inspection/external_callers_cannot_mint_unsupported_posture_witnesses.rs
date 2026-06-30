use worth_ui::facade::inspection::{
    UiInspectionMilestoneExpectation, UiInspectionPosture, UiInspectionSupportReason,
    UiInspectionUnsupportedPosture,
};

fn main() {
    let witness = UiInspectionUnsupportedPosture::new(
        UiInspectionSupportReason::BelongsArchitecturallyNotYetAdmitted,
        Some(UiInspectionMilestoneExpectation::Milestone31),
    );
    let _ = UiInspectionPosture::Unsupported(witness);
}
