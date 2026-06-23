use crate::application::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationAspectCoverageBasis,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityChecked,
    ForgeQueryDeclarationLegalityDenial, ForgeQueryDeclarationLegalityEvidence,
    ForgeQueryDeclarationProgressionChecked, ForgeQueryDeclarationProgressionDeferred,
    ForgeQueryDeclarationProgressionDenied, ForgeQueryDeclarationProgressionFailed,
    ForgeQueryDeclarationProgressionRebindRequired, ForgeQueryDeclarationProgressionStale,
    ForgeQueryDomainEntryMarker,
};
use crate::ForgeQueryEvidenceIdentity;

use super::class::ForgeQueryDeclarationFoundationalEvidenceClass;

pub enum ForgeQueryDeclarationFoundationalEvidenceInput<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    LegalityEvidence(ForgeQueryDeclarationLegalityEvidence<D, I>),
    LegalityDenial(ForgeQueryDeclarationLegalityDenial<D, I>),
    AdmittedProgression(ForgeQueryAdmittedDeclarationProgression<D, I>),
    ProgressionDeferred(ForgeQueryDeclarationProgressionDeferred<D, I>),
    ProgressionDenied(ForgeQueryDeclarationProgressionDenied<D, I>),
    ProgressionStale(ForgeQueryDeclarationProgressionStale<D, I>),
    ProgressionRebindRequired(ForgeQueryDeclarationProgressionRebindRequired<D, I>),
    ProgressionFailed(ForgeQueryDeclarationProgressionFailed<D, I>),
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationFoundationalEvidenceInput<D, I>
{
    pub fn legality_evidence(evidence: ForgeQueryDeclarationLegalityEvidence<D, I>) -> Self {
        Self::LegalityEvidence(evidence)
    }

    pub fn legality_denial(denial: ForgeQueryDeclarationLegalityDenial<D, I>) -> Self {
        Self::LegalityDenial(denial)
    }

    pub fn legality_checked(checked: ForgeQueryDeclarationLegalityChecked<D, I>) -> Self {
        match checked {
            ForgeQueryDeclarationLegalityChecked::Legal(evidence) => {
                Self::LegalityEvidence(evidence)
            }
            ForgeQueryDeclarationLegalityChecked::Illegal(denial) => Self::LegalityDenial(denial),
        }
    }

    pub fn admitted_progression(
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> Self {
        Self::AdmittedProgression(progressed)
    }

    pub fn progression_checked(checked: ForgeQueryDeclarationProgressionChecked<D, I>) -> Self {
        match checked {
            ForgeQueryDeclarationProgressionChecked::Admitted(progressed) => {
                Self::AdmittedProgression(progressed)
            }
            ForgeQueryDeclarationProgressionChecked::Deferred(progress) => {
                Self::ProgressionDeferred(progress)
            }
            ForgeQueryDeclarationProgressionChecked::Denied(progress) => {
                Self::ProgressionDenied(progress)
            }
            ForgeQueryDeclarationProgressionChecked::Stale(progress) => {
                Self::ProgressionStale(progress)
            }
            ForgeQueryDeclarationProgressionChecked::RebindRequired(progress) => {
                Self::ProgressionRebindRequired(progress)
            }
            ForgeQueryDeclarationProgressionChecked::Failed(progress) => {
                Self::ProgressionFailed(progress)
            }
        }
    }

    pub(crate) fn class(&self) -> ForgeQueryDeclarationFoundationalEvidenceClass {
        match self {
            Self::LegalityEvidence(_) => {
                ForgeQueryDeclarationFoundationalEvidenceClass::LegalityAdmitted
            }
            Self::LegalityDenial(_) => {
                ForgeQueryDeclarationFoundationalEvidenceClass::LegalityDenied
            }
            Self::AdmittedProgression(_) => {
                ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionAdmitted
            }
            Self::ProgressionDeferred(_) => {
                ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionDeferred
            }
            Self::ProgressionDenied(_) => {
                ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionDenied
            }
            Self::ProgressionStale(_) => {
                ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionStale
            }
            Self::ProgressionRebindRequired(_) => {
                ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionRebindRequired
            }
            Self::ProgressionFailed(_) => {
                ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionFailed
            }
        }
    }

    pub(crate) fn canonical_declaration(
        &self,
    ) -> &crate::application::ForgeQueryCanonicalDeclarationArtifact<D, I> {
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
    ) -> &crate::application::ForgeQueryDeclarationFamilySupportReport<D, I::Family> {
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

    pub(crate) fn handle_identity(&self) -> Option<&ForgeQueryEvidenceIdentity> {
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

    pub(crate) fn aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
        self.support_report().aspect_contract()
    }

    pub(crate) fn aspect_coverage(&self) -> ForgeQueryDeclarationAspectCoverage {
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

    pub(crate) fn aspect_coverage_basis(&self) -> ForgeQueryDeclarationAspectCoverageBasis {
        match self {
            Self::LegalityEvidence(_)
            | Self::AdmittedProgression(_)
            | Self::ProgressionDeferred(_)
            | Self::ProgressionDenied(_)
            | Self::ProgressionStale(_)
            | Self::ProgressionRebindRequired(_)
            | Self::ProgressionFailed(_) => {
                ForgeQueryDeclarationAspectCoverageBasis::ReviewedRetainedCoverage
            }
            Self::LegalityDenial(_) => {
                ForgeQueryDeclarationAspectCoverageBasis::SupportReportedCoverage
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
    ) -> crate::application::ForgeQueryDeclarationLegalityContract {
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
    ) -> Option<crate::application::ForgeQueryDeclarationProgressionContract> {
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
