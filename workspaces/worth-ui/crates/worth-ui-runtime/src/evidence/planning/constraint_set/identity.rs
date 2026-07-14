#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAllocationConstraintSetIdentity {
    identity_digest: u64,
}

impl UiAllocationConstraintSetIdentity {
    pub(crate) const fn new(identity_digest: u64) -> Self {
        Self { identity_digest }
    }

    pub const fn identity_digest(self) -> u64 {
        self.identity_digest
    }
}
