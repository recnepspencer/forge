#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiScrollOwnerSourceKind {
    HostContainerViewport,
    QueryContentExtent,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiScrollOwnerSourceEvidence {
    kind: UiScrollOwnerSourceKind,
    identity_digest: u64,
}

impl UiScrollOwnerSourceEvidence {
    pub(super) fn seal(kind: UiScrollOwnerSourceKind, identity_digest: u64) -> Self {
        Self {
            kind,
            identity_digest,
        }
    }
    pub(crate) fn seal_graph(
        kind: UiScrollOwnerSourceKind,
        identity_digest: u64,
        _: &crate::graph::UiGraphConstraintMintAuthority,
    ) -> Self {
        Self::seal(kind, identity_digest)
    }

    pub fn kind(self) -> UiScrollOwnerSourceKind {
        self.kind
    }

    pub fn identity_digest(self) -> u64 {
        self.identity_digest
    }

    pub fn is_host_container_viewport(self) -> bool {
        self.kind == UiScrollOwnerSourceKind::HostContainerViewport
    }
}
