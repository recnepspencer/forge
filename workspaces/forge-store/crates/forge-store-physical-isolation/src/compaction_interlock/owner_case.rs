//! Read-only topology projected from cases issued by physical owners.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompactionCutoverState {
    PlanAdmitted,
    RewriteLowered,
    PublicationCommitted,
    RecoveryVisibilityAdmitted,
    ReclaimDeferred,
    Reclaimed,
    Denied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompactionOwnerCaseId(&'static str);

impl CompactionOwnerCaseId {
    pub(super) const fn owned(name: &'static str) -> Self {
        Self(name)
    }

    pub const fn name(self) -> &'static str {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompactionOwnerCase {
    id: CompactionOwnerCaseId,
    from: CompactionCutoverState,
    to: CompactionCutoverState,
}

impl CompactionOwnerCase {
    pub(super) const fn issued_by_owner(
        id: CompactionOwnerCaseId,
        from: CompactionCutoverState,
        to: CompactionCutoverState,
    ) -> Self {
        Self { id, from, to }
    }

    pub const fn id(self) -> CompactionOwnerCaseId {
        self.id
    }
    pub const fn from(self) -> CompactionCutoverState {
        self.from
    }
    pub const fn to(self) -> CompactionCutoverState {
        self.to
    }
}
