use super::LsmMembershipDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LsmMembershipOperation {
    Open,
    PersistRecord,
    SelectCompaction,
    ReplaceMembership,
    LookupPublishedReplacement,
}

impl LsmMembershipOperation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::PersistRecord => "persist_record",
            Self::SelectCompaction => "select_compaction",
            Self::ReplaceMembership => "replace_membership",
            Self::LookupPublishedReplacement => "lookup_published_replacement",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LsmMembershipDisposition {
    Admitted,
    Denied(LsmMembershipDenial),
}

impl LsmMembershipDisposition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Denied(denial) => denial.as_str(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LsmMembershipOwnerCaseId {
    operation: LsmMembershipOperation,
    disposition: LsmMembershipDisposition,
}

impl LsmMembershipOwnerCaseId {
    pub(super) const fn admitted(operation: LsmMembershipOperation) -> Self {
        Self {
            operation,
            disposition: LsmMembershipDisposition::Admitted,
        }
    }

    pub(super) const fn denied(
        operation: LsmMembershipOperation,
        denial: LsmMembershipDenial,
    ) -> Self {
        Self {
            operation,
            disposition: LsmMembershipDisposition::Denied(denial),
        }
    }

    pub const fn operation(self) -> LsmMembershipOperation {
        self.operation
    }

    pub const fn disposition(self) -> LsmMembershipDisposition {
        self.disposition
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmMembershipOwnerCaseDeclaration {
    id: LsmMembershipOwnerCaseId,
}

impl LsmMembershipOwnerCaseDeclaration {
    pub(super) const fn owned(id: LsmMembershipOwnerCaseId) -> Self {
        Self { id }
    }

    pub const fn id(self) -> LsmMembershipOwnerCaseId {
        self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmMembershipOwnerCaseObservation {
    id: LsmMembershipOwnerCaseId,
}

impl LsmMembershipOwnerCaseObservation {
    pub(super) const fn issued(id: LsmMembershipOwnerCaseId) -> Self {
        Self { id }
    }

    pub const fn id(self) -> LsmMembershipOwnerCaseId {
        self.id
    }
}
