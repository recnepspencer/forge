use crate::diagnostics::data::DiagnosticCode;
use crate::schema::data::{DescriptorSemanticsVersion, SchemaVersionId};
use crate::transactions::data::{AspectDeltaFailureFields, ExistingRecordTarget, SavepointId};
use crate::validation::data::InvariantViolationFields;
use serde::{Deserialize, Serialize};

mod aspect_field_target_rejection;
mod authoritative_aspect_source_locator;
mod entity_authoritative_aspect_state_denial;
mod entity_authoritative_aspect_state_presentation;
mod entity_field_aspect_patch_denial;
mod entity_field_aspect_patch_presentation;
mod mutation_state_inconsistency;
mod relation_authoritative_aspect_state_denial;
mod relation_authoritative_aspect_state_presentation;
pub use aspect_field_target_rejection::AspectFieldTargetRejectionReason;
pub use entity_authoritative_aspect_state_denial::EntityAuthoritativeAspectStateDenial;
pub use entity_field_aspect_patch_denial::EntityFieldAspectPatchDenial;
pub use mutation_state_inconsistency::{
    BulkMutationAdmissionDenial, EntityCascadeDeleteMissingState,
    EntityFieldIntentValidationMissingState, MutationStateInconsistencyEvidence,
};
pub use relation_authoritative_aspect_state_denial::RelationAuthoritativeAspectStateDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityFieldUpdateMissingState {
    Partition,
    Slot,
    KindId,
    AuthoritativeAspectState,
}

impl EntityFieldUpdateMissingState {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Partition => "partition",
            Self::Slot => "slot",
            Self::KindId => "kind_id",
            Self::AuthoritativeAspectState => "authoritative aspect state",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityUpdateMissingState {
    Partition,
    Slot,
    KindId,
    AuthoritativeAspectState,
}

impl EntityUpdateMissingState {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Partition => "partition",
            Self::Slot => "slot",
            Self::KindId => "kind_id",
            Self::AuthoritativeAspectState => "authoritative aspect state",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationEndpointUpdateMissingState {
    Partition,
    Slot,
    KindId,
    Endpoints,
}

impl RelationEndpointUpdateMissingState {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Partition => "partition",
            Self::Slot => "slot",
            Self::KindId => "kind_id",
            Self::Endpoints => "endpoints",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BulkImportRowDomain {
    Entity,
    Relation,
}

impl BulkImportRowDomain {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Entity => "entity",
            Self::Relation => "relation",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BulkImportStage {
    EntityCreate,
    RelationCreate,
}

impl BulkImportStage {
    pub const fn label(self) -> &'static str {
        match self {
            Self::EntityCreate => "bulk_entity_stage_import",
            Self::RelationCreate => "bulk_relation_stage_import",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictClass {
    StaleTarget {
        target: ExistingRecordTarget,
        context: String,
    },
    InvalidRelationEndpoint {
        detail: String,
    },
    DuplicateRelationIdentity {
        detail: String,
    },
    InvariantViolation {
        code: DiagnosticCode,
        detail: String,
        fields: InvariantViolationFields,
    },
    KindSchemaMismatch {
        detail: String,
    },
    MutationStateInconsistency {
        detail: String,
        evidence: MutationStateInconsistencyEvidence,
    },
    EntityUpdateStateInconsistency {
        entity_id: crate::identity::data::EntityId,
        missing: EntityUpdateMissingState,
    },
    EntityFieldUpdateStateInconsistency {
        entity_id: crate::identity::data::EntityId,
        missing: EntityFieldUpdateMissingState,
    },
    RelationEndpointUpdateStateInconsistency {
        relation_id: crate::identity::data::RelationId,
        missing: RelationEndpointUpdateMissingState,
    },
    RelationEndpointUpdateKindMismatch {
        relation_id: crate::identity::data::RelationId,
        intent_kind_id: crate::identity::data::KindId,
        authoritative_kind_id: crate::identity::data::KindId,
    },
    BulkImportDomainMismatch {
        expected: BulkImportRowDomain,
        actual: BulkImportRowDomain,
        stage: BulkImportStage,
    },
    EntityFieldAspectPatchDenied {
        entity_id: crate::identity::data::EntityId,
        denial: EntityFieldAspectPatchDenial,
    },
    EntityAuthoritativeAspectStateDenied {
        kind_id: crate::identity::data::KindId,
        denial: EntityAuthoritativeAspectStateDenial,
    },
    RelationAuthoritativeAspectStateDenied {
        kind_id: crate::identity::data::KindId,
        denial: RelationAuthoritativeAspectStateDenial,
    },
    AspectDeltaFailure {
        detail: String,
        fields: AspectDeltaFailureFields,
    },
    ConflictingIntent {
        target: ExistingRecordTarget,
    },
    InvalidSavepoint {
        savepoint_id: SavepointId,
    },
    InvalidMergeParent {
        detail: String,
    },
    StaleValidationBasis {
        detail: String,
    },
    MergeConflictOverlap {
        detail: String,
    },
    MissingMergeBase {
        detail: String,
    },
    UndeclaredSchemaTransition {
        previous_schema_version: SchemaVersionId,
        current_schema_version: SchemaVersionId,
        previous_descriptor_semantics_version: DescriptorSemanticsVersion,
        current_descriptor_semantics_version: DescriptorSemanticsVersion,
    },
    DescriptorVersionIncompatibility {
        previous_descriptor_semantics_version: DescriptorSemanticsVersion,
        current_descriptor_semantics_version: DescriptorSemanticsVersion,
    },
    InvalidSchemaTransitionSourceBasis {
        declared_schema_id: crate::schema::data::SchemaId,
        declared_schema_version: SchemaVersionId,
        expected_schema_id: crate::schema::data::SchemaId,
        expected_schema_version: SchemaVersionId,
    },
    InvalidSchemaTransitionTargetBasis {
        declared_schema_id: crate::schema::data::SchemaId,
        declared_schema_version: SchemaVersionId,
        expected_schema_id: crate::schema::data::SchemaId,
        expected_schema_version: SchemaVersionId,
    },
    MissingSchemaBasisForTransition {
        role: String,
    },
    UnsupportedBridgeDescriptor {
        detail: String,
    },
    HistoricalReinterpretationViolation {
        detail: String,
    },
    TypeIncompatibleSchemaTransition {
        detail: String,
    },
    StructuralIncompatibleSchemaTransition {
        detail: String,
    },
    DirectionalityMismatchUnderCanonicalReconciliation {
        detail: String,
    },
    InvalidSchemaTransitionShape {
        detail: String,
    },
}

mod presentation;

#[cfg(test)]
mod tests;
