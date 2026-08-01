#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryAuthoredMutationAdmissionDenial {
    AspectContracts(super::super::WorthQueryAspectContractRegistrationDenial),
    MutationContract(super::super::WorthQueryMutationContractDenial),
}

impl From<super::super::WorthQueryAspectContractRegistrationDenial>
    for WorthQueryAuthoredMutationAdmissionDenial
{
    fn from(denial: super::super::WorthQueryAspectContractRegistrationDenial) -> Self {
        Self::AspectContracts(denial)
    }
}

impl From<super::super::WorthQueryMutationContractDenial>
    for WorthQueryAuthoredMutationAdmissionDenial
{
    fn from(denial: super::super::WorthQueryMutationContractDenial) -> Self {
        Self::MutationContract(denial)
    }
}
