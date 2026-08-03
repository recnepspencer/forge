/// Explicit front-to-back order owned by component hit-test meaning.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ComponentHitTestOrder(u32);

impl ComponentHitTestOrder {
    pub const fn front_to_back(rank: u32) -> Self {
        Self(rank)
    }

    pub const fn rank(self) -> u32 {
        self.0
    }
}
