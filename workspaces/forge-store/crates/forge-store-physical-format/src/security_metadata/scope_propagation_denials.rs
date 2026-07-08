#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSecurityScopePropagationDenial {
    kind: PhysicalSecurityScopePropagationDenialKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSecurityScopePropagationDenialKind {
    MissingPropagatedSecurityScope,
    StalePropagatedSecurityScope,
    ScopeDriftBeforeLogicalDecode,
    UnsupportedPropagatedSecurityScope,
    UnavailablePropagatedSecurityScope,
}

impl PhysicalSecurityScopePropagationDenial {
    pub const fn new(kind: PhysicalSecurityScopePropagationDenialKind) -> Self {
        Self { kind }
    }

    pub const fn missing() -> Self {
        Self::new(PhysicalSecurityScopePropagationDenialKind::MissingPropagatedSecurityScope)
    }

    pub const fn stale() -> Self {
        Self::new(PhysicalSecurityScopePropagationDenialKind::StalePropagatedSecurityScope)
    }

    pub const fn drift() -> Self {
        Self::new(PhysicalSecurityScopePropagationDenialKind::ScopeDriftBeforeLogicalDecode)
    }

    pub const fn unsupported() -> Self {
        Self::new(PhysicalSecurityScopePropagationDenialKind::UnsupportedPropagatedSecurityScope)
    }

    pub const fn unavailable() -> Self {
        Self::new(PhysicalSecurityScopePropagationDenialKind::UnavailablePropagatedSecurityScope)
    }

    pub const fn kind(self) -> PhysicalSecurityScopePropagationDenialKind {
        self.kind
    }
}
