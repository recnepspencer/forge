use std::collections::BTreeMap;

pub(in crate::runtime::tests) use super::super::*;
pub(in crate::runtime::tests) use crate::declarative_live::{
    DeclarativeLiveViewShape, DeclarativeProjectionField,
};
use crate::memory_workspace::ForgeQueryEntityIdentity;
pub(in crate::runtime::tests) use crate::program::{
    ForgeQueryAspectValueTemplate, ForgeQueryOperation, ForgeQueryPortType,
    ForgeQueryProgramSource, ForgeQueryProgramValue, ForgeQuerySchemaAdapter, ForgeQueryTypedPort,
    ForgeQueryValueExpr, ForgeQueryWriteCommandTemplate,
};
pub(in crate::runtime::tests) use crate::runtime::async_result_state::ForgeQueryRuntimeAsyncResultProjection;
pub(in crate::runtime::tests) use crate::runtime::remask_posture::ForgeQueryRuntimeRemaskProjection;
pub(in crate::runtime::tests) use crate::schema_view::{SchemaFieldKind, SchemaFieldView};
pub(in crate::runtime::tests) use crate::subscription::QueryPatchGroupKind;
pub(in crate::runtime::tests) use forge_foundational::facade::{
    AspectKey, AspectLocator, AspectValue, CanonicalFieldPath, FieldKey, LocatorAuthority,
    ScalarAspectType,
};
pub(in crate::runtime::tests) use forge_runtime_bridge::facade::{
    BridgeAsyncCompletionClass, BridgeAsyncCompletionDenialClass, BridgeAsyncCompletionState,
    BridgeAsyncForwardCausalityClass, BridgeCommittedPatchEnvelope,
    BridgeCommittedPatchEnvelopeIdentity, BridgeCommittedPatchItem, BridgeCommittedPatchTarget,
    BridgeDeliveryReceipt, BridgeMappingId, BridgeMappingRegistration, CoarseRoutingMode,
    InvalidationSink, MappingSelector, RelationalBridgeRecordIdentityKind,
    RelationalBridgeRecordIdentityParts, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest, RuntimeBridgeBuilder, SignalBridgeSinkError,
    SignalInvalidationScope, SnapshotReadContract, SnapshotReadPacket, SnapshotReadPacketResult,
    SnapshotReadRecord, SnapshotReadSource, TruthBranchIdentity, TruthCommitIdentity,
    TruthPatchIdentity, TruthPatchScope, TruthSnapshotIdentity, TruthSnapshotReader,
};
mod adapters;
mod bridge;
mod commands;
mod domains;
mod graph_composition;
mod mixed_cause;
mod mutation_authority_bridge;
mod mutation_receipt_support;
mod program;
mod schema;
mod stateful_bridge_runtime;

pub(in crate::runtime::tests) use adapters::*;
pub(crate) use bridge::*;
pub(in crate::runtime::tests) use commands::*;
pub(in crate::runtime::tests) use domains::*;
pub(in crate::runtime::tests) use graph_composition::*;
pub(in crate::runtime::tests) use mixed_cause::*;
pub(in crate::runtime::tests) use mutation_authority_bridge::*;
pub(in crate::runtime::tests) use mutation_receipt_support::*;
pub(in crate::runtime::tests) use program::*;
pub(in crate::runtime::tests) use schema::*;
pub(crate) use stateful_bridge_runtime::*;

pub(in crate::runtime::tests) fn test_session_label(
    label: impl AsRef<str>,
) -> ForgeQuerySessionLabel {
    ForgeQuerySessionLabel::scoped_strs("forge-query-runtime-tests", [label.as_ref()])
        .expect("test session label should build")
}

pub(in crate::runtime::tests) fn test_mutation_metadata_key(
    key: impl Into<String>,
) -> ForgeQueryMutationMetadataKey {
    ForgeQueryMutationMetadataKey::new(key).expect("test metadata key should admit")
}

pub(in crate::runtime::tests) fn test_entity_identity(
    identity: impl AsRef<str>,
) -> ForgeQueryEntityIdentity {
    relational_test_entity_identity_from_label(identity.as_ref())
        .unwrap_or_else(|| crate::memory_workspace::admit_authored_entity_label(identity))
}

