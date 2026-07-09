use crate::application::{
    WorthQueryAdmittedDeclarationProgression, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationAspectCoverageBasis,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityChecked,
    WorthQueryDeclarationLegalityDenial, WorthQueryDeclarationLegalityEvidence,
    WorthQueryDeclarationProgressionChecked, WorthQueryDeclarationProgressionDeferred,
    WorthQueryDeclarationProgressionDenied, WorthQueryDeclarationProgressionFailed,
    WorthQueryDeclarationProgressionRebindRequired, WorthQueryDeclarationProgressionStale,
    WorthQueryDomainEntryMarker,
};
use crate::WorthQueryEvidenceIdentity;

use super::class::WorthQueryDeclarationFoundationalEvidenceClass;

pub enum WorthQueryDeclarationFoundationalEvidenceInput<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    LegalityEvidence(WorthQueryDeclarationLegalityEvidence<D, I>),
    LegalityDenial(WorthQueryDeclarationLegalityDenial<D, I>),
    AdmittedProgression(WorthQueryAdmittedDeclarationProgression<D, I>),
    ProgressionDeferred(WorthQueryDeclarationProgressionDeferred<D, I>),
    ProgressionDenied(WorthQueryDeclarationProgressionDenied<D, I>),
    ProgressionStale(WorthQueryDeclarationProgressionStale<D, I>),
    ProgressionRebindRequired(WorthQueryDeclarationProgressionRebindRequired<D, I>),
    ProgressionFailed(WorthQueryDeclarationProgressionFailed<D, I>),
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationFoundationalEvidenceInput<D, I>
{
    pub fn legality_evidence(evidence: WorthQueryDeclarationLegalityEvidence<D, I>) -> Self {
        Self::LegalityEvidence(evidence)
    }

    pub fn legality_denial(denial: WorthQueryDeclarationLegalityDenial<D, I>) -> Self {
        Self::LegalityDenial(denial)
    }

    pub fn legality_checked(checked: WorthQueryDeclarationLegalityChecked<D, I>) -> Self {
        match checked {
            WorthQueryDeclarationLegalityChecked::Legal(evidence) => {
                Self::LegalityEvidence(evidence)
            }
            WorthQueryDeclarationLegalityChecked::Illegal(denial) => Self::LegalityDenial(denial),
        }
    }

    pub fn admitted_progression(
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
    ) -> Self {
        Self::AdmittedProgression(progressed)
    }

    pub fn progression_checked(checked: WorthQueryDeclarationProgressionChecked<D, I>) -> Self {
        match checked {
            WorthQueryDeclarationProgressionChecked::Admitted(progressed) => {
                Self::AdmittedProgression(progressed)
            }
            WorthQueryDeclarationProgressionChecked::Deferred(progress) => {
                Self::ProgressionDeferred(progress)
            }
            WorthQueryDeclarationProgressionChecked::Denied(progress) => {
                Self::ProgressionDenied(progress)
            }
            WorthQueryDeclarationProgressionChecked::Stale(progress) => {
                Self::ProgressionStale(progress)
            }
            WorthQueryDeclarationProgressionChecked::RebindRequired(progress) => {
                Self::ProgressionRebindRequired(progress)
            }
            WorthQueryDeclarationProgressionChecked::Failed(progress) => {
                Self::ProgressionFailed(progress)
            }
        }
    }

    pub(crate) fn class(&self) -> WorthQueryDeclarationFoundationalEvidenceClass {
        match self {
            Self::LegalityEvidence(_) => {
                WorthQueryDeclarationFoundationalEvidenceClass::LegalityAdmitted
            }
            Self::LegalityDenial(_) => {
                WorthQueryDeclarationFoundationalEvidenceClass::LegalityDenied
            }
            Self::AdmittedProgression(_) => {
                WorthQueryDeclarationFoundationalEvidenceClass::ProgressionAdmitted
            }
            Self::ProgressionDeferred(_) => {
                WorthQueryDeclarationFoundationalEvidenceClass::ProgressionDeferred
            }
            Self::ProgressionDenied(_) => {
                WorthQueryDeclarationFoundationalEvidenceClass::ProgressionDenied
            }
            Self::ProgressionStale(_) => {
                WorthQueryDeclarationFoundationalEvidenceClass::ProgressionStale
            }
            Self::ProgressionRebindRequired(_) => {
                WorthQueryDeclarationFoundationalEvidenceClass::ProgressionRebindRequired
            }
            Self::ProgressionFailed(_) => {
                WorthQueryDeclarationFoundationalEvidenceClass::ProgressionFailed
            }
        }
    }

    pub(crate) fn canonical_declaration(
        &self,
    ) -> &crate::application::WorthQueryCanonicalDeclarationArtifact<D, I> {
        match self {
            Self::LegalityEvidence(evidence) => evidence.canonical_declaration(),
            Self::LegalityDenial(denial) => denial.canonical_declaration(),
            Self::AdmittedProgression(progressed) => progressed.canonical_declaration(),
            Self::ProgressionDeferred(progress) => {
                progress.legality_evidence().canonical_declaration()
            }
            Self::ProgressionDenied(progress) => {
                progress.legality_evidence().canonical_declaration()
            }
            Self::ProgressionStale(progress) => {
                progress.legality_evidence().canonical_declaration()
            }
            Self::ProgressionRebindRequired(progress) => {
                progress.legality_evidence().canonical_declaration()
            }
            Self::ProgressionFailed(progress) => {
                progress.legality_evidence().canonical_declaration()
            }
        }
    }

    pub(crate) fn support_report(
        &self,
    ) -> &crate::application::WorthQueryDeclarationFamilySupportReport<D, I::Family> {
        match self {
            Self::LegalityEvidence(evidence) => evidence.support_report(),
            Self::LegalityDenial(denial) => denial.support_report(),
            Self::AdmittedProgression(progressed) => progressed.support_report(),
            Self::ProgressionDeferred(progress) => progress.support_report(),
            Self::ProgressionDenied(progress) => progress.support_report(),
            Self::ProgressionStale(progress) => progress.support_report(),
            Self::ProgressionRebindRequired(progress) => progress.support_report(),
            Self::ProgressionFailed(progress) => progress.support_report(),
        }
    }

    pub(crate) fn declaration_family_key(&self) -> &'static str {
        self.canonical_declaration().declaration_family_key()
    }

    pub(crate) fn handle_identity_digest(&self) -> &str {
        self.canonical_declaration().handle_identity_digest()
    }

    pub(crate) fn handle_identity(&self) -> Option<&WorthQueryEvidenceIdentity> {
        match self {
            Self::LegalityEvidence(evidence) => Some(evidence.world_basis().handle_identity()),
            Self::LegalityDenial(_) => None,
            Self::AdmittedProgression(progressed) => {
                Some(progressed.retained_world_basis().handle_identity())
            }
            Self::ProgressionDeferred(progress) => {
                Some(progress.legality_evidence().world_basis().handle_identity())
            }
            Self::ProgressionDenied(progress) => {
                Some(progress.legality_evidence().world_basis().handle_identity())
            }
            Self::ProgressionStale(progress) => {
                Some(progress.legality_evidence().world_basis().handle_identity())
            }
            Self::ProgressionRebindRequired(progress) => {
                Some(progress.legality_evidence().world_basis().handle_identity())
            }
            Self::ProgressionFailed(progress) => {
                Some(progress.legality_evidence().world_basis().handle_identity())
            }
        }
    }

    pub(crate) fn operating_context_identity_digest(&self) -> &str {
        match self {
            Self::LegalityEvidence(evidence) => evidence.operating_context_identity_digest(),
            Self::LegalityDenial(denial) => denial.operating_context_identity_digest(),
            Self::AdmittedProgression(progressed) => progressed.operating_context_identity_digest(),
            Self::ProgressionDeferred(progress) => progress.operating_context_identity_digest(),
            Self::ProgressionDenied(progress) => progress.operating_context_identity_digest(),
            Self::ProgressionStale(progress) => progress.operating_context_identity_digest(),
            Self::ProgressionRebindRequired(progress) => {
                progress.operating_context_identity_digest()
            }
            Self::ProgressionFailed(progress) => progress.operating_context_identity_digest(),
        }
    }

    pub(crate) fn declaration_digest_string(&self) -> String {
        format!("{:?}", self.canonical_declaration().declaration_digest())
    }

    pub(crate) fn support_digest(&self) -> &str {
        self.support_report().support_digest()
    }

    pub(crate) fn aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
        self.support_report().aspect_contract()
    }

    pub(crate) fn aspect_coverage(&self) -> WorthQueryDeclarationAspectCoverage {
        match self {
            Self::LegalityEvidence(evidence) => evidence.reviewed_aspect_coverage().clone(),
            Self::LegalityDenial(denial) => denial.support_report().aspect_coverage().clone(),
            Self::AdmittedProgression(progressed) => progressed.reviewed_aspect_coverage().clone(),
            Self::ProgressionDeferred(progress) => progress
                .legality_evidence()
                .reviewed_aspect_coverage()
                .clone(),
            Self::ProgressionDenied(progress) => progress
                .legality_evidence()
                .reviewed_aspect_coverage()
                .clone(),
            Self::ProgressionStale(progress) => progress
                .legality_evidence()
                .reviewed_aspect_coverage()
                .clone(),
            Self::ProgressionRebindRequired(progress) => progress
                .legality_evidence()
                .reviewed_aspect_coverage()
                .clone(),
            Self::ProgressionFailed(progress) => progress
                .legality_evidence()
                .reviewed_aspect_coverage()
                .clone(),
        }
    }

    pub(crate) fn aspect_coverage_basis(&self) -> WorthQueryDeclarationAspectCoverageBasis {
        match self {
            Self::LegalityEvidence(_)
            | Self::AdmittedProgression(_)
            | Self::ProgressionDeferred(_)
            | Self::ProgressionDenied(_)
            | Self::ProgressionStale(_)
            | Self::ProgressionRebindRequired(_)
            | Self::ProgressionFailed(_) => {
                WorthQueryDeclarationAspectCoverageBasis::ReviewedRetainedCoverage
            }
            Self::LegalityDenial(_) => {
                WorthQueryDeclarationAspectCoverageBasis::SupportReportedCoverage
            }
        }
    }

    pub(crate) fn legality_digest(&self) -> Option<&str> {
        match self {
            Self::LegalityEvidence(evidence) => Some(evidence.legality_digest()),
            Self::LegalityDenial(_) => None,
            Self::AdmittedProgression(progressed) => {
                Some(progressed.legality_evidence().legality_digest())
            }
            Self::ProgressionDeferred(progress) => {
                Some(progress.legality_evidence().legality_digest())
            }
            Self::ProgressionDenied(progress) => {
                Some(progress.legality_evidence().legality_digest())
            }
            Self::ProgressionStale(progress) => {
                Some(progress.legality_evidence().legality_digest())
            }
            Self::ProgressionRebindRequired(progress) => {
                Some(progress.legality_evidence().legality_digest())
            }
            Self::ProgressionFailed(progress) => {
                Some(progress.legality_evidence().legality_digest())
            }
        }
    }

    pub(crate) fn legality_contract(
        &self,
    ) -> crate::application::WorthQueryDeclarationLegalityContract {
        match self {
            Self::LegalityEvidence(evidence) => evidence.legality_contract(),
            Self::LegalityDenial(denial) => denial.legality_contract(),
            Self::AdmittedProgression(progressed) => progressed.legality_contract(),
            Self::ProgressionDeferred(progress) => progress.legality_contract(),
            Self::ProgressionDenied(progress) => progress.legality_contract(),
            Self::ProgressionStale(progress) => progress.legality_contract(),
            Self::ProgressionRebindRequired(progress) => progress.legality_contract(),
            Self::ProgressionFailed(progress) => progress.legality_contract(),
        }
    }

    pub(crate) fn progression_digest(&self) -> Option<&str> {
        match self {
            Self::LegalityEvidence(_) | Self::LegalityDenial(_) => None,
            Self::AdmittedProgression(progressed) => Some(progressed.progression_digest()),
            Self::ProgressionDeferred(progress) => Some(progress.progression_digest()),
            Self::ProgressionDenied(progress) => Some(progress.progression_digest()),
            Self::ProgressionStale(progress) => Some(progress.progression_digest()),
            Self::ProgressionRebindRequired(progress) => Some(progress.progression_digest()),
            Self::ProgressionFailed(progress) => Some(progress.progression_digest()),
        }
    }

    pub(crate) fn progression_contract(
        &self,
    ) -> Option<crate::application::WorthQueryDeclarationProgressionContract> {
        match self {
            Self::LegalityEvidence(_) | Self::LegalityDenial(_) => None,
            Self::AdmittedProgression(_) => None,
            Self::ProgressionDeferred(progress) => Some(progress.progression_contract()),
            Self::ProgressionDenied(progress) => Some(progress.progression_contract()),
            Self::ProgressionFailed(progress) => Some(progress.progression_contract()),
            Self::ProgressionStale(_) | Self::ProgressionRebindRequired(_) => None,
        }
    }
}
