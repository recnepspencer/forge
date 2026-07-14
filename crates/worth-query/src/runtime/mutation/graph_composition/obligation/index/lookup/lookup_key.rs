use crate::runtime::{
    WorthQueryGraphTouchLifecycleFamily, WorthQueryGraphTouchReadVerb, WorthQueryMutationFamily,
};
use worth_relational::facade::identity::KindId;

use super::super::super::registration::WorthQueryGraphTouchSelectorClass;
use super::super::selection::WorthQueryGraphObligationOperatingWorldDescriptorKind;
use crate::runtime::{
    WorthQueryAspectMutationOperation, WorthQueryAspectTouch,
    WorthQueryGraphObligationOperatingWorldSelector, WorthQueryGraphTouchSelector,
    WorthQueryMutationTargetCollectionIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(in crate::runtime::mutation::graph_composition::obligation::index) struct WorthQueryGraphObligationCollectionLookupIdentity
{
    label: String,
}

impl WorthQueryGraphObligationCollectionLookupIdentity {
    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn from_collection_identity(
        collection: &WorthQueryMutationTargetCollectionIdentity,
    ) -> Self {
        Self {
            label: collection.as_str().to_string(),
        }
    }

    fn as_str(&self) -> &str {
        &self.label
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(in crate::runtime::mutation::graph_composition::obligation::index) enum WorthQueryGraphObligationTouchLookupKey
{
    AnyGraphTouch,
    Collection(WorthQueryGraphObligationCollectionLookupIdentity),
    RelationKindId(KindId),
    AspectTouch(WorthQueryAspectTouch),
    DeclaredAspectOperation(WorthQueryAspectMutationOperation),
    MutationFamily(WorthQueryMutationFamily),
    LifecycleFamily(WorthQueryGraphTouchLifecycleFamily),
    ReadVerb(WorthQueryGraphTouchReadVerb),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(in crate::runtime::mutation::graph_composition::obligation::index) enum WorthQueryGraphObligationOperatingWorldLookupKey
{
    AnyOperatingWorld,
    AnyCommittedAuthority,
    Preview,
    Branch,
    ConfiguredDomainHandle,
}

impl WorthQueryGraphObligationTouchLookupKey {
    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn from_selector(
        selector: &WorthQueryGraphTouchSelector,
    ) -> Self {
        match selector.selector_class() {
            WorthQueryGraphTouchSelectorClass::Any => Self::AnyGraphTouch,
            WorthQueryGraphTouchSelectorClass::Collection => Self::Collection(
                WorthQueryGraphObligationCollectionLookupIdentity::from_collection_identity(
                    selector
                        .collection_identity()
                        .expect("collection selector has a native collection"),
                ),
            ),
            WorthQueryGraphTouchSelectorClass::RelationKindId => Self::RelationKindId(
                selector
                    .relation_kind_id_value()
                    .expect("relation kind id selector has a native kind id"),
            ),
            WorthQueryGraphTouchSelectorClass::AspectTouch => Self::AspectTouch(
                selector
                    .aspect_touch_value()
                    .expect("aspect touch selector has a native touch")
                    .clone(),
            ),
            WorthQueryGraphTouchSelectorClass::DeclaredAspectOperation => {
                Self::DeclaredAspectOperation(
                    selector
                        .declared_aspect_operation_value()
                        .expect("declared aspect selector has a native operation")
                        .clone(),
                )
            }
            WorthQueryGraphTouchSelectorClass::DeclaredMutationCollection => Self::Collection(
                WorthQueryGraphObligationCollectionLookupIdentity::from_collection_identity(
                    selector
                        .declared_mutation_collection_value()
                        .expect("declared mutation collection selector has a native collection"),
                ),
            ),
            WorthQueryGraphTouchSelectorClass::MutationFamily => Self::MutationFamily(
                selector
                    .mutation_family_value()
                    .expect("mutation family selector has a native family"),
            ),
            WorthQueryGraphTouchSelectorClass::LifecycleFamily => Self::LifecycleFamily(
                selector
                    .lifecycle_family_value()
                    .expect("lifecycle family selector has a native family"),
            ),
            WorthQueryGraphTouchSelectorClass::ReadVerb => Self::ReadVerb(
                selector
                    .read_verb_value()
                    .expect("read verb selector has a native verb"),
            ),
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn as_kind_str(
        &self,
    ) -> &'static str {
        match self {
            Self::AnyGraphTouch => "any-graph-touch",
            Self::Collection(_) => "collection",
            Self::RelationKindId(_) => "relation-kind-id",
            Self::AspectTouch(_) => "aspect-touch",
            Self::DeclaredAspectOperation(_) => "declared-aspect-operation",
            Self::MutationFamily(_) => "mutation-family",
            Self::LifecycleFamily(_) => "lifecycle-family",
            Self::ReadVerb(_) => "read-verb",
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn terminal_value_projection(
        &self,
    ) -> Option<String> {
        match self {
            Self::AnyGraphTouch => None,
            Self::Collection(value) => Some(value.as_str().to_string()),
            Self::AspectTouch(value) => Some(value.admitted_touch_digest_part()),
            Self::DeclaredAspectOperation(value) => {
                Some(terminal_declared_aspect_operation_digest_part(value))
            }
            Self::RelationKindId(value) => Some(value.0.to_string()),
            Self::MutationFamily(value) => Some(value.as_str().to_string()),
            Self::LifecycleFamily(value) => Some(value.as_str().to_string()),
            Self::ReadVerb(value) => Some(value.as_str().to_string()),
        }
    }
}

impl WorthQueryGraphObligationOperatingWorldLookupKey {
    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn from_selector(
        selector: WorthQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        match selector {
            WorthQueryGraphObligationOperatingWorldSelector::AnyCommittedAuthority => {
                Self::AnyCommittedAuthority
            }
            WorthQueryGraphObligationOperatingWorldSelector::Preview => Self::Preview,
            WorthQueryGraphObligationOperatingWorldSelector::Branch => Self::Branch,
            WorthQueryGraphObligationOperatingWorldSelector::ConfiguredDomainHandle => {
                Self::ConfiguredDomainHandle
            }
            WorthQueryGraphObligationOperatingWorldSelector::AnyOperatingWorld => {
                Self::AnyOperatingWorld
            }
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn from_descriptor_kind(
        kind: WorthQueryGraphObligationOperatingWorldDescriptorKind,
    ) -> Self {
        match kind {
            WorthQueryGraphObligationOperatingWorldDescriptorKind::AnyCommittedAuthority => {
                Self::AnyCommittedAuthority
            }
            WorthQueryGraphObligationOperatingWorldDescriptorKind::Preview => Self::Preview,
            WorthQueryGraphObligationOperatingWorldDescriptorKind::Branch => Self::Branch,
            WorthQueryGraphObligationOperatingWorldDescriptorKind::ConfiguredDomainHandle => {
                Self::ConfiguredDomainHandle
            }
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn as_str(
        self,
    ) -> &'static str {
        match self {
            Self::AnyOperatingWorld => "any-operating-world",
            Self::AnyCommittedAuthority => "any-committed-authority",
            Self::Preview => "preview",
            Self::Branch => "branch",
            Self::ConfiguredDomainHandle => "configured-domain-handle",
        }
    }
}

fn terminal_declared_aspect_operation_digest_part(
    operation: &WorthQueryAspectMutationOperation,
) -> String {
    format!(
        "{}:{}",
        operation.kind().as_str(),
        operation.aspect_touch().admitted_touch_digest_part()
    )
}
