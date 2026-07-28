/// Symmetric logical-point margins for a component whose allocation is inset
/// from the admitted viewport.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComponentViewportInset {
    horizontal_logical_points: u16,
    vertical_logical_points: u16,
}

impl ComponentViewportInset {
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
            "viewport-inset:{}:{}",
            self.horizontal_logical_points, self.vertical_logical_points
        )
    }
}
