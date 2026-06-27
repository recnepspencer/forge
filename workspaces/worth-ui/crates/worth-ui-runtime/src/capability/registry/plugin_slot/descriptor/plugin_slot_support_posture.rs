/// Runtime support posture for a plugin contribution slot.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PluginSlotSupportPosture {
    Supported,
    Experimental,
    Deferred,
}

impl PluginSlotSupportPosture {
    pub fn supported() -> Self {
        Self::Supported
    }

    pub fn experimental() -> Self {
        Self::Experimental
    }

    pub fn deferred() -> Self {
        Self::Deferred
    }

    pub(crate) fn digest_basis(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Experimental => "experimental",
            Self::Deferred => "deferred",
        }
    }
}
