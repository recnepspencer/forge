use std::collections::BTreeSet;
use std::sync::Arc;

use crate::authority::intent_merge::canonical_intent_key;
use crate::capabilities::RuntimeConfigSource;
use crate::symbols::data::{InternedString, StringInterner, SymbolPolicy};
use crate::transactions::data::{
    BulkMutationLineagePlan, BulkMutationLocalityFootprint, BulkMutationNamingPlan,
    BulkMutationProvenancePlan, BulkMutationScope, CommitConflict, ConflictClass, CreateIntent,
    EntityMutationIntent, LineageSafeBulkMutationBatch, MergedCommitPlan, MutationIntent,
    NamingStableBulkMutationBatch, PlannedBulkMutationBatch, PlannedLineageTransition,
    ProvenanceCompleteBulkMutationBatch, RelationMutationIntent, SavepointId, TransactionOptions,
    WorkerIntentBatch,
};

use crate::logic::runtime::RelationalRuntime;

#[derive(Debug)]
pub struct RelationalTransaction<'a> {
    pub(crate) runtime: &'a mut RelationalRuntime,
    pub(crate) transaction_id: crate::transactions::data::TransactionId,
    pub(crate) options: TransactionOptions,
    pub(crate) batches: Vec<WorkerIntentBatch>,
    pub(crate) savepoints: Vec<(SavepointId, usize)>,
    pub(crate) last_merged_plan: Option<MergedCommitPlan>,
}

impl<'a> RelationalTransaction<'a> {
    pub fn plan_bulk_mutation_batch(&self) -> Option<PlannedBulkMutationBatch> {
        let mut intents = self
            .batches
            .iter()
            .flat_map(|batch| batch.intents.iter().cloned())
            .collect::<Vec<_>>();
        if intents.is_empty() {
            return None;
        }

        normalize_intents_for_bulk_plan(
            &mut intents,
            self.runtime.runtime_config().identity.symbol_policy,
            self.runtime.services.symbols.clone(),
        );
        intents.sort_by_key(canonical_intent_key);

        let scope = bulk_mutation_scope(&intents);
        let locality = bulk_mutation_locality(&intents);
        let naming = bulk_mutation_naming(&intents);
        let lineage = bulk_mutation_lineage(&intents);
        let provenance = bulk_mutation_provenance(
            self.transaction_id,
            self.options.target_branch.clone(),
            &self.batches,
        );

        Some(PlannedBulkMutationBatch {
            transaction_id: self.transaction_id,
            scope,
            locality,
            naming,
            lineage,
            provenance,
            intents: intents.into(),
        })
    }

    pub fn admit_naming_stable_bulk_mutation_batch(
        &self,
    ) -> Result<Option<NamingStableBulkMutationBatch>, CommitConflict> {
        let Some(planned) = self.plan_bulk_mutation_batch() else {
            return Ok(None);
        };
        validate_naming_plan(
            &planned,
            self.runtime.runtime_config().identity.symbol_policy,
        )?;
        Ok(Some(
            crate::transactions::data::naming_stable_bulk_mutation_batch(planned),
        ))
    }

    pub fn admit_lineage_safe_bulk_mutation_batch(
        &self,
    ) -> Result<Option<LineageSafeBulkMutationBatch>, CommitConflict> {
        let Some(naming_stable) = self.admit_naming_stable_bulk_mutation_batch()? else {
            return Ok(None);
        };
        validate_lineage_plan(naming_stable.planned())?;
        Ok(Some(
            crate::transactions::data::lineage_safe_bulk_mutation_batch(naming_stable),
        ))
    }

    pub fn admit_provenance_complete_bulk_mutation_batch(
        &self,
    ) -> Result<Option<ProvenanceCompleteBulkMutationBatch>, CommitConflict> {
        let Some(lineage_safe) = self.admit_lineage_safe_bulk_mutation_batch()? else {
            return Ok(None);
        };
        validate_provenance_plan(lineage_safe.planned(), &self.batches)?;
        Ok(Some(
            crate::transactions::data::provenance_complete_bulk_mutation_batch(lineage_safe),
        ))
    }

