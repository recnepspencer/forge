use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectTouch, ForgeQueryGraphCompositionBreadth,
    ForgeQueryGraphCompositionProgram, ForgeQueryMutationTargetCollectionIdentity,
    ForgeQueryWriteCommand,
};
use forge_relational::facade::identity::KindId;

use super::denial::ForgeQueryGraphTouchDescriptorDenial;
use super::descriptor_inventory::ForgeQueryGraphTouchDescriptorInventory;
use super::descriptor_kind::ForgeQueryGraphTouchDescriptorKind;
use super::lifecycle_family::ForgeQueryGraphTouchLifecycleFamily;
use super::read_verb::ForgeQueryGraphTouchReadVerb;
use super::touch_rows::{
    derive_command_touch_rows, derive_read_touch_rows, ForgeQueryGraphReadTouchShape,
    ForgeQueryGraphTouchDescriptorRow, ForgeQueryGraphTouchDescriptorRowInput,
};
use super::validation::validate_graph_touch_descriptor_inputs;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphTouchDescriptor {
    kind: ForgeQueryGraphTouchDescriptorKind,
    rows: Vec<ForgeQueryGraphTouchDescriptorRow>,
    component_count: usize,
    symbolic_entity_declaration_count: usize,
    symbolic_relation_declaration_count: usize,
    insert_command_count: usize,
    update_command_count: usize,
    assertion_command_count: usize,
    delete_command_count: usize,
    declared_collection_count: usize,
    relation_kind_count: usize,
    declared_aspect_touch_count: usize,
    declared_aspect_operation_count: usize,
    touched_aspect_count: usize,
    descriptor_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphTouchDescriptor {
    pub(crate) fn from_authoritative_mutation_batch(
        program: &ForgeQueryGraphCompositionProgram,
        breadth: &ForgeQueryGraphCompositionBreadth,
        commands: &[ForgeQueryWriteCommand],
    ) -> Result<Self, ForgeQueryGraphTouchDescriptorDenial> {
        Self::derive(
            ForgeQueryGraphTouchDescriptorKind::AuthoritativeMutationBatch,
            program,
            breadth,
            commands,
        )
    }

    pub(crate) fn from_mutation_command_batch(
        commands: &[ForgeQueryWriteCommand],
    ) -> Result<Self, ForgeQueryGraphTouchDescriptorDenial> {
        Self::derive(
            ForgeQueryGraphTouchDescriptorKind::AuthoritativeMutationBatch,
            &ForgeQueryGraphCompositionProgram::empty(),
            &ForgeQueryGraphCompositionBreadth::empty(),
            commands,
        )
    }

    pub fn read_family(
        collection: impl Into<String>,
        verbs: impl IntoIterator<Item = ForgeQueryGraphTouchReadVerb>,
    ) -> Result<Self, ForgeQueryGraphTouchDescriptorDenial> {
        Self::from_read_rows(
            ForgeQueryGraphTouchDescriptorKind::ReadFamily,
            collection,
            verbs,
            ForgeQueryGraphReadTouchShape::default(),
        )
    }

    pub fn read_family_shape(
        collection: impl Into<String>,
        verbs: impl IntoIterator<Item = ForgeQueryGraphTouchReadVerb>,
        shape: ForgeQueryGraphReadTouchShape,
    ) -> Result<Self, ForgeQueryGraphTouchDescriptorDenial> {
        Self::from_read_rows(
            ForgeQueryGraphTouchDescriptorKind::ReadFamily,
            collection,
            verbs,
            shape,
        )
    }

    pub fn live_read(
        collection: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphTouchDescriptorDenial> {
        Self::from_read_rows(
            ForgeQueryGraphTouchDescriptorKind::LiveRead,
            collection,
            [ForgeQueryGraphTouchReadVerb::RetainsLiveSubscription],
            ForgeQueryGraphReadTouchShape::default(),
        )
    }

    pub fn live_read_shape(
        collection: impl Into<String>,
        verbs: impl IntoIterator<Item = ForgeQueryGraphTouchReadVerb>,
        shape: ForgeQueryGraphReadTouchShape,
    ) -> Result<Self, ForgeQueryGraphTouchDescriptorDenial> {
        Self::from_read_rows(
            ForgeQueryGraphTouchDescriptorKind::LiveRead,
            collection,
            verbs,
            shape,
        )
    }

    pub fn declared_mutation_collection(
        collection: impl Into<String>,
        mutation_family: crate::runtime::ForgeQueryMutationFamily,
        lifecycle_family: Option<ForgeQueryGraphTouchLifecycleFamily>,
        declared_aspect_operations: impl IntoIterator<Item = ForgeQueryAspectMutationOperation>,
        touched_aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>,
    ) -> Result<Self, ForgeQueryGraphTouchDescriptorDenial> {
        let collection = collection.into().trim().to_string();
        if collection.is_empty() {
            return Err(ForgeQueryGraphTouchDescriptorDenial::new(
                super::denial::ForgeQueryGraphTouchDescriptorDenialKind::EmptyDeclaredMutationCollection,
                "declared mutation graph touch descriptor requires a collection",
            ));
        }
        let row = ForgeQueryGraphTouchDescriptorRow::new(ForgeQueryGraphTouchDescriptorRowInput {
            component_index: 0,
            mutation_family,
            read_verb: None,
            program_step_kind: None,
            lifecycle_family,
            declared_collection: Some(collection),
            relation_kind_id: None,
            declared_symbol: None,
            declared_aspect_operations: sorted_unique_operations(declared_aspect_operations),
            touched_aspects: sorted_unique_touches(touched_aspects),
            has_symbolic_target_reference: false,
            has_existing_truth_binding: false,
            symbolic_aspect_reference_count: 0,
        });
        let breadth = ForgeQueryGraphCompositionBreadth::empty();
        let program = ForgeQueryGraphCompositionProgram::empty();
        Self::from_rows(
            ForgeQueryGraphTouchDescriptorKind::AuthoritativeMutationBatch,
            vec![row],
            &breadth,
            &program,
            1,
        )
    }

    fn derive(
        kind: ForgeQueryGraphTouchDescriptorKind,
        program: &ForgeQueryGraphCompositionProgram,
        breadth: &ForgeQueryGraphCompositionBreadth,
        commands: &[ForgeQueryWriteCommand],
    ) -> Result<Self, ForgeQueryGraphTouchDescriptorDenial> {
        validate_graph_touch_descriptor_inputs(program, breadth, commands)?;
        let rows = derive_command_touch_rows(program, commands);
        Self::from_rows(kind, rows, breadth, program, commands.len())
    }

    fn from_read_rows(
        kind: ForgeQueryGraphTouchDescriptorKind,
        collection: impl Into<String>,
        verbs: impl IntoIterator<Item = ForgeQueryGraphTouchReadVerb>,
        shape: ForgeQueryGraphReadTouchShape,
    ) -> Result<Self, ForgeQueryGraphTouchDescriptorDenial> {
        let collection = collection.into().trim().to_string();
        if collection.is_empty() {
            return Err(ForgeQueryGraphTouchDescriptorDenial::new(
                super::denial::ForgeQueryGraphTouchDescriptorDenialKind::EmptyReadCollection,
                "read graph touch descriptor requires a collection",
            ));
        }
        let rows = derive_read_touch_rows(&collection, verbs, &shape);
        let breadth = ForgeQueryGraphCompositionBreadth::empty();
        let program = ForgeQueryGraphCompositionProgram::empty();
        Self::from_rows(kind, rows, &breadth, &program, 0)
    }

    fn from_rows(
        kind: ForgeQueryGraphTouchDescriptorKind,
        rows: Vec<ForgeQueryGraphTouchDescriptorRow>,
        breadth: &ForgeQueryGraphCompositionBreadth,
        program: &ForgeQueryGraphCompositionProgram,
        command_count: usize,
    ) -> Result<Self, ForgeQueryGraphTouchDescriptorDenial> {
        let inventory = ForgeQueryGraphTouchDescriptorInventory::from_rows(&rows);
        let row_digests = rows
            .iter()
            .map(ForgeQueryGraphTouchDescriptorRow::row_evidence_digest)
            .collect::<Vec<_>>();
        let descriptor_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphTouchDescriptor)
                .field_shape(ForgeQueryEvidenceTag::new("role"), "graph-touch-descriptor")
                .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("graph_breadth"),
                    breadth.breadth_evidence_digest(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("graph_program"),
                    program.program_evidence_digest(),
                )
                .field_usize(ForgeQueryEvidenceTag::new("command_count"), command_count)
                .field_usize(
                    ForgeQueryEvidenceTag::new("insert_command_count"),
                    inventory.insert_command_count(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("update_command_count"),
                    inventory.update_command_count(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("assertion_command_count"),
                    inventory.assertion_command_count(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("delete_command_count"),
                    inventory.delete_command_count(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("declared_collection_count"),
                    inventory.declared_collection_count(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("relation_kind_count"),
                    inventory.relation_kind_count(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("declared_aspect_touch_count"),
                    inventory.declared_aspect_touch_count(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("declared_aspect_operation_count"),
                    inventory.declared_aspect_operation_count(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("touched_aspect_count"),
                    inventory.touched_aspect_count(),
                )
                .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("row"), row_digests)
                .seal();
        Ok(Self {
            kind,
            rows,
            component_count: breadth.component_count(),
            symbolic_entity_declaration_count: breadth.symbolic_entity_declaration_count(),
            symbolic_relation_declaration_count: breadth.symbolic_relation_declaration_count(),
            insert_command_count: inventory.insert_command_count(),
            update_command_count: inventory.update_command_count(),
            assertion_command_count: inventory.assertion_command_count(),
            delete_command_count: inventory.delete_command_count(),
            declared_collection_count: inventory.declared_collection_count(),
            relation_kind_count: inventory.relation_kind_count(),
            declared_aspect_touch_count: inventory.declared_aspect_touch_count(),
            declared_aspect_operation_count: inventory.declared_aspect_operation_count(),
            touched_aspect_count: inventory.touched_aspect_count(),
            descriptor_digest,
        })
    }

    pub fn kind(&self) -> ForgeQueryGraphTouchDescriptorKind {
        self.kind
    }

    pub fn rows(&self) -> &[ForgeQueryGraphTouchDescriptorRow] {
        &self.rows
    }

    pub fn component_count(&self) -> usize {
        self.component_count
    }

    pub fn symbolic_entity_declaration_count(&self) -> usize {
        self.symbolic_entity_declaration_count
    }

    pub fn symbolic_relation_declaration_count(&self) -> usize {
        self.symbolic_relation_declaration_count
    }

    pub fn insert_command_count(&self) -> usize {
        self.insert_command_count
    }

    pub fn update_command_count(&self) -> usize {
        self.update_command_count
    }

    pub fn assertion_command_count(&self) -> usize {
        self.assertion_command_count
    }

    pub fn delete_command_count(&self) -> usize {
        self.delete_command_count
    }

    pub fn declared_collection_count(&self) -> usize {
        self.declared_collection_count
    }

    pub fn relation_kind_count(&self) -> usize {
        self.relation_kind_count
    }

    pub fn declared_aspect_touch_count(&self) -> usize {
        self.declared_aspect_touch_count
    }

    pub fn declared_aspect_operation_count(&self) -> usize {
        self.declared_aspect_operation_count
    }

    pub fn touched_aspect_count(&self) -> usize {
        self.touched_aspect_count
    }

    pub fn touches_target_collection(
        &self,
        collection: &ForgeQueryMutationTargetCollectionIdentity,
    ) -> bool {
        self.touches_collection(collection.as_str())
    }

    pub(crate) fn touches_collection(&self, collection: &str) -> bool {
        self.rows
            .iter()
            .any(|row| row.declared_collection() == Some(collection))
    }

    pub fn touches_relation_kind_id(&self, relation_kind_id: KindId) -> bool {
        self.rows
            .iter()
            .any(|row| row.relation_kind_id() == Some(relation_kind_id))
    }

    pub fn touches_declared_aspect_operation(
        &self,
        operation: &ForgeQueryAspectMutationOperation,
    ) -> bool {
        self.rows.iter().any(|row| {
            row.declared_aspect_operations()
                .iter()
                .any(|item| item == operation)
        })
    }

    pub fn touches_aspect(&self, aspect_touch: &ForgeQueryAspectTouch) -> bool {
        self.rows.iter().any(|row| {
            row.declared_aspect_operations()
                .iter()
                .any(|operation| operation.aspect_touch() == aspect_touch)
                || row
                    .admitted_touched_aspects()
                    .iter()
                    .any(|touch| touch == aspect_touch)
        })
    }

    pub fn descriptor_digest(&self) -> &str {
        self.descriptor_digest.as_str()
    }

    pub fn descriptor_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.descriptor_digest
    }
}

fn sorted_unique_operations(
    values: impl IntoIterator<Item = ForgeQueryAspectMutationOperation>,
) -> Vec<ForgeQueryAspectMutationOperation> {
    values
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn sorted_unique_touches(
    values: impl IntoIterator<Item = ForgeQueryAspectTouch>,
) -> Vec<ForgeQueryAspectTouch> {
    values
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}
