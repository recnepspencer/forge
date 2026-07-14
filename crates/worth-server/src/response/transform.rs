#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerResponseTransform {
    WorthNative,
    CompatHttp,
}

impl WorthServerResponseTransform {
    pub const fn worth_native() -> Self {
        Self::WorthNative
    }

    pub const fn compat_http() -> Self {
        Self::CompatHttp
    }
}
