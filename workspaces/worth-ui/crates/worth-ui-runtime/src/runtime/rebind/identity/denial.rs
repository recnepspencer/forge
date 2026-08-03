use crate::graph::UiGraphFactConsumerKey;
use crate::runtime::WorthUiNodeLifecycleTransition;

#[derive(Debug)]
pub enum UiIdentityLifecycleDenial {
    AmbiguousDeclarationProvenance {
        provenance_digest: u64,
        declaration_count: usize,
    },
    ConflictingDeclarationTransition {
        authored_identity: Box<str>,
        first: WorthUiNodeLifecycleTransition,
        second: WorthUiNodeLifecycleTransition,
    },
    ConflictingConsumerTransition {
        key: UiGraphFactConsumerKey,
    },
    MissingSelectedConsumer {
        key: UiGraphFactConsumerKey,
    },
    SelectedConsumerIdentityMismatch {
        key: UiGraphFactConsumerKey,
    },
    ImpossibleSelectedTransition {
        key: UiGraphFactConsumerKey,
        transition: WorthUiNodeLifecycleTransition,
    },
}
