use worth_proof::{
    ProofOutcomeKind, RebindRequiredBasis, Recipe, RecipeStageDxExt, RecipeStageKind, Resolved,
};

use crate::application::{
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract,
    WorthQueryDeclarationLegalityEvidence, WorthQueryDomainEntryMarker,
};

use super::payload::{derive_progression_digest, WorthQueryDeclarationProgressionPayload};
use super::review::{
    WorthQueryDeclarationProgressionBasis, WorthQueryDeclarationProgressionContractClass,
    WorthQueryDeclarationProgressionOutcomeView,
};

pub struct WorthQueryDeclarationProgressionRebindRequired<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    recipe: Recipe<
        Resolved,
        WorthQueryDeclarationProgressionPayload<D, I>,
        RebindRequiredBasis<WorthQueryDeclarationProgressionBasis>,
    >,
    progression_digest: String,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationProgressionRebindRequired<D, I>
{
    pub(crate) fn new(
        recipe: Recipe<
            Resolved,
            WorthQueryDeclarationProgressionPayload<D, I>,
            RebindRequiredBasis<WorthQueryDeclarationProgressionBasis>,
        >,
    ) -> Self {
        let progression_digest = derive_progression_digest(
            recipe.payload(),
            WorthQueryDeclarationProgressionContractClass::RebindRequired,
        );
        Self {
            recipe,
            progression_digest,
        }
    }

    pub fn legality_evidence(&self) -> &WorthQueryDeclarationLegalityEvidence<D, I> {
        self.recipe.payload().legality_evidence()
    }

    pub fn support_report(
        &self,
    ) -> &crate::application::WorthQueryDeclarationFamilySupportReport<D, I::Family> {
        self.legality_evidence().support_report()
    }

    pub fn legality_contract(&self) -> WorthQueryDeclarationLegalityContract {
        self.legality_evidence().legality_contract()
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.legality_evidence().declaration_family_key()
    }

    pub fn progression_digest(&self) -> &str {
        &self.progression_digest
    }

    pub fn outcome(&self) -> WorthQueryDeclarationProgressionOutcomeView {
        WorthQueryDeclarationProgressionOutcomeView::new(ProofOutcomeKind::RebindRequired)
    }

    pub fn stage(&self) -> RecipeStageKind {
        self.recipe.stage()
    }

    pub(crate) fn operating_context_identity_digest(&self) -> &str {
        self.recipe.payload().operating_context_identity_digest()
    }
}
