/// Non-authoritative correlation identity for one immutable visible-region
/// index retained by a runtime visual snapshot.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiVisibleRegionIndexIdentity {
    capture: u64,
    structure: u64,
}

/// Non-authoritative correlation identity for one immutable hit-test-region
/// index retained by a runtime visual snapshot.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiHitTestRegionIndexIdentity {
    capture: u64,
    structure: u64,
}

impl UiVisibleRegionIndexIdentity {
    #[doc(hidden)]
    pub const fn from_runtime_projection(capture: u64, structure: u64) -> Self {
        Self { capture, structure }
    }

    pub const fn diagnostic_value(self) -> u64 {
        self.capture
    }

    pub const fn structural_digest(self) -> u64 {
        self.structure
    }
}

impl UiHitTestRegionIndexIdentity {
    #[doc(hidden)]
    pub const fn from_runtime_projection(capture: u64, structure: u64) -> Self {
        Self { capture, structure }
    }

    pub const fn diagnostic_value(self) -> u64 {
        self.capture
    }

    pub const fn structural_digest(self) -> u64 {
        self.structure
    }
}
