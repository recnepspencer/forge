use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectTouch,
    ForgeQueryGraphCompositionProgramStepKind, ForgeQueryMutationFamily,
    ForgeQueryMutationTargetCollectionIdentity,
};
use forge_relational::facade::identity::KindId;

use super::super::lifecycle_family::ForgeQueryGraphTouchLifecycleFamily;
use super::super::read_verb::ForgeQueryGraphTouchReadVerb;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphTouchDescriptorRow {
    component_index: usize,
    mutation_family: ForgeQueryMutationFamily,
    read_verb: Option<ForgeQueryGraphTouchReadVerb>,
    program_step_kind: Option<ForgeQueryGraphCompositionProgramStepKind>,
    lifecycle_family: Option<ForgeQueryGraphTouchLifecycleFamily>,
    declared_collection: Option<ForgeQueryMutationTargetCollectionIdentity>,
    relation_kind_id: Option<KindId>,
    declared_symbol: Option<String>,
    declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
    touched_aspects: Vec<ForgeQueryAspectTouch>,
    has_symbolic_target_reference: bool,
    has_existing_truth_binding: bool,
    symbolic_aspect_reference_count: usize,
    row_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphTouchDescriptorRow {
    pub(crate) fn new(input: ForgeQueryGraphTouchDescriptorRowInput) -> Self {
        let row_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphTouchDescriptorRow)
                .field_shape(ForgeQueryEvidenceTag::new("role"), "graph-touch-row")
                .field_usize(
                    ForgeQueryEvidenceTag::new("component"),
                    input.component_index,
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("mutation_family"),
                    input.mutation_family.as_str(),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("read_verb"),
                    input.read_verb.map(|verb| verb.as_str()),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("program_step_kind"),
                    input.program_step_kind.map(|kind| kind.as_str()),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("lifecycle_family"),
                    input.lifecycle_family.map(|family| family.as_str()),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("declared_collection"),
                    input
                        .declared_collection
                        .as_ref()
                        .map(ForgeQueryMutationTargetCollectionIdentity::as_str),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("relation_kind_id"),
                    input
                        .relation_kind_id
                        .map(|kind_id| kind_id.0.to_string())
                        .as_deref(),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("declared_symbol"),
                    input.declared_symbol.as_deref(),
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("declared_aspect_operation"),
                    input
                        .declared_aspect_operations
                        .iter()
                        .map(terminal_declared_aspect_operation_digest_part),
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("touched_aspect"),
                    input
                        .touched_aspects
                        .iter()
                        .map(ForgeQueryAspectTouch::admitted_touch_digest_part),
                )
                .field_bool(
                    ForgeQueryEvidenceTag::new("has_symbolic_target_reference"),
                    input.has_symbolic_target_reference,
                )
                .field_bool(
                    ForgeQueryEvidenceTag::new("has_existing_truth_binding"),
                    input.has_existing_truth_binding,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("symbolic_aspect_reference_count"),
                    input.symbolic_aspect_reference_count,
                )
                .seal();
        Self {
            component_index: input.component_index,
            mutation_family: input.mutation_family,
            read_verb: input.read_verb,
            program_step_kind: input.program_step_kind,
            lifecycle_family: input.lifecycle_family,
            declared_collection: input.declared_collection,
            relation_kind_id: input.relation_kind_id,
            declared_symbol: input.declared_symbol,
            declared_aspect_operations: input.declared_aspect_operations,
            touched_aspects: input.touched_aspects,
            has_symbolic_target_reference: input.has_symbolic_target_reference,
            has_existing_truth_binding: input.has_existing_truth_binding,
            symbolic_aspect_reference_count: input.symbolic_aspect_reference_count,
            row_digest,
        }
    }

    pub fn component_index(&self) -> usize {
        self.component_index
    }

    pub fn mutation_family(&self) -> ForgeQueryMutationFamily {
        self.mutation_family
    }

    pub fn read_verb(&self) -> Option<ForgeQueryGraphTouchReadVerb> {
        self.read_verb
    }

    pub fn program_step_kind(&self) -> Option<ForgeQueryGraphCompositionProgramStepKind> {
        self.program_step_kind
    }

    pub fn lifecycle_family(&self) -> Option<ForgeQueryGraphTouchLifecycleFamily> {
        self.lifecycle_family
    }

    pub fn declared_collection(&self) -> Option<&str> {
        self.declared_collection
            .as_ref()
            .map(ForgeQueryMutationTargetCollectionIdentity::as_str)
    }

    pub fn declared_collection_identity(
        &self,
    ) -> Option<&ForgeQueryMutationTargetCollectionIdentity> {
        self.declared_collection.as_ref()
    }

    pub(crate) fn touches_declared_collection(
        &self,
        collection: &ForgeQueryMutationTargetCollectionIdentity,
    ) -> bool {
        self.declared_collection
            .as_ref()
            .is_some_and(|declared| declared.same_target_collection_as(collection))
    }

    pub fn relation_kind_id(&self) -> Option<KindId> {
        self.relation_kind_id
    }

    pub fn declared_symbol(&self) -> Option<&str> {
        self.declared_symbol.as_deref()
    }

    pub fn declared_aspect_operations(&self) -> &[ForgeQueryAspectMutationOperation] {
        &self.declared_aspect_operations
    }

    pub fn admitted_touched_aspects(&self) -> &[ForgeQueryAspectTouch] {
        &self.touched_aspects
    }

    pub fn has_symbolic_target_reference(&self) -> bool {
        self.has_symbolic_target_reference
    }

    pub fn has_existing_truth_binding(&self) -> bool {
        self.has_existing_truth_binding
    }

    pub fn symbolic_aspect_reference_count(&self) -> usize {
        self.symbolic_aspect_reference_count
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }

    pub fn row_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.row_digest
    }
}

pub(crate) struct ForgeQueryGraphTouchDescriptorRowInput {
    pub component_index: usize,
    pub mutation_family: ForgeQueryMutationFamily,
    pub read_verb: Option<ForgeQueryGraphTouchReadVerb>,
    pub program_step_kind: Option<ForgeQueryGraphCompositionProgramStepKind>,
    pub lifecycle_family: Option<ForgeQueryGraphTouchLifecycleFamily>,
    pub declared_collection: Option<ForgeQueryMutationTargetCollectionIdentity>,
    pub relation_kind_id: Option<KindId>,
    pub declared_symbol: Option<String>,
    pub declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
    pub touched_aspects: Vec<ForgeQueryAspectTouch>,
    pub has_symbolic_target_reference: bool,
    pub has_existing_truth_binding: bool,
    pub symbolic_aspect_reference_count: usize,
}

pub(super) fn terminal_declared_aspect_operation_digest_part(
    operation: &ForgeQueryAspectMutationOperation,
) -> String {
    format!(
        "{}:{}",
        operation.kind().as_str(),
        operation.aspect_touch().admitted_touch_digest_part()
    )
}
