use crate::source::WorthUiBoundCommandReference;

use super::digest::fold_text;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiCommandPlanMeaning {
    owner_identity: String,
    reference: WorthUiBoundCommandReference,
}

impl WorthUiCommandPlanMeaning {
    pub(crate) fn new(owner_identity: String, reference: WorthUiBoundCommandReference) -> Self {
        Self {
            owner_identity,
            reference,
        }
    }

    pub(crate) fn semantic_digest(&self) -> u64 {
        let descriptor = self.reference.descriptor();
        let digest = fold_text(0x636f_6d6d_616e_6401, &self.owner_identity);
        let digest = fold_text(digest, descriptor.id().as_str());
        let digest = fold_text(digest, descriptor.label());
        let digest = descriptor
            .description()
            .map_or(digest, |value| fold_text(digest, value));
        let digest = descriptor
            .default_shortcut()
            .map_or(digest, |value| fold_text(digest, &value.digest_basis()));
        descriptor
            .route()
            .map_or(digest, |value| fold_text(digest, &value.digest_basis()))
    }

    pub(crate) fn reference(&self) -> &WorthUiBoundCommandReference {
        &self.reference
    }
}
