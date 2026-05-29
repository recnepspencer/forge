use std::collections::{BTreeMap, BTreeSet};

use crate::authority::intent_merge::{
    canonical_intent_key, collect_created_entity_refs, detect_conflicting_updates, validate_intent,
};
use crate::capabilities::{InstrumentationSource, RuntimeConfigSource};
use crate::history::data::{BranchId, CommitId};
use crate::logic::runtime::PartitionAccess;
use crate::transactions::data::{CommitConflict, ConflictClass, MergedCommitPlan, MutationIntent};
use crate::transactions::logic::RelationalTransaction;
use std::time::Instant;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct MergedPlanPreparationTiming {
    pub(crate) validation_micros: u64,
    pub(crate) sort_micros: u64,
    pub(crate) conflict_detection_micros: u64,
}

impl<'a> RelationalTransaction<'a> {
    pub fn merged_plan(&mut self) -> Result<&MergedCommitPlan, CommitConflict> {
        if self.last_merged_plan.is_none() {
            let intents = self.normalized_intents_for_merge();
            let current_state = self.runtime.storage_access().current_state();
            let plan = self.build_merged_plan_for_state(&current_state, intents)?;
            self.last_merged_plan = Some(plan);
        }
        Ok(self.last_merged_plan.as_ref().expect("merged plan"))
    }

    pub(crate) fn build_merged_plan_for_state(
        &self,
        current_state: &impl PartitionAccess,
        intents: Vec<MutationIntent>,
    ) -> Result<MergedCommitPlan, CommitConflict> {
        self.build_merged_plan_for_state_with_timing(current_state, intents)
            .map(|(plan, _)| plan)
    }

    pub(crate) fn build_merged_plan_for_state_with_timing(
        &self,
        current_state: &impl PartitionAccess,
        mut intents: Vec<MutationIntent>,
    ) -> Result<(MergedCommitPlan, MergedPlanPreparationTiming), CommitConflict> {
        let history = self.runtime.history();
        let created_entities = collect_created_entity_refs(&intents);
        let branch_basis_version_id = self
            .options
            .target_branch
            .as_ref()
            .and_then(|branch_id| history.branch_head(branch_id).map(|head| head.version_id));
        let validation_started = Instant::now();
        for intent in &intents {
            validate_intent(
                self.runtime,
                current_state,
                self.runtime.runtime_config(),
                self.runtime.runtime_config().storage.cross_context_policy,
                self.runtime.runtime_instrumentation(),
                branch_basis_version_id,
                &created_entities,
                intent,
            )?;
        }
        let sort_started = Instant::now();
        intents.sort_by_key(canonical_intent_key);
        let conflict_started = Instant::now();
        detect_conflicting_updates(&intents)?;
        Ok((
            MergedCommitPlan {
                transaction_id: self.transaction_id,
                merged_intents: intents,
            },
            MergedPlanPreparationTiming {
                validation_micros: validation_started.elapsed().as_micros() as u64,
                sort_micros: sort_started.elapsed().as_micros() as u64,
                conflict_detection_micros: conflict_started.elapsed().as_micros() as u64,
            },
        ))
    }

    pub(crate) fn normalized_intents_for_merge(&mut self) -> Vec<MutationIntent> {
        let mut intents = self
            .batches
            .iter()
            .flat_map(|batch| batch.intents.iter().cloned())
            .collect::<Vec<_>>();
        self.normalize_intents_for_merge(&mut intents);
        intents
    }

