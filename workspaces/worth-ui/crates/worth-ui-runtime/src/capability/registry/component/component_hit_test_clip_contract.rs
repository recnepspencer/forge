/// Component-owned clipping applied inside its already-admitted allocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentHitTestClipContract {
    AllocationBounds,
    Inset(super::ComponentHitTestInset),
}

impl ComponentHitTestClipContract {
    pub const fn allocation_bounds() -> Self {
        Self::AllocationBounds
    }

    pub const fn inset(inset: super::ComponentHitTestInset) -> Self {
        Self::Inset(inset)
    }

    pub(crate) fn digest_basis(self) -> String {
        match self {
            Self::AllocationBounds => "allocation-bounds".to_owned(),
            Self::Inset(inset) => format!("allocation-clip:{}", inset.digest_basis()),
        }
    }
}
