//! Installed correction-authority axis.

/// Installed correction authority for one aftermath contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InstalledCorrectionAuthority {
    RuntimeAlone,
    RuntimeWithExternalOwner,
    NotCorrectable,
}

impl From<worth_query_declaration::facade::application_aftermath::DeclaredCorrectionAuthority>
    for InstalledCorrectionAuthority
{
    fn from(
        value: worth_query_declaration::facade::application_aftermath::DeclaredCorrectionAuthority,
    ) -> Self {
        match value {
            worth_query_declaration::facade::application_aftermath::DeclaredCorrectionAuthority::RuntimeAlone => {
                Self::RuntimeAlone
            }
            worth_query_declaration::facade::application_aftermath::DeclaredCorrectionAuthority::RuntimeWithExternalOwner => {
                Self::RuntimeWithExternalOwner
            }
            worth_query_declaration::facade::application_aftermath::DeclaredCorrectionAuthority::NotCorrectable => {
                Self::NotCorrectable
            }
        }
    }
}
