use super::{
    WorthQueryDomainInstallationDenial, WorthQueryDomainPackageAdmissionDenial,
    WorthQueryDomainPackageValidationDenial,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainPackageInstallationError {
    Validation(WorthQueryDomainPackageValidationDenial),
    Admission(WorthQueryDomainPackageAdmissionDenial),
    Installation(WorthQueryDomainInstallationDenial),
}

impl WorthQueryDomainPackageInstallationError {
    pub fn validation_denial(&self) -> Option<&WorthQueryDomainPackageValidationDenial> {
        match self {
            Self::Validation(denial) => Some(denial),
            Self::Admission(_) | Self::Installation(_) => None,
        }
    }

    pub fn admission_denial(&self) -> Option<&WorthQueryDomainPackageAdmissionDenial> {
        match self {
            Self::Admission(denial) => Some(denial),
            Self::Validation(_) | Self::Installation(_) => None,
        }
    }

    pub fn installation_denial(&self) -> Option<&WorthQueryDomainInstallationDenial> {
        match self {
            Self::Installation(denial) => Some(denial),
            Self::Validation(_) | Self::Admission(_) => None,
        }
    }
}

impl std::fmt::Display for WorthQueryDomainPackageInstallationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation(denial) => {
                write!(formatter, "domain package validation denied: {denial}")
            }
            Self::Admission(denial) => {
                write!(formatter, "domain package admission denied: {denial}")
            }
            Self::Installation(denial) => denial.fmt(formatter),
        }
    }
}

impl std::error::Error for WorthQueryDomainPackageInstallationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(denial) => Some(denial),
            Self::Admission(denial) => Some(denial),
            Self::Installation(denial) => Some(denial),
        }
    }
}
