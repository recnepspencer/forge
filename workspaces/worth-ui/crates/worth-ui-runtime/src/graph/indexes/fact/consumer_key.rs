#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiGraphFactConsumerKind {
    GraphNode,
    MountEligibilitySlot,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiGraphFactConsumerKey {
    kind: UiGraphFactConsumerKind,
    authored_identity: Box<str>,
    repeated_instance_basis_digest: u64,
}

impl UiGraphFactConsumerKey {
    pub(crate) fn new(
        kind: UiGraphFactConsumerKind,
        authored_identity: impl Into<Box<str>>,
        repeated_instance_basis_digest: u64,
    ) -> Self {
        Self {
            kind,
            authored_identity: authored_identity.into(),
            repeated_instance_basis_digest,
        }
    }

    pub const fn kind(&self) -> UiGraphFactConsumerKind {
        self.kind
    }

    pub fn authored_identity(&self) -> &str {
        &self.authored_identity
    }

    pub const fn repeated_instance_basis_digest(&self) -> u64 {
        self.repeated_instance_basis_digest
    }
}
