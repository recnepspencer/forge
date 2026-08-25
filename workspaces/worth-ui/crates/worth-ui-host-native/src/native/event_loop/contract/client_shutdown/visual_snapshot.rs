#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeClientVisualSnapshotRelation {
    Current,
    RetainedPredecessor,
    Historical,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeClientVisualPixelColorSpace {
    Srgb,
    AdapterDeclared,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeClientVisualCoordinateOrientation {
    TopLeftOrigin,
    BottomLeftOrigin,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeClientVisualCoordinateRounding {
    PixelCenterNearest,
    FloorEdges,
}

#[doc(hidden)]
pub struct UiNativeClientVisualSnapshotInput {
    pub affinity: [u64; 7],
    pub relation: UiNativeClientVisualSnapshotRelation,
    pub native_client_origin: [i32; 2],
    pub client_physical_dimensions: [u32; 2],
    pub viewport_logical_dimension_bits: [u32; 2],
    pub scale_bits: [u32; 2],
    pub translation_bits: [u32; 2],
    pub coordinate_orientation: UiNativeClientVisualCoordinateOrientation,
    pub coordinate_rounding: UiNativeClientVisualCoordinateRounding,
    pub pixel_dimensions: [u32; 2],
    pub pixel_stride: u32,
    pub pixel_color_space: UiNativeClientVisualPixelColorSpace,
    pub pixel_bytes: Box<[u8]>,
    pub visible_region_count: u64,
    pub hit_test_region_count: u64,
    pub cost_counters: [u64; 11],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiNativeClientVisualSnapshotObservation {
    affinity: [u64; 7],
    relation: UiNativeClientVisualSnapshotRelation,
    native_client_origin: [i32; 2],
    client_physical_dimensions: [u32; 2],
    viewport_logical_dimension_bits: [u32; 2],
    scale_bits: [u32; 2],
    translation_bits: [u32; 2],
    coordinate_orientation: UiNativeClientVisualCoordinateOrientation,
    coordinate_rounding: UiNativeClientVisualCoordinateRounding,
    pixel_dimensions: [u32; 2],
    pixel_stride: u32,
    pixel_color_space: UiNativeClientVisualPixelColorSpace,
    pixel_bytes: Box<[u8]>,
    visible_region_count: u64,
    hit_test_region_count: u64,
    cost_counters: [u64; 11],
}

impl UiNativeClientVisualSnapshotObservation {
    #[doc(hidden)]
    pub fn reported(input: UiNativeClientVisualSnapshotInput) -> Self {
        Self {
            affinity: input.affinity,
            relation: input.relation,
            native_client_origin: input.native_client_origin,
            client_physical_dimensions: input.client_physical_dimensions,
            viewport_logical_dimension_bits: input.viewport_logical_dimension_bits,
            scale_bits: input.scale_bits,
            translation_bits: input.translation_bits,
            coordinate_orientation: input.coordinate_orientation,
            coordinate_rounding: input.coordinate_rounding,
            pixel_dimensions: input.pixel_dimensions,
            pixel_stride: input.pixel_stride,
            pixel_color_space: input.pixel_color_space,
            pixel_bytes: input.pixel_bytes,
            visible_region_count: input.visible_region_count,
            hit_test_region_count: input.hit_test_region_count,
            cost_counters: input.cost_counters,
        }
    }

    pub const fn affinity(&self) -> [u64; 7] {
        self.affinity
    }

    pub const fn relation(&self) -> UiNativeClientVisualSnapshotRelation {
        self.relation
    }

    pub const fn native_client_origin(&self) -> [i32; 2] {
        self.native_client_origin
    }

    pub const fn client_physical_dimensions(&self) -> [u32; 2] {
        self.client_physical_dimensions
    }

    pub const fn viewport_logical_dimension_bits(&self) -> [u32; 2] {
        self.viewport_logical_dimension_bits
    }

    pub const fn scale_bits(&self) -> [u32; 2] {
        self.scale_bits
    }

    pub const fn translation_bits(&self) -> [u32; 2] {
        self.translation_bits
    }

    pub const fn coordinate_orientation(&self) -> UiNativeClientVisualCoordinateOrientation {
        self.coordinate_orientation
    }

    pub const fn coordinate_rounding(&self) -> UiNativeClientVisualCoordinateRounding {
        self.coordinate_rounding
    }

    pub const fn pixel_dimensions(&self) -> [u32; 2] {
        self.pixel_dimensions
    }

    pub const fn pixel_stride(&self) -> u32 {
        self.pixel_stride
    }

    pub const fn pixel_color_space(&self) -> UiNativeClientVisualPixelColorSpace {
        self.pixel_color_space
    }

    pub fn pixel_bytes(&self) -> &[u8] {
        &self.pixel_bytes
    }

    pub const fn visible_region_count(&self) -> u64 {
        self.visible_region_count
    }

    pub const fn hit_test_region_count(&self) -> u64 {
        self.hit_test_region_count
    }

    pub const fn cost_counters(&self) -> [u64; 11] {
        self.cost_counters
    }
}
