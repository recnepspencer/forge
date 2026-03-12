use serde::{Deserialize, Serialize};

use crate::transactions::data::{
    CreateIntent, EntityMutationIntent, MergedCommitPlan, MutationIntent, RelationMutationIntent,
};

use super::groups::InvariantGroup;
use super::rules::InvariantRule;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct InvariantPlanContract {
    pub may_break: u32,
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
        self.may_break == 0
    }

    pub fn may_break_groups(self) -> u32 {
        self.may_break
    }

    pub fn intersects_groups(self, groups_mask: u32) -> bool {
        self.is_empty() || (self.may_break & groups_mask) != 0
    }

    pub(crate) fn applies_to_rule(self, rule: &InvariantRule) -> bool {
        if self.is_empty() {
            return true;
        }
        (rule.groups().mask() & self.may_break) != 0
    }

    fn observe_intent(&mut self, intent: &MutationIntent) {
        let mask = match intent {
            MutationIntent::Create(CreateIntent::Entity(_))
            | MutationIntent::Create(CreateIntent::BulkEntities(_)) => {
                InvariantGroup::StorageCoherence.mask()
                    | InvariantGroup::IdentityCoherence.mask()
                    | InvariantGroup::SchemaCompliance.mask()
                    | InvariantGroup::PublicationCoherence.mask()
                    | InvariantGroup::VersionVisibility.mask()
            }
            MutationIntent::Entity(EntityMutationIntent::Update(_))
            | MutationIntent::Entity(EntityMutationIntent::Replace(_)) => {
                InvariantGroup::IdentityCoherence.mask()
                    | InvariantGroup::SchemaCompliance.mask()
                    | InvariantGroup::PublicationCoherence.mask()
                    | InvariantGroup::VersionVisibility.mask()
            }
            MutationIntent::Entity(EntityMutationIntent::Delete(_)) => {
                InvariantGroup::AdjacencyIntegrity.mask()
                    | InvariantGroup::StorageCoherence.mask()
                    | InvariantGroup::LineageIntegrity.mask()
                    | InvariantGroup::PublicationCoherence.mask()
                    | InvariantGroup::VersionVisibility.mask()
            }
            MutationIntent::Create(CreateIntent::Relation(_))
            | MutationIntent::Create(CreateIntent::BulkRelations(_)) => {
                InvariantGroup::AdjacencyIntegrity.mask()
                    | InvariantGroup::StorageCoherence.mask()
                    | InvariantGroup::SchemaCompliance.mask()
                    | InvariantGroup::PublicationCoherence.mask()
                    | InvariantGroup::VersionVisibility.mask()
            }
            MutationIntent::Relation(RelationMutationIntent::Delete(_)) => {
                InvariantGroup::AdjacencyIntegrity.mask()
                    | InvariantGroup::StorageCoherence.mask()
                    | InvariantGroup::PublicationCoherence.mask()
                    | InvariantGroup::VersionVisibility.mask()
            }
        };
        self.may_break |= mask;
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
        assert_ne!(contract.may_break, 0);
        assert!(contract
            .may_break_groups()
            & crate::validation::data::InvariantGroup::SchemaCompliance.mask()
            != 0);
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
        assert!(contract
            .may_break_groups()
            & crate::validation::data::InvariantGroup::AdjacencyIntegrity.mask()
            != 0);
        assert!(contract
            .may_break_groups()
            & crate::validation::data::InvariantGroup::LineageIntegrity.mask()
            != 0);
    }
}
