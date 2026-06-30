/// Explicit platform support posture for a native capability seam.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NativePlatformPosture {
    RuntimeDeclared,
    Deferred,
    Unsupported,
}

impl NativePlatformPosture {
    pub fn runtime_declared() -> Self {
        Self::RuntimeDeclared
    }

    pub fn deferred() -> Self {
        Self::Deferred
    }

    pub fn unsupported() -> Self {
        Self::Unsupported
    }

    pub(crate) fn digest_basis(self) -> &'static str {
        match self {
            Self::RuntimeDeclared => "runtime_declared",
            Self::Deferred => "deferred",
            Self::Unsupported => "unsupported",
        }
    }
}
