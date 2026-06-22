use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectTouch, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchLifecycleFamily, ForgeQueryGraphTouchReadVerb, ForgeQueryMutationFamily,
};
use forge_relational::facade::identity::KindId;

use super::registration_denial::ForgeQueryGraphObligationRegistrationDenial;
use super::selector_class::ForgeQueryGraphTouchSelectorClass;
use super::selector_helpers::{
    contains_all_aspect_touches, contains_all_operations,
    native_declared_aspect_operation_digest_part, native_touch_digest_parts,
    non_empty_selector_value, sorted_unique_operations, sorted_unique_touches,
    terminal_mutation_family,
};

#[derive(Clone, Debug, Eq, PartialEq)]
enum ForgeQueryGraphTouchSelectorKind {
    Any,
    Collection(String),
    RelationKindId(u32),
    AspectTouch(ForgeQueryAspectTouch),
    DeclaredAspectOperation(ForgeQueryAspectMutationOperation),
    DeclaredMutationCollection {
        collection: String,
        mutation_family: ForgeQueryMutationFamily,
        declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
        touched_aspects: Vec<ForgeQueryAspectTouch>,
    },
    MutationFamily(ForgeQueryMutationFamily),
    LifecycleFamily(ForgeQueryGraphTouchLifecycleFamily),
    ReadVerb(ForgeQueryGraphTouchReadVerb),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphTouchSelector {
    kind: ForgeQueryGraphTouchSelectorKind,
    selector_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphTouchSelector {
    pub fn any_graph_touch() -> Self {
        Self::new(ForgeQueryGraphTouchSelectorKind::Any)
    }

    pub fn collection(
        collection: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationRegistrationDenial> {
        Ok(Self::new(ForgeQueryGraphTouchSelectorKind::Collection(
            non_empty_selector_value(collection.into(), "collection")?,
        )))
    }

    pub fn relation_kind(
        relation_kind: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationRegistrationDenial> {
        Self::collection(relation_kind)
    }

    pub fn relation_kind_id(relation_kind_id: u32) -> Self {
        Self::new(ForgeQueryGraphTouchSelectorKind::RelationKindId(
            relation_kind_id,
        ))
    }

    pub fn relational_kind_id(relation_kind_id: KindId) -> Self {
        Self::relation_kind_id(relation_kind_id.0)
    }

    pub fn aspect_touch(aspect_touch: ForgeQueryAspectTouch) -> Self {
        Self::new(ForgeQueryGraphTouchSelectorKind::AspectTouch(aspect_touch))
    }

    pub fn declared_aspect_operation(operation: ForgeQueryAspectMutationOperation) -> Self {
        Self::new(ForgeQueryGraphTouchSelectorKind::DeclaredAspectOperation(
            operation,
        ))
    }

    pub fn declared_mutation_collection(
        collection: impl Into<String>,
        mutation_family: ForgeQueryMutationFamily,
        declared_aspect_operations: impl IntoIterator<Item = ForgeQueryAspectMutationOperation>,
        touched_aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>,
    ) -> Result<Self, ForgeQueryGraphObligationRegistrationDenial> {
        Ok(Self::new(
            ForgeQueryGraphTouchSelectorKind::DeclaredMutationCollection {
                collection: non_empty_selector_value(collection.into(), "collection")?,
                mutation_family,
                declared_aspect_operations: sorted_unique_operations(declared_aspect_operations),
                touched_aspects: sorted_unique_touches(touched_aspects),
            },
        ))
    }

    pub fn mutation_family(family: ForgeQueryMutationFamily) -> Self {
        Self::new(ForgeQueryGraphTouchSelectorKind::MutationFamily(family))
    }

    pub fn lifecycle_family(family: ForgeQueryGraphTouchLifecycleFamily) -> Self {
        Self::new(ForgeQueryGraphTouchSelectorKind::LifecycleFamily(family))
    }

    pub fn read_verb(verb: ForgeQueryGraphTouchReadVerb) -> Self {
        Self::new(ForgeQueryGraphTouchSelectorKind::ReadVerb(verb))
    }

    fn new(kind: ForgeQueryGraphTouchSelectorKind) -> Self {
        let selector_kind_value = selector_kind_value(&kind);
        let selector_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationTouchSelector)
                .field_shape(
                    ForgeQueryEvidenceTag::new("kind"),
                    selector_kind_name(&kind),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("value"),
                    selector_kind_value.as_deref(),
                )
                .seal();
        Self {
            kind,
            selector_digest,
        }
    }

    pub fn matches_descriptor(&self, descriptor: &ForgeQueryGraphTouchDescriptor) -> bool {
        match &self.kind {
            ForgeQueryGraphTouchSelectorKind::Any => true,
            ForgeQueryGraphTouchSelectorKind::Collection(collection) => {
                descriptor.touches_collection(collection)
            }
            ForgeQueryGraphTouchSelectorKind::RelationKindId(relation_kind_id) => {
                descriptor.touches_relation_kind_id(KindId(*relation_kind_id))
            }
            ForgeQueryGraphTouchSelectorKind::AspectTouch(aspect_touch) => {
                descriptor.touches_aspect(aspect_touch)
            }
            ForgeQueryGraphTouchSelectorKind::DeclaredAspectOperation(operation) => {
                descriptor.touches_declared_aspect_operation(operation)
            }
            ForgeQueryGraphTouchSelectorKind::DeclaredMutationCollection {
                collection,
                mutation_family,
                declared_aspect_operations,
                touched_aspects,
            } => descriptor.rows().iter().any(|row| {
                row.read_verb().is_none()
                    && row.declared_collection() == Some(collection.as_str())
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
            ForgeQueryGraphTouchSelectorKind::MutationFamily(family) => descriptor
                .rows()
                .iter()
                .any(|row| row.read_verb().is_none() && row.mutation_family() == *family),
            ForgeQueryGraphTouchSelectorKind::LifecycleFamily(family) => descriptor
                .rows()
                .iter()
                .any(|row| row.lifecycle_family() == Some(*family)),
            ForgeQueryGraphTouchSelectorKind::ReadVerb(verb) => descriptor
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
    ) -> ForgeQueryGraphTouchSelectorClass {
        match self.kind {
            ForgeQueryGraphTouchSelectorKind::Any => ForgeQueryGraphTouchSelectorClass::Any,
            ForgeQueryGraphTouchSelectorKind::Collection(_) => {
                ForgeQueryGraphTouchSelectorClass::Collection
            }
            ForgeQueryGraphTouchSelectorKind::RelationKindId(_) => {
                ForgeQueryGraphTouchSelectorClass::RelationKindId
            }
            ForgeQueryGraphTouchSelectorKind::AspectTouch(_) => {
                ForgeQueryGraphTouchSelectorClass::AspectTouch
            }
            ForgeQueryGraphTouchSelectorKind::DeclaredAspectOperation(_) => {
                ForgeQueryGraphTouchSelectorClass::DeclaredAspectOperation
            }
            ForgeQueryGraphTouchSelectorKind::DeclaredMutationCollection { .. } => {
                ForgeQueryGraphTouchSelectorClass::DeclaredMutationCollection
            }
            ForgeQueryGraphTouchSelectorKind::MutationFamily(_) => {
                ForgeQueryGraphTouchSelectorClass::MutationFamily
            }
            ForgeQueryGraphTouchSelectorKind::LifecycleFamily(_) => {
                ForgeQueryGraphTouchSelectorClass::LifecycleFamily
            }
            ForgeQueryGraphTouchSelectorKind::ReadVerb(_) => {
                ForgeQueryGraphTouchSelectorClass::ReadVerb
            }
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation) fn collection_value(
        &self,
    ) -> Option<&str> {
        match &self.kind {
            ForgeQueryGraphTouchSelectorKind::Collection(collection) => Some(collection),
            _ => None,
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation) fn relation_kind_id_value(
        &self,
    ) -> Option<KindId> {
        match self.kind {
            ForgeQueryGraphTouchSelectorKind::RelationKindId(relation_kind_id) => {
                Some(KindId(relation_kind_id))
            }
            _ => None,
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation) fn aspect_touch_value(
        &self,
    ) -> Option<&ForgeQueryAspectTouch> {
        match &self.kind {
            ForgeQueryGraphTouchSelectorKind::AspectTouch(aspect_touch) => Some(aspect_touch),
            _ => None,
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation) fn declared_mutation_collection_value(
        &self,
    ) -> Option<&str> {
        match &self.kind {
            ForgeQueryGraphTouchSelectorKind::DeclaredMutationCollection { collection, .. } => {
                Some(collection)
            }
            _ => None,
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation) fn mutation_family_value(
        &self,
    ) -> Option<ForgeQueryMutationFamily> {
        match self.kind {
            ForgeQueryGraphTouchSelectorKind::MutationFamily(family) => Some(family),
            _ => None,
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation) fn lifecycle_family_value(
        &self,
    ) -> Option<ForgeQueryGraphTouchLifecycleFamily> {
        match self.kind {
            ForgeQueryGraphTouchSelectorKind::LifecycleFamily(family) => Some(family),
            _ => None,
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation) fn read_verb_value(
        &self,
    ) -> Option<ForgeQueryGraphTouchReadVerb> {
        match self.kind {
            ForgeQueryGraphTouchSelectorKind::ReadVerb(verb) => Some(verb),
            _ => None,
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation) fn declared_aspect_operation_value(
        &self,
    ) -> Option<&ForgeQueryAspectMutationOperation> {
        match &self.kind {
            ForgeQueryGraphTouchSelectorKind::DeclaredAspectOperation(operation) => Some(operation),
            _ => None,
        }
    }

    pub(crate) fn selector_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.selector_digest
    }
}

fn selector_kind_name(kind: &ForgeQueryGraphTouchSelectorKind) -> &'static str {
    match kind {
        ForgeQueryGraphTouchSelectorKind::Any => "any-graph-touch",
        ForgeQueryGraphTouchSelectorKind::Collection(_) => "collection",
        ForgeQueryGraphTouchSelectorKind::RelationKindId(_) => "relation-kind-id",
        ForgeQueryGraphTouchSelectorKind::AspectTouch(_) => "aspect-touch",
        ForgeQueryGraphTouchSelectorKind::DeclaredAspectOperation(_) => "declared-aspect-operation",
        ForgeQueryGraphTouchSelectorKind::DeclaredMutationCollection { .. } => {
            "declared-mutation-collection"
        }
        ForgeQueryGraphTouchSelectorKind::MutationFamily(_) => "mutation-family",
        ForgeQueryGraphTouchSelectorKind::LifecycleFamily(_) => "lifecycle-family",
        ForgeQueryGraphTouchSelectorKind::ReadVerb(_) => "read-verb",
    }
}

fn selector_kind_value(kind: &ForgeQueryGraphTouchSelectorKind) -> Option<String> {
    match kind {
        ForgeQueryGraphTouchSelectorKind::Any => None,
        ForgeQueryGraphTouchSelectorKind::Collection(value) => Some(value.clone()),
        ForgeQueryGraphTouchSelectorKind::DeclaredAspectOperation(value) => {
            Some(native_declared_aspect_operation_digest_part(value))
        }
        ForgeQueryGraphTouchSelectorKind::AspectTouch(value) => {
            Some(value.admitted_touch_digest_part())
        }
        ForgeQueryGraphTouchSelectorKind::DeclaredMutationCollection {
            collection,
            mutation_family,
            declared_aspect_operations,
            touched_aspects,
        } => Some(format!(
            "{}|{}|{}|{}",
            collection,
            mutation_family.as_str(),
            declared_aspect_operations
                .iter()
                .map(native_declared_aspect_operation_digest_part)
                .collect::<Vec<_>>()
                .join(","),
            native_touch_digest_parts(touched_aspects).join(",")
        )),
        ForgeQueryGraphTouchSelectorKind::RelationKindId(value) => Some(value.to_string()),
        ForgeQueryGraphTouchSelectorKind::MutationFamily(family) => {
            Some(terminal_mutation_family(*family))
        }
        ForgeQueryGraphTouchSelectorKind::LifecycleFamily(family) => {
            Some(family.as_str().to_string())
        }
        ForgeQueryGraphTouchSelectorKind::ReadVerb(verb) => Some(verb.as_str().to_string()),
    }
}
