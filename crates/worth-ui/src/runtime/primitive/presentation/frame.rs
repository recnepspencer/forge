#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorthUiPrimitiveFrame {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl WorthUiPrimitiveFrame {
    pub(crate) fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }
}
