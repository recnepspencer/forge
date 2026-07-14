use super::LsmMaintenanceAdmissionDenialKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LsmMaintenanceOperation {
    AdmitRunPublication,
    AdmitReplay,
    AdmitCompaction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LsmMaintenanceDisposition {
    Admitted,
    Denied(LsmMaintenanceAdmissionDenialKind),
}

impl LsmMaintenanceDisposition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Denied(denial) => denial.as_str(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LsmMaintenanceOwnerCaseId {
    operation: LsmMaintenanceOperation,
    disposition: LsmMaintenanceDisposition,
}

impl LsmMaintenanceOwnerCaseId {
    pub(super) const fn admitted(operation: LsmMaintenanceOperation) -> Self {
        Self {
            operation,
            disposition: LsmMaintenanceDisposition::Admitted,
        }
    }

    pub(super) const fn denied(
        operation: LsmMaintenanceOperation,
        denial: LsmMaintenanceAdmissionDenialKind,
    ) -> Self {
        Self {
            operation,
            disposition: LsmMaintenanceDisposition::Denied(denial),
        }
    }

    pub const fn operation(self) -> LsmMaintenanceOperation {
        self.operation
    }

    pub const fn disposition(self) -> LsmMaintenanceDisposition {
        self.disposition
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmMaintenanceOwnerCaseDeclaration {
    id: LsmMaintenanceOwnerCaseId,
}

impl LsmMaintenanceOwnerCaseDeclaration {
    pub(super) const fn new(id: LsmMaintenanceOwnerCaseId) -> Self {
        Self { id }
    }

    pub const fn id(self) -> LsmMaintenanceOwnerCaseId {
        self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmMaintenanceOwnerCaseObservation {
    id: LsmMaintenanceOwnerCaseId,
}

impl LsmMaintenanceOwnerCaseObservation {
    pub(super) const fn new(id: LsmMaintenanceOwnerCaseId) -> Self {
        Self { id }
    }

    pub const fn id(self) -> LsmMaintenanceOwnerCaseId {
        self.id
    }
}
