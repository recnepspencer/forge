use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::diagnostics::data::DiagnosticCode;
use crate::schema::data::{
    EndpointDeletionIntegrityMode, LoweredAcyclicityContract, LoweredCardinalityMaximumContract,
    LoweredCardinalityMinimumContract, LoweredConnectivityMinimumContract,
    LoweredEndpointDeletionIntegrityContract, LoweredEndpointKindContract,
    LoweredPartitionIsolationContract, LoweredPayloadSchemaContract, LoweredSymmetryContract,
    LoweredUniquenessContract, PairMinimumSemantics, PayloadContractRecordKind,
    PayloadFieldConstraint, PayloadSchemaValueType, SymmetryMode, UniquenessScope,
};
use crate::storage::data::RecordLifecycleState;
use crate::storage::logic::state::HistoricalMetadata;
use crate::storage::logic::state::{EntityRecordKind, RecordKind, RelationRecordKind, SlotView};
use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::{
    InvariantClass, InvariantRule, InvariantViolation, InvariantViolationFields, RecordKindTag,
    RelationCardinalityBoundary, RelationEndpointBoundary,
};
use serde_json::json;

use super::context::InvariantExecutionContext;
use super::state_view::InvariantStateView;

pub(crate) fn evaluate_rule(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    rule: &InvariantRule,
) -> Vec<InvariantViolation> {
    match rule {
        InvariantRule::LiveRecordRequiresSidecar(kind) => {
            single_violation(evaluate_live_record_sidecar_rule(context, class, kind))
        }
        InvariantRule::MaxMergedIntents(limit) => {
            let merged_len = context
                .merged_plan()
                .map(|plan| plan.merged_intents.len())
                .unwrap_or(0);
            if merged_len > *limit {
                vec![InvariantViolation {
                    class,
                    code: DiagnosticCode::InvariantViolation,
                    detail: format!(
                        "merged commit plan has {} intents, limit is {}",
                        merged_len, limit
                    ),
                    fields: InvariantViolationFields::MergedIntentLimit {
                        merged_intent_count: merged_len,
                        limit: *limit,
                    },
                }]
            } else {
                Vec::new()
            }
        }
        InvariantRule::RelationIntegrityScopeBudget(_) => Vec::new(),
        InvariantRule::MaxSnapshotEntities(limit) => {
            single_violation(evaluate_max_snapshot_entities(context, class, *limit))
        }
        InvariantRule::UniqueEntityPayloadField(field) => {
            single_violation(evaluate_unique_entity_payload_field(context, class, field))
        }
        InvariantRule::EndpointKindContract(contract) => {
            evaluate_endpoint_kind_contract(context, class, contract)
        }
        InvariantRule::CardinalityMaximumContract(contract) => {
            evaluate_cardinality_maximum_contract(context, class, contract)
        }
        InvariantRule::CardinalityMinimumContract(contract) => {
            evaluate_cardinality_minimum_contract(context, class, contract)
        }
        InvariantRule::UniquenessContract(contract) => {
            evaluate_uniqueness_contract(context, class, contract)
        }
        InvariantRule::SymmetryContract(contract) => {
            evaluate_symmetry_contract(context, class, contract)
        }
        InvariantRule::EndpointDeletionIntegrityContract(contract) => {
            evaluate_endpoint_deletion_integrity_contract(context, class, contract)
        }
        InvariantRule::PayloadSchemaContract(contract) => {
            evaluate_payload_schema_contract(context, class, contract)
        }
        InvariantRule::PartitionIsolationContract(contract) => {
            evaluate_partition_isolation_contract(context, class, contract)
        }
        InvariantRule::AcyclicityContract(contract) => {
            single_violation(evaluate_acyclicity_contract(context, class, contract))
        }
        InvariantRule::ConnectivityMinimumContract(contract) => single_violation(
            evaluate_connectivity_minimum_contract(context, class, contract),
        ),
    }
}

fn single_violation(violation: Option<InvariantViolation>) -> Vec<InvariantViolation> {
    violation.into_iter().collect()
}

fn canonicalize_violations(mut violations: Vec<InvariantViolation>) -> Vec<InvariantViolation> {
    violations.sort_by(|left, right| left.witness_key().cmp(&right.witness_key()));
    violations
}

fn evaluate_live_record_sidecar_rule(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    kind: &RecordKindTag,
) -> Option<InvariantViolation> {
    match kind {
        RecordKindTag::Entity => evaluate_live_record_sidecar::<EntityRecordKind>(
            context,
            class,
            |state, partition_id| state.touched_entity_slots(partition_id),
            |slot_view| slot_view.kind_id().is_some(),
            "kind id",
            |context, slots| {
                context.metrics().count_entity_slot_scans(slots);
            },
        ),
        RecordKindTag::Relation => evaluate_live_record_sidecar::<RelationRecordKind>(
            context,
            class,
            |state, partition_id| state.touched_relation_slots(partition_id),
            |slot_view| slot_view.extra().is_some(),
            "endpoints",
            |context, slots| {
                context.metrics().count_relation_slot_scans(slots);
            },
        ),
    }
}

