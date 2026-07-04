use crate::evidence::{
    evidence_authority_binding, UiEvidenceAuthorityBinding, UiEvidenceAuthorityGeneration,
    UiEvidenceAuthorityKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationEvidenceAuthoritySource {
    SelectedObligationSet,
    DispatchPlan,
    ObligationVerdict,
    AdmissionReport,
}

impl UiObligationEvidenceAuthoritySource {
    pub(crate) fn into_public_binding(
        self,
        authority_digest: u64,
        authority_generation: UiEvidenceAuthorityGeneration,
    ) -> UiEvidenceAuthorityBinding {
        evidence_authority_binding(
            self.into_authority_kind(),
            authority_digest,
            authority_generation,
            None,
        )
    }

    fn into_authority_kind(self) -> UiEvidenceAuthorityKind {
        match self {
            Self::SelectedObligationSet | Self::DispatchPlan | Self::ObligationVerdict => {
                UiEvidenceAuthorityKind::ObligationAuthority
            }
            Self::AdmissionReport => UiEvidenceAuthorityKind::AdmissionReport,
        }
    }
}
