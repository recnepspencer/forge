#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum WorthServerSurfaceFamily {
    WorthNative,
    CompatHttp,
    Sync,
    Lease,
    Binary,
    Integration,
}

impl WorthServerSurfaceFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WorthNative => "Worth-native",
            Self::CompatHttp => "compat-http",
            Self::Sync => "sync",
            Self::Lease => "lease",
            Self::Binary => "binary",
            Self::Integration => "integration",
        }
    }
}