fn evaluate_live_record_sidecar<K: RecordKind>(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    touched_slots: impl Fn(
        &dyn crate::logic::runtime::PartitionAccess,
        crate::identity::data::PartitionId,
    ) -> Option<Vec<usize>>,
    has_required_sidecar: impl Fn(&SlotView<'_, K>) -> bool,
    missing_label: &str,
    count_scans: impl Fn(&InvariantExecutionContext<'_>, usize),
) -> Option<InvariantViolation> {
    for partition_id in context.partition_access().partition_ids() {
        let Some(partition) = context.partition_access().get_partition(partition_id) else {
            return Some(storage_inconsistency_violation(
                class,
                format!(
                    "partition {:?} missing during invariant sidecar scan",
                    partition_id
                ),
                json!({
                    "partition_id": partition_id.0,
                    "scan": "live_record_sidecar",
                }),
            ));
        };
        if let Some(slots) = touched_slots(context.partition_access(), partition_id) {
            count_scans(context, slots.len());
            for slot in slots {
                if let Some(violation) = sidecar_violation_for_slot(
                    class,
                    partition,
                    slot,
                    &has_required_sidecar,
                    missing_label,
                ) {
                    return Some(violation);
                }
            }
        } else {
            let arena = K::arena(partition);
            count_scans(context, arena.slot_count());
            for slot in 0..arena.slot_count() {
                if let Some(violation) = sidecar_violation_for_slot(
                    class,
                    partition,
                    slot,
                    &has_required_sidecar,
                    missing_label,
                ) {
                    return Some(violation);
                }
            }
        }
    }
    None
}

fn sidecar_violation_for_slot<K: RecordKind>(
    class: InvariantClass,
    partition: &crate::storage::logic::state::PartitionState,
    slot: usize,
    has_required_sidecar: &impl Fn(&SlotView<'_, K>) -> bool,
    missing_label: &str,
) -> Option<InvariantViolation> {
    let slot_view = K::arena(partition).get_slot(slot)?;
    if slot_view.lifecycle() == RecordLifecycleState::Live && !has_required_sidecar(&slot_view) {
        return Some(InvariantViolation {
            class,
            code: DiagnosticCode::SidecarConsistencyFailure,
            detail: format!(
                "live slot {} in partition {} missing {}",
                slot, partition.partition_id.0, missing_label
            ),
            fields: InvariantViolationFields::SidecarConsistency {
                partition_id: partition.partition_id,
                slot,
                missing_label: missing_label.to_string(),
            },
        });
    }
    None
}

fn evaluate_max_snapshot_entities(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    limit: usize,
) -> Option<InvariantViolation> {
    let state_view = context.state_view();
    let mut visible_entities = 0;
    if state_view.version_id() == context.current_version_id() {
        for partition_id in state_view.state().partition_ids() {
            let Some(partition) = state_view.state().get_partition(partition_id) else {
                return Some(storage_inconsistency_violation(
                    class,
                    format!(
                        "partition {:?} missing during snapshot entity count",
                        partition_id
                    ),
                    json!({
                        "partition_id": partition_id.0,
                        "scan": "max_snapshot_entities",
                    }),
                ));
            };
            visible_entities += partition.entity_arena.live_bitset.count_ones();
        }
    } else {
        for partition_id in state_view.state().partition_ids() {
            let Some(partition) = state_view.state().get_partition(partition_id) else {
                return Some(storage_inconsistency_violation(
                    class,
                    format!(
                        "partition {:?} missing during historical entity scan",
                        partition_id
                    ),
                    json!({
                        "partition_id": partition_id.0,
                        "scan": "historical_max_snapshot_entities",
                    }),
                ));
            };
            context
                .metrics()
                .count_entity_slot_scans(partition.entity_arena.slot_count());
            visible_entities += (0..partition.entity_arena.slot_count())
                .filter(|slot| state_view.entity_visible_at_version(&partition.entity_arena, *slot))
                .count();
        }
    }
    if visible_entities > limit {
        return Some(InvariantViolation {
            class,
            code: DiagnosticCode::InvariantViolation,
            detail: format!(
                "snapshot at version {} has {} entities, limit is {}",
                state_view.version_id().0,
                visible_entities,
                limit
            ),
            fields: InvariantViolationFields::SnapshotEntityLimit {
                version_id: state_view.version_id(),
                visible_entities,
                limit,
            },
        });
    }
    None
}

fn evaluate_unique_entity_payload_field(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    field: &str,
) -> Option<InvariantViolation> {
    let state_view = context.state_view();
    if let Some(violation) = planned_unique_entity_payload_violation(context, class, field) {
        return Some(violation);
    } else if let Some(touched_entity_ids) = state_view.touched_visible_entity_ids() {
        let mut touched_value_to_entity = HashMap::with_capacity(touched_entity_ids.len());
        let touched_set = InvariantStateView::touched_entity_set(&touched_entity_ids);
        for entity_id in touched_entity_ids {
            context.metrics().count_entity_slot_scans(1);
            let Some(payload) = state_view.entity_payload(entity_id) else {
                continue;
            };
            let Some(value) = payload
                .as_json()
                .and_then(|value| value.get(field))
                .and_then(|value| value.as_str())
            else {
                continue;
            };
            if touched_value_to_entity
                .insert(value.to_owned(), entity_id)
                .is_some()
            {
                return Some(duplicate_field_violation(class, field, value));
            }
            if context
                .indexes()
                .conflicts_with_entity_value_outside(field, value, &touched_set)
            {
                return Some(duplicate_field_violation(class, field, value));
            }
        }
    } else if state_view.version_id() == context.current_version_id() {
        if let Some(value) = context.indexes().first_duplicate_entity_value(field) {
            return Some(duplicate_field_violation(class, field, value));
        }
    } else {
        let mut seen = HashSet::new();
        for partition_id in state_view.state().partition_ids() {
            let Some(partition) = state_view.state().get_partition(partition_id) else {
                return Some(storage_inconsistency_violation(
                    class,
                    format!(
                        "partition {:?} missing during historical uniqueness scan",
                        partition_id
                    ),
                    json!({
                        "partition_id": partition_id.0,
                        "scan": "historical_unique_entity_payload_field",
                        "field": field,
                    }),
                ));
            };
            for slot in 0..partition.entity_arena.slot_count() {
                context.metrics().count_entity_slot_scans(1);
                let Some(payload) = partition
                    .entity_arena
                    .payload_history_at(slot)
                    .and_then(|history| state_view.visible_payload(history))
                else {
                    continue;
                };
                let Some(value) = payload
                    .as_json()
                    .and_then(|value| value.get(field))
                    .and_then(|value| value.as_str())
                else {
                    continue;
                };
                if !seen.insert(value.to_string()) {
                    return Some(duplicate_field_violation(class, field, value));
                }
            }
        }
    }
    None
}

fn planned_entity_field_values(
    merged_plan: Option<&MergedCommitPlan>,
    field: &str,
) -> Option<Vec<(Option<crate::identity::data::EntityId>, String)>> {
    let plan = merged_plan?;
    let mut values = Vec::new();
    for intent in &plan.merged_intents {
        match intent {
            crate::transactions::data::MutationIntent::Create(
                crate::transactions::data::CreateIntent::Entity(create),
            ) => {
                let Some(value) = payload_field_value(&create.payload, field) else {
                    continue;
                };
                values.push((None, value));
            }
            crate::transactions::data::MutationIntent::Create(
                crate::transactions::data::CreateIntent::BulkEntities(bulk),
            ) => {
                for payload in &bulk.payloads {
                    let Some(value) = payload_field_value(payload, field) else {
                        continue;
                    };
                    values.push((None, value));
                }
            }
            crate::transactions::data::MutationIntent::Entity(
                crate::transactions::data::EntityMutationIntent::Update(update),
            ) => {
                let Some(value) = payload_field_value(&update.payload, field) else {
                    continue;
                };
                values.push((Some(update.entity_id), value));
            }
            crate::transactions::data::MutationIntent::Entity(
                crate::transactions::data::EntityMutationIntent::Replace(replace),
            ) => {
                let Some(value) = payload_field_value(&replace.replacement.payload, field) else {
                    continue;
                };
                values.push((Some(replace.entity_id), value));
            }
            _ => {}
        }
    }
    Some(values)
}

fn planned_unique_entity_payload_violation(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    field: &str,
) -> Option<InvariantViolation> {
    let planned_values = planned_entity_field_values(context.merged_plan(), field)?;
    let mut planned_value_to_entity = HashMap::with_capacity(planned_values.len());
    for (entity_id, value) in planned_values {
        context.metrics().count_entity_slot_scans(1);
        if let Some(existing_entity_id) = planned_value_to_entity.insert(value.clone(), entity_id) {
            if existing_entity_id != entity_id || entity_id.is_none() {
                return Some(duplicate_field_violation(class, field, &value));
            }
        }
        if context
            .indexes()
            .conflicts_with_entity_value(field, &value, entity_id)
        {
            return Some(duplicate_field_violation(class, field, &value));
        }
    }
    None
}

fn payload_field_value(
    payload: &crate::payloads::data::RecordPayload,
    field: &str,
) -> Option<String> {
    payload
        .as_json()?
        .get(field)
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned)
}

fn duplicate_field_violation(
    class: InvariantClass,
    field: &str,
    value: &str,
) -> InvariantViolation {
    InvariantViolation {
        class,
        code: DiagnosticCode::InvariantViolation,
        detail: format!(
            "entity payload field '{}' must be unique, duplicate value '{}'",
            field, value
        ),
        fields: InvariantViolationFields::UniqueEntityPayloadField {
            field: field.to_string(),
            value: value.to_string(),
        },
    }
}

fn evaluate_payload_schema_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredPayloadSchemaContract,
) -> Vec<InvariantViolation> {
    context.metrics().count_relation_contracts_evaluated(1);
    let mut violations = Vec::new();
    match contract.record_kind {
        PayloadContractRecordKind::Entity => {
            if let Some(plan) = context.merged_plan() {
                for intent in &plan.merged_intents {
                    match intent {
                        crate::transactions::data::MutationIntent::Create(
                            crate::transactions::data::CreateIntent::Entity(spec),
                        ) if spec.kind_id == contract.kind_id => {
                            if let Some(violation) =
                                payload_schema_violation_for_payload(class, contract, &spec.payload)
                            {
                                violations.push(violation);
                            }
                        }
                        crate::transactions::data::MutationIntent::Create(
                            crate::transactions::data::CreateIntent::BulkEntities(spec),
                        ) if spec.kind_id == contract.kind_id => {
                            for payload in &spec.payloads {
                                if let Some(violation) =
                                    payload_schema_violation_for_payload(class, contract, payload)
                                {
                                    violations.push(violation);
                                }
                            }
                        }
                        crate::transactions::data::MutationIntent::Entity(
                            crate::transactions::data::EntityMutationIntent::Update(update),
                        ) => {
                            let Ok(Some(kind_id)) =
                                entity_kind_in_state(context, class, update.entity_id)
                            else {
                                continue;
                            };
                            if kind_id == contract.kind_id {
                                if let Some(violation) = payload_schema_violation_for_payload(
                                    class,
                                    contract,
                                    &update.payload,
                                ) {
                                    violations.push(violation);
                                }
                            }
                        }
                        crate::transactions::data::MutationIntent::Entity(
                            crate::transactions::data::EntityMutationIntent::Replace(replace),
                        ) if replace.replacement.kind_id == contract.kind_id => {
                            if let Some(violation) = payload_schema_violation_for_payload(
                                class,
                                contract,
                                &replace.replacement.payload,
                            ) {
                                violations.push(violation);
                            }
                        }
                        _ => {}
                    }
                }
            }
            if let Some(entity_ids) = context.state_view().touched_visible_entity_ids() {
                for entity_id in entity_ids {
                    let Ok(Some(kind_id)) = entity_kind_in_state(context, class, entity_id) else {
                        continue;
                    };
                    if kind_id != contract.kind_id {
                        continue;
                    }
                    context.metrics().count_entity_slot_scans(1);
                    let Some(payload) = context.state_view().entity_payload(entity_id) else {
                        continue;
                    };
                    if let Some(violation) =
                        payload_schema_violation_for_payload(class, contract, payload)
                    {
                        violations.push(violation);
                    }
                }
            }
        }
        PayloadContractRecordKind::Relation => {
            if let Some(plan) = context.merged_plan() {
                for intent in &plan.merged_intents {
                    match intent {
                        crate::transactions::data::MutationIntent::Create(
                            crate::transactions::data::CreateIntent::Relation(spec),
                        ) if spec.kind_id == contract.kind_id => {
                            if let Some(payload) = &spec.payload {
                                if let Some(violation) =
                                    payload_schema_violation_for_payload(class, contract, payload)
                                {
                                    violations.push(violation);
                                }
                            } else if let Some(violation) =
                                payload_schema_violation_for_missing_payload(class, contract)
                            {
                                violations.push(violation);
                            }
                        }
                        crate::transactions::data::MutationIntent::Create(
                            crate::transactions::data::CreateIntent::BulkRelations(spec),
                        ) if spec.kind_id == contract.kind_id => {
                            for payload in &spec.payloads {
                                match payload {
                                    Some(payload) => {
                                        if let Some(violation) =
                                            payload_schema_violation_for_payload(
                                                class, contract, payload,
                                            )
                                        {
                                            violations.push(violation);
                                        }
                                    }
                                    None => {
                                        if let Some(violation) =
                                            payload_schema_violation_for_missing_payload(
                                                class, contract,
                                            )
                                        {
                                            violations.push(violation);
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            if let Some(relation_ids) = context.state_view().touched_visible_relation_ids() {
                for relation_id in relation_ids {
                    let Some(metadata) = context.state_view().relation_metadata(relation_id) else {
                        continue;
                    };
                    if metadata.kind_id != contract.kind_id {
                        continue;
                    }
                    context.metrics().count_relation_slot_scans(1);
                    let Some(payload) = context.state_view().relation_payload(relation_id) else {
                        continue;
                    };
                    if let Some(violation) =
                        payload_schema_violation_for_payload(class, contract, payload)
                    {
                        violations.push(violation);
                    }
                }
            }
        }
    }
    canonicalize_violations(violations)
}

fn evaluate_partition_isolation_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredPartitionIsolationContract,
) -> Vec<InvariantViolation> {
    context.metrics().count_relation_contracts_evaluated(1);
    let mut violations = Vec::new();
    if let Some(plan) = context.merged_plan() {
        for intent in &plan.merged_intents {
            match intent {
                crate::transactions::data::MutationIntent::Create(
                    crate::transactions::data::CreateIntent::Relation(spec),
                ) if spec.kind_id == contract.relation_kind_id => {
                    context.metrics().count_relation_slot_scans(1);
                    if spec.source.partition_id != spec.target.partition_id {
                        violations.push(InvariantViolation {
                            class,
                            code: DiagnosticCode::InvariantViolation,
                            detail: format!(
                                "relation contract '{}' requires same-partition endpoints for relation kind {:?}",
                                contract.contract_id, contract.relation_kind_id
                            ),
                            fields: InvariantViolationFields::PartitionIsolation {
                                contract_id: contract.contract_id.clone(),
                                relation_kind_id: contract.relation_kind_id,
                                relation_id: None,
                                source_partition_id: spec.source.partition_id,
                                target_partition_id: spec.target.partition_id,
                            },
                        });
                    }
                }
                crate::transactions::data::MutationIntent::Create(
                    crate::transactions::data::CreateIntent::BulkRelations(spec),
                ) if spec.kind_id == contract.relation_kind_id => {
                    for (source, target) in &spec.endpoints {
                        context.metrics().count_relation_slot_scans(1);
                        if source.partition_id != target.partition_id {
                            violations.push(InvariantViolation {
                                class,
                                code: DiagnosticCode::InvariantViolation,
                                detail: format!(
                                    "relation contract '{}' requires same-partition endpoints for relation kind {:?}",
                                    contract.contract_id, contract.relation_kind_id
                                ),
                                fields: InvariantViolationFields::PartitionIsolation {
                                    contract_id: contract.contract_id.clone(),
                                    relation_kind_id: contract.relation_kind_id,
                                    relation_id: None,
                                    source_partition_id: source.partition_id,
                                    target_partition_id: target.partition_id,
                                },
                            });
                        }
                    }
                }
                _ => {}
            }
        }
    }

    if let Some(relation_ids) = context.state_view().touched_visible_relation_ids() {
        for relation_id in relation_ids {
            context.metrics().count_relation_slot_scans(1);
            let Some(metadata) = context.state_view().relation_metadata(relation_id) else {
                continue;
            };
            if metadata.kind_id != contract.relation_kind_id {
                continue;
            }
            if metadata.source.partition_id != metadata.target.partition_id {
                violations.push(InvariantViolation {
                    class,
                    code: DiagnosticCode::InvariantViolation,
                    detail: format!(
                        "relation contract '{}' requires same-partition endpoints for relation kind {:?}",
                        contract.contract_id, contract.relation_kind_id
                    ),
                    fields: InvariantViolationFields::PartitionIsolation {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        relation_id: Some(metadata.relation_id),
                        source_partition_id: metadata.source.partition_id,
                        target_partition_id: metadata.target.partition_id,
                    },
                });
            }
        }
    }
    canonicalize_violations(violations)
}

fn payload_schema_violation_for_missing_payload(
    class: InvariantClass,
    contract: &LoweredPayloadSchemaContract,
) -> Option<InvariantViolation> {
    if contract.allowed_payload_class == crate::payloads::data::PayloadClass::OpaqueBytes
        && contract.field_constraints.is_empty()
    {
        return None;
    }
    Some(InvariantViolation {
        class,
        code: DiagnosticCode::InvariantViolation,
        detail: format!(
            "payload schema contract '{}' requires a payload for {:?} kind {:?}",
            contract.contract_id, contract.record_kind, contract.kind_id
        ),
        fields: InvariantViolationFields::PayloadSchema {
            contract_id: contract.contract_id.clone(),
            record_kind: contract.record_kind,
            kind_id: contract.kind_id,
            field: "__payload__".to_string(),
            failure_kind: "missing_payload".to_string(),
            expected_type: None,
        },
    })
}

fn payload_schema_violation_for_payload(
    class: InvariantClass,
    contract: &LoweredPayloadSchemaContract,
    payload: &crate::payloads::data::RecordPayload,
) -> Option<InvariantViolation> {
    if payload.payload_class() != contract.allowed_payload_class {
        return Some(InvariantViolation {
            class,
            code: DiagnosticCode::InvariantViolation,
            detail: format!(
                "payload schema contract '{}' expected {:?} payloads for {:?} kind {:?}",
                contract.contract_id,
                contract.allowed_payload_class,
                contract.record_kind,
                contract.kind_id
            ),
            fields: InvariantViolationFields::PayloadSchema {
                contract_id: contract.contract_id.clone(),
                record_kind: contract.record_kind,
                kind_id: contract.kind_id,
                field: "__payload__".to_string(),
                failure_kind: "payload_class".to_string(),
                expected_type: None,
            },
        });
    }

    let json = payload.as_json();
    for constraint in &contract.field_constraints {
        match constraint {
            PayloadFieldConstraint::Required { field } => {
                let has_field = json
                    .and_then(|value| value.as_object())
                    .is_some_and(|object| object.contains_key(field));
                if !has_field {
                    return Some(InvariantViolation {
                        class,
                        code: DiagnosticCode::InvariantViolation,
                        detail: format!(
                            "payload schema contract '{}' requires field '{}' on {:?} kind {:?}",
                            contract.contract_id, field, contract.record_kind, contract.kind_id
                        ),
                        fields: InvariantViolationFields::PayloadSchema {
                            contract_id: contract.contract_id.clone(),
                            record_kind: contract.record_kind,
                            kind_id: contract.kind_id,
                            field: field.clone(),
                            failure_kind: "required".to_string(),
                            expected_type: None,
                        },
                    });
                }
            }
            PayloadFieldConstraint::Type { field, expected } => {
                let value = json
                    .and_then(|value| value.as_object())
                    .and_then(|object| object.get(field));
                let Some(value) = value else {
                    return Some(InvariantViolation {
                        class,
                        code: DiagnosticCode::InvariantViolation,
                        detail: format!(
                            "payload schema contract '{}' requires typed field '{}' on {:?} kind {:?}",
                            contract.contract_id, field, contract.record_kind, contract.kind_id
                        ),
                        fields: InvariantViolationFields::PayloadSchema {
                            contract_id: contract.contract_id.clone(),
                            record_kind: contract.record_kind,
                            kind_id: contract.kind_id,
                            field: field.clone(),
                            failure_kind: "missing_for_type".to_string(),
                            expected_type: Some(*expected),
                        },
                    });
                };
                if payload_value_type(value) != *expected {
                    return Some(InvariantViolation {
                        class,
                        code: DiagnosticCode::InvariantViolation,
                        detail: format!(
                            "payload schema contract '{}' rejected field '{}' type on {:?} kind {:?}",
                            contract.contract_id, field, contract.record_kind, contract.kind_id
                        ),
                        fields: InvariantViolationFields::PayloadSchema {
                            contract_id: contract.contract_id.clone(),
                            record_kind: contract.record_kind,
                            kind_id: contract.kind_id,
                            field: field.clone(),
                            failure_kind: "type".to_string(),
                            expected_type: Some(*expected),
                        },
                    });
                }
            }
        }
    }
    None
}

fn payload_value_type(value: &serde_json::Value) -> PayloadSchemaValueType {
    match value {
        serde_json::Value::Null => PayloadSchemaValueType::Null,
        serde_json::Value::Bool(_) => PayloadSchemaValueType::Boolean,
        serde_json::Value::Number(_) => PayloadSchemaValueType::Number,
        serde_json::Value::String(_) => PayloadSchemaValueType::String,
        serde_json::Value::Array(_) => PayloadSchemaValueType::Array,
        serde_json::Value::Object(_) => PayloadSchemaValueType::Object,
    }
}

fn evaluate_acyclicity_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredAcyclicityContract,
) -> Option<InvariantViolation> {
    let Some(scope) = context.relation_integrity_scope(contract.relation_kind_id) else {
        return None;
    };
    if scope.planned_edges.is_empty() {
        return None;
    }

    context.metrics().count_relation_contracts_evaluated(1);
    let planned_successors = planned_successor_map(&scope.planned_edges);
    for edge in &scope.planned_edges {
        context.metrics().count_relation_slot_scans(1);
        let reaches_cycle = if edge.source == edge.target {
            Ok(true)
        } else {
            relation_kind_reaches(
                context,
                class,
                &contract.contract_id,
                contract.relation_kind_id,
                edge.target,
                edge.source,
                &planned_successors,
            )
        };
        match reaches_cycle {
            Ok(true) => {
                return Some(InvariantViolation {
                    class,
                    code: DiagnosticCode::InvariantViolation,
                    detail: format!(
                        "acyclicity contract '{}' detected a cycle for relation kind {:?}",
                        contract.contract_id, contract.relation_kind_id
                    ),
                    fields: InvariantViolationFields::Acyclicity {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        source: edge.source,
                        target: edge.target,
                    },
                });
            }
            Ok(false) => {}
            Err(violation) => return Some(violation),
        }
    }
    None
}

fn evaluate_connectivity_minimum_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredConnectivityMinimumContract,
) -> Option<InvariantViolation> {
    context.metrics().count_relation_contracts_evaluated(1);
    let source_entities = visible_entities_of_kinds(context, &contract.source_kind_ids);
    if source_entities.is_empty() {
        return None;
    }

    let planned_successors = context
        .relation_integrity_scope(contract.relation_kind_id)
        .map(|scope| planned_successor_map(&scope.planned_edges))
        .unwrap_or_default();
    for source in source_entities {
        let reachable_target_count = match reachable_target_count_for_connectivity(
            context,
            class,
            &contract.contract_id,
            contract.relation_kind_id,
            source,
            &contract.target_kind_ids,
            &planned_successors,
        ) {
            Ok(count) => count,
            Err(violation) => return Some(violation),
        };
        if reachable_target_count < contract.minimum_reachable_targets as usize {
            return Some(InvariantViolation {
                class,
                code: DiagnosticCode::InvariantViolation,
                detail: format!(
                    "connectivity minimum contract '{}' requires at least {} reachable target(s) for {:?}",
                    contract.contract_id,
                    contract.minimum_reachable_targets,
                    source
                ),
                fields: InvariantViolationFields::ConnectivityMinimum {
                    contract_id: contract.contract_id.clone(),
                    relation_kind_id: contract.relation_kind_id,
                    source,
                    reachable_target_count,
                    minimum_reachable_targets: contract.minimum_reachable_targets,
                },
            });
        }
    }
    None
}

fn visible_entities_of_kinds(
    context: &InvariantExecutionContext<'_>,
    kind_ids: &[crate::identity::data::KindId],
) -> Vec<crate::identity::data::EntityId> {
    let state_view = context.state_view();
    let mut entities = Vec::new();
    for partition_id in state_view.state().partition_ids() {
        let Some(partition) = state_view.state().get_partition(partition_id) else {
            continue;
        };
        if state_view.version_id() == context.current_version_id() {
            for slot in partition.entity_arena.live_bitset.iter_set_slots() {
                context.metrics().count_entity_slot_scans(1);
                let Some(slot_view) = partition.entity_arena.get_slot(slot) else {
                    continue;
                };
                let Some(kind_id) = slot_view.kind_id() else {
                    continue;
                };
                if contract_candidate_kind_matches(kind_id, kind_ids) {
                    entities.push(crate::identity::data::EntityId::new(
                        partition_id,
                        slot as u64,
                        slot_view.generation(),
                    ));
                }
            }
        } else {
            for slot in 0..partition.entity_arena.slot_count() {
                let Some(metadata) =
                    state_view.entity_metadata_at(&partition.entity_arena, partition_id, slot)
                else {
                    continue;
                };
                context.metrics().count_entity_slot_scans(1);
                if contract_candidate_kind_matches(metadata.kind_id, kind_ids) {
                    entities.push(metadata.entity_id);
                }
            }
        }
    }
    entities
}

fn reachable_target_count_for_connectivity(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract_id: &crate::schema::data::ContractId,
    relation_kind_id: crate::identity::data::KindId,
    source: crate::identity::data::EntityId,
    target_kind_ids: &[crate::identity::data::KindId],
    planned_successors: &BTreeMap<
        crate::identity::data::EntityId,
        Vec<crate::identity::data::EntityId>,
    >,
) -> Result<usize, InvariantViolation> {
    let mut visited = BTreeSet::new();
    let mut frontier = vec![source];
    let mut reachable_targets = BTreeSet::new();
    let planned_edge_count = planned_successor_count(planned_successors);
    let mut traversal_budget = RelationTraversalBudget::new(
        context.relation_integrity_scope_budget(),
        planned_edge_count,
    );
    visited.insert(source);
    if traversal_budget.record_entity_visit().is_err() {
        return Err(traversal_budget_exceeded_violation(
            class,
            contract_id,
            relation_kind_id,
            traversal_budget,
            planned_edge_count,
        ));
    }

    while let Some(entity_id) = frontier.pop() {
        for next in relation_kind_successors(
            context,
            class,
            contract_id,
            relation_kind_id,
            entity_id,
            planned_successors,
            &mut traversal_budget,
        )? {
            if !visited.insert(next) {
                continue;
            }
            if traversal_budget.record_entity_visit().is_err() {
                return Err(traversal_budget_exceeded_violation(
                    class,
                    contract_id,
                    relation_kind_id,
                    traversal_budget,
                    planned_edge_count,
                ));
            }
            if let Some(kind_id) = context
                .state_view()
                .entity_metadata(next)
                .map(|metadata| metadata.kind_id)
            {
                if contract_candidate_kind_matches(kind_id, target_kind_ids) {
                    reachable_targets.insert(next);
                }
            }
            frontier.push(next);
        }
    }

    Ok(reachable_targets.len())
}

fn relation_kind_reaches(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract_id: &crate::schema::data::ContractId,
    relation_kind_id: crate::identity::data::KindId,
    start: crate::identity::data::EntityId,
    target: crate::identity::data::EntityId,
    planned_successors: &BTreeMap<
        crate::identity::data::EntityId,
        Vec<crate::identity::data::EntityId>,
    >,
) -> Result<bool, InvariantViolation> {
    let mut visited = BTreeSet::new();
    let mut frontier = vec![start];
    let planned_edge_count = planned_successor_count(planned_successors);
    let mut traversal_budget = RelationTraversalBudget::new(
        context.relation_integrity_scope_budget(),
        planned_edge_count,
    );
    visited.insert(start);
    if traversal_budget.record_entity_visit().is_err() {
        return Err(traversal_budget_exceeded_violation(
            class,
            contract_id,
            relation_kind_id,
            traversal_budget,
            planned_edge_count,
        ));
    }

    while let Some(entity_id) = frontier.pop() {
        for next in relation_kind_successors(
            context,
            class,
            contract_id,
            relation_kind_id,
            entity_id,
            planned_successors,
            &mut traversal_budget,
        )? {
            if next == target {
                return Ok(true);
            }
            if visited.insert(next) {
                if traversal_budget.record_entity_visit().is_err() {
                    return Err(traversal_budget_exceeded_violation(
                        class,
                        contract_id,
                        relation_kind_id,
                        traversal_budget,
                        planned_edge_count,
                    ));
                }
                frontier.push(next);
            }
        }
    }

    Ok(false)
}

fn relation_kind_successors(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract_id: &crate::schema::data::ContractId,
    relation_kind_id: crate::identity::data::KindId,
    entity_id: crate::identity::data::EntityId,
    planned_successors: &BTreeMap<
        crate::identity::data::EntityId,
        Vec<crate::identity::data::EntityId>,
    >,
    traversal_budget: &mut RelationTraversalBudget,
) -> Result<Vec<crate::identity::data::EntityId>, InvariantViolation> {
    let mut successors = BTreeSet::new();
    let planned_edge_count = planned_successor_count(planned_successors);
    if let Some(edges) = planned_successors.get(&entity_id) {
        for &target in edges {
            if traversal_budget.record_relation_scan().is_err() {
                return Err(traversal_budget_exceeded_violation(
                    class,
                    contract_id,
                    relation_kind_id,
                    *traversal_budget,
                    planned_edge_count,
                ));
            }
            successors.insert(target);
        }
    }
    let Some(partition) = context
        .partition_access()
        .get_partition(entity_id.partition_id)
    else {
        return Ok(successors.into_iter().collect());
    };
    let slot = entity_id.local_slot.0 as usize;
    let outgoing = partition
        .adjacency
        .get(slot)
        .map(|set| set.as_slice())
        .into_iter()
        .flatten();
    for relation_id in outgoing.copied() {
        context.metrics().count_relation_slot_scans(1);
        if traversal_budget.record_relation_scan().is_err() {
            return Err(traversal_budget_exceeded_violation(
                class,
                contract_id,
                relation_kind_id,
                *traversal_budget,
                planned_edge_count,
            ));
        }
        let Some(metadata) = context.state_view().relation_metadata(relation_id) else {
            continue;
        };
        if metadata.kind_id == relation_kind_id {
            successors.insert(metadata.target);
        }
    }
    Ok(successors.into_iter().collect())
}

#[derive(Debug, Clone, Copy)]
struct RelationTraversalBudget {
    max_relation_scans: usize,
    max_visited_entities: usize,
    relation_scans: usize,
    visited_entities: usize,
}

impl RelationTraversalBudget {
    fn new(
        budget: &crate::config::data::RelationIntegrityScopeBudget,
        planned_edge_count: usize,
    ) -> Self {
        Self {
            max_relation_scans: budget
                .max_scanned_relations
                .saturating_add(planned_edge_count),
            max_visited_entities: budget
                .max_scanned_relations
                .saturating_add(planned_edge_count)
                .saturating_add(1),
            relation_scans: 0,
            visited_entities: 0,
        }
    }

    fn record_relation_scan(&mut self) -> Result<(), ()> {
        self.relation_scans = self.relation_scans.saturating_add(1);
        if self.relation_scans > self.max_relation_scans {
            return Err(());
        }
        Ok(())
    }

    fn record_entity_visit(&mut self) -> Result<(), ()> {
        self.visited_entities = self.visited_entities.saturating_add(1);
        if self.visited_entities > self.max_visited_entities {
            return Err(());
        }
        Ok(())
    }
}

fn planned_successor_map(
    planned_edges: &[super::request::PlannedRelationEdge],
) -> BTreeMap<crate::identity::data::EntityId, Vec<crate::identity::data::EntityId>> {
    let mut successors = BTreeMap::new();
    for edge in planned_edges {
        successors
            .entry(edge.source)
            .or_insert_with(Vec::new)
            .push(edge.target);
    }
    successors
}

fn planned_successor_count(
    planned_successors: &BTreeMap<
        crate::identity::data::EntityId,
        Vec<crate::identity::data::EntityId>,
    >,
) -> usize {
    planned_successors.values().map(Vec::len).sum()
}

fn traversal_budget_exceeded_violation(
    class: InvariantClass,
    contract_id: &crate::schema::data::ContractId,
    relation_kind_id: crate::identity::data::KindId,
    traversal_budget: RelationTraversalBudget,
    planned_edge_count: usize,
) -> InvariantViolation {
    InvariantViolation {
        class,
        code: DiagnosticCode::InvariantViolation,
        detail: format!(
            "relation contract '{}' exceeded evaluator traversal budget for relation kind {:?}",
            contract_id, relation_kind_id
        ),
        fields: InvariantViolationFields::RelationIntegrityScopeBudgetExceeded {
            limit_name: "max_scanned_relations".to_string(),
            limit: traversal_budget.max_relation_scans,
            observed: traversal_budget.relation_scans,
            relation_kind_count: 1,
            touched_entity_count: traversal_budget.visited_entities,
            deleted_entity_count: 0,
            scanned_relation_count: traversal_budget.relation_scans,
            planned_edge_count,
        },
    }
}

fn evaluate_endpoint_kind_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredEndpointKindContract,
) -> Vec<InvariantViolation> {
    let Some(scope) = context.relation_integrity_scope(contract.relation_kind_id) else {
        return Vec::new();
    };
    if scope.planned_edges.is_empty() {
        return Vec::new();
    }

    context.metrics().count_relation_contracts_evaluated(1);
    let mut violations = Vec::new();
    for edge in &scope.planned_edges {
        context.metrics().count_relation_endpoint_kind_checks(1);
        let source_kind = match entity_kind_in_state(context, class, edge.source) {
            Ok(Some(kind_id)) => kind_id,
            Ok(None) => continue,
            Err(violation) => {
                violations.push(violation);
                continue;
            }
        };
        let target_kind = match entity_kind_in_state(context, class, edge.target) {
            Ok(Some(kind_id)) => kind_id,
            Ok(None) => continue,
            Err(violation) => {
                violations.push(violation);
                continue;
            }
        };
        if !contract.allows_source_kind(source_kind) {
            violations.push(relation_violation(
                class,
                DiagnosticCode::RelationEndpointKindViolation,
                format!(
                    "relation contract '{}' rejected source kind {:?} for relation kind {:?}",
                    contract.contract_id, source_kind, contract.relation_kind_id
                ),
                InvariantViolationFields::RelationEndpointKindMismatch {
                    contract_id: contract.contract_id.clone(),
                    relation_kind_id: contract.relation_kind_id,
                    source: edge.source,
                    target: edge.target,
                    source_kind_id: source_kind,
                    target_kind_id: target_kind,
                    boundary: RelationEndpointBoundary::Source,
                },
            ));
        }
        if !contract.allows_target_kind(target_kind) {
            violations.push(relation_violation(
                class,
                DiagnosticCode::RelationEndpointKindViolation,
                format!(
                    "relation contract '{}' rejected target kind {:?} for relation kind {:?}",
                    contract.contract_id, target_kind, contract.relation_kind_id
                ),
                InvariantViolationFields::RelationEndpointKindMismatch {
                    contract_id: contract.contract_id.clone(),
                    relation_kind_id: contract.relation_kind_id,
                    source: edge.source,
                    target: edge.target,
                    source_kind_id: source_kind,
                    target_kind_id: target_kind,
                    boundary: RelationEndpointBoundary::Target,
                },
            ));
        }
        if !contract.self_edges_allowed && edge.source == edge.target {
            violations.push(relation_violation(
                class,
                DiagnosticCode::RelationEndpointKindViolation,
                format!(
                    "relation contract '{}' forbids self edges for relation kind {:?}",
                    contract.contract_id, contract.relation_kind_id
                ),
                InvariantViolationFields::RelationEndpointKindSelfEdge {
                    contract_id: contract.contract_id.clone(),
                    relation_kind_id: contract.relation_kind_id,
                    source: edge.source,
                    target: edge.target,
                    self_edge: true,
                },
            ));
        }
        if edge.source.partition_id != edge.target.partition_id
            && contract.cross_context_policy
                != crate::config::data::CrossContextPolicy::AllowExplicit
        {
            violations.push(relation_violation(
                class,
                DiagnosticCode::InvalidRelationEndpoint,
                format!(
                    "relation contract '{}' forbids cross-context endpoints for relation kind {:?}",
                    contract.contract_id, contract.relation_kind_id
                ),
                InvariantViolationFields::RelationEndpointKindCrossContext {
                    contract_id: contract.contract_id.clone(),
                    relation_kind_id: contract.relation_kind_id,
                    source_partition_id: edge.source.partition_id,
                    target_partition_id: edge.target.partition_id,
                },
            ));
        }
    }
    canonicalize_violations(violations)
}

fn evaluate_cardinality_maximum_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredCardinalityMaximumContract,
) -> Vec<InvariantViolation> {
    let Some(scope) = context.relation_integrity_scope(contract.relation_kind_id) else {
        return Vec::new();
    };
    if scope.is_empty() {
        return Vec::new();
    }
    context.metrics().count_relation_contracts_evaluated(1);
    let mut violations = Vec::new();
    for (key, count) in &scope.source_counts {
        if let Some(limit) = contract.source_max {
            context.metrics().count_relation_cardinality_checks(1);
            if (*count as u64) > limit {
                violations.push(relation_violation(
                    class,
                    DiagnosticCode::RelationCardinalityViolation,
                    format!(
                        "relation contract '{}' overflowed source cardinality for entity {:?}: {} > {}",
                        contract.contract_id, key.entity_id, count, limit
                    ),
                    InvariantViolationFields::RelationCardinalityEndpoint {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        entity_id: key.entity_id,
                        boundary: RelationCardinalityBoundary::Source,
                        count: *count,
                        limit,
                    },
                ));
            }
        }
    }
    for (key, count) in &scope.target_counts {
        if let Some(limit) = contract.target_max {
            context.metrics().count_relation_cardinality_checks(1);
            if (*count as u64) > limit {
                violations.push(relation_violation(
                    class,
                    DiagnosticCode::RelationCardinalityViolation,
                    format!(
                        "relation contract '{}' overflowed target cardinality for entity {:?}: {} > {}",
                        contract.contract_id, key.entity_id, count, limit
                    ),
                    InvariantViolationFields::RelationCardinalityEndpoint {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        entity_id: key.entity_id,
                        boundary: RelationCardinalityBoundary::Target,
                        count: *count,
                        limit,
                    },
                ));
            }
        }
    }
    for (key, count) in &scope.directed_pair_counts {
        if let Some(limit) = contract.pair_max {
            context.metrics().count_relation_cardinality_checks(1);
            if (*count as u64) > limit {
                violations.push(relation_violation(
                    class,
                    DiagnosticCode::RelationCardinalityViolation,
                    format!(
                        "relation contract '{}' overflowed pair cardinality for {:?}->{:?}: {} > {}",
                        contract.contract_id, key.source, key.target, count, limit
                    ),
                    InvariantViolationFields::RelationCardinalityPair {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        source: key.source,
                        target: key.target,
                        count: *count,
                        limit,
                    },
                ));
            }
        }
    }
    canonicalize_violations(violations)
}

fn evaluate_cardinality_minimum_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredCardinalityMinimumContract,
) -> Vec<InvariantViolation> {
    let snapshot = visible_relation_counts(context, contract);
    context.metrics().count_relation_contracts_evaluated(1);
    context
        .metrics()
        .count_relation_cardinality_minimum_certification(
            1,
            snapshot.entity_slot_scans,
            snapshot.relation_slot_scans,
        );
    let mut violations = Vec::new();

    if let Some(minimum) = contract.source_min {
        for entity_id in snapshot
            .candidate_source_entities
            .iter()
            .copied()
            .collect::<Vec<_>>()
        {
            let count = snapshot
                .source_counts
                .get(&entity_id)
                .copied()
                .unwrap_or_default();
            context.metrics().count_relation_cardinality_checks(1);
            if (count as u64) < minimum {
                violations.push(relation_violation(
                    class,
                    DiagnosticCode::RelationCardinalityViolation,
                    format!(
                        "relation contract '{}' underflowed source cardinality for entity {:?}: {} < {}",
                        contract.contract_id, entity_id, count, minimum
                    ),
                    InvariantViolationFields::RelationCardinalityEndpoint {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        entity_id,
                        boundary: RelationCardinalityBoundary::Source,
                        count,
                        limit: minimum,
                    },
                ));
            }
        }
    }

    if let Some(minimum) = contract.target_min {
        for entity_id in snapshot
            .candidate_target_entities
            .iter()
            .copied()
            .collect::<Vec<_>>()
        {
            let count = snapshot
                .target_counts
                .get(&entity_id)
                .copied()
                .unwrap_or_default();
            context.metrics().count_relation_cardinality_checks(1);
            if (count as u64) < minimum {
                violations.push(relation_violation(
                    class,
                    DiagnosticCode::RelationCardinalityViolation,
                    format!(
                        "relation contract '{}' underflowed target cardinality for entity {:?}: {} < {}",
                        contract.contract_id, entity_id, count, minimum
                    ),
                    InvariantViolationFields::RelationCardinalityEndpoint {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        entity_id,
                        boundary: RelationCardinalityBoundary::Target,
                        count,
                        limit: minimum,
                    },
                ));
            }
        }
    }

    if let Some(minimum) = contract.pair_min {
        match contract.pair_min_semantics {
            PairMinimumSemantics::ObservedDirectedPairs => {
                for ((source, target), count) in snapshot.directed_pair_counts {
                    context.metrics().count_relation_cardinality_checks(1);
                    if (count as u64) < minimum {
                        violations.push(relation_violation(
                            class,
                            DiagnosticCode::RelationCardinalityViolation,
                            format!(
                                "relation contract '{}' underflowed observed directed pair cardinality for {:?}->{:?}: {} < {}",
                                contract.contract_id, source, target, count, minimum
                            ),
                            InvariantViolationFields::RelationCardinalityPair {
                                contract_id: contract.contract_id.clone(),
                                relation_kind_id: contract.relation_kind_id,
                                source,
                                target,
                                count,
                                limit: minimum,
                            },
                        ));
                    }
                }
            }
        }
    }

    canonicalize_violations(violations)
}

#[derive(Default)]
struct VisibleRelationCountSnapshot {
    source_counts: BTreeMap<crate::identity::data::EntityId, usize>,
    target_counts: BTreeMap<crate::identity::data::EntityId, usize>,
    directed_pair_counts: BTreeMap<
        (
            crate::identity::data::EntityId,
            crate::identity::data::EntityId,
        ),
        usize,
    >,
    candidate_source_entities: BTreeSet<crate::identity::data::EntityId>,
    candidate_target_entities: BTreeSet<crate::identity::data::EntityId>,
    relation_slot_scans: usize,
    entity_slot_scans: usize,
}

fn visible_relation_counts(
    context: &InvariantExecutionContext<'_>,
    contract: &LoweredCardinalityMinimumContract,
) -> VisibleRelationCountSnapshot {
    let state_view = context.state_view();
    let mut snapshot = VisibleRelationCountSnapshot::default();

    for partition_id in state_view.state().partition_ids() {
        let Some(partition) = state_view.state().get_partition(partition_id) else {
            continue;
        };
        if state_view.version_id() == context.current_version_id() {
            for slot in partition.relation_arena.live_bitset.iter_set_slots() {
                context.metrics().count_relation_slot_scans(1);
                snapshot.relation_slot_scans += 1;
                let Some(slot_view) = partition.relation_arena.get_slot(slot) else {
                    continue;
                };
                let Some(kind_id) = slot_view.kind_id() else {
                    continue;
                };
                if kind_id != contract.relation_kind_id {
                    continue;
                }
                let Some(endpoints) = slot_view.extra().as_ref() else {
                    continue;
                };
                *snapshot.source_counts.entry(endpoints.source).or_insert(0) += 1;
                *snapshot.target_counts.entry(endpoints.target).or_insert(0) += 1;
                *snapshot
                    .directed_pair_counts
                    .entry((endpoints.source, endpoints.target))
                    .or_insert(0) += 1;
            }

            for slot in partition.entity_arena.live_bitset.iter_set_slots() {
                context.metrics().count_entity_slot_scans(1);
                snapshot.entity_slot_scans += 1;
                let Some(slot_view) = partition.entity_arena.get_slot(slot) else {
                    continue;
                };
                let Some(kind_id) = slot_view.kind_id() else {
                    continue;
                };
                let entity_id = crate::identity::data::EntityId::new(
                    partition_id,
                    slot as u64,
                    slot_view.generation(),
                );
                if contract_candidate_kind_matches(kind_id, &contract.candidate_source_kinds) {
                    snapshot.candidate_source_entities.insert(entity_id);
                }
                if contract_candidate_kind_matches(kind_id, &contract.candidate_target_kinds) {
                    snapshot.candidate_target_entities.insert(entity_id);
                }
            }
        } else {
            for slot in 0..partition.relation_arena.slot_count() {
                context.metrics().count_relation_slot_scans(1);
                snapshot.relation_slot_scans += 1;
                let Some(metadata) =
                    visible_relation_metadata(&state_view, &partition.relation_arena, slot)
                else {
                    continue;
                };
                if metadata.kind_id != contract.relation_kind_id {
                    continue;
                }
                *snapshot
                    .source_counts
                    .entry(metadata.endpoints.source)
                    .or_insert(0) += 1;
                *snapshot
                    .target_counts
                    .entry(metadata.endpoints.target)
                    .or_insert(0) += 1;
                *snapshot
                    .directed_pair_counts
                    .entry((metadata.endpoints.source, metadata.endpoints.target))
                    .or_insert(0) += 1;
            }

            for slot in 0..partition.entity_arena.slot_count() {
                context.metrics().count_entity_slot_scans(1);
                snapshot.entity_slot_scans += 1;
                let Some(metadata) =
                    state_view.entity_metadata_at(&partition.entity_arena, partition_id, slot)
                else {
                    continue;
                };
                if contract_candidate_kind_matches(
                    metadata.kind_id,
                    &contract.candidate_source_kinds,
                ) {
                    snapshot
                        .candidate_source_entities
                        .insert(metadata.entity_id);
                }
                if contract_candidate_kind_matches(
                    metadata.kind_id,
                    &contract.candidate_target_kinds,
                ) {
                    snapshot
                        .candidate_target_entities
                        .insert(metadata.entity_id);
                }
            }
        }
    }

    snapshot
}

fn contract_candidate_kind_matches(
    kind_id: crate::identity::data::KindId,
    candidate_kinds: &[crate::identity::data::KindId],
) -> bool {
    candidate_kinds.binary_search(&kind_id).is_ok()
}

fn visible_relation_metadata<'state>(
    state_view: &InvariantStateView<'state>,
    arena: &'state crate::storage::logic::state::RelationArena,
    slot: usize,
) -> Option<&'state crate::storage::logic::state::VersionedRelationMetadata> {
    let history = arena.metadata_history_at(slot)?;
    let end = history.partition_point(|entry| entry.effective_at() <= state_view.version_id());
    history[..end].iter().rev().find(|entry| {
        entry.effective_at() <= state_view.version_id()
            && entry
                .retired_at()
                .is_none_or(|retired| state_view.version_id() < retired)
    })
}

fn evaluate_uniqueness_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredUniquenessContract,
) -> Vec<InvariantViolation> {
    let Some(scope) = context.relation_integrity_scope(contract.relation_kind_id) else {
        return Vec::new();
    };
    if scope.is_empty() {
        return Vec::new();
    }
    context.metrics().count_relation_contracts_evaluated(1);
    let mut violations = Vec::new();
    match contract.scope {
        UniquenessScope::DirectedSemanticEdge => {
            for (key, count) in &scope.directed_pair_counts {
                context.metrics().count_relation_uniqueness_checks(1);
                if *count > 1 {
                    violations.push(relation_violation(
                        class,
                        DiagnosticCode::RelationUniquenessViolation,
                        format!(
                            "relation contract '{}' forbids duplicate directed edge {:?}->{:?}",
                            contract.contract_id, key.source, key.target
                        ),
                        InvariantViolationFields::RelationUniqueness {
                            contract_id: contract.contract_id.clone(),
                            relation_kind_id: contract.relation_kind_id,
                            scope: contract.scope,
                            source: key.source,
                            target: key.target,
                            count: *count,
                        },
                    ));
                }
            }
        }
        UniquenessScope::NormalizedSymmetricEdge => {
            for (key, count) in &scope.normalized_pair_counts {
                context.metrics().count_relation_uniqueness_checks(1);
                if *count > 1 {
                    violations.push(relation_violation(
                        class,
                        DiagnosticCode::RelationUniquenessViolation,
                        format!(
                            "relation contract '{}' forbids duplicate normalized edge {:?}<->{:?}",
                            contract.contract_id, key.source, key.target
                        ),
                        InvariantViolationFields::RelationUniqueness {
                            contract_id: contract.contract_id.clone(),
                            relation_kind_id: contract.relation_kind_id,
                            scope: contract.scope,
                            source: key.source,
                            target: key.target,
                            count: *count,
                        },
                    ));
                }
            }
        }
    }
    canonicalize_violations(violations)
}

