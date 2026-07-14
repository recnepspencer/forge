use super::BaselineLsmExecutionAdmissionDenialKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LsmExecutionOperation {
    PrepareCompaction,
    BindPhysicalCompaction,
    PrepareMembershipActivation,
    PublishCompaction,
    ExecuteReplay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LsmExecutionDisposition {
    Admitted,
    Denied(BaselineLsmExecutionAdmissionDenialKind),
}

impl LsmExecutionDisposition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Denied(denial) => denial.as_str(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LsmExecutionOwnerCaseId {
    operation: LsmExecutionOperation,
    disposition: LsmExecutionDisposition,
}

impl LsmExecutionOwnerCaseId {
    pub(in crate::strategy::lsm) const fn admitted(operation: LsmExecutionOperation) -> Self {
        Self {
            operation,
            disposition: LsmExecutionDisposition::Admitted,
        }
    }

    pub(in crate::strategy::lsm) const fn denied(
        operation: LsmExecutionOperation,
        denial: BaselineLsmExecutionAdmissionDenialKind,
    ) -> Self {
        Self {
            operation,
            disposition: LsmExecutionDisposition::Denied(denial),
        }
    }

    pub const fn operation(self) -> LsmExecutionOperation {
        self.operation
    }

    pub const fn disposition(self) -> LsmExecutionDisposition {
        self.disposition
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmExecutionOwnerCaseDeclaration {
    id: LsmExecutionOwnerCaseId,
}

impl LsmExecutionOwnerCaseDeclaration {
    pub(in crate::strategy::lsm) const fn new(id: LsmExecutionOwnerCaseId) -> Self {
        Self { id }
    }

    pub const fn id(self) -> LsmExecutionOwnerCaseId {
        self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmExecutionOwnerCaseObservation {
    id: LsmExecutionOwnerCaseId,
}

impl LsmExecutionOwnerCaseObservation {
    pub(in crate::strategy::lsm) const fn new(id: LsmExecutionOwnerCaseId) -> Self {
        Self { id }
    }

    pub const fn id(self) -> LsmExecutionOwnerCaseId {
        self.id
    }
}
