use forge_foundational::{aspects, AspectIdentity, FieldKey, ScalarAspectType};

use crate::config::data::{CascadeDeletePolicy, CrossContextPolicy};
use crate::facade::harness::RelationalHarnessError;
use crate::facade::identity::{EntityId, KindId, PartitionId, RelationId};
use crate::facade::schema::{
    AspectBinding, DeclaredAspect, EntityKindRegistration, KindAspectDeclarations,
    RelationIntegrityDeclarations, RelationKindRegistration, RelationalSchemaRegistry, SchemaId,
    SchemaVersionId,
};
use crate::transactions::data::RecordRef;

pub(super) fn resolve_targets(
    request: &forge_harness::facade::ExecutionRequest<String>,
) -> Vec<String> {
    if request.targets.is_empty() {
        Vec::new()
    } else {
        request.targets.clone()
    }
}

pub(super) fn parse_target(target: &str) -> Result<RecordRef, RelationalHarnessError> {
    let mut parts = target.split(':');
    let kind = parts
        .next()
        .ok_or_else(|| RelationalHarnessError("missing target kind".to_string()))?;
    let remainder = parts.collect::<Vec<_>>();
    let (partition_id, slot, generation) = match remainder.as_slice() {
        [slot, generation] => (
            PartitionId::main(),
            slot.parse::<u64>()
                .map_err(|_| RelationalHarnessError("invalid target slot".to_string()))?,
            generation
                .parse::<u32>()
                .map_err(|_| RelationalHarnessError("invalid target generation".to_string()))?,
        ),
        [partition, slot, generation] => (
            PartitionId(
                partition
                    .parse::<u32>()
                    .map_err(|_| RelationalHarnessError("invalid target partition".to_string()))?,
            ),
            slot.parse::<u64>()
                .map_err(|_| RelationalHarnessError("invalid target slot".to_string()))?,
            generation
                .parse::<u32>()
                .map_err(|_| RelationalHarnessError("invalid target generation".to_string()))?,
        ),
        _ => {
            return Err(RelationalHarnessError(
                "target must be kind:slot:generation or kind:partition:slot:generation".to_string(),
            ))
        }
    };
    match kind {
        "entity" => Ok(RecordRef::Entity(EntityId::new(
            partition_id,
            slot,
            generation,
        ))),
        "relation" => Ok(RecordRef::Relation(RelationId::new(
            partition_id,
            slot,
            generation,
        ))),
        _ => Err(RelationalHarnessError("unknown target kind".to_string())),
    }
}

pub(super) fn commit_error_to_harness_error(
    error: crate::facade::transactions::TransactionCommitError,
) -> RelationalHarnessError {
    match error {
        crate::facade::transactions::TransactionCommitError::Conflict {
            error: conflict, ..
        } => RelationalHarnessError(conflict.detail),
        crate::facade::transactions::TransactionCommitError::Publication {
            error: publication,
            ..
        } => RelationalHarnessError(publication.detail),
    }
}

pub(super) fn default_harness_schema_registry() -> RelationalSchemaRegistry {
    RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "fixture.entity".to_string(),
            schema_id: SchemaId("fixture".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::new(default_harness_entity_aspects()),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "fixture.relation".to_string(),
                schema_id: SchemaId("fixture".to_string()),
                schema_version_id: SchemaVersionId(1),
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: KindAspectDeclarations::new(default_harness_relation_aspects()),
                relation_integrity: RelationIntegrityDeclarations::default(),
            })
        })
        .expect("valid default harness schema registry")
}

fn default_harness_entity_aspects() -> Vec<DeclaredAspect> {
    vec![scalar_entity_field_aspect("name", "name")]
}

fn default_harness_relation_aspects() -> Vec<DeclaredAspect> {
    vec![scalar_relation_field_aspect("label", "label")]
}

fn scalar_entity_field_aspect(aspect_name: &str, field_key_text: &str) -> DeclaredAspect {
    DeclaredAspect {
        binding: AspectBinding::EntityField {
            field: field_key(field_key_text),
        },
        contract: scalar_string_contract(aspect_name),
    }
}

fn scalar_relation_field_aspect(aspect_name: &str, field_key_text: &str) -> DeclaredAspect {
    DeclaredAspect {
        binding: AspectBinding::RelationField {
            field: field_key(field_key_text),
        },
        contract: scalar_string_contract(aspect_name),
    }
}

fn scalar_string_contract(aspect_name: &str) -> forge_foundational::AspectContract {
    aspects()
        .contract()
        .for_key(
            aspects()
                .vocabulary()
                .key(aspect_name)
                .expect("default harness aspect name must be a foundational key"),
        )
        .identified_by(AspectIdentity(stable_harness_aspect_identity(aspect_name)))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String)
}

fn field_key(field_key_text: &str) -> FieldKey {
    FieldKey::new(field_key_text).expect("default harness field key must be foundational")
}

fn stable_harness_aspect_identity(aspect_name: &str) -> u64 {
    let mut hash = 14695981039346656037_u64;
    for byte in aspect_name.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(1099511628211_u64);
    }
    hash
}
