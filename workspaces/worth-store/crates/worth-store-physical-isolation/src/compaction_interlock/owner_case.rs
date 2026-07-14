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
pub struct CompactionOwnerCaseDeclaration {
    id: CompactionOwnerCaseId,
    from: CompactionCutoverState,
    to: CompactionCutoverState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompactionOwnerCaseObservation {
    declaration: CompactionOwnerCaseDeclaration,
}

impl CompactionOwnerCaseDeclaration {
    pub(super) const fn declared_by_owner(
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

impl CompactionOwnerCaseObservation {
    pub(super) const fn issued_by_owner(declaration: CompactionOwnerCaseDeclaration) -> Self {
        Self { declaration }
    }

    pub const fn declaration(self) -> CompactionOwnerCaseDeclaration {
        self.declaration
    }

    pub const fn id(self) -> CompactionOwnerCaseId {
        self.declaration.id()
    }

    pub const fn from(self) -> CompactionCutoverState {
        self.declaration.from()
    }

    pub const fn to(self) -> CompactionCutoverState {
        self.declaration.to()
    }
}
