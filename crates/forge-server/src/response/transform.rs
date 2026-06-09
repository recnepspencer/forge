#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerResponseTransform {
    ForgeNative,
    CompatHttp,
}

impl ForgeServerResponseTransform {
    pub const fn forge_native() -> Self {
        Self::ForgeNative
    }

    pub const fn compat_http() -> Self {
        Self::CompatHttp
    }
}
