use forge_proof::{
    Admitted, AssumptionBasis, CurrentValidity, FreshnessScopedBasis, ProofOutcomeKind, Recipe,
    RecipeStageDxExt, RecipeStageKind,
};

use crate::application::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationLegalityEvidence, ForgeQueryDomainEntryMarker,
};
use crate::target_binding::ForgeQueryAdmittedDeclarationProgressionBindingTarget;

use super::payload::{derive_progression_digest, ForgeQueryDeclarationProgressionPayload};
use super::review::{
    ForgeQueryDeclarationProgressionContractClass, ForgeQueryDeclarationProgressionOutcomeView,
};

pub struct ForgeQueryAdmittedDeclarationProgression<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    recipe: Recipe<
        Admitted,
        ForgeQueryDeclarationProgressionPayload<D, I>,
        FreshnessScopedBasis<
            CurrentValidity,
            AssumptionBasis<super::review::ForgeQueryDeclarationProgressionBasis>,
        >,
    >,
    progression_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryAdmittedDeclarationProgression<D, I>
{
    pub(crate) fn new(
        recipe: Recipe<
            Admitted,
            ForgeQueryDeclarationProgressionPayload<D, I>,
            FreshnessScopedBasis<
                CurrentValidity,
                AssumptionBasis<super::review::ForgeQueryDeclarationProgressionBasis>,
            >,
        >,
    ) -> Self {
        let progression_digest = derive_progression_digest(
            recipe.payload(),
            ForgeQueryDeclarationProgressionContractClass::Admitted,
        );
        Self {
            recipe,
            progression_digest,
        }
    }

    pub fn legality_evidence(&self) -> &ForgeQueryDeclarationLegalityEvidence<D, I> {
        self.recipe.payload().legality_evidence()
    }

    pub fn canonical_declaration(
        &self,
    ) -> &crate::application::ForgeQueryCanonicalDeclarationArtifact<D, I> {
        self.legality_evidence().canonical_declaration()
    }

    pub fn support_report(
        &self,
    ) -> &crate::application::ForgeQueryDeclarationFamilySupportReport<D, I::Family> {
        self.legality_evidence().support_report()
    }

    pub fn legality_contract(&self) -> ForgeQueryDeclarationLegalityContract {
        self.legality_evidence().legality_contract()
    }

    pub fn aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
        self.legality_evidence().aspect_contract()
    }

    pub fn reviewed_aspect_coverage(&self) -> &ForgeQueryDeclarationAspectCoverage {
        self.legality_evidence().reviewed_aspect_coverage()
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.legality_evidence().declaration_family_key()
    }

    pub fn progression_digest(&self) -> &str {
        &self.progression_digest
    }

    pub fn binding_target(&self) -> ForgeQueryAdmittedDeclarationProgressionBindingTarget {
        ForgeQueryAdmittedDeclarationProgressionBindingTarget::for_progressed(self)
    }

    pub fn outcome(&self) -> ForgeQueryDeclarationProgressionOutcomeView {
        ForgeQueryDeclarationProgressionOutcomeView::new(ProofOutcomeKind::Success)
    }

    pub fn stage(&self) -> RecipeStageKind {
        self.recipe.stage()
    }

    pub(crate) fn operating_context_identity_digest(&self) -> &str {
        self.recipe.payload().operating_context_identity_digest()
    }
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> Clone
    for ForgeQueryAdmittedDeclarationProgression<D, I>
{
    fn clone(&self) -> Self {
        Self {
            recipe: self.recipe.clone(),
            progression_digest: self.progression_digest.clone(),
        }
    }
}
