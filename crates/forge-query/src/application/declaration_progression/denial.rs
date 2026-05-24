use forge_proof::ProofOutcomeKind;

use crate::application::{
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationLegalityEvidence, ForgeQueryDomainEntryMarker,
};

use super::payload::{derive_progression_digest, ForgeQueryDeclarationProgressionPayload};
use super::review::{
    ForgeQueryDeclarationProgressionContract, ForgeQueryDeclarationProgressionContractClass,
    ForgeQueryDeclarationProgressionOutcomeView,
};

macro_rules! define_payload_wrapper {
    ($name:ident, $class:expr, $kind:expr) => {
        pub struct $name<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> {
            payload: ForgeQueryDeclarationProgressionPayload<D, I>,
            progression_digest: String,
        }

        impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> $name<D, I> {
            pub(crate) fn new(payload: ForgeQueryDeclarationProgressionPayload<D, I>) -> Self {
                let progression_digest = derive_progression_digest(&payload, $class);
                Self {
                    payload,
                    progression_digest,
                }
            }

            pub fn legality_evidence(&self) -> &ForgeQueryDeclarationLegalityEvidence<D, I> {
                self.payload.legality_evidence()
            }

            pub fn support_report(
                &self,
            ) -> &crate::application::ForgeQueryDeclarationFamilySupportReport<D, I::Family> {
                self.legality_evidence().support_report()
            }

            pub fn legality_contract(&self) -> ForgeQueryDeclarationLegalityContract {
                self.legality_evidence().legality_contract()
            }

            pub fn progression_contract(&self) -> ForgeQueryDeclarationProgressionContract {
                self.payload.progression_contract()
            }

            pub fn declaration_family_key(&self) -> &'static str {
                self.payload.declaration_family_key()
            }

            pub fn progression_digest(&self) -> &str {
                &self.progression_digest
            }

            pub fn outcome(&self) -> ForgeQueryDeclarationProgressionOutcomeView {
                ForgeQueryDeclarationProgressionOutcomeView::new($kind)
            }

            pub(crate) fn operating_context_identity_digest(&self) -> &str {
                self.payload.operating_context_identity_digest()
            }
        }
    };
}

define_payload_wrapper!(
    ForgeQueryDeclarationProgressionDeferred,
    ForgeQueryDeclarationProgressionContractClass::Deferred,
    ProofOutcomeKind::Deferred
);
define_payload_wrapper!(
    ForgeQueryDeclarationProgressionDenied,
    ForgeQueryDeclarationProgressionContractClass::Denied,
    ProofOutcomeKind::Denied
);
define_payload_wrapper!(
    ForgeQueryDeclarationProgressionFailed,
    ForgeQueryDeclarationProgressionContractClass::Failed,
    ProofOutcomeKind::Failed
);
