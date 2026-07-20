use worth_proof::{
    Admitted, AssumptionBasis, CurrentValidity, FreshnessScopedBasis, ProofOutcomeKind, Recipe,
    RecipeStageDxExt, RecipeStageKind,
};

use crate::application::{
    WorthQueryAdmittedWorldBasis, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationInput,
    WorthQueryDeclarationLegalityContract, WorthQueryDeclarationLegalityEvidence,
    WorthQueryDomainEntryMarker,
};
use crate::target_binding::WorthQueryAdmittedDeclarationProgressionBindingTarget;

use super::payload::{derive_progression_digest, WorthQueryDeclarationProgressionPayload};
use super::review::{
    WorthQueryDeclarationProgressionContractClass, WorthQueryDeclarationProgressionOutcomeView,
};

pub struct WorthQueryAdmittedDeclarationProgression<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    recipe: Recipe<
        Admitted,
        WorthQueryDeclarationProgressionPayload<D, I>,
        FreshnessScopedBasis<
            CurrentValidity,
            AssumptionBasis<super::review::WorthQueryDeclarationProgressionBasis>,
        >,
    >,
    progression_digest: String,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryAdmittedDeclarationProgression<D, I>
{
    pub(crate) fn new(
        recipe: Recipe<
            Admitted,
            WorthQueryDeclarationProgressionPayload<D, I>,
            FreshnessScopedBasis<
                CurrentValidity,
                AssumptionBasis<super::review::WorthQueryDeclarationProgressionBasis>,
            >,
        >,
    ) -> Self {
        let progression_digest = derive_progression_digest(
            recipe.payload(),
            WorthQueryDeclarationProgressionContractClass::Admitted,
        );
        Self {
            recipe,
            progression_digest,
        }
    }

    pub fn legality_evidence(&self) -> &WorthQueryDeclarationLegalityEvidence<D, I> {
        self.recipe.payload().legality_evidence()
    }

    pub fn canonical_declaration(
        &self,
    ) -> &crate::application::WorthQueryCanonicalDeclarationArtifact<D, I> {
        self.legality_evidence().canonical_declaration()
    }

    pub fn support_report(
        &self,
    ) -> &crate::application::WorthQueryDeclarationFamilySupportReport<D, I::Family> {
        self.legality_evidence().support_report()
    }

    pub fn legality_contract(&self) -> WorthQueryDeclarationLegalityContract {
        self.legality_evidence().legality_contract()
    }

    pub fn aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
        self.legality_evidence().aspect_contract()
    }

    pub fn reviewed_aspect_coverage(&self) -> &WorthQueryDeclarationAspectCoverage {
        self.legality_evidence().reviewed_aspect_coverage()
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.legality_evidence().declaration_family_key()
    }

    pub fn progression_digest(&self) -> &str {
        &self.progression_digest
    }

    pub fn binding_target(&self) -> WorthQueryAdmittedDeclarationProgressionBindingTarget {
        WorthQueryAdmittedDeclarationProgressionBindingTarget::for_progressed(self)
    }

    pub fn outcome(&self) -> WorthQueryDeclarationProgressionOutcomeView {
        WorthQueryDeclarationProgressionOutcomeView::new(ProofOutcomeKind::Success)
    }

    pub fn stage(&self) -> RecipeStageKind {
        self.recipe.stage()
    }

    pub(crate) fn operating_context_identity_digest(&self) -> &str {
        self.recipe.payload().operating_context_identity_digest()
    }

    pub(crate) fn retained_world_basis(&self) -> &WorthQueryAdmittedWorldBasis {
        self.recipe.payload().world_basis()
    }
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> Clone
    for WorthQueryAdmittedDeclarationProgression<D, I>
{
    fn clone(&self) -> Self {
        Self {
            recipe: self.recipe.clone(),
            progression_digest: self.progression_digest.clone(),
        }
    }
}
