use serde::{Deserialize, Serialize};

use crate::transactions::data::{
    CreateIntent, EntityMutationIntent, MergedCommitPlan, MutationIntent, RelationMutationIntent,
};

use super::rules::{InvariantRule, RecordKindTag};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct InvariantPlanContract {
    pub touches_entity_existence: bool,
    pub touches_entity_payload: bool,
    pub touches_relation_existence: bool,
    pub touches_relation_payload: bool,
    pub touches_uniqueness: bool,
    pub touches_publication_surface: bool,
    pub touches_snapshot_surface: bool,
}

impl InvariantPlanContract {
    pub fn from_merged_plan(plan: &MergedCommitPlan) -> Self {
        let mut contract = Self::default();
        for intent in &plan.merged_intents {
            contract.observe_intent(intent);
        }
        contract
    }

    pub fn is_empty(self) -> bool {
        !self.touches_entity_existence
            && !self.touches_entity_payload
            && !self.touches_relation_existence
            && !self.touches_relation_payload
            && !self.touches_uniqueness
            && !self.touches_publication_surface
            && !self.touches_snapshot_surface
    }

    pub(crate) fn applies_to_rule(self, rule: &InvariantRule) -> bool {
        if self.is_empty() {
            return true;
        }
        match rule {
            InvariantRule::LiveRecordRequiresSidecar(RecordKindTag::Entity) => {
                self.touches_entity_existence || self.touches_entity_payload
            }
            InvariantRule::LiveRecordRequiresSidecar(RecordKindTag::Relation) => {
                self.touches_relation_existence || self.touches_relation_payload
            }
            InvariantRule::MaxMergedIntents(_) => true,
            InvariantRule::MaxSnapshotEntities(_) => self.touches_snapshot_surface,
            InvariantRule::UniqueEntityPayloadField(_) => self.touches_uniqueness,
        }
    }

    fn observe_intent(&mut self, intent: &MutationIntent) {
        match intent {
            MutationIntent::Create(CreateIntent::Entity(_)) => {
                self.touches_entity_existence = true;
                self.touches_entity_payload = true;
                self.touches_uniqueness = true;
                self.touches_publication_surface = true;
                self.touches_snapshot_surface = true;
            }
            MutationIntent::Create(CreateIntent::BulkEntities(_)) => {
                self.touches_entity_existence = true;
                self.touches_entity_payload = true;
                self.touches_uniqueness = true;
                self.touches_publication_surface = true;
                self.touches_snapshot_surface = true;
            }
            MutationIntent::Entity(EntityMutationIntent::Update(_))
            | MutationIntent::Entity(EntityMutationIntent::Replace(_)) => {
                self.touches_entity_payload = true;
                self.touches_uniqueness = true;
                self.touches_publication_surface = true;
                self.touches_snapshot_surface = true;
            }
            MutationIntent::Entity(EntityMutationIntent::Delete(_)) => {
                self.touches_entity_existence = true;
                self.touches_publication_surface = true;
                self.touches_snapshot_surface = true;
            }
            MutationIntent::Create(CreateIntent::Relation(_))
            | MutationIntent::Create(CreateIntent::BulkRelations(_)) => {
                self.touches_relation_existence = true;
                self.touches_relation_payload = true;
                self.touches_publication_surface = true;
                self.touches_snapshot_surface = true;
            }
            MutationIntent::Relation(RelationMutationIntent::Delete(_)) => {
                self.touches_relation_existence = true;
                self.touches_publication_surface = true;
                self.touches_snapshot_surface = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InvariantPlanContract;
    use crate::identity::data::{EntityId, Generation, KindId, LocalSlot, PartitionId};
    use crate::payloads::data::RecordPayload;
    use crate::symbols::data::InternedString;
    use crate::transactions::data::{
        BulkEntityCreateIntent, CreateIntent, DeleteEntityIntent, EntityMutationIntent,
        MergedCommitPlan, MutationIntent, TransactionId,
    };

    #[test]
    fn contract_marks_entity_create_as_entity_payload_and_uniqueness_sensitive() {
        let plan = MergedCommitPlan {
            transaction_id: TransactionId(1),
            merged_intents: vec![MutationIntent::Create(CreateIntent::BulkEntities(
                BulkEntityCreateIntent {
                    partition_id: PartitionId(7),
                    kind_id: KindId(9),
                    client_keys: vec![InternedString::Raw("a".to_string())],
                    payloads: vec![RecordPayload::OpaqueBytes(vec![1])],
                },
            ))],
        };

        let contract = InvariantPlanContract::from_merged_plan(&plan);
        assert!(contract.touches_entity_existence);
        assert!(contract.touches_entity_payload);
        assert!(contract.touches_uniqueness);
        assert!(contract.touches_publication_surface);
        assert!(contract.touches_snapshot_surface);
        assert!(!contract.touches_relation_existence);
    }

    #[test]
    fn contract_marks_entity_delete_without_payload_or_uniqueness_surface() {
        let plan = MergedCommitPlan {
            transaction_id: TransactionId(2),
            merged_intents: vec![MutationIntent::Entity(EntityMutationIntent::Delete(
                DeleteEntityIntent {
                    entity_id: EntityId::new(
                        PartitionId(1),
                        LocalSlot(0).0,
                        Generation(1).0,
                    ),
                },
            ))],
        };

        let contract = InvariantPlanContract::from_merged_plan(&plan);
        assert!(contract.touches_entity_existence);
        assert!(!contract.touches_entity_payload);
        assert!(!contract.touches_uniqueness);
        assert!(contract.touches_publication_surface);
        assert!(contract.touches_snapshot_surface);
    }
}
