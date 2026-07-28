/// Explicit back-to-front order owned by component paint meaning.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ComponentStaticPaintOrder(u32);

impl ComponentStaticPaintOrder {
    pub const fn back_to_front(rank: u32) -> Self {
        Self(rank)
    }

    pub const fn rank(self) -> u32 {
        self.0
    }
}
