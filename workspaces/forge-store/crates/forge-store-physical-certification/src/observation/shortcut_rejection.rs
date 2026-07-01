#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutRejectionObservationKind {
    PrivateMutationDenied,
    JsonAuthorityDenied,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutRejectionObservation {
    kind: ShortcutRejectionObservationKind,
}

impl ShortcutRejectionObservation {
    pub const fn private_mutation_denied() -> Self {
        Self {
            kind: ShortcutRejectionObservationKind::PrivateMutationDenied,
        }
    }

    pub const fn json_authority_denied() -> Self {
        Self {
            kind: ShortcutRejectionObservationKind::JsonAuthorityDenied,
        }
    }

    pub const fn kind(&self) -> ShortcutRejectionObservationKind {
        self.kind
    }
}
