#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IconSourceKind {
    Symbol,
    VectorAsset,
    RasterAsset,
    IconPack,
    Unsupported,
}

impl IconSourceKind {
    pub fn symbol() -> Self {
        Self::Symbol
    }

    pub fn vector_asset() -> Self {
        Self::VectorAsset
    }

    pub fn raster_asset() -> Self {
        Self::RasterAsset
    }

    pub fn icon_pack() -> Self {
        Self::IconPack
    }

    pub fn unsupported_for_diagnostics() -> Self {
        Self::Unsupported
    }

    pub(crate) fn is_supported(self) -> bool {
        !matches!(self, Self::Unsupported)
    }

    pub(crate) fn digest_basis(self) -> &'static str {
        match self {
            Self::Symbol => "symbol",
            Self::VectorAsset => "vector_asset",
            Self::RasterAsset => "raster_asset",
            Self::IconPack => "icon_pack",
            Self::Unsupported => "unsupported",
        }
    }
}