fn relational_test_entity_identity_from_label(label: &str) -> Option<ForgeQueryEntityIdentity> {
    parse_typed_relational_record_parts(label)
        .or_else(|| parse_collection_slot_relational_record_parts(label))
        .or_else(|| relational_test_entity_identity_named_fixture(label))
        .map(ForgeQueryEntityIdentity::from_relational_record)
}

fn parse_typed_relational_record_parts(label: &str) -> Option<RelationalBridgeRecordIdentityParts> {
    let mut segments = label.split(':');
    let kind = segments.next()?;
    let partition_id = segments.next()?.parse().ok()?;
    let local_slot = segments.next()?.parse().ok()?;
    let generation = segments.next()?.parse().ok()?;
    if segments.next().is_some() {
        return None;
    }
    match kind {
        "entity" => Some(RelationalBridgeRecordIdentityParts::entity(
            partition_id,
            local_slot,
            generation,
        )),
        "relation" => Some(RelationalBridgeRecordIdentityParts::relation(
            partition_id,
            local_slot,
            generation,
        )),
        _ => None,
    }
}

fn parse_collection_slot_relational_record_parts(
    label: &str,
) -> Option<RelationalBridgeRecordIdentityParts> {
    let (collection, slot_text) = label.rsplit_once(':')?;
    let local_slot = slot_text.parse().ok()?;
    if relational_test_entity_identity_is_relation_collection(collection) {
        Some(RelationalBridgeRecordIdentityParts::relation(
            2, local_slot, 0,
        ))
    } else {
        Some(RelationalBridgeRecordIdentityParts::entity(
            1, local_slot, 0,
        ))
    }
}

fn relational_test_entity_identity_is_relation_collection(collection: &str) -> bool {
    collection.ends_with("Relation") || matches!(collection, "Edge" | "TaskEdge")
}

fn relational_test_entity_identity_named_fixture(
    label: &str,
) -> Option<RelationalBridgeRecordIdentityParts> {
    match label {
        "task-existing" | "vertex-a" | "face-1" => {
            Some(RelationalBridgeRecordIdentityParts::entity(1, 1, 0))
        }
        "task-existing-left" | "vertex-b" => {
            Some(RelationalBridgeRecordIdentityParts::entity(1, 2, 0))
        }
        "task-existing-right" => Some(RelationalBridgeRecordIdentityParts::entity(1, 3, 0)),
        _ => None,
    }
}

pub(in crate::runtime::tests) fn test_relational_endpoint_identity_label(
    identity: &ForgeQueryEntityIdentity,
) -> String {
    let parts = identity
        .relational_record_parts()
        .expect("test relational endpoint identity must carry relational record authority");
    let kind = match parts.kind() {
        RelationalBridgeRecordIdentityKind::Entity => "entity",
        RelationalBridgeRecordIdentityKind::Relation => "relation",
    };
    format!(
        "{kind}:{}:{}:{}",
        parts.partition_id(),
        parts.local_slot(),
        parts.generation()
    )
}

pub(in crate::runtime::tests) fn test_write_adjacent_origin_identity(
    class: ForgeQueryEffectWriteAdjacentTriggerClass,
    origin: impl AsRef<str>,
) -> crate::evidence_identity::ForgeQueryEvidenceIdentity {
    crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
        crate::evidence_identity::ForgeQueryEvidenceScope::EffectIntentReceiptPhase,
    )
    .field_shape(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("identity_family"),
        "test_effect_write_adjacent_origin_v1",
    )
    .field_shape(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("class"),
        class.as_str(),
    )
    .field_value(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("origin_fixture"),
        origin.as_ref(),
    )
    .seal()
}

pub(in crate::runtime::tests) fn live_subscription_async_identity(
    runtime: &ForgeQueryRuntime,
    view_name: &str,
) -> (
    crate::evidence_identity::ForgeQueryEvidenceIdentity,
    crate::evidence_identity::ForgeQueryEvidenceIdentity,
) {
    let state = runtime
        .live_subscriptions
        .get(view_name)
        .expect("live subscription state should exist");
    (
        state.installation.basis_binding_identity().clone(),
        state.active_lane_handle.checkpoint_identity().clone(),
    )
}

