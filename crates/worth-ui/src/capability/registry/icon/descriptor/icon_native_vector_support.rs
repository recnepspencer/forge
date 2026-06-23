#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IconNativeVectorSupport {
    Supported,
    UnsupportedByHost,
}

impl IconNativeVectorSupport {
    pub fn supported() -> Self {
        Self::Supported
    }

    pub fn unsupported_by_host() -> Self {
        Self::UnsupportedByHost
    }

    pub fn supports_native_vector(self) -> bool {
        matches!(self, Self::Supported)
    }

    pub(crate) fn digest_basis(self) -> &'static str {
        match self {
            Self::Supported => "native_vector_supported",
            Self::UnsupportedByHost => "native_vector_unsupported_by_host",
        }
    }
}
