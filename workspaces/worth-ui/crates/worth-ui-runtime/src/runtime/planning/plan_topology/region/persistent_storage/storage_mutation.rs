use super::*;
use crate::runtime::planning::plan_topology::region::slot_set;
use crate::runtime::planning::plan_topology::WorthUiPlanRegionExecutable;

impl WorthUiPlanRegionStore {
    pub(super) fn upsert(
        &mut self,
        schema: WorthUiPlanRegionSchema,
        evidence: &mut Vec<WorthUiPlanRegionTransitionEvidence>,
        counters: &mut WorthUiPlanRegionStorageCounters,
    ) -> Result<(), crate::runtime::WorthUiHandleCapacityExhaustion> {
        let existing = identity_trie::lookup(&self.identity_root, schema.identity()).cloned();
        let (handle, transition) = match existing {
            Some(record) if schemas_match(&record.schema, &schema, counters) => {
                counters.record_reuse();
                evidence.push(WorthUiPlanRegionTransitionEvidence::new(
                    schema.identity().clone(),
                    WorthUiPlanRegionTransition::Reused,
                ));
                return Ok(());
            }
            Some(record) => {
                counters.record_retirement();
                let handle = record.handle.replacement_successor()?;
                (handle, WorthUiPlanRegionTransition::Replaced)
            }
            None => {
                let handle = WorthUiPlanRegionHandle::initial(
                    schema.identity().clone(),
                    self.next_stable_slot,
                );
                self.next_stable_slot =
                    crate::runtime::execution::handle_allocation::WorthUiHandleCapacity::next_stable_slot(
                        self.next_stable_slot,
                    )?;
                self.region_count += 1;
                (handle, WorthUiPlanRegionTransition::Inserted)
            }
        };
        counters.record_region_construction();
        let identity = schema.identity().clone();
        let executable = WorthUiPlanRegionExecutable::lower(schema.input(), |identity| {
            self.handle_for(&WorthUiPlanRegionIdentity::from_exact_basis(identity))
                .cloned()
        })
        .expect("single-region legacy mutations reference only sealed predecessor rows");
        self.insert_record(
            Rc::new(WorthUiPlanRegionRecord::new(schema, handle, executable)),
            counters,
        );
        evidence.push(WorthUiPlanRegionTransitionEvidence::new(
            identity, transition,
        ));
        Ok(())
    }

    pub(crate) fn retire(
        &mut self,
        identity: WorthUiPlanRegionIdentity,
        evidence: &mut Vec<WorthUiPlanRegionTransitionEvidence>,
        counters: &mut WorthUiPlanRegionStorageCounters,
    ) {
        let Some(record) = identity_trie::lookup(&self.identity_root, &identity).cloned() else {
            return;
        };
        let stable_slot = record.handle.stable_slot();
        let exhausted = super::lane_contract::realtime_budget_exhausted(&record.executable);
        let record_digest = record.semantic_digest();
        self.semantic_digest ^= record_digest;
        self.family_index
            .toggle_semantic_digest(record.executable.family(), record_digest);
        self.family_index
            .remove(record.executable.family(), stable_slot, counters);
        self.replace_realtime_budget_index(stable_slot, exhausted, false, counters);
        self.remove_mounted_projection_record(&record);
        if record.executable.is_root_shell() {
            self.root_shell_root = slot_set::remove(&self.root_shell_root, stable_slot, counters);
            self.root_shell_count -= 1;
        }
        self.identity_root = identity_trie::remove(&self.identity_root, &identity, counters);
        self.slot_root = slot_trie::remove(&self.slot_root, stable_slot, counters);
        self.region_count -= 1;
        counters.record_retirement();
        evidence.push(WorthUiPlanRegionTransitionEvidence::new(
            identity,
            WorthUiPlanRegionTransition::Retired,
        ));
    }