    pub(crate) fn resolve_parent_commits(
        &self,
        target_branch: &BranchId,
    ) -> Result<(Vec<CommitId>, Vec<CommitId>), CommitConflict> {
        let mut parents = Vec::new();
        let mut merge_bases = Vec::new();
        let target_head = self
            .runtime
            .history()
            .branch_head(target_branch)
            .map(|head| head.commit_id);
        if let Some(head) = self.runtime.history().branch_head(target_branch) {
            parents.push(head.commit_id);
        }
        for merge_branch in self
            .options
            .merge_parent_branches
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>()
        {
            if &merge_branch == target_branch {
                continue;
            }
            let history = self.runtime.history();
            let Some(head) = history.branch_head(&merge_branch) else {
                return Err(CommitConflict::new(ConflictClass::InvalidMergeParent {
                    detail: format!("requested additional branch {:?} has no head", merge_branch),
                }));
            };
            if !parents.contains(&head.commit_id) {
                if let Some(target_head) = target_head {
                    let inspection = history.inspect_merge(&merge_branch, target_branch);
                    if !inspection.conflicting_records.is_empty() {
                        return Err(CommitConflict::new(ConflictClass::MergeConflictOverlap {
                            detail: format!(
                                "merge between {:?} and {:?} has overlapping authority on {:?}",
                                merge_branch, target_branch, inspection.conflicting_records
                            ),
                        }));
                    }
                    let Some(merge_base) = self
                        .runtime
                        .history()
                        .max_commit_id_common_ancestor(target_head, head.commit_id)
                    else {
                        return Err(CommitConflict::new(ConflictClass::MissingMergeBase {
                                detail: format!(
                                "requested additional branch {:?} has no common ancestor with target branch {:?}",
                                merge_branch, target_branch
                            ),
                            }));
                    };
                    merge_bases.push(merge_base);
                }
                parents.push(head.commit_id);
            }
        }
        Ok((
            parents,
            merge_bases
                .into_iter()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
        ))
    }

    fn normalize_intents_for_merge(&mut self, intents: &mut [MutationIntent]) {
        let client_key_symbol_policy = self
            .runtime
            .runtime_config()
            .identity
            .client_key_symbol_policy;
        if !client_key_symbol_policy.interns_requested_strings() {
            return;
        }

        let interner = &mut self.runtime.services.symbols;
        let mut raw_values = BTreeSet::new();
        for intent in intents.iter() {
            intent.collect_raw_client_keys(&mut raw_values);
        }
        if raw_values.is_empty() {
            return;
        }
        let mut new_snapshot_entries = Vec::new();
        for raw in raw_values {
            if !interner.contains(&raw) {
                let symbol = interner.intern(&raw);
                new_snapshot_entries.push((symbol, raw));
            }
        }

        for intent in intents {
            intent.normalize_client_keys(interner, client_key_symbol_policy);
        }
        if !new_snapshot_entries.is_empty() {
            self.runtime
                .config
                .identity
                .symbol_table
                .merge_new_entries(new_snapshot_entries);
        }
    }
}

pub(crate) fn bulk_reservations_for_plan(
    state: &impl PartitionAccess,
    plan: &MergedCommitPlan,
) -> (usize, usize) {
    let mut entity_requests = BTreeMap::new();
    let mut relation_requests = BTreeMap::new();
    for intent in &plan.merged_intents {
        if let Some((partition_id, requested)) = intent.bulk_entity_reservation() {
            *entity_requests.entry(partition_id).or_insert(0usize) += requested;
        }
        if let Some((partition_id, requested)) = intent.bulk_relation_reservation() {
            *relation_requests.entry(partition_id).or_insert(0usize) += requested;
        }
    }

    let entity_reserved = entity_requests
        .into_iter()
        .map(|(partition_id, requested)| {
            let reusable = state
                .get_partition(partition_id)
                .map(|partition| partition.entity_arena.free_list.len())
                .unwrap_or(0);
            requested.saturating_sub(reusable)
        })
        .sum();
    let relation_reserved = relation_requests
        .into_iter()
        .map(|(partition_id, requested)| {
            let reusable = state
                .get_partition(partition_id)
                .map(|partition| partition.relation_arena.free_list.len())
                .unwrap_or(0);
            requested.saturating_sub(reusable)
        })
        .sum();
    (entity_reserved, relation_reserved)
}
