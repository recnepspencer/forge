use worth_proof::ProofOutcomeKind;

use crate::application::{
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract,
    WorthQueryDeclarationLegalityEvidence, WorthQueryDomainEntryMarker,
};

use super::payload::{derive_progression_digest, WorthQueryDeclarationProgressionPayload};
use super::review::{
    WorthQueryDeclarationProgressionContract, WorthQueryDeclarationProgressionContractClass,
    WorthQueryDeclarationProgressionOutcomeView,
};

macro_rules! define_payload_wrapper {
    ($name:ident, $class:expr, $kind:expr) => {
        pub struct $name<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> {
            payload: WorthQueryDeclarationProgressionPayload<D, I>,
            progression_digest: String,
        }

        impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> $name<D, I> {
            pub(crate) fn new(payload: WorthQueryDeclarationProgressionPayload<D, I>) -> Self {
                let progression_digest = derive_progression_digest(&payload, $class);
                Self {
                    payload,
                    progression_digest,
                }
            }

            pub fn legality_evidence(&self) -> &WorthQueryDeclarationLegalityEvidence<D, I> {
                self.payload.legality_evidence()
            }

            pub fn support_report(
                &self,
            ) -> &crate::application::WorthQueryDeclarationFamilySupportReport<D, I::Family> {
                self.legality_evidence().support_report()
            }

            pub fn legality_contract(&self) -> WorthQueryDeclarationLegalityContract {
                self.legality_evidence().legality_contract()
            }

            pub fn progression_contract(&self) -> WorthQueryDeclarationProgressionContract {
                self.payload.progression_contract()
            }

            pub fn declaration_family_key(&self) -> &'static str {
                self.payload.declaration_family_key()
            }

            pub fn progression_digest(&self) -> &str {
                &self.progression_digest
            }

            pub fn outcome(&self) -> WorthQueryDeclarationProgressionOutcomeView {
                WorthQueryDeclarationProgressionOutcomeView::new($kind)
            }

            pub(crate) fn operating_context_identity_digest(&self) -> &str {
                self.payload.operating_context_identity_digest()
            }
        }
    };
}

define_payload_wrapper!(
    WorthQueryDeclarationProgressionDeferred,
    WorthQueryDeclarationProgressionContractClass::Deferred,
    ProofOutcomeKind::Deferred
);
define_payload_wrapper!(
    WorthQueryDeclarationProgressionDenied,
    WorthQueryDeclarationProgressionContractClass::Denied,
    ProofOutcomeKind::Denied
);
define_payload_wrapper!(
    WorthQueryDeclarationProgressionFailed,
    WorthQueryDeclarationProgressionContractClass::Failed,
    ProofOutcomeKind::Failed
);
