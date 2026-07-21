use super::digest::fold_text;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiChildRangePlanMeaning {
    owner_identity: String,
    child_identities: Vec<String>,
}

impl WorthUiChildRangePlanMeaning {
    pub(crate) fn new(owner_identity: String, child_identities: Vec<String>) -> Self {
        Self {
            owner_identity,
            child_identities,
        }
    }

    pub(crate) fn child_identities(&self) -> &[String] {
        &self.child_identities
    }

    pub(crate) fn semantic_digest(&self) -> u64 {
        self.child_identities.iter().fold(
            fold_text(0x6368_696c_6472_616e, &self.owner_identity),
            |digest, identity| fold_text(digest, identity),
        )
    }
}
