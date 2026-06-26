pub(crate) const GOLDEN_HEADER_LEN: usize = 37;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalGoldenFormatHeaderFixture {
    bytes: [u8; GOLDEN_HEADER_LEN],
}

impl PhysicalGoldenFormatHeaderFixture {
    pub const fn s1_canonical() -> Self {
        Self {
            bytes: [
                70, 71, 83, 49, 70, 77, 84, 0, 1, 0, 1, 0, 64, 0, 0, 64, 0, 64, 0, 64, 0, 16, 0,
                32, 0, 0, 16, 8, 0, 8, 0, 0, 16, 8, 0, 1, 1,
            ],
        }
    }

    pub const fn bytes(&self) -> &[u8; GOLDEN_HEADER_LEN] {
        &self.bytes
    }

    pub const fn len(&self) -> usize {
        GOLDEN_HEADER_LEN
    }

    pub const fn is_empty(&self) -> bool {
        false
    }
}