fn evaluate_symmetry_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredSymmetryContract,
) -> Vec<InvariantViolation> {
    let Some(scope) = context.relation_integrity_scope(contract.relation_kind_id) else {
        return Vec::new();
    };
    if scope.planned_edges.is_empty() {
        return Vec::new();
    }
    context.metrics().count_relation_contracts_evaluated(1);
    let mut violations = Vec::new();
    for edge in &scope.planned_edges {
        context.metrics().count_relation_symmetry_checks(1);
        match contract.mode {
            SymmetryMode::CanonicalUndirected => {
                if edge.target < edge.source {
                    violations.push(relation_violation(
                        class,
                        DiagnosticCode::RelationSymmetryViolation,
                        format!(
                            "relation contract '{}' requires canonical undirected ordering",
                            contract.contract_id
                        ),
                        InvariantViolationFields::RelationSymmetry {
                            contract_id: contract.contract_id.clone(),
                            relation_kind_id: contract.relation_kind_id,
                            source: edge.source,
                            target: edge.target,
                            mode: contract.mode,
                        },
                    ));
                }
            }
            SymmetryMode::PairedInverseRequired | SymmetryMode::PairedTwinRequired => {
                let inverse = super::request::PreparedRelationPairKey {
                    source: edge.target,
                    target: edge.source,
                };
                if scope
                    .directed_pair_counts
                    .get(&inverse)
                    .copied()
                    .unwrap_or_default()
                    == 0
                {
                    violations.push(relation_violation(
                        class,
                        DiagnosticCode::RelationSymmetryViolation,
                        format!(
                            "relation contract '{}' requires an inverse/twin edge for {:?}->{:?}",
                            contract.contract_id, edge.source, edge.target
                        ),
                        InvariantViolationFields::RelationSymmetry {
                            contract_id: contract.contract_id.clone(),
                            relation_kind_id: contract.relation_kind_id,
                            source: edge.source,
                            target: edge.target,
                            mode: contract.mode,
                        },
                    ));
                }
            }
            SymmetryMode::InverseProhibited => {
                let inverse = super::request::PreparedRelationPairKey {
                    source: edge.target,
                    target: edge.source,
                };
                if scope
                    .directed_pair_counts
                    .get(&inverse)
                    .copied()
                    .unwrap_or_default()
                    > 0
                {
                    violations.push(relation_violation(
                        class,
                        DiagnosticCode::RelationSymmetryViolation,
                        format!(
                            "relation contract '{}' prohibits inverse duplication for {:?}->{:?}",
                            contract.contract_id, edge.source, edge.target
                        ),
                        InvariantViolationFields::RelationSymmetry {
                            contract_id: contract.contract_id.clone(),
                            relation_kind_id: contract.relation_kind_id,
                            source: edge.source,
                            target: edge.target,
                            mode: contract.mode,
                        },
                    ));
                }
            }
        }
    }
    canonicalize_violations(violations)
}

