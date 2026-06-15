use crate::evidence::HarnessEvidenceValidationDenial;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HarnessHonestyDenial {
    AppLocalShellStateInjection,
    EvidenceValidation(HarnessEvidenceValidationDenial),
}

impl From<HarnessEvidenceValidationDenial> for HarnessHonestyDenial {
    fn from(denial: HarnessEvidenceValidationDenial) -> Self {
        Self::EvidenceValidation(denial)
    }
}
