use super::CapabilitySupportId;

/// Machine-readable support posture for a capability family or entry.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CapabilitySupportKind {
    Admitted,
    Deferred,
    Unsupported,
    PlatformInternal,
}

/// Classified support posture before a requirement has produced a proof witness.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CapabilitySupportPosture<T: CapabilitySupportId> {
    id: T,
    kind: CapabilitySupportKind,
}

impl<T: CapabilitySupportId> CapabilitySupportPosture<T> {
    pub fn admitted(id: T) -> Self {
        Self {
            id,
            kind: CapabilitySupportKind::Admitted,
        }
    }

    pub fn deferred(id: T) -> Self {
        Self {
            id,
            kind: CapabilitySupportKind::Deferred,
        }
    }

    pub fn unsupported(id: T) -> Self {
        Self {
            id,
            kind: CapabilitySupportKind::Unsupported,
        }
    }

    pub fn platform_internal(id: T) -> Self {
        Self {
            id,
            kind: CapabilitySupportKind::PlatformInternal,
        }
    }

    pub fn id(&self) -> &T {
        &self.id
    }

    pub fn kind(&self) -> CapabilitySupportKind {
        self.kind
    }

    pub fn is_admitted(&self) -> bool {
        self.kind == CapabilitySupportKind::Admitted
    }

    pub fn is_deferred(&self) -> bool {
        self.kind == CapabilitySupportKind::Deferred
    }

    pub fn is_unsupported(&self) -> bool {
        self.kind == CapabilitySupportKind::Unsupported
    }

    pub fn is_platform_internal(&self) -> bool {
        self.kind == CapabilitySupportKind::PlatformInternal
    }

    pub(crate) fn into_id_and_kind(self) -> (T, CapabilitySupportKind) {
        (self.id, self.kind)
    }
}