fn evaluate_endpoint_deletion_integrity_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredEndpointDeletionIntegrityContract,
) -> Vec<InvariantViolation> {
    let Some(scope) = context.relation_integrity_scope(contract.relation_kind_id) else {
        return Vec::new();
    };
    if scope.deleted_entities.is_empty() {
        return Vec::new();
    }
    context.metrics().count_relation_contracts_evaluated(1);
    let mut violations = Vec::new();
    for entity_id in &scope.deleted_entities {
        let endpoint_key = super::request::PreparedRelationEndpointKey {
            entity_id: *entity_id,
        };
        let live_relations = scope
            .source_counts
            .get(&endpoint_key)
            .copied()
            .unwrap_or_default()
            + scope
                .target_counts
                .get(&endpoint_key)
                .copied()
                .unwrap_or_default();
        context.metrics().count_relation_endpoint_deletion_checks(1);
        if live_relations > 0 {
            match contract.mode {
                EndpointDeletionIntegrityMode::RejectDeleteWithLiveRelations => {
                    violations.push(relation_violation(
                        class,
                        DiagnosticCode::RelationEndpointDeletionIntegrityViolation,
                        format!(
                            "relation contract '{}' forbids deleting endpoint {:?} while {} relation endpoints remain live",
                            contract.contract_id, entity_id, live_relations
                        ),
                        InvariantViolationFields::RelationEndpointDeletionIntegrity {
                            contract_id: contract.contract_id.clone(),
                            relation_kind_id: contract.relation_kind_id,
                            entity_id: *entity_id,
                            remaining_relation_endpoint_count: live_relations,
                            mode: contract.mode,
                            cascade_delete_policy: None,
                        },
                    ));
                }
                EndpointDeletionIntegrityMode::RequireRelationDeletionInSameCommit => {
                    if contract.cascade_delete_policy
                        != crate::config::data::CascadeDeletePolicy::CascadeDeleteRelations
                    {
                        violations.push(relation_violation(
                            class,
                            DiagnosticCode::RelationEndpointDeletionIntegrityViolation,
                            format!(
                                "relation contract '{}' requires deleting dependent relations in the same commit before deleting endpoint {:?}",
                                contract.contract_id, entity_id
                            ),
                            InvariantViolationFields::RelationEndpointDeletionIntegrity {
                                contract_id: contract.contract_id.clone(),
                                relation_kind_id: contract.relation_kind_id,
                                entity_id: *entity_id,
                                remaining_relation_endpoint_count: live_relations,
                                mode: contract.mode,
                                cascade_delete_policy: Some(contract.cascade_delete_policy),
                            },
                        ));
                    }
                }
                EndpointDeletionIntegrityMode::RequireRelationRetirement => {
                    if contract.cascade_delete_policy
                        != crate::config::data::CascadeDeletePolicy::RetainDanglingForAudit
                    {
                        violations.push(relation_violation(
                            class,
                            DiagnosticCode::RelationEndpointDeletionIntegrityViolation,
                            format!(
                                "relation contract '{}' requires audit-retained relation retirement before deleting endpoint {:?}",
                                contract.contract_id, entity_id
                            ),
                            InvariantViolationFields::RelationEndpointDeletionIntegrity {
                                contract_id: contract.contract_id.clone(),
                                relation_kind_id: contract.relation_kind_id,
                                entity_id: *entity_id,
                                remaining_relation_endpoint_count: live_relations,
                                mode: contract.mode,
                                cascade_delete_policy: Some(contract.cascade_delete_policy),
                            },
                        ));
                    }
                }
            }
        }
    }
    canonicalize_violations(violations)
}

