use std::collections::BTreeSet;

use crate::authority::intent_merge::{
    entity_exists_in_version_basis, relation_exists_in_version_basis,
};
use crate::authority::mutation::BranchLocalDeleteAllowance;
use crate::history::data::BranchId;
use crate::runtime::RelationalRuntime;
use crate::transactions::data::{MergedCommitPlan, MutationIntent};

pub(crate) fn branch_local_delete_allowance_for_plan(
    runtime: &RelationalRuntime,
    merged_plan: &MergedCommitPlan,
    target_branch: Option<&BranchId>,
) -> BranchLocalDeleteAllowance {
    let Some(branch_id) = target_branch else {
        return BranchLocalDeleteAllowance::default();
    };
    let history = runtime.history();
    let Some(branch_head) = history.branch_head(branch_id) else {
        return BranchLocalDeleteAllowance::default();
    };
    let current_state = runtime.storage_access().current_state();
    let mut entity_ids = BTreeSet::new();
    let mut relation_ids = BTreeSet::new();

    for intent in &merged_plan.merged_intents {
        match intent {
            MutationIntent::Entity(crate::transactions::data::EntityMutationIntent::Delete(
                spec,
            )) => {
                if !crate::authority::intent_merge::entity_exists_in_state(
                    &current_state,
                    spec.entity_id,
                ) && entity_exists_in_version_basis(
                    runtime,
                    branch_head.version_id,
                    spec.entity_id,
                ) {
                    entity_ids.insert(spec.entity_id);
                }
            }
            MutationIntent::Relation(
                crate::transactions::data::RelationMutationIntent::UpdateEndpoints(spec),
            ) => {
                let relation_id = spec.relation_id;
                if !crate::authority::intent_merge::relation_exists_in_state(
                    &current_state,
                    relation_id,
                ) && relation_exists_in_version_basis(
                    runtime,
                    branch_head.version_id,
                    relation_id,
                ) {
                    relation_ids.insert(relation_id);
                }
            }
            MutationIntent::Relation(
                crate::transactions::data::RelationMutationIntent::ApplyAspectPatch(_),
            ) => {}
            MutationIntent::Relation(
                crate::transactions::data::RelationMutationIntent::Delete(spec),
            ) => {
                let relation_id = spec.relation_id;
                if !crate::authority::intent_merge::relation_exists_in_state(
                    &current_state,
                    relation_id,
                ) && relation_exists_in_version_basis(
                    runtime,
                    branch_head.version_id,
                    relation_id,
                ) {
                    relation_ids.insert(relation_id);
                }
            }
            MutationIntent::Create(_) | MutationIntent::Entity(_) => {}
        }
    }

    BranchLocalDeleteAllowance {
        entity_ids,
        relation_ids,
    }
}
