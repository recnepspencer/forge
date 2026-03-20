use std::collections::BTreeSet;

use crate::transactions::data::{
    MergedCommitPlan, SavepointId, TransactionOptions, WorkerIntentBatch,
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
    pub fn inspect_staging(&self) -> crate::inspection::data::TransactionInspectionSurface {
        use crate::inspection::data::{
            InspectionAccessPath, InspectionAvailability, InspectionOrigin,
            SavepointInspectionSurface, TransactionInspectionSurface, TransactionIntentCounts,
        };
        use crate::transactions::data::{CreateIntent, EntityMutationIntent, MutationIntent, RecordRef};

        let mut touched_records = BTreeSet::new();
        let mut intent_counts = TransactionIntentCounts::default();
        let mut reserved_bulk_entity_slots = 0;
        let mut reserved_bulk_relation_slots = 0;
        let mut contains_lineage_affecting_intents = false;

        for batch in &self.batches {
            for intent in &batch.intents {
                match intent {
                    MutationIntent::Create(create_intent) => {
                        intent_counts.create_count += 1;
                        match create_intent {
                            CreateIntent::Entity(_) | CreateIntent::Relation(_) => {}
                            CreateIntent::BulkEntities(intent) => {
                                reserved_bulk_entity_slots += intent.payloads.len();
                            }
                            CreateIntent::BulkRelations(intent) => {
                                reserved_bulk_relation_slots += intent.endpoints.len();
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
            batch_count: self.batches.len(),
            savepoints: self
                .savepoints
                .iter()
                .map(|(savepoint_id, retained_batch_count)| SavepointInspectionSurface {
                    savepoint_id: *savepoint_id,
                    retained_batch_count: *retained_batch_count,
                })
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
