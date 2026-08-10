#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IncrementalMaintenanceClass {
    Incremental,
    RefreshAdmitted,
    Forbidden,
}

impl IncrementalMaintenanceClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Incremental => "incremental",
            Self::RefreshAdmitted => "refresh_admitted",
            Self::Forbidden => "forbidden",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IncrementalPatchEligibility {
    maintenance_class: IncrementalMaintenanceClass,
    reason: String,
}

impl IncrementalPatchEligibility {
    pub fn maintenance_class(&self) -> &IncrementalMaintenanceClass {
        &self.maintenance_class
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn incremental(reason: impl Into<String>) -> Self {
        Self {
            maintenance_class: IncrementalMaintenanceClass::Incremental,
            reason: reason.into(),
        }
    }

    pub fn refresh_admitted(reason: impl Into<String>) -> Self {
        Self {
            maintenance_class: IncrementalMaintenanceClass::RefreshAdmitted,
            reason: reason.into(),
        }
    }

    pub fn forbidden(reason: impl Into<String>) -> Self {
        Self {
            maintenance_class: IncrementalMaintenanceClass::Forbidden,
            reason: reason.into(),
        }
    }
}
