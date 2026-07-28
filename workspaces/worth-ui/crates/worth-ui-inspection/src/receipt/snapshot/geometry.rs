#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualCoordinateDenial {
    Negative,
    InvertedRect,
    EmptyRect,
    Overflow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualCoordinateOrientation {
    TopLeftOrigin,
    BottomLeftOrigin,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualCoordinateRounding {
    PixelCenterNearest,
    FloorEdges,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiVisualCoordinateObservation {
    native_client_origin: [i32; 2],
    client_physical_dimensions: [u32; 2],
    viewport_logical_dimensions: [f32; 2],
    scale: [f32; 2],
    translation: [f32; 2],
    orientation: UiVisualCoordinateOrientation,
    rounding: UiVisualCoordinateRounding,
}

#[doc(hidden)]
pub struct UiVisualCoordinateObservationInput {
    pub native_client_origin: [i32; 2],
    pub client_physical_dimensions: [u32; 2],
    pub viewport_logical_dimensions: [f32; 2],
    pub scale: [f32; 2],
    pub translation: [f32; 2],
    pub orientation: UiVisualCoordinateOrientation,
    pub rounding: UiVisualCoordinateRounding,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeScreenPhysicalPixel {
    x: u32,
    y: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiClientPhysicalPixel {
    x: u32,
    y: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiViewportLogicalPoint {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiHostSurfaceLogicalPoint {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiClientPhysicalRect {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

macro_rules! physical_pixel {
    ($name:ident) => {
        impl $name {
            pub fn new(x: i64, y: i64) -> Result<Self, UiVisualCoordinateDenial> {
                Ok(Self {
                    x: u32::try_from(x).map_err(|_| UiVisualCoordinateDenial::Negative)?,
                    y: u32::try_from(y).map_err(|_| UiVisualCoordinateDenial::Negative)?,
                })
            }

            pub const fn x(self) -> u32 {
                self.x
            }

            pub const fn y(self) -> u32 {
                self.y
            }
        }
    };
}

physical_pixel!(UiNativeScreenPhysicalPixel);
physical_pixel!(UiClientPhysicalPixel);

impl UiVisualCoordinateObservation {
    #[doc(hidden)]
    pub const fn from_runtime_projection(input: UiVisualCoordinateObservationInput) -> Self {
        Self {
            native_client_origin: input.native_client_origin,
            client_physical_dimensions: input.client_physical_dimensions,
            viewport_logical_dimensions: input.viewport_logical_dimensions,
            scale: input.scale,
            translation: input.translation,
            orientation: input.orientation,
            rounding: input.rounding,
        }
    }

    pub const fn native_client_origin(self) -> [i32; 2] {
        self.native_client_origin
    }

    pub const fn client_physical_dimensions(self) -> [u32; 2] {
        self.client_physical_dimensions
    }

    pub const fn viewport_logical_dimensions(self) -> [f32; 2] {
        self.viewport_logical_dimensions
    }

    pub const fn scale(self) -> [f32; 2] {
        self.scale
    }

    pub const fn translation(self) -> [f32; 2] {
        self.translation
    }

    pub const fn orientation(self) -> UiVisualCoordinateOrientation {
        self.orientation
    }

    pub const fn rounding(self) -> UiVisualCoordinateRounding {
        self.rounding
    }
}

impl UiViewportLogicalPoint {
    pub fn new(x: f32, y: f32) -> Result<Self, UiVisualCoordinateDenial> {
        finite_nonnegative(x, y).map(|()| Self { x, y })
    }

    pub const fn x(self) -> f32 {
        self.x
    }

    pub const fn y(self) -> f32 {
        self.y
    }
}

impl UiHostSurfaceLogicalPoint {
    pub fn new(x: f32, y: f32) -> Result<Self, UiVisualCoordinateDenial> {
        finite_nonnegative(x, y).map(|()| Self { x, y })
    }

    pub const fn x(self) -> f32 {
        self.x
    }

    pub const fn y(self) -> f32 {
        self.y
    }
}

impl UiClientPhysicalRect {
    pub fn new(
        left: u32,
        top: u32,
        right: u32,
        bottom: u32,
    ) -> Result<Self, UiVisualCoordinateDenial> {
        if right < left || bottom < top {
            return Err(UiVisualCoordinateDenial::InvertedRect);
        }
        if right == left || bottom == top {
            return Err(UiVisualCoordinateDenial::EmptyRect);
        }
        Ok(Self {
            left,
            top,
            right,
            bottom,
        })
    }

    pub const fn contains(self, point: UiClientPhysicalPixel) -> bool {
        point.x >= self.left && point.x < self.right && point.y >= self.top && point.y < self.bottom
    }

    pub const fn left(self) -> u32 {
        self.left
    }

    pub const fn top(self) -> u32 {
        self.top
    }

    pub const fn right(self) -> u32 {
        self.right
    }

    pub const fn bottom(self) -> u32 {
        self.bottom
    }
}

fn finite_nonnegative(x: f32, y: f32) -> Result<(), UiVisualCoordinateDenial> {
    if !x.is_finite() || !y.is_finite() {
        return Err(UiVisualCoordinateDenial::Overflow);
    }
    if x < 0.0 || y < 0.0 {
        return Err(UiVisualCoordinateDenial::Negative);
    }
    Ok(())
}