fn entity_kind_in_state(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    entity_id: crate::identity::data::EntityId,
) -> Result<Option<crate::identity::data::KindId>, InvariantViolation> {
    let Some(partition) = context
        .partition_access()
        .get_partition(entity_id.partition_id)
    else {
        return Err(storage_inconsistency_violation(
            class,
            format!(
                "entity endpoint {:?} references missing partition {:?}",
                entity_id, entity_id.partition_id
            ),
            json!({
                "entity_id": entity_id,
                "partition_id": entity_id.partition_id.0,
                "lookup": "entity_kind_in_state",
            }),
        ));
    };
    let Some(slot) = partition.entity_arena.get(&entity_id) else {
        return Err(storage_inconsistency_violation(
            class,
            format!(
                "entity endpoint {:?} references missing entity slot",
                entity_id
            ),
            json!({
                "entity_id": entity_id,
                "partition_id": entity_id.partition_id.0,
                "lookup": "entity_kind_in_state",
                "failure": "missing_slot",
            }),
        ));
    };
    if slot.lifecycle() != RecordLifecycleState::Live {
        return Ok(None);
    }
    slot.kind_id()
        .ok_or_else(|| {
            storage_inconsistency_violation(
                class,
                format!(
                    "entity endpoint {:?} is live but missing kind id",
                    entity_id
                ),
                json!({
                    "entity_id": entity_id,
                    "partition_id": entity_id.partition_id.0,
                    "lookup": "entity_kind_in_state",
                    "failure": "missing_kind_id",
                }),
            )
        })
        .map(Some)
}

