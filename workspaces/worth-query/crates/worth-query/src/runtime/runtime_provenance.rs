use super::WorthQueryRuntimeAuthorityIdentity;

/// Opaque identity of one concrete Query runtime authority.
///
/// Query mints this value. Audiences may compare provenance issued by
/// different Query artifacts, but cannot construct or decompose it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryRuntimeProvenance {
    authority: WorthQueryRuntimeAuthorityIdentity,
}

impl WorthQueryRuntimeProvenance {
    pub(crate) const fn from_authority(authority: WorthQueryRuntimeAuthorityIdentity) -> Self {
        Self { authority }
    }
}

impl super::WorthQueryRuntime {
    pub fn runtime_provenance(&self) -> WorthQueryRuntimeProvenance {
        WorthQueryRuntimeProvenance::from_authority(self.authority_identity)
    }
}
