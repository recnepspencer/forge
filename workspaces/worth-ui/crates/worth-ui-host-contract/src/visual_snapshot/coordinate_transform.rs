#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostCoordinateOrientation {
    TopLeftOrigin,
    BottomLeftOrigin,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostCoordinateRounding {
    PixelCenterNearest,
    FloorEdges,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiHostCoordinateTransform {
    native_client_origin: [i32; 2],
    client_physical_dimensions: [u32; 2],
    viewport_logical_dimensions: [f32; 2],
    scale: [f32; 2],
    translation: [f32; 2],
    orientation: UiHostCoordinateOrientation,
    rounding: UiHostCoordinateRounding,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiHostClientAreaObservation {
    native_origin: [i32; 2],
    physical_dimensions: [u32; 2],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiHostViewportTransformObservation {
    logical_dimensions: [f32; 2],
    scale: [f32; 2],
    translation: [f32; 2],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostCoordinatePosture {
    orientation: UiHostCoordinateOrientation,
    rounding: UiHostCoordinateRounding,
}

impl UiHostCoordinateTransform {
    #[doc(hidden)]
    pub const fn observed_by_host(
        client: UiHostClientAreaObservation,
        viewport: UiHostViewportTransformObservation,
        posture: UiHostCoordinatePosture,
    ) -> Self {
        Self {
            native_client_origin: client.native_origin,
            client_physical_dimensions: client.physical_dimensions,
            viewport_logical_dimensions: viewport.logical_dimensions,
            scale: viewport.scale,
            translation: viewport.translation,
            orientation: posture.orientation,
            rounding: posture.rounding,
        }
    }

    pub const fn client_physical_dimensions(self) -> [u32; 2] {
        self.client_physical_dimensions
    }

    pub const fn native_client_origin(self) -> [i32; 2] {
        self.native_client_origin
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

    pub const fn orientation(self) -> UiHostCoordinateOrientation {
        self.orientation
    }

    pub const fn rounding(self) -> UiHostCoordinateRounding {
        self.rounding
    }
}

impl UiHostClientAreaObservation {
    #[doc(hidden)]
    pub const fn observed_by_host(native_origin: [i32; 2], physical_dimensions: [u32; 2]) -> Self {
        Self {
            native_origin,
            physical_dimensions,
        }
    }
}

impl UiHostViewportTransformObservation {
    #[doc(hidden)]
    pub const fn observed_by_host(
        logical_dimensions: [f32; 2],
        scale: [f32; 2],
        translation: [f32; 2],
    ) -> Self {
        Self {
            logical_dimensions,
            scale,
            translation,
        }
    }
}

impl UiHostCoordinatePosture {
    #[doc(hidden)]
    pub const fn observed_by_host(
        orientation: UiHostCoordinateOrientation,
        rounding: UiHostCoordinateRounding,
    ) -> Self {
        Self {
            orientation,
            rounding,
        }
    }
}
