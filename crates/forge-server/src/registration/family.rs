#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum ForgeServerSurfaceFamily {
    ForgeNative,
    CompatHttp,
    Sync,
    Lease,
    Binary,
    Integration,
}

impl ForgeServerSurfaceFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ForgeNative => "forge-native",
            Self::CompatHttp => "compat-http",
            Self::Sync => "sync",
            Self::Lease => "lease",
            Self::Binary => "binary",
            Self::Integration => "integration",
        }
    }
}
