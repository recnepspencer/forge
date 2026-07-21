#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiSpatialViewportPoint {
    x: i32,
    y: i32,
}

impl WorthUiSpatialViewportPoint {
    pub fn viewport(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(self) -> i32 {
        self.x
    }

    pub fn y(self) -> i32 {
        self.y
    }
}
