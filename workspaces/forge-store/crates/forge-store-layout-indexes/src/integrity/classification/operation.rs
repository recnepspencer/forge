#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutCorruptionAssessment;

pub const fn layout_corruption() -> LayoutCorruptionAssessment {
    LayoutCorruptionAssessment
}
