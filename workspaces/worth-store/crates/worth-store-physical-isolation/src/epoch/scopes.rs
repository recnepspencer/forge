#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EpochStabilityScopeKind {
    ReadPlanAdmission,
    RootReadmission,
    ReferenceValidation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EpochComparisonScope {
    kind: EpochStabilityScopeKind,
    root_scope_id: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EpochComparisonScopeMismatch {
    expected: EpochComparisonScope,
    actual: EpochComparisonScope,
}

impl EpochComparisonScope {
    pub const fn read_plan_admission(root_scope_id: u64) -> Self {
        Self {
            kind: EpochStabilityScopeKind::ReadPlanAdmission,
            root_scope_id,
        }
    }

    pub const fn root_readmission(root_scope_id: u64) -> Self {
        Self {
            kind: EpochStabilityScopeKind::RootReadmission,
            root_scope_id,
        }
    }

    pub const fn reference_validation(root_scope_id: u64) -> Self {
        Self {
            kind: EpochStabilityScopeKind::ReferenceValidation,
            root_scope_id,
        }
    }

    pub const fn kind(self) -> EpochStabilityScopeKind {
        self.kind
    }

    pub const fn root_scope_id(self) -> u64 {
        self.root_scope_id
    }

    pub const fn require_same(
        self,
        actual: EpochComparisonScope,
    ) -> Result<(), EpochComparisonScopeMismatch> {
        if self.kind as u8 == actual.kind as u8 && self.root_scope_id == actual.root_scope_id {
            Ok(())
        } else {
            Err(EpochComparisonScopeMismatch {
                expected: self,
                actual,
            })
        }
    }
}

impl EpochComparisonScopeMismatch {
    pub const fn expected(self) -> EpochComparisonScope {
        self.expected
    }

    pub const fn actual(self) -> EpochComparisonScope {
        self.actual
    }
}
