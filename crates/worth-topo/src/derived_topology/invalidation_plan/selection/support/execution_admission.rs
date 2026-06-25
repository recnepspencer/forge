use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DerivedInvalidationExecutionAdmission {
    Admitted,
    Denied,
}

impl DerivedInvalidationExecutionAdmission {
    pub const fn from_denial_count(denial_count: usize) -> Self {
        if denial_count == 0 {
            Self::Admitted
        } else {
            Self::Denied
        }
    }

    pub const fn is_admitted(self) -> bool {
        matches!(self, Self::Admitted)
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Denied => "denied",
        }
    }
}
