/// Symmetric logical-point margins inside one admitted component allocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComponentHitTestInset {
    horizontal_logical_points: u16,
    vertical_logical_points: u16,
}

impl ComponentHitTestInset {
    pub const fn symmetric(horizontal_logical_points: u16, vertical_logical_points: u16) -> Self {
        Self {
            horizontal_logical_points,
            vertical_logical_points,
        }
    }

    pub const fn horizontal_logical_points(self) -> u16 {
        self.horizontal_logical_points
    }

    pub const fn vertical_logical_points(self) -> u16 {
        self.vertical_logical_points
    }

    pub(crate) fn digest_basis(self) -> String {
        format!(
            "hit-test-inset:{}:{}",
            self.horizontal_logical_points, self.vertical_logical_points
        )
    }
}
