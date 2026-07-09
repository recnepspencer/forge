use crate::application::{
    WorthQueryAdmittedWorldBasis, WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput,
    WorthQueryDeclarationLegalityEvidence, WorthQueryDomainEntryMarker,
};
use crate::identity::hash_parts;

use super::review::{
    WorthQueryDeclarationProgressionContract, WorthQueryDeclarationProgressionContractClass,
};

pub struct WorthQueryDeclarationProgressionPayload<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    legality_evidence: WorthQueryDeclarationLegalityEvidence<D, I>,
    world_basis: WorthQueryAdmittedWorldBasis,
    declaration_digest: String,
    support_digest: String,
    legality_digest: String,
    progression_contract: WorthQueryDeclarationProgressionContract,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationProgressionPayload<D, I>
{
    pub(crate) fn new(
        legality_evidence: WorthQueryDeclarationLegalityEvidence<D, I>,
        world_basis: WorthQueryAdmittedWorldBasis,
    ) -> Self {
        let declaration_digest = format!(
            "{:?}",
            legality_evidence
                .canonical_declaration()
                .declaration_digest()
        );
        let support_digest = legality_evidence
            .support_report()
            .support_digest()
            .to_string();
        let legality_digest = legality_evidence.legality_digest().to_string();
        let progression_contract = I::Family::progression_contract(
            world_basis.handle_identity_for_reporting(),
            world_basis.operating_context_identity_digest(),
        );

        Self {
            legality_evidence,
            world_basis,
            declaration_digest,
            support_digest,
            legality_digest,
            progression_contract,
        }
    }

    pub fn legality_evidence(&self) -> &WorthQueryDeclarationLegalityEvidence<D, I> {
        &self.legality_evidence
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.legality_evidence.declaration_family_key()
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.world_basis.handle_identity_for_reporting()
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        self.world_basis.operating_context_identity_digest()
    }

    pub(crate) fn world_basis(&self) -> &WorthQueryAdmittedWorldBasis {
        &self.world_basis
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }

    pub fn legality_digest(&self) -> &str {
        &self.legality_digest
    }

    pub fn progression_contract(&self) -> WorthQueryDeclarationProgressionContract {
        self.progression_contract
    }
}

pub(crate) fn derive_progression_digest<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    payload: &WorthQueryDeclarationProgressionPayload<D, I>,
    class: WorthQueryDeclarationProgressionContractClass,
) -> String {
    hash_parts(&[
        format!("handle:{}", payload.handle_identity_digest()),
        format!(
            "operating_context:{}",
            payload.operating_context_identity_digest()
        ),
        format!("declaration:{}", payload.declaration_digest()),
        format!("family:{}", payload.declaration_family_key()),
        format!("support:{}", payload.support_digest()),
        format!("legality:{}", payload.legality_digest()),
        format!("progression:{class:?}"),
    ])
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> Clone
    for WorthQueryDeclarationProgressionPayload<D, I>
{
    fn clone(&self) -> Self {
        Self {
            legality_evidence: self.legality_evidence.clone(),
            world_basis: self.world_basis.clone(),
            declaration_digest: self.declaration_digest.clone(),
            support_digest: self.support_digest.clone(),
            legality_digest: self.legality_digest.clone(),
            progression_contract: self.progression_contract,
        }
    }
}
