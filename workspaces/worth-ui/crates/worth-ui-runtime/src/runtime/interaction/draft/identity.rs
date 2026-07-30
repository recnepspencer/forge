/// Declaration-owned slot for one draft-backed payload field.
///
/// The interaction runtime can carry this identity, but only the declaration
/// compiler may mint it.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiDraftFieldIdentity(u16);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiDraftSessionIdentity(u64);

impl UiDraftFieldIdentity {
    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) const fn from_declared_slot(slot: u16) -> Self {
        Self(slot)
    }

    pub const fn declared_slot(self) -> u16 {
        self.0
    }
}

impl UiDraftSessionIdentity {
    pub(super) const fn mint(value: u64) -> Self {
        Self(value)
    }

    pub const fn diagnostic_value(self) -> u64 {
        self.0
    }
}
