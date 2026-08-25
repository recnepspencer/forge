use std::collections::BTreeMap;

use crate::authority::intent_merge::{
    canonical_intent_key, collect_created_entity_refs, detect_conflicting_updates,
    validate_branch_locality, validate_intent,
};
use crate::capabilities::{InstrumentationSource, RuntimeConfigSource};
use crate::runtime::PartitionAccess;
use crate::transactions::data::{CommitConflict, ConflictClass, MergedCommitPlan, MutationIntent};
use std::time::Instant;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct MergedPlanPreparationTiming {
    pub(crate) validation_micros: u64,
    pub(crate) sort_micros: u64,
    pub(crate) conflict_detection_micros: u64,
}

impl crate::mvcc::BranchBoundRelationalTransaction {
    pub fn merged_plan(
        &mut self,
        runtime: &mut crate::runtime::RelationalRuntime,
    ) -> Result<&MergedCommitPlan, CommitConflict> {
        if self.last_merged_plan.is_none() {
            self.ensure_current_basis(runtime)?;
            let selected_state =
                crate::branch::SelectedRelationalBranchState::from_admitted_basis(&self.basis);
            self.validate_staged_branch_locality(
                selected_state.state(),
                &runtime.services.symbols,
            )?;
            let intents = self.normalized_intents_for_merge(runtime);
            let plan =
                self.build_merged_plan_for_state(runtime, selected_state.state(), intents)?;
            self.last_merged_plan = Some(plan);
        }
        Ok(self.last_merged_plan.as_ref().expect("merged plan"))
    }

    pub(crate) fn validate_staged_branch_locality(
        &self,
        current_state: &impl PartitionAccess,
        interner: &crate::symbols::data::StringInterner,
    ) -> Result<(), CommitConflict> {
        validate_branch_locality(current_state, self.batches(), interner)
    }

    pub(crate) fn build_merged_plan_for_state(
        &self,
        runtime: &crate::runtime::RelationalRuntime,
        current_state: &impl PartitionAccess,
        intents: Vec<MutationIntent>,
    ) -> Result<MergedCommitPlan, CommitConflict> {
        self.build_merged_plan_for_state_with_timing(runtime, current_state, intents)
            .map(|(plan, _)| plan)
    }

    pub(crate) fn build_merged_plan_for_state_with_timing(
        &self,
        runtime: &crate::runtime::RelationalRuntime,
        current_state: &impl PartitionAccess,
        mut intents: Vec<MutationIntent>,
    ) -> Result<(MergedCommitPlan, MergedPlanPreparationTiming), CommitConflict> {
        let created_entities = collect_created_entity_refs(&intents);
        let branch_basis_version_id = Some(self.basis.observation().version_id());
        let validation_started = Instant::now();
        for intent in &intents {
            validate_intent(
                runtime,
                current_state,
                self.schema_authority.as_ref(),
                runtime.runtime_config().storage.cross_context_policy,
                runtime.runtime_instrumentation(),
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

    pub(crate) fn normalized_intents_for_merge(
        &mut self,
        runtime: &mut crate::runtime::RelationalRuntime,
    ) -> Vec<MutationIntent> {
        self.normalize_intents_for_merge(runtime);
        self.batches()
            .iter()
            .flat_map(|batch| batch.intents.iter().cloned())
            .collect()
    }

    fn normalize_intents_for_merge(&mut self, runtime: &mut crate::runtime::RelationalRuntime) {
        let client_key_symbol_policy = runtime.runtime_config().identity.client_key_symbol_policy;
        if !client_key_symbol_policy.interns_requested_strings() {
            return;
        }

        let new_snapshot_entries = self.overlay.normalize_client_keys(
            &mut self.footprint,
            &mut runtime.services.symbols,
            client_key_symbol_policy,
        );
        if !new_snapshot_entries.is_empty() {
            runtime
                .config
                .identity
                .symbol_table
                .merge_new_entries(new_snapshot_entries);
        }
    }

    pub(crate) fn ensure_runtime_affinity(
        &self,
        runtime: &crate::runtime::RelationalRuntime,
    ) -> Result<(), CommitConflict> {
        if self.basis.identity().runtime_instance_id() == runtime.runtime_instance_id() {
            Ok(())
        } else {
            Err(CommitConflict::new(ConflictClass::ForeignRuntime {
                expected_runtime_instance_id: runtime.runtime_instance_id(),
                actual_runtime_instance_id: self.basis.identity().runtime_instance_id(),
            }))
        }
    }

    pub(crate) fn ensure_current_basis(
        &self,
        runtime: &crate::runtime::RelationalRuntime,
    ) -> Result<(), CommitConflict> {
        self.ensure_runtime_affinity(runtime)?;
        if runtime.admitted_branch_basis_is_current(&self.basis) {
            Ok(())
        } else {
            Err(CommitConflict::new(ConflictClass::StaleValidationBasis {
                detail: "transaction basis is no longer current".to_owned(),
            }))
        }
    }
}

pub(crate) fn bulk_reservations_for_plan(
    _state: &impl PartitionAccess,
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

    let entity_reserved = entity_requests.into_values().sum();
    let relation_reserved = relation_requests.into_values().sum();
    (entity_reserved, relation_reserved)
}