    pub fn inspect_staging(&self) -> crate::inspection::data::TransactionInspectionSurface {
        use crate::inspection::data::{
            InspectionAccessPath, InspectionAvailability, InspectionOrigin,
            SavepointInspectionSurface, TransactionInspectionSurface, TransactionIntentCounts,
        };
        use crate::transactions::data::{
            CreateIntent, EntityMutationIntent, MutationIntent, RecordRef,
        };

        let mut touched_records = BTreeSet::new();
        let mut intent_counts = TransactionIntentCounts::default();
        let mut reserved_bulk_entity_slots = 0_u64;
        let mut reserved_bulk_relation_slots = 0_u64;
        let mut contains_lineage_affecting_intents = false;

        for batch in &self.batches {
            for intent in &batch.intents {
                match intent {
                    MutationIntent::Create(create_intent) => {
                        intent_counts.create_count += 1;
                        match create_intent {
                            CreateIntent::Entity(_) | CreateIntent::Relation(_) => {}
                            CreateIntent::BulkEntities(intent) => {
                                reserved_bulk_entity_slots += intent.payloads.len() as u64;
                            }
                            CreateIntent::BulkRelations(intent) => {
                                reserved_bulk_relation_slots += intent.endpoints.len() as u64;
                            }
                        }
                    }
                    MutationIntent::Entity(entity_intent) => {
                        intent_counts.entity_mutation_count += 1;
                        contains_lineage_affecting_intents |=
                            matches!(entity_intent, EntityMutationIntent::Replace(_));
                        match entity_intent {
                            EntityMutationIntent::Update(intent) => {
                                touched_records.insert(RecordRef::Entity(intent.entity_id));
                            }
                            EntityMutationIntent::UpdateFields(intent) => {
                                touched_records.insert(RecordRef::Entity(intent.entity_id));
                            }
                            EntityMutationIntent::Replace(intent) => {
                                touched_records.insert(RecordRef::Entity(intent.entity_id));
                            }
                            EntityMutationIntent::Delete(intent) => {
                                touched_records.insert(RecordRef::Entity(intent.entity_id));
                            }
                        }
                    }
                    MutationIntent::Relation(relation_intent) => {
                        intent_counts.relation_mutation_count += 1;
                        match relation_intent {
                            crate::transactions::data::RelationMutationIntent::Delete(intent) => {
                                touched_records.insert(RecordRef::Relation(intent.relation_id));
                            }
                        }
                    }
                }
            }
        }

        TransactionInspectionSurface {
            transaction_id: self.transaction_id,
            target_branch: self.options.target_branch.clone(),
            batch_count: self.batches.len() as u64,
            savepoints: self
                .savepoints
                .iter()
                .map(
                    |(savepoint_id, retained_batch_count)| SavepointInspectionSurface {
                        savepoint_id: *savepoint_id,
                        retained_batch_count: *retained_batch_count as u64,
                    },
                )
                .collect(),
            touched_records: touched_records.into_iter().collect(),
            intent_counts,
            reserved_bulk_entity_slots,
            reserved_bulk_relation_slots,
            contains_lineage_affecting_intents,
            origin: InspectionOrigin::TransactionStaging,
            access_path: InspectionAccessPath::DirectLookup,
            availability: InspectionAvailability::Direct,
        }
    }
}

fn normalize_intents_for_bulk_plan(
    intents: &mut [MutationIntent],
    symbol_policy: SymbolPolicy,
    mut interner: StringInterner,
) {
    if symbol_policy == SymbolPolicy::Disabled {
        return;
    }

    let mut raw_values = BTreeSet::new();
    for intent in intents.iter() {
        intent.collect_raw_client_keys(&mut raw_values);
    }
    for raw in raw_values {
        interner.intern(&raw);
    }
    for intent in intents {
        intent.normalize_client_keys(&mut interner, symbol_policy);
    }
}

fn bulk_mutation_scope(intents: &[MutationIntent]) -> BulkMutationScope {
    let mut saw_entity_create = false;
    let mut saw_relation_create = false;
    let mut saw_topology_rewrite = false;

    for intent in intents {
        match intent {
            MutationIntent::Create(CreateIntent::Entity(_))
            | MutationIntent::Create(CreateIntent::BulkEntities(_)) => {
                saw_entity_create = true;
            }
            MutationIntent::Create(CreateIntent::Relation(_))
            | MutationIntent::Create(CreateIntent::BulkRelations(_)) => {
                saw_relation_create = true;
            }
            MutationIntent::Entity(EntityMutationIntent::Replace(_))
            | MutationIntent::Entity(EntityMutationIntent::Delete(_))
            | MutationIntent::Relation(RelationMutationIntent::Delete(_)) => {
                saw_topology_rewrite = true;
            }
            MutationIntent::Entity(EntityMutationIntent::Update(_))
            | MutationIntent::Entity(EntityMutationIntent::UpdateFields(_)) => {}
        }
    }

    if saw_topology_rewrite {
        BulkMutationScope::TopologyRegionRewrite
    } else if saw_entity_create && saw_relation_create {
        BulkMutationScope::BulkMixedMutation
    } else if saw_relation_create {
        BulkMutationScope::BulkRelationCreate
    } else {
        BulkMutationScope::BulkEntityCreate
    }
}

