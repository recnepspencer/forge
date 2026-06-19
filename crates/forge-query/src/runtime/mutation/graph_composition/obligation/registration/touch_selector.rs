use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchLifecycleFamily,
    ForgeQueryGraphTouchReadVerb, ForgeQueryMutationFamily,
};
use forge_relational::facade::identity::KindId;

use super::registration_denial::{
    ForgeQueryGraphObligationRegistrationDenial, ForgeQueryGraphObligationRegistrationDenialKind,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum ForgeQueryGraphTouchSelectorKind {
    Any,
    Collection(String),
    RelationKindId(u32),
    AspectPath(String),
    DeclaredAspectOperation(String),
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

    pub fn aspect_path(
        aspect_path: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationRegistrationDenial> {
        Ok(Self::new(ForgeQueryGraphTouchSelectorKind::AspectPath(
            non_empty_selector_value(aspect_path.into(), "aspect path")?,
        )))
    }

    pub fn declared_aspect_operation(
        operation: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationRegistrationDenial> {
        Ok(Self::new(
            ForgeQueryGraphTouchSelectorKind::DeclaredAspectOperation(non_empty_selector_value(
                operation.into(),
                "declared aspect operation",
            )?),
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
            ForgeQueryGraphTouchSelectorKind::AspectPath(aspect_path) => {
                descriptor.touches_aspect_path(aspect_path)
            }
            ForgeQueryGraphTouchSelectorKind::DeclaredAspectOperation(operation) => {
                descriptor.touches_declared_aspect_operation(operation)
            }
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

    pub fn selector_kind(&self) -> &'static str {
        selector_kind_name(&self.kind)
    }

    pub fn selector_value(&self) -> Option<String> {
        selector_kind_value(&self.kind)
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
        ForgeQueryGraphTouchSelectorKind::AspectPath(_) => "aspect-path",
        ForgeQueryGraphTouchSelectorKind::DeclaredAspectOperation(_) => "declared-aspect-operation",
        ForgeQueryGraphTouchSelectorKind::MutationFamily(_) => "mutation-family",
        ForgeQueryGraphTouchSelectorKind::LifecycleFamily(_) => "lifecycle-family",
        ForgeQueryGraphTouchSelectorKind::ReadVerb(_) => "read-verb",
    }
}

fn selector_kind_value(kind: &ForgeQueryGraphTouchSelectorKind) -> Option<String> {
    match kind {
        ForgeQueryGraphTouchSelectorKind::Any => None,
        ForgeQueryGraphTouchSelectorKind::Collection(value)
        | ForgeQueryGraphTouchSelectorKind::AspectPath(value)
        | ForgeQueryGraphTouchSelectorKind::DeclaredAspectOperation(value) => Some(value.clone()),
        ForgeQueryGraphTouchSelectorKind::RelationKindId(value) => Some(value.to_string()),
        ForgeQueryGraphTouchSelectorKind::MutationFamily(family) => {
            Some(family.as_str().to_string())
        }
        ForgeQueryGraphTouchSelectorKind::LifecycleFamily(family) => {
            Some(family.as_str().to_string())
        }
        ForgeQueryGraphTouchSelectorKind::ReadVerb(verb) => Some(verb.as_str().to_string()),
    }
}

fn non_empty_selector_value(
    value: String,
    label: &'static str,
) -> Result<String, ForgeQueryGraphObligationRegistrationDenial> {
    let value = value.trim().to_string();
    if value.is_empty() {
        return Err(ForgeQueryGraphObligationRegistrationDenial::new(
            ForgeQueryGraphObligationRegistrationDenialKind::EmptySelectorValue,
            format!("graph obligation {label} selector value must not be empty"),
        ));
    }
    Ok(value)
}
