use std::collections::{BTreeMap, BTreeSet};

use super::RelationalTransaction;
use crate::diagnostics::data::DiagnosticCode;
use crate::history::data::{BranchId, CommitId};
use crate::logic::runtime::merge::{
    canonical_intent_key, detect_conflicting_updates, validate_intent,
};
use crate::logic::runtime::{PartitionAccess, WorkingState};
use crate::symbols::data::{InternedString, SymbolPolicy};
use crate::transactions::data::{CommitConflict, MergedCommitPlan, TransactionIntent};

impl<'a> RelationalTransaction<'a> {
    pub fn merged_plan(&mut self) -> Result<&MergedCommitPlan, CommitConflict> {
        if self.last_merged_plan.is_none() {
            let current_state = WorkingState::new(
                self.runtime.partitions.clone(),
                self.runtime.config.adjacency_policy.clone(),
            );
            let plan = self.build_merged_plan_for_state(&current_state)?;
            self.last_merged_plan = Some(plan);
        }
        Ok(self.last_merged_plan.as_ref().expect("merged plan"))
    }

    pub(crate) fn build_merged_plan_for_state(
        &mut self,
        current_state: &impl PartitionAccess,
    ) -> Result<MergedCommitPlan, CommitConflict> {
        let mut intents = self
            .batches
            .iter()
            .flat_map(|batch| batch.intents.iter().cloned())
            .collect::<Vec<_>>();
        self.normalize_intents_for_merge(&mut intents);
        for intent in &intents {
            validate_intent(
                current_state,
                &self.runtime.config.schema_registry,
                self.runtime.config.cross_context_policy,
                &self.runtime.instrumentation.complexity_counters,
                intent,
            )?;
        }
        intents.sort_by_key(canonical_intent_key);
        detect_conflicting_updates(&intents)?;
        Ok(MergedCommitPlan {
            transaction_id: self.transaction_id,
            merged_intents: intents,
        })
    }

    pub(crate) fn resolve_parent_commits(
        &self,
        target_branch: &BranchId,
    ) -> Result<(Vec<CommitId>, Vec<CommitId>), CommitConflict> {
        let mut parents = Vec::new();
        let mut merge_bases = Vec::new();
        let target_head = self.runtime.branch_head(target_branch).map(|head| head.commit_id);
        if let Some(head) = self.runtime.branch_head(target_branch) {
            parents.push(head.commit_id);
        }
        let mut merge_branches = self.options.merge_parent_branches.clone();
        merge_branches.sort();
        merge_branches.dedup();
        for merge_branch in merge_branches {
            if &merge_branch == target_branch {
                continue;
            }
            let Some(head) = self.runtime.branch_head(&merge_branch) else {
                return Err(CommitConflict {
                    code: DiagnosticCode::InvalidMergeParent,
                    detail: format!("merge parent branch {:?} has no head", merge_branch),
                });
            };
            if !parents.contains(&head.commit_id) {
                if let Some(target_head) = target_head {
                    let inspection = self.runtime.inspect_merge(&merge_branch, target_branch);
                    if !inspection.conflicting_records.is_empty() {
                        return Err(CommitConflict {
                            code: DiagnosticCode::MergeConflictOverlap,
                            detail: format!(
                                "merge between {:?} and {:?} has overlapping authority on {:?}",
                                merge_branch, target_branch, inspection.conflicting_records
                            ),
                        });
                    }
                    let Some(merge_base) =
                        self.runtime.latest_common_ancestor(target_head, head.commit_id)
                    else {
                        return Err(CommitConflict {
                            code: DiagnosticCode::MissingMergeBase,
                            detail: format!(
                                "merge parent branch {:?} has no common ancestor with target branch {:?}",
                                merge_branch, target_branch
                            ),
                        });
                    };
                    merge_bases.push(merge_base);
                }
                parents.push(head.commit_id);
            }
        }
        merge_bases.sort_by_key(|commit_id| commit_id.0);
        merge_bases.dedup();
        Ok((parents, merge_bases))
    }

