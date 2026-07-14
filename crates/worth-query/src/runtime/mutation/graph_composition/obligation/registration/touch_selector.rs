use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryAspectMutationOperation, WorthQueryAspectTouch, WorthQueryGraphTouchDescriptor,
    WorthQueryGraphTouchLifecycleFamily, WorthQueryGraphTouchReadVerb, WorthQueryMutationFamily,
    WorthQueryMutationTargetCollectionIdentity,
};
use worth_relational::facade::identity::KindId;

use super::registration_denial::WorthQueryGraphObligationRegistrationDenial;
use super::selector_class::WorthQueryGraphTouchSelectorClass;
use super::selector_helpers::{
    contains_all_aspect_touches, contains_all_operations, non_empty_selector_value,
    sorted_unique_operations, sorted_unique_touches,
    terminal_declared_aspect_operation_digest_part, terminal_mutation_family,
    terminal_touch_digest_parts,
};

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorthQueryGraphTouchSelectorKind {
    Any,
    Collection(WorthQueryMutationTargetCollectionIdentity),
    RelationKindId(u32),
    AspectTouch(WorthQueryAspectTouch),
    DeclaredAspectOperation(WorthQueryAspectMutationOperation),
    DeclaredMutationCollection {
        collection: WorthQueryMutationTargetCollectionIdentity,
        mutation_family: WorthQueryMutationFamily,
        declared_aspect_operations: Vec<WorthQueryAspectMutationOperation>,
        touched_aspects: Vec<WorthQueryAspectTouch>,
    },
    MutationFamily(WorthQueryMutationFamily),
    LifecycleFamily(WorthQueryGraphTouchLifecycleFamily),
    ReadVerb(WorthQueryGraphTouchReadVerb),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphTouchSelector {
    kind: WorthQueryGraphTouchSelectorKind,
    selector_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphTouchSelector {
    pub fn any_graph_touch() -> Self {
        Self::new(WorthQueryGraphTouchSelectorKind::Any)
    }

    pub fn collection(
        collection: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationRegistrationDenial> {
        let collection = non_empty_selector_value(collection.into(), "collection")?;
        Ok(Self::new(WorthQueryGraphTouchSelectorKind::Collection(
            WorthQueryMutationTargetCollectionIdentity::new("graph-touch-selector", collection),
        )))
    }

    pub fn relation_kind(
        relation_kind: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationRegistrationDenial> {
        Self::collection(relation_kind)
    }

    pub fn relation_kind_id(relation_kind_id: u32) -> Self {
        Self::new(WorthQueryGraphTouchSelectorKind::RelationKindId(
            relation_kind_id,
        ))
    }

    pub fn relational_kind_id(relation_kind_id: KindId) -> Self {
        Self::relation_kind_id(relation_kind_id.0)
    }

    pub fn aspect_touch(aspect_touch: WorthQueryAspectTouch) -> Self {
        Self::new(WorthQueryGraphTouchSelectorKind::AspectTouch(aspect_touch))
    }

    pub fn declared_aspect_operation(operation: WorthQueryAspectMutationOperation) -> Self {
        Self::new(WorthQueryGraphTouchSelectorKind::DeclaredAspectOperation(
            operation,
        ))
    }

    pub fn declared_mutation_collection(
        collection: impl Into<String>,
        mutation_family: WorthQueryMutationFamily,
        declared_aspect_operations: impl IntoIterator<Item = WorthQueryAspectMutationOperation>,
        touched_aspects: impl IntoIterator<Item = WorthQueryAspectTouch>,
    ) -> Result<Self, WorthQueryGraphObligationRegistrationDenial> {
        let collection = non_empty_selector_value(collection.into(), "collection")?;
        Ok(Self::new(
            WorthQueryGraphTouchSelectorKind::DeclaredMutationCollection {
                collection: WorthQueryMutationTargetCollectionIdentity::new(
                    "graph-touch-declared-mutation-selector",
                    collection,
                ),
                mutation_family,
                declared_aspect_operations: sorted_unique_operations(declared_aspect_operations),
                touched_aspects: sorted_unique_touches(touched_aspects),
            },
        ))
    }

    pub fn mutation_family(family: WorthQueryMutationFamily) -> Self {
        Self::new(WorthQueryGraphTouchSelectorKind::MutationFamily(family))
    }

    pub fn lifecycle_family(family: WorthQueryGraphTouchLifecycleFamily) -> Self {
        Self::new(WorthQueryGraphTouchSelectorKind::LifecycleFamily(family))
    }

    pub fn read_verb(verb: WorthQueryGraphTouchReadVerb) -> Self {
        Self::new(WorthQueryGraphTouchSelectorKind::ReadVerb(verb))
    }

    fn new(kind: WorthQueryGraphTouchSelectorKind) -> Self {
        let selector_kind_value = selector_kind_value(&kind);
        let selector_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationTouchSelector)
                .field_shape(
                    WorthQueryEvidenceTag::new("kind"),
                    selector_kind_name(&kind),
                )
                .optional_value(
                    WorthQueryEvidenceTag::new("value"),
                    selector_kind_value.as_deref(),
                )
                .seal();
        Self {
            kind,
            selector_digest,
        }
    }

    pub fn matches_descriptor(&self, descriptor: &WorthQueryGraphTouchDescriptor) -> bool {
        match &self.kind {
            WorthQueryGraphTouchSelectorKind::Any => true,
            WorthQueryGraphTouchSelectorKind::Collection(collection) => {
                descriptor.touches_target_collection(collection)
            }
            WorthQueryGraphTouchSelectorKind::RelationKindId(relation_kind_id) => {
                descriptor.touches_relation_kind_id(KindId(*relation_kind_id))
            }
            WorthQueryGraphTouchSelectorKind::AspectTouch(aspect_touch) => {
                descriptor.touches_aspect(aspect_touch)
            }
            WorthQueryGraphTouchSelectorKind::DeclaredAspectOperation(operation) => {
                descriptor.touches_declared_aspect_operation(operation)
            }
            WorthQueryGraphTouchSelectorKind::DeclaredMutationCollection {
                collection,
                mutation_family,
                declared_aspect_operations,
                touched_aspects,
            } => descriptor.rows().iter().any(|row| {
                row.read_verb().is_none()
                    && row.touches_declared_collection(collection)
                    && row.mutation_family() == *mutation_family
                    && contains_all_operations(
                        row.declared_aspect_operations(),
                        declared_aspect_operations,
                    )
                    && contains_all_aspect_touches(
                        row.declared_aspect_operations(),
                        row.admitted_touched_aspects(),
                        touched_aspects,
                    )
            }),
            WorthQueryGraphTouchSelectorKind::MutationFamily(family) => descriptor
                .rows()
                .iter()
                .any(|row| row.read_verb().is_none() && row.mutation_family() == *family),
            WorthQueryGraphTouchSelectorKind::LifecycleFamily(family) => descriptor
                .rows()
                .iter()
                .any(|row| row.lifecycle_family() == Some(*family)),
            WorthQueryGraphTouchSelectorKind::ReadVerb(verb) => descriptor
                .rows()
                .iter()
                .any(|row| row.read_verb() == Some(*verb)),
        }
    }

    pub fn selector_digest(&self) -> &str {
        self.selector_digest.as_str()
    }

    pub(crate) fn terminal_selector_kind_for_boundary(&self) -> &'static str {
        selector_kind_name(&self.kind)
    }

    pub(crate) fn terminal_selector_value_for_boundary(&self) -> Option<String> {
        selector_kind_value(&self.kind)
    }

    pub(in crate::runtime::mutation::graph_composition::obligation) fn selector_class(
        &self,
    ) -> WorthQueryGraphTouchSelectorClass {
        match self.kind {
            WorthQueryGraphTouchSelectorKind::Any => WorthQueryGraphTouchSelectorClass::Any,
            WorthQueryGraphTouchSelectorKind::Collection(_) => {
                WorthQueryGraphTouchSelectorClass::Collection
            }
            WorthQueryGraphTouchSelectorKind::RelationKindId(_) => {
                WorthQueryGraphTouchSelectorClass::RelationKindId
            }
            WorthQueryGraphTouchSelectorKind::AspectTouch(_) => {
                WorthQueryGraphTouchSelectorClass::AspectTouch
            }
            WorthQueryGraphTouchSelectorKind::DeclaredAspectOperation(_) => {
                WorthQueryGraphTouchSelectorClass::DeclaredAspectOperation
            }
            WorthQueryGraphTouchSelectorKind::DeclaredMutationCollection { .. } => {
                WorthQueryGraphTouchSelectorClass::DeclaredMutationCollection
            }
            WorthQueryGraphTouchSelectorKind::MutationFamily(_) => {
                WorthQueryGraphTouchSelectorClass::MutationFamily
            }
            WorthQueryGraphTouchSelectorKind::LifecycleFamily(_) => {
                WorthQueryGraphTouchSelectorClass::LifecycleFamily
            }
            WorthQueryGraphTouchSelectorKind::ReadVerb(_) => {
                WorthQueryGraphTouchSelectorClass::ReadVerb
            }
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation) fn collection_identity(
        &self,
    ) -> Option<&WorthQueryMutationTargetCollectionIdentity> {
        match &self.kind {
            WorthQueryGraphTouchSelectorKind::Collection(collection) => Some(collection),
            _ => None,
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation) fn relation_kind_id_value(
        &self,
    ) -> Option<KindId> {
        match self.kind {
            WorthQueryGraphTouchSelectorKind::RelationKindId(relation_kind_id) => {
                Some(KindId(relation_kind_id))
            }
            _ => None,
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation) fn aspect_touch_value(
        &self,
    ) -> Option<&WorthQueryAspectTouch> {
        match &self.kind {
            WorthQueryGraphTouchSelectorKind::AspectTouch(aspect_touch) => Some(aspect_touch),
            _ => None,
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation) fn declared_mutation_collection_value(
        &self,
    ) -> Option<&WorthQueryMutationTargetCollectionIdentity> {
        match &self.kind {
            WorthQueryGraphTouchSelectorKind::DeclaredMutationCollection { collection, .. } => {
                Some(collection)
            }
            _ => None,
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation) fn mutation_family_value(
        &self,
    ) -> Option<WorthQueryMutationFamily> {
        match self.kind {
            WorthQueryGraphTouchSelectorKind::MutationFamily(family) => Some(family),
            _ => None,
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation) fn lifecycle_family_value(
        &self,
    ) -> Option<WorthQueryGraphTouchLifecycleFamily> {
        match self.kind {
            WorthQueryGraphTouchSelectorKind::LifecycleFamily(family) => Some(family),
            _ => None,
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation) fn read_verb_value(
        &self,
    ) -> Option<WorthQueryGraphTouchReadVerb> {
        match self.kind {
            WorthQueryGraphTouchSelectorKind::ReadVerb(verb) => Some(verb),
            _ => None,
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation) fn declared_aspect_operation_value(
        &self,
    ) -> Option<&WorthQueryAspectMutationOperation> {
        match &self.kind {
            WorthQueryGraphTouchSelectorKind::DeclaredAspectOperation(operation) => Some(operation),
            _ => None,
        }
    }

    pub(crate) fn selector_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.selector_digest
    }
}

fn selector_kind_name(kind: &WorthQueryGraphTouchSelectorKind) -> &'static str {
    match kind {
        WorthQueryGraphTouchSelectorKind::Any => "any-graph-touch",
        WorthQueryGraphTouchSelectorKind::Collection(_) => "collection",
        WorthQueryGraphTouchSelectorKind::RelationKindId(_) => "relation-kind-id",
        WorthQueryGraphTouchSelectorKind::AspectTouch(_) => "aspect-touch",
        WorthQueryGraphTouchSelectorKind::DeclaredAspectOperation(_) => "declared-aspect-operation",
        WorthQueryGraphTouchSelectorKind::DeclaredMutationCollection { .. } => {
            "declared-mutation-collection"
        }
        WorthQueryGraphTouchSelectorKind::MutationFamily(_) => "mutation-family",
        WorthQueryGraphTouchSelectorKind::LifecycleFamily(_) => "lifecycle-family",
        WorthQueryGraphTouchSelectorKind::ReadVerb(_) => "read-verb",
    }
}

fn selector_kind_value(kind: &WorthQueryGraphTouchSelectorKind) -> Option<String> {
    match kind {
        WorthQueryGraphTouchSelectorKind::Any => None,
        WorthQueryGraphTouchSelectorKind::Collection(value) => Some(value.as_str().to_string()),
        WorthQueryGraphTouchSelectorKind::DeclaredAspectOperation(value) => {
            Some(terminal_declared_aspect_operation_digest_part(value))
        }
        WorthQueryGraphTouchSelectorKind::AspectTouch(value) => {
            Some(value.admitted_touch_digest_part())
        }
        WorthQueryGraphTouchSelectorKind::DeclaredMutationCollection {
            collection,
            mutation_family,
            declared_aspect_operations,
            touched_aspects,
        } => Some(format!(
            "{}|{}|{}|{}",
            collection.as_str(),
            mutation_family.as_str(),
            declared_aspect_operations
                .iter()
                .map(terminal_declared_aspect_operation_digest_part)
                .collect::<Vec<_>>()
                .join(","),
            terminal_touch_digest_parts(touched_aspects).join(",")
        )),
        WorthQueryGraphTouchSelectorKind::RelationKindId(value) => Some(value.to_string()),
        WorthQueryGraphTouchSelectorKind::MutationFamily(family) => {
            Some(terminal_mutation_family(*family))
        }
        WorthQueryGraphTouchSelectorKind::LifecycleFamily(family) => {
            Some(family.as_str().to_string())
        }
        WorthQueryGraphTouchSelectorKind::ReadVerb(verb) => Some(verb.as_str().to_string()),
    }
}
