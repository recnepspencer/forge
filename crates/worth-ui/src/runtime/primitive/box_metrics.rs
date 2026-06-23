#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorthUiBoxEdges {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

impl WorthUiBoxEdges {
    pub(crate) fn uniform(points: f32) -> Self {
        Self {
            top: points,
            right: points,
            bottom: points,
            left: points,
        }
    }

    pub(crate) fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn top(self) -> f32 {
        self.top
    }

    pub fn right(self) -> f32 {
        self.right
    }

    pub fn bottom(self) -> f32 {
        self.bottom
    }

    pub fn left(self) -> f32 {
        self.left
    }

    pub fn horizontal(self) -> f32 {
        self.left + self.right
    }

    pub fn vertical(self) -> f32 {
        self.top + self.bottom
    }

    pub fn max_axis_point(self) -> f32 {
        self.top.max(self.right).max(self.bottom).max(self.left)
    }

    pub(crate) fn digest_basis(self) -> String {
        format!(
            "edges:{:.3}:{:.3}:{:.3}:{:.3}",
            self.top, self.right, self.bottom, self.left
        )
    }
}