fn relation_violation(
    class: InvariantClass,
    code: DiagnosticCode,
    detail: String,
    fields: InvariantViolationFields,
) -> InvariantViolation {
    InvariantViolation {
        class,
        code,
        detail,
        fields,
    }
}

fn storage_inconsistency_violation(
    class: InvariantClass,
    detail: String,
    fields: serde_json::Value,
) -> InvariantViolation {
    let mut entity_id = None;
    let mut partition_id = None;
    let mut slot = None;
    let mut missing_label = None;
    let mut scan = None;
    let mut lookup = None;
    let mut failure = None;
    if let Some(object) = fields.as_object() {
        entity_id = object
            .get("entity_id")
            .cloned()
            .and_then(|value| serde_json::from_value(value).ok());
        partition_id = object
            .get("partition_id")
            .and_then(|value| value.as_u64())
            .map(|value| crate::identity::data::PartitionId(value as u32));
        slot = object
            .get("slot")
            .and_then(|value| value.as_u64())
            .map(|value| value as usize);
        missing_label = object
            .get("missing_label")
            .and_then(|value| value.as_str())
            .map(ToOwned::to_owned);
        scan = object
            .get("scan")
            .and_then(|value| value.as_str())
            .map(ToOwned::to_owned);
        lookup = object
            .get("lookup")
            .and_then(|value| value.as_str())
            .map(ToOwned::to_owned);
        failure = object
            .get("failure")
            .and_then(|value| value.as_str())
            .map(ToOwned::to_owned);
    }
    InvariantViolation {
        class,
        code: DiagnosticCode::StorageInconsistencyDetected,
        detail,
        fields: InvariantViolationFields::StorageInconsistency {
            entity_id,
            partition_id,
            slot,
            missing_label,
            scan,
            lookup,
            failure,
        },
    }
}
