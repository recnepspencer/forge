#[derive(Debug, Eq, PartialEq)]
pub struct UiHostViewportChangedFact {
    width_subpixels: i64,
    height_subpixels: i64,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiHostDeviceScaleChangedFact {
    micros: u32,
}

impl UiHostViewportChangedFact {
    pub(crate) const fn new(width_subpixels: i64, height_subpixels: i64) -> Self {
        Self {
            width_subpixels,
            height_subpixels,
        }
    }

    pub const fn width_subpixels(&self) -> i64 {
        self.width_subpixels
    }

    pub const fn height_subpixels(&self) -> i64 {
        self.height_subpixels
    }
}

impl UiHostDeviceScaleChangedFact {
    pub(crate) const fn new(micros: u32) -> Self {
        Self { micros }
    }

    pub const fn micros(&self) -> u32 {
        self.micros
    }
}
