use worth_ui_dsl::UiDslSemanticArtifact;

use crate::declaration::{
    UiAspectContractAdmission, UiConsumedAspectContract, UiPublishedAspectContract,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAspectContract {
    published: UiPublishedAspectContract,
    consumed: UiConsumedAspectContract,
}

impl UiAspectContract {
    pub(crate) fn admit(semantic_artifact: &UiDslSemanticArtifact) -> UiAspectContractAdmission {
        match (
            UiPublishedAspectContract::admit(semantic_artifact.published_aspects()),
            UiConsumedAspectContract::admit(semantic_artifact.consumed_aspects()),
        ) {
            (Ok(published), Ok(consumed)) => UiAspectContractAdmission::Admitted(Self {
                published,
                consumed,
            }),
            (Err(denial), _) | (_, Err(denial)) => UiAspectContractAdmission::Denied(denial),
        }
    }

    pub(crate) fn digest_raw(&self) -> u64 {
        self.published.digest_raw() ^ self.consumed.digest_raw().rotate_left(17)
    }

    pub fn published(&self) -> &UiPublishedAspectContract {
        &self.published
    }

    pub fn consumed(&self) -> &UiConsumedAspectContract {
        &self.consumed
    }
}