fn bulk_mutation_locality(intents: &[MutationIntent]) -> BulkMutationLocalityFootprint {
    let mut touched_partitions = BTreeSet::new();
    let mut cross_partition_relation_count = 0usize;
    let mut entity_target_count = 0usize;
    let mut relation_target_count = 0usize;

    for intent in intents {
        intent.seed_touched_partitions(&mut touched_partitions);
        match intent {
            MutationIntent::Create(CreateIntent::Entity(_))
            | MutationIntent::Entity(EntityMutationIntent::Update(_))
            | MutationIntent::Entity(EntityMutationIntent::UpdateFields(_))
            | MutationIntent::Entity(EntityMutationIntent::Replace(_))
            | MutationIntent::Entity(EntityMutationIntent::Delete(_)) => {
                entity_target_count += 1;
            }
            MutationIntent::Create(CreateIntent::BulkEntities(spec)) => {
                entity_target_count += spec.payloads.len();
            }
            MutationIntent::Create(CreateIntent::Relation(spec)) => {
                relation_target_count += 1;
                if spec.source.partition_id() != spec.target.partition_id() {
                    cross_partition_relation_count += 1;
                }
            }
            MutationIntent::Create(CreateIntent::BulkRelations(spec)) => {
                relation_target_count += spec.endpoints.len();
                cross_partition_relation_count += spec
                    .endpoints
                    .iter()
                    .filter(|(source, target)| source.partition_id() != target.partition_id())
                    .count();
            }
            MutationIntent::Relation(RelationMutationIntent::Delete(_)) => {
                relation_target_count += 1;
            }
        }
    }

    BulkMutationLocalityFootprint {
        touched_partitions: touched_partitions.into_iter().collect::<Vec<_>>().into(),
        cross_partition_relation_count,
        entity_target_count,
        relation_target_count,
    }
}

fn bulk_mutation_naming(intents: &[MutationIntent]) -> BulkMutationNamingPlan {
    let mut normalized_client_keys = Vec::new();
    for intent in intents {
        match intent {
            MutationIntent::Create(CreateIntent::Entity(spec)) => {
                normalized_client_keys.push(spec.client_key.clone());
            }
            MutationIntent::Create(CreateIntent::BulkEntities(spec)) => {
                normalized_client_keys.extend(spec.client_keys.iter().cloned());
            }
            MutationIntent::Create(CreateIntent::Relation(spec)) => {
                normalized_client_keys.push(spec.client_key.clone());
            }
            MutationIntent::Create(CreateIntent::BulkRelations(spec)) => {
                normalized_client_keys.extend(spec.client_keys.iter().cloned());
            }
            MutationIntent::Entity(EntityMutationIntent::Replace(spec)) => {
                normalized_client_keys.push(spec.replacement.client_key.clone());
            }
            MutationIntent::Entity(EntityMutationIntent::Update(_))
            | MutationIntent::Entity(EntityMutationIntent::UpdateFields(_))
            | MutationIntent::Entity(EntityMutationIntent::Delete(_))
            | MutationIntent::Relation(RelationMutationIntent::Delete(_)) => {}
        }
    }
    normalized_client_keys.sort();

    BulkMutationNamingPlan {
        naming_digest: crate::transactions::data::certification_digest(&normalized_client_keys),
        normalized_client_keys: Arc::<[InternedString]>::from(normalized_client_keys),
    }
}

