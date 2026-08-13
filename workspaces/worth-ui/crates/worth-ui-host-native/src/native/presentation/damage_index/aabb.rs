use worth_ui_host_contract::UiMountedCanonicalBox;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Aabb {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

impl Aabb {
    pub(super) fn from_bounds(bounds: UiMountedCanonicalBox) -> Self {
        Self {
            left: bounds.x(),
            top: bounds.y(),
            right: bounds.x() + bounds.width(),
            bottom: bounds.y() + bounds.height(),
        }
    }

    pub(super) fn union(self, other: Self) -> Self {
        Self {
            left: self.left.min(other.left),
            top: self.top.min(other.top),
            right: self.right.max(other.right),
            bottom: self.bottom.max(other.bottom),
        }
    }

    pub(super) fn perimeter(self) -> f32 {
        2.0 * ((self.right - self.left) + (self.bottom - self.top))
    }

    pub(super) fn intersects(self, other: Self) -> bool {
        self.left < other.right
            && other.left < self.right
            && self.top < other.bottom
            && other.top < self.bottom
    }
}
