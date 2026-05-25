use crate::application::{
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityEvidence, ForgeQueryDomainEntryMarker,
};
use crate::identity::hash_parts;

use super::review::{
    ForgeQueryDeclarationProgressionContract, ForgeQueryDeclarationProgressionContractClass,
};

pub struct ForgeQueryDeclarationProgressionPayload<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    legality_evidence: ForgeQueryDeclarationLegalityEvidence<D, I>,
    handle_identity_digest: String,
    operating_context_identity_digest: String,
    declaration_digest: String,
    support_digest: String,
    legality_digest: String,
    progression_contract: ForgeQueryDeclarationProgressionContract,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationProgressionPayload<D, I>
{
    pub(crate) fn new(
        legality_evidence: ForgeQueryDeclarationLegalityEvidence<D, I>,
        operating_context_identity_digest: String,
    ) -> Self {
        let handle_identity_digest = legality_evidence
            .canonical_declaration()
            .handle_identity_digest()
            .to_string();
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
            &handle_identity_digest,
            &operating_context_identity_digest,
        );

        Self {
            legality_evidence,
            handle_identity_digest,
            operating_context_identity_digest,
            declaration_digest,
            support_digest,
            legality_digest,
            progression_contract,
        }
    }

    pub fn legality_evidence(&self) -> &ForgeQueryDeclarationLegalityEvidence<D, I> {
        &self.legality_evidence
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.legality_evidence.declaration_family_key()
    }

    pub fn handle_identity_digest(&self) -> &str {
        &self.handle_identity_digest
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }

    pub fn legality_digest(&self) -> &str {
        &self.legality_digest
    }

    pub fn progression_contract(&self) -> ForgeQueryDeclarationProgressionContract {
        self.progression_contract
    }
}

pub(crate) fn derive_progression_digest<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    payload: &ForgeQueryDeclarationProgressionPayload<D, I>,
    class: ForgeQueryDeclarationProgressionContractClass,
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