    fn normalize_intents_for_merge(&mut self, intents: &mut [TransactionIntent]) {
        if self.runtime.config.symbol_policy == SymbolPolicy::Disabled {
            return;
        }

        let interner = &mut self.runtime.symbols;
        let mut raw_values = Vec::new();
        for intent in intents.iter() {
            match intent {
                TransactionIntent::CreateEntity(spec) => {
                    if let InternedString::Raw(raw) = &spec.client_key {
                        raw_values.push(raw.clone());
                    }
                }
                TransactionIntent::BulkCreateEntities { client_keys, .. }
                | TransactionIntent::BulkCreateRelations { client_keys, .. } => {
                    for client_key in client_keys {
                        if let InternedString::Raw(raw) = client_key {
                            raw_values.push(raw.clone());
                        }
                    }
                }
                TransactionIntent::CreateRelation(spec) => {
                    if let InternedString::Raw(raw) = &spec.client_key {
                        raw_values.push(raw.clone());
                    }
                }
                TransactionIntent::UpdateEntity { .. }
                | TransactionIntent::ReplaceEntity { .. }
                | TransactionIntent::DeleteEntity { .. }
                | TransactionIntent::DeleteRelation { .. } => {}
            }
        }
        raw_values.sort();
        raw_values.dedup();
        for raw in &raw_values {
            interner.intern(raw);
        }

        for intent in intents {
            match intent {
                TransactionIntent::CreateEntity(spec) => {
                    spec.client_key = normalize_interned_string(
                        interner,
                        self.runtime.config.symbol_policy,
                        spec.client_key.clone(),
                    );
                }
                TransactionIntent::BulkCreateEntities { client_keys, .. } => {
                    for client_key in client_keys {
                        *client_key = normalize_interned_string(
                            interner,
                            self.runtime.config.symbol_policy,
                            client_key.clone(),
                        );
                    }
                }
                TransactionIntent::CreateRelation(spec) => {
                    spec.client_key = normalize_interned_string(
                        interner,
                        self.runtime.config.symbol_policy,
                        spec.client_key.clone(),
                    );
                }
                TransactionIntent::BulkCreateRelations { client_keys, .. } => {
                    for client_key in client_keys {
                        *client_key = normalize_interned_string(
                            interner,
                            self.runtime.config.symbol_policy,
                            client_key.clone(),
                        );
                    }
                }
                TransactionIntent::UpdateEntity { .. }
                | TransactionIntent::ReplaceEntity { .. }
                | TransactionIntent::DeleteEntity { .. }
                | TransactionIntent::DeleteRelation { .. } => {}
            }
        }
        self.runtime.config.symbol_table = interner.snapshot();
    }
}

pub(crate) fn touched_partitions_for_plan(plan: &MergedCommitPlan) -> usize {
    let mut touched = BTreeSet::new();
    for intent in &plan.merged_intents {
        match intent {
            TransactionIntent::CreateEntity(spec) => {
                touched.insert(spec.partition_id);
            }
            TransactionIntent::BulkCreateEntities { partition_id, .. } => {
                touched.insert(*partition_id);
            }
            TransactionIntent::UpdateEntity { entity_id, .. }
            | TransactionIntent::DeleteEntity { entity_id }
            | TransactionIntent::ReplaceEntity { entity_id, .. } => {
                touched.insert(entity_id.partition_id);
                if let TransactionIntent::ReplaceEntity { replacement, .. } = intent {
                    touched.insert(replacement.partition_id);
                }
            }
            TransactionIntent::CreateRelation(spec) => {
                touched.insert(spec.partition_id);
                touched.insert(spec.source.partition_id);
                touched.insert(spec.target.partition_id);
            }
            TransactionIntent::BulkCreateRelations {
                partition_id,
                endpoints,
                ..
            } => {
                touched.insert(*partition_id);
                for (source, target) in endpoints {
                    touched.insert(source.partition_id);
                    touched.insert(target.partition_id);
                }
            }
            TransactionIntent::DeleteRelation { relation_id } => {
                touched.insert(relation_id.partition_id);
            }
        }
    }
    touched.len()
}

pub(crate) fn bulk_reservations_for_plan(
    state: &impl PartitionAccess,
    plan: &MergedCommitPlan,
) -> (usize, usize) {
    let mut entity_requests = BTreeMap::new();
    let mut relation_requests = BTreeMap::new();
    for intent in &plan.merged_intents {
        match intent {
            TransactionIntent::BulkCreateEntities {
                partition_id,
                payloads,
                ..
            } => {
                *entity_requests.entry(*partition_id).or_insert(0usize) += payloads.len();
            }
            TransactionIntent::BulkCreateRelations {
                partition_id,
                endpoints,
                ..
            } => {
                *relation_requests.entry(*partition_id).or_insert(0usize) += endpoints.len();
            }
            _ => {}
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

fn normalize_interned_string(
    interner: &mut crate::symbols::data::StringInterner,
    policy: SymbolPolicy,
    value: InternedString,
) -> InternedString {
    match policy {
        SymbolPolicy::Disabled => value,
        SymbolPolicy::PreferInterned | SymbolPolicy::RequireInterned => interner.normalize(value),
    }
}