fn bulk_mutation_lineage(intents: &[MutationIntent]) -> BulkMutationLineagePlan {
    let mut transitions = Vec::new();
    for intent in intents {
        match intent {
            MutationIntent::Create(CreateIntent::Entity(spec)) => {
                transitions.push(PlannedLineageTransition::CreateEntity {
                    partition_id: spec.partition_id,
                    kind_id: spec.kind_id,
                    client_key: spec.client_key.clone(),
                });
            }
            MutationIntent::Create(CreateIntent::BulkEntities(spec)) => {
                for client_key in &spec.client_keys {
                    transitions.push(PlannedLineageTransition::CreateEntity {
                        partition_id: spec.partition_id,
                        kind_id: spec.kind_id,
                        client_key: client_key.clone(),
                    });
                }
            }
            MutationIntent::Create(CreateIntent::Relation(spec)) => {
                transitions.push(PlannedLineageTransition::CreateRelation {
                    partition_id: spec.partition_id,
                    kind_id: spec.kind_id,
                    source: spec.source.clone(),
                    target: spec.target.clone(),
                    client_key: spec.client_key.clone(),
                });
            }
            MutationIntent::Create(CreateIntent::BulkRelations(spec)) => {
                for (client_key, (source, target)) in
                    spec.client_keys.iter().zip(spec.endpoints.iter())
                {
                    transitions.push(PlannedLineageTransition::CreateRelation {
                        partition_id: spec.partition_id,
                        kind_id: spec.kind_id,
                        source: source.clone(),
                        target: target.clone(),
                        client_key: client_key.clone(),
                    });
                }
            }
            MutationIntent::Entity(EntityMutationIntent::Replace(spec)) => {
                transitions.push(PlannedLineageTransition::ReplaceEntity {
                    entity_id: spec.entity_id,
                    replacement_partition_id: spec.replacement.partition_id,
                    replacement_kind_id: spec.replacement.kind_id,
                    replacement_client_key: spec.replacement.client_key.clone(),
                });
            }
            MutationIntent::Entity(EntityMutationIntent::Delete(spec)) => {
                transitions.push(PlannedLineageTransition::DeleteEntity {
                    entity_id: spec.entity_id,
                });
            }
            MutationIntent::Relation(RelationMutationIntent::Delete(spec)) => {
                transitions.push(PlannedLineageTransition::DeleteRelation {
                    relation_id: spec.relation_id,
                });
            }
            MutationIntent::Entity(EntityMutationIntent::Update(_))
            | MutationIntent::Entity(EntityMutationIntent::UpdateFields(_)) => {}
        }
    }

    BulkMutationLineagePlan {
        lineage_scope_digest: crate::transactions::data::certification_digest(&transitions),
        transitions: transitions.into(),
    }
}

fn bulk_mutation_provenance(
    transaction_id: crate::transactions::data::TransactionId,
    target_branch: Option<crate::history::data::BranchId>,
    batches: &[WorkerIntentBatch],
) -> BulkMutationProvenancePlan {
    let batch_name = format!("transaction-{}", transaction_id.0);
    let worker_batch_names = batches
        .iter()
        .map(|batch| batch.name.clone())
        .collect::<Vec<_>>();
    let worker_partition_keys = batches
        .iter()
        .map(|batch| batch.partition_key.clone())
        .collect::<Vec<_>>();
    let worker_local_only_flags = batches
        .iter()
        .map(|batch| batch.worker_local_only)
        .collect::<Vec<_>>();
    let provenance_digest = crate::transactions::data::certification_digest(&(
        transaction_id,
        &target_branch,
        &batch_name,
        &worker_batch_names,
        &worker_partition_keys,
        &worker_local_only_flags,
    ));

    BulkMutationProvenancePlan {
        batch_name,
        target_branch,
        worker_batch_names: worker_batch_names.into(),
        worker_partition_keys: worker_partition_keys.into(),
        worker_local_only_flags: worker_local_only_flags.into(),
        provenance_digest,
    }
}

