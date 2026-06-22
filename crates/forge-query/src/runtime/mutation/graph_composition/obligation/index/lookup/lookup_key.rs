use crate::runtime::{
    ForgeQueryGraphTouchLifecycleFamily, ForgeQueryGraphTouchReadVerb, ForgeQueryMutationFamily,
};
use forge_relational::facade::identity::KindId;

use super::super::super::registration::ForgeQueryGraphTouchSelectorClass;
use super::super::selection::ForgeQueryGraphObligationOperatingWorldDescriptorKind;
use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectTouch,
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphTouchSelector,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(in crate::runtime::mutation::graph_composition::obligation::index) enum ForgeQueryGraphObligationTouchLookupKey
{
    AnyGraphTouch,
    Collection(String),
    RelationKindId(KindId),
    AspectTouch(ForgeQueryAspectTouch),
    DeclaredAspectOperation(ForgeQueryAspectMutationOperation),
    MutationFamily(ForgeQueryMutationFamily),
    LifecycleFamily(ForgeQueryGraphTouchLifecycleFamily),
    ReadVerb(ForgeQueryGraphTouchReadVerb),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(in crate::runtime::mutation::graph_composition::obligation::index) enum ForgeQueryGraphObligationOperatingWorldLookupKey
{
    AnyOperatingWorld,
    AnyCommittedAuthority,
    Preview,
    Branch,
    ConfiguredDomainHandle,
}

impl ForgeQueryGraphObligationTouchLookupKey {
    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn from_selector(
        selector: &ForgeQueryGraphTouchSelector,
    ) -> Self {
        match selector.selector_class() {
            ForgeQueryGraphTouchSelectorClass::Any => Self::AnyGraphTouch,
            ForgeQueryGraphTouchSelectorClass::Collection => Self::Collection(
                selector
                    .collection_value()
                    .expect("collection selector has a native collection")
                    .to_string(),
            ),
            ForgeQueryGraphTouchSelectorClass::RelationKindId => Self::RelationKindId(
                selector
                    .relation_kind_id_value()
                    .expect("relation kind id selector has a native kind id"),
            ),
            ForgeQueryGraphTouchSelectorClass::AspectTouch => Self::AspectTouch(
                selector
                    .aspect_touch_value()
                    .expect("aspect touch selector has a native touch")
                    .clone(),
            ),
            ForgeQueryGraphTouchSelectorClass::DeclaredAspectOperation => {
                Self::DeclaredAspectOperation(
                    selector
                        .declared_aspect_operation_value()
                        .expect("declared aspect selector has a native operation")
                        .clone(),
                )
            }
            ForgeQueryGraphTouchSelectorClass::DeclaredMutationCollection => Self::Collection(
                selector
                    .declared_mutation_collection_value()
                    .expect("declared mutation collection selector has a native collection")
                    .to_string(),
            ),
            ForgeQueryGraphTouchSelectorClass::MutationFamily => Self::MutationFamily(
                selector
                    .mutation_family_value()
                    .expect("mutation family selector has a native family"),
            ),
            ForgeQueryGraphTouchSelectorClass::LifecycleFamily => Self::LifecycleFamily(
                selector
                    .lifecycle_family_value()
                    .expect("lifecycle family selector has a native family"),
            ),
            ForgeQueryGraphTouchSelectorClass::ReadVerb => Self::ReadVerb(
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

    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn value(
        &self,
    ) -> Option<String> {
        match self {
            Self::AnyGraphTouch => None,
            Self::Collection(value) => Some(value.clone()),
            Self::AspectTouch(value) => Some(value.admitted_touch_digest_part()),
            Self::DeclaredAspectOperation(value) => {
                Some(native_declared_aspect_operation_digest_part(value))
            }
            Self::RelationKindId(value) => Some(value.0.to_string()),
            Self::MutationFamily(value) => Some(value.as_str().to_string()),
            Self::LifecycleFamily(value) => Some(value.as_str().to_string()),
            Self::ReadVerb(value) => Some(value.as_str().to_string()),
        }
    }
}

impl ForgeQueryGraphObligationOperatingWorldLookupKey {
    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn from_selector(
        selector: ForgeQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        match selector {
            ForgeQueryGraphObligationOperatingWorldSelector::AnyCommittedAuthority => {
                Self::AnyCommittedAuthority
            }
            ForgeQueryGraphObligationOperatingWorldSelector::Preview => Self::Preview,
            ForgeQueryGraphObligationOperatingWorldSelector::Branch => Self::Branch,
            ForgeQueryGraphObligationOperatingWorldSelector::ConfiguredDomainHandle => {
                Self::ConfiguredDomainHandle
            }
            ForgeQueryGraphObligationOperatingWorldSelector::AnyOperatingWorld => {
                Self::AnyOperatingWorld
            }
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn from_descriptor_kind(
        kind: ForgeQueryGraphObligationOperatingWorldDescriptorKind,
    ) -> Self {
        match kind {
            ForgeQueryGraphObligationOperatingWorldDescriptorKind::AnyCommittedAuthority => {
                Self::AnyCommittedAuthority
            }
            ForgeQueryGraphObligationOperatingWorldDescriptorKind::Preview => Self::Preview,
            ForgeQueryGraphObligationOperatingWorldDescriptorKind::Branch => Self::Branch,
            ForgeQueryGraphObligationOperatingWorldDescriptorKind::ConfiguredDomainHandle => {
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

fn native_declared_aspect_operation_digest_part(
    operation: &ForgeQueryAspectMutationOperation,
) -> String {
    format!(
        "{}:{}",
        operation.kind().as_str(),
        operation.aspect_touch().admitted_touch_digest_part()
    )
}
