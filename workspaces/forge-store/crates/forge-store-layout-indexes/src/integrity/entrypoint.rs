#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutCorruptionFacade;

pub const fn layout_corruption() -> LayoutCorruptionFacade {
    LayoutCorruptionFacade
}