fn validate_naming_plan(
    planned: &PlannedBulkMutationBatch,
    symbol_policy: SymbolPolicy,
) -> Result<(), CommitConflict> {
    let mut expected = bulk_mutation_naming(planned.intents.as_ref())
        .normalized_client_keys
        .to_vec();
    let actual = planned.naming.normalized_client_keys.as_ref().to_vec();
    if expected != actual {
        return Err(CommitConflict::new(
            ConflictClass::MutationStateInconsistency {
                detail: "bulk mutation naming plan no longer matches canonicalized intents"
                    .to_string(),
                fields: serde_json::json!({
                    "transaction_id": planned.transaction_id.0,
                    "expected_count": expected.len(),
                    "actual_count": actual.len(),
                }),
            },
        ));
    }

    if symbol_policy != SymbolPolicy::Disabled
        && planned
            .naming
            .normalized_client_keys
            .iter()
            .any(|value| !matches!(value, InternedString::Symbol(_)))
    {
        return Err(CommitConflict::new(
            ConflictClass::MutationStateInconsistency {
                detail: "bulk mutation naming admission requires normalized interned client keys"
                    .to_string(),
                fields: serde_json::json!({
                    "transaction_id": planned.transaction_id.0,
                    "symbol_policy": format!("{symbol_policy:?}"),
                }),
            },
        ));
    }

    let expected_digest = crate::transactions::data::certification_digest(&expected);
    if planned.naming.naming_digest != expected_digest {
        return Err(CommitConflict::new(
            ConflictClass::MutationStateInconsistency {
                detail: "bulk mutation naming digest does not match canonical naming set"
                    .to_string(),
                fields: serde_json::json!({
                    "transaction_id": planned.transaction_id.0,
                    "expected_digest": expected_digest,
                    "actual_digest": planned.naming.naming_digest,
                }),
            },
        ));
    }

    expected.clear();
    Ok(())
}

fn validate_lineage_plan(planned: &PlannedBulkMutationBatch) -> Result<(), CommitConflict> {
    let expected_transitions = bulk_mutation_lineage(planned.intents.as_ref())
        .transitions
        .to_vec();
    let actual_transitions = planned.lineage.transitions.as_ref().to_vec();
    if expected_transitions != actual_transitions {
        return Err(CommitConflict::new(
            ConflictClass::MutationStateInconsistency {
                detail: "bulk mutation lineage plan no longer matches canonicalized intents"
                    .to_string(),
                fields: serde_json::json!({
                    "transaction_id": planned.transaction_id.0,
                    "expected_count": expected_transitions.len(),
                    "actual_count": actual_transitions.len(),
                }),
            },
        ));
    }

    let expected_digest = crate::transactions::data::certification_digest(&expected_transitions);
    if planned.lineage.lineage_scope_digest != expected_digest {
        return Err(CommitConflict::new(
            ConflictClass::MutationStateInconsistency {
                detail: "bulk mutation lineage digest does not match canonical lineage transitions"
                    .to_string(),
                fields: serde_json::json!({
                    "transaction_id": planned.transaction_id.0,
                    "expected_digest": expected_digest,
                    "actual_digest": planned.lineage.lineage_scope_digest,
                }),
            },
        ));
    }

    if matches!(planned.scope, BulkMutationScope::TopologyRegionRewrite)
        && planned.lineage.transitions.is_empty()
    {
        return Err(CommitConflict::new(
            ConflictClass::MutationStateInconsistency {
                detail: "topology rewrite admission requires explicit lineage transitions"
                    .to_string(),
                fields: serde_json::json!({
                    "transaction_id": planned.transaction_id.0,
                }),
            },
        ));
    }

    Ok(())
}

