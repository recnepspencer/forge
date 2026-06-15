#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiArtifactIdentitySeedKind {
    Authored,
    StructuralFallback,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactIdentitySeed {
    kind: WorthUiArtifactIdentitySeedKind,
    basis: String,
}

impl WorthUiArtifactIdentitySeed {
    pub(crate) fn authored(basis: String) -> Self {
        Self {
            kind: WorthUiArtifactIdentitySeedKind::Authored,
            basis,
        }
    }

    pub(crate) fn structural_fallback(basis: String) -> Self {
        Self {
            kind: WorthUiArtifactIdentitySeedKind::StructuralFallback,
            basis,
        }
    }

    pub(crate) fn kind(&self) -> &WorthUiArtifactIdentitySeedKind {
        &self.kind
    }

    pub(crate) fn basis(&self) -> &str {
        &self.basis
    }

    pub(crate) fn is_stable(&self) -> bool {
        !self.basis.is_empty()
    }
}
