//! Opaque identity for one text-owned admitted raster batch.

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiGlyphRasterBatchIdentity([u8; 32]);

impl UiGlyphRasterBatchIdentity {
    #[doc(hidden)]
    pub const fn from_text_mechanics(digest: [u8; 32]) -> Self {
        Self(digest)
    }

    pub const fn digest(self) -> [u8; 32] {
        self.0
    }
}