    pub(super) fn force_replace(
        &mut self,
        schema: WorthUiPlanRegionSchema,
        replacement_transition: WorthUiPlanRegionTransition,
        evidence: &mut Vec<WorthUiPlanRegionTransitionEvidence>,
        counters: &mut WorthUiPlanRegionStorageCounters,
    ) -> Result<(), crate::runtime::WorthUiHandleCapacityExhaustion> {
        let existing = identity_trie::lookup(&self.identity_root, schema.identity()).cloned();
        let (handle, transition) = match existing {
            Some(record) => {
                counters.record_retirement();
                (
                    record.handle.replacement_successor()?,
                    replacement_transition,
                )
            }
            None => {
                let handle = WorthUiPlanRegionHandle::initial(
                    schema.identity().clone(),
                    self.next_stable_slot,
                );
                self.next_stable_slot =
                    crate::runtime::execution::handle_allocation::WorthUiHandleCapacity::next_stable_slot(
                        self.next_stable_slot,
                    )?;
                self.region_count += 1;
                (handle, WorthUiPlanRegionTransition::Inserted)
            }
        };
        counters.record_region_construction();
        let identity = schema.identity().clone();
        let executable = WorthUiPlanRegionExecutable::lower(schema.input(), |identity| {
            self.handle_for(&WorthUiPlanRegionIdentity::from_exact_basis(identity))
                .cloned()
        })
        .expect("single-region legacy mutations reference only sealed predecessor rows");
        self.insert_record(
            Rc::new(WorthUiPlanRegionRecord::new(schema, handle, executable)),
            counters,
        );
        evidence.push(WorthUiPlanRegionTransitionEvidence::new(
            identity, transition,
        ));
        Ok(())
    }

    pub(in crate::runtime::planning::plan_topology::region) fn insert_sealed_record(
        &mut self,
        record: Rc<WorthUiPlanRegionRecord>,
        counters: &mut WorthUiPlanRegionStorageCounters,
    ) {
        let stable_slot = record.handle.stable_slot();
        let predecessor = slot_trie::lookup(&self.slot_root, stable_slot).cloned();
        let family = record.executable.family();
        let predecessor_family = predecessor
            .as_ref()
            .map(|predecessor| predecessor.executable.family());
        let predecessor_root_shell = predecessor
            .as_ref()
            .is_some_and(|predecessor| predecessor.executable.is_root_shell());
        let predecessor_realtime_exhausted = predecessor.as_ref().is_some_and(|predecessor| {
            super::lane_contract::realtime_budget_exhausted(&predecessor.executable)
        });
        let successor_realtime_exhausted =
            super::lane_contract::realtime_budget_exhausted(&record.executable);
        if let Some(predecessor) = predecessor.as_ref() {
            let predecessor_digest = predecessor.semantic_digest();
            self.semantic_digest ^= predecessor_digest;
            self.family_index
                .toggle_semantic_digest(predecessor.executable.family(), predecessor_digest);
            self.remove_mounted_projection_record(predecessor);
        }
        let record_digest = record.semantic_digest();
        self.semantic_digest ^= record_digest;
        self.family_index
            .toggle_semantic_digest(family, record_digest);
        if predecessor_family != Some(family) {
            if let Some(predecessor_family) = predecessor_family {
                self.family_index
                    .remove(predecessor_family, stable_slot, counters);
            }
            self.family_index.insert(family, stable_slot, counters);
        }
        if predecessor_root_shell != record.executable.is_root_shell() {
            if predecessor_root_shell {
                self.root_shell_root =
                    slot_set::remove(&self.root_shell_root, stable_slot, counters);
                self.root_shell_count -= 1;
            } else {
                self.root_shell_root =
                    slot_set::insert(&self.root_shell_root, stable_slot, counters);
                self.root_shell_count += 1;
            }
        }
        self.replace_realtime_budget_index(
            stable_slot,
            predecessor_realtime_exhausted,
            successor_realtime_exhausted,
            counters,
        );
        self.identity_root =
            identity_trie::insert(&self.identity_root, Rc::clone(&record), counters);
        self.insert_mounted_projection_record(&record);
        self.slot_root = slot_trie::insert(&self.slot_root, record, counters);
    }

    fn insert_record(
        &mut self,
        record: Rc<WorthUiPlanRegionRecord>,
        counters: &mut WorthUiPlanRegionStorageCounters,
    ) {
        self.insert_sealed_record(record, counters);
    }
}
