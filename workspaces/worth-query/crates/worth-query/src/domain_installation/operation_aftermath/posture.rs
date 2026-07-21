#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryAftermathPosture {
    Irreversible,
    ProvisionalDiscard,
    ExactInverse {
        operation: crate::domain_installation::WorthQueryDomainOperationIdentity,
        lowering_family: String,
        postcondition: crate::domain_installation::WorthQueryAftermathPostcondition,
    },
    Compensation {
        operation: crate::domain_installation::WorthQueryDomainOperationIdentity,
        postcondition: crate::domain_installation::WorthQueryAftermathPostcondition,
    },
    RebuildRequired {
        recovery_family: String,
    },
    DeclarationIncomplete,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryAftermathKind {
    ExactInverse,
    Compensation,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryAftermathCounters {
    pub runtime_authority_checks: usize,
    pub installation_generation_checks: usize,
    pub basis_checks: usize,
    pub candidate_operation_checks: usize,
    pub candidate_lowering_checks: usize,
    pub effect_receipt_checks: usize,
    pub postcondition_checks: usize,
    pub candidate_effect_receipt_checks: usize,
    pub postcondition_verification_checks: usize,
    pub execution_contacts: usize,
    pub unrelated_trace_scans: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryAftermathAdmissionDenial {
    OriginalInstallationStale,
    CandidateInstallationStale,
    ForeignRuntime,
    InstallationGenerationMismatch,
    BasisMismatch,
    NoExecutedEffects,
    DeclarationIncomplete,
    Irreversible,
    ProvisionalDiscardOnly,
    RebuildRequired,
    CandidateOperationMismatch,
    CandidateLoweringMismatch,
    InvalidPostcondition,
}