pub(in crate::runtime::tests) fn retained_test_row(
    fields: impl IntoIterator<Item = (impl Into<String>, AspectValue)>,
) -> ForgeQueryRetainedMaterializedRow {
    ForgeQueryRetainedMaterializedRow::from_scalar_values(
        fields
            .into_iter()
            .map(|(field, value)| {
                (
                    retained_test_field_path(field.into())
                        .expect("test retained field path should parse"),
                    value,
                )
            })
            .collect::<BTreeMap<_, _>>(),
    )
    .expect("test retained row should admit native scalar values")
}

pub(in crate::runtime::tests) fn retained_test_field_path(
    path: impl Into<String>,
) -> Result<ForgeQueryRetainedFieldPath, String> {
    let path = path.into();
    let fields = path
        .split('.')
        .map(|field| {
            FieldKey::new(field.to_string())
                .ok_or_else(|| format!("`{path}` is not a test retained scalar field path"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let path = CanonicalFieldPath::new(fields)
        .ok_or_else(|| format!("`{path}` is not a test retained scalar field path"))?;
    Ok(ForgeQueryRetainedFieldPath::from_canonical_field_path(path))
}

pub(in crate::runtime::tests) fn retained_string_test_row(
    field: impl Into<String>,
    value: impl Into<String>,
) -> ForgeQueryRetainedMaterializedRow {
    retained_test_row([(field.into(), AspectValue::String(value.into().into()))])
}

pub(in crate::runtime::tests) fn test_native_scalar_value<'a>(
    row: &'a ForgeQueryEntity,
    field_path: &str,
) -> Option<&'a AspectValue> {
    let path = CanonicalFieldPath::new(
        field_path
            .split('.')
            .map(|field| FieldKey::new(field.to_string()))
            .collect::<Option<Vec<_>>>()?,
    )?;
    row.scalar_value_at(&path)
}

pub(in crate::runtime::tests) fn test_native_string_value(
    row: &ForgeQueryEntity,
    field_path: &str,
) -> Option<String> {
    let AspectValue::String(value) = test_native_scalar_value(row, field_path)? else {
        return None;
    };
    Some(match value {
        forge_foundational::facade::InternedString::Raw(value) => value.clone(),
        forge_foundational::facade::InternedString::Symbol(symbol) => {
            format!("symbol:{}", symbol.0)
        }
    })
}

pub(in crate::runtime::tests) fn test_has_native_field_prefix(
    row: &ForgeQueryEntity,
    prefix: &str,
) -> bool {
    row.native_field_values().any(|(field_path, _)| {
        field_path
            .fields()
            .first()
            .is_some_and(|field| field.as_str() == prefix)
    })
}

pub(in crate::runtime::tests) fn test_aspect_touch(aspect_path: &str) -> ForgeQueryAspectTouch {
    ForgeQueryAspectTouch::from_authoring_path(aspect_path.to_string())
        .expect("test aspect path should parse")
}

pub(in crate::runtime::tests) fn test_aspect_touches<const N: usize>(
    aspect_paths: [&str; N],
) -> [ForgeQueryAspectTouch; N] {
    aspect_paths.map(test_aspect_touch)
}

pub(in crate::runtime::tests) fn test_string_aspect_value(value: impl Into<String>) -> AspectValue {
    AspectValue::String(value.into().into())
}

pub(in crate::runtime::tests) fn test_update_string_aspect_command(
    entity_identity: ForgeQueryEntityIdentity,
    aspect_path: &str,
    value: impl Into<String>,
) -> ForgeQueryWriteCommand {
    ForgeQueryWriteCommand::UpdateAspect {
        entity_identity,
        aspect: ForgeQueryAspectValue::new_set(
            test_aspect_touch(aspect_path),
            test_string_aspect_value(value),
        )
        .expect("test update aspect should build"),
    }
}

pub(in crate::runtime::tests) fn test_intent_input(
    fields: impl IntoIterator<Item = (&'static str, &'static str)>,
) -> ForgeQueryIntentInput {
    ForgeQueryIntentInput::object(
        fields
            .into_iter()
            .map(|(field, value)| (field, ForgeQueryIntentInput::string(value))),
    )
}
