#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostPixelColorSpace {
    Srgb,
    AdapterDeclared,
}

#[derive(Debug)]
pub struct UiHostPixelArtifact {
    dimensions: [u32; 2],
    stride: u32,
    bytes: Box<[u8]>,
    color_space: UiHostPixelColorSpace,
}

impl UiHostPixelArtifact {
    #[doc(hidden)]
    pub fn copied_by_host(
        dimensions: [u32; 2],
        stride: u32,
        bytes: Box<[u8]>,
        color_space: UiHostPixelColorSpace,
    ) -> Self {
        Self {
            dimensions,
            stride,
            bytes,
            color_space,
        }
    }

    pub const fn dimensions(&self) -> [u32; 2] {
        self.dimensions
    }

    pub const fn stride(&self) -> u32 {
        self.stride
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub const fn color_space(&self) -> UiHostPixelColorSpace {
        self.color_space
    }

    pub fn into_parts(self) -> ([u32; 2], u32, Box<[u8]>, UiHostPixelColorSpace) {
        (self.dimensions, self.stride, self.bytes, self.color_space)
    }
}