fn validate_provenance_plan(
    planned: &PlannedBulkMutationBatch,
    batches: &[WorkerIntentBatch],
) -> Result<(), CommitConflict> {
    let expected = bulk_mutation_provenance(
        planned.transaction_id,
        planned.provenance.target_branch.clone(),
        batches,
    );
    if planned.provenance.batch_name != expected.batch_name
        || planned.provenance.worker_batch_names != expected.worker_batch_names
        || planned.provenance.worker_partition_keys != expected.worker_partition_keys
        || planned.provenance.worker_local_only_flags != expected.worker_local_only_flags
    {
        return Err(CommitConflict::new(
            ConflictClass::MutationStateInconsistency {
                detail: "bulk mutation provenance plan no longer matches staged worker evidence"
                    .to_string(),
                fields: serde_json::json!({
                    "transaction_id": planned.transaction_id.0,
                    "expected_batch_count": expected.worker_batch_names.len(),
                    "actual_batch_count": planned.provenance.worker_batch_names.len(),
                }),
            },
        ));
    }

    if planned.provenance.provenance_digest != expected.provenance_digest {
        return Err(CommitConflict::new(
            ConflictClass::MutationStateInconsistency {
                detail: "bulk mutation provenance digest does not match staged worker evidence"
                    .to_string(),
                fields: serde_json::json!({
                    "transaction_id": planned.transaction_id.0,
                    "expected_digest": expected.provenance_digest,
                    "actual_digest": planned.provenance.provenance_digest,
                }),
            },
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::facade::identity::{KindId, PartitionId};
    use crate::facade::transactions::{
        BulkEntityCreateIntent, BulkRelationCreateIntent, ReplaceEntityIntent,
    };
    use crate::payloads::data::RecordPayload;
    use crate::tests::support::{create_entity, runtime_with_test_schema};

    #[test]
    fn naming_admission_rejects_uninterned_keys_when_symbol_policy_requires_normalization() {
        let mut runtime = runtime_with_test_schema();
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(WorkerIntentBatch::new("bulk").push(MutationIntent::Create(
            CreateIntent::BulkEntities(BulkEntityCreateIntent {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_keys: vec![InternedString::Raw("raw-key".to_string())],
                payloads: vec![RecordPayload::StructuredJson(
                    serde_json::json!({"name":"raw"}),
                )],
            }),
        )));

        let mut planned = txn.plan_bulk_mutation_batch().expect("planned batch");
        planned.naming.normalized_client_keys =
            Arc::<[InternedString]>::from(vec![InternedString::Raw("raw-key".to_string())]);

        let error = validate_naming_plan(&planned, runtime.runtime_config().identity.symbol_policy)
            .expect_err("naming admission should reject raw key");
        assert!(matches!(
            error.class,
            ConflictClass::MutationStateInconsistency { .. }
        ));
    }

    #[test]
    fn lineage_admission_rejects_tampered_transition_digest() {
        let mut runtime = runtime_with_test_schema();
        let entity = create_entity(&mut runtime, "replace-me");
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("rewrite").push(MutationIntent::Entity(
                EntityMutationIntent::Replace(ReplaceEntityIntent {
                    entity_id: entity,
                    replacement: crate::transactions::data::EntitySpec {
                        partition_id: PartitionId::main(),
                        kind_id: KindId(1),
                        client_key: InternedString::Raw("replacement".to_string()),
                        payload: RecordPayload::StructuredJson(
                            serde_json::json!({"name":"replacement"}),
                        ),
                    },
                }),
            )),
        );

        let mut planned = txn.plan_bulk_mutation_batch().expect("planned batch");
        planned.lineage.lineage_scope_digest = "tampered".to_string();

        let error = validate_lineage_plan(&planned).expect_err("lineage admission should reject");
        assert!(matches!(
            error.class,
            ConflictClass::MutationStateInconsistency { .. }
        ));
    }

    #[test]
    fn provenance_admission_rejects_tampered_worker_evidence() {
        let mut runtime = runtime_with_test_schema();
        let source = create_entity(&mut runtime, "source");
        let target = create_entity(&mut runtime, "target");
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("worker-a").push(MutationIntent::Create(
                CreateIntent::BulkRelations(BulkRelationCreateIntent {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(2),
                    client_keys: vec![InternedString::Raw("edge".to_string())],
                    endpoints: vec![(
                        crate::transactions::data::EntityReference::Existing(source),
                        crate::transactions::data::EntityReference::Existing(target),
                    )],
                    payloads: vec![Some(RecordPayload::StructuredJson(
                        serde_json::json!({"label":"edge"}),
                    ))],
                }),
            )),
        );

        let mut planned = txn.plan_bulk_mutation_batch().expect("planned batch");
        planned.provenance.worker_batch_names = Arc::<[String]>::from(vec!["tampered".to_string()]);

        let error = validate_provenance_plan(&planned, &txn.batches)
            .expect_err("provenance admission should reject");
        assert!(matches!(
            error.class,
            ConflictClass::MutationStateInconsistency { .. }
        ));
    }

    #[test]
    fn naming_admission_does_not_mutate_runtime_counters() {
        let mut runtime = runtime_with_test_schema();
        runtime.performance_access().reset_counters();
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(WorkerIntentBatch::new("bulk").push(MutationIntent::Create(
            CreateIntent::BulkEntities(BulkEntityCreateIntent {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_keys: vec![InternedString::Raw("raw-key".to_string())],
                payloads: vec![RecordPayload::StructuredJson(
                    serde_json::json!({"name":"raw"}),
                )],
            }),
        )));

        let admitted = txn
            .admit_naming_stable_bulk_mutation_batch()
            .expect("admission should succeed");
        let counters = txn.runtime.performance_access().counters();

        assert!(admitted.is_some());
        assert_eq!(counters.bulk_mutation_batch_count, 0);
        assert_eq!(counters.bulk_mutation_naming_normalization_count, 0);
        assert_eq!(counters.bulk_mutation_lineage_transition_count, 0);
    }
}
