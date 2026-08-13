use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

#[derive(Default)]
pub(crate) struct IndexQueryScratch {
    entity_capacity_hint: usize,
    relation_capacity_hint: usize,
}

impl IndexQueryScratch {
    pub(crate) fn entity_buffer(
        &self,
        candidate_count: usize,
    ) -> Vec<crate::storage::data::EntityReadRecord> {
        Vec::with_capacity(self.entity_capacity_hint.max(candidate_count))
    }

    pub(crate) fn relation_buffer(
        &self,
        candidate_count: usize,
    ) -> Vec<crate::storage::data::RelationReadRecord> {
        Vec::with_capacity(self.relation_capacity_hint.max(candidate_count))
    }

    pub(crate) fn remember_entity_capacity(&mut self, len: usize) {
        self.entity_capacity_hint = self.entity_capacity_hint.max(len);
    }

    pub(crate) fn remember_relation_capacity(&mut self, len: usize) {
        self.relation_capacity_hint = self.relation_capacity_hint.max(len);
    }
}

fn index_query_scratch_hints() -> &'static Mutex<BTreeMap<u64, IndexQueryScratch>> {
    static HINTS: OnceLock<Mutex<BTreeMap<u64, IndexQueryScratch>>> = OnceLock::new();
    HINTS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

pub(crate) fn index_query_scratch_for_runtime(
    runtime: &crate::runtime::RelationalRuntime,
    entity_query: bool,
) -> IndexQueryScratch {
    let runtime_id = runtime.runtime_instance_id();
    let mut hints = index_query_scratch_hints()
        .lock()
        .expect("index query scratch hints lock poisoned");
    let scratch = hints.entry(runtime_id).or_default();
    if (entity_query && scratch.entity_capacity_hint > 0)
        || (!entity_query && scratch.relation_capacity_hint > 0)
    {
        runtime
            .performance_access()
            .count_query_index_scratch_reuse();
    }
    IndexQueryScratch {
        entity_capacity_hint: scratch.entity_capacity_hint,
        relation_capacity_hint: scratch.relation_capacity_hint,
    }
}

pub(crate) fn retain_index_query_scratch(runtime_id: u64, scratch: &IndexQueryScratch) {
    let mut hints = index_query_scratch_hints()
        .lock()
        .expect("index query scratch hints lock poisoned");
    let shared = hints.entry(runtime_id).or_default();
    shared.remember_entity_capacity(scratch.entity_capacity_hint);
    shared.remember_relation_capacity(scratch.relation_capacity_hint);
}

pub(crate) fn purge_index_query_scratch_hints(runtime_id: u64) {
    let mut hints = index_query_scratch_hints()
        .lock()
        .expect("index query scratch hints lock poisoned");
    hints.remove(&runtime_id);
}

#[cfg(test)]
pub(crate) fn index_query_scratch_hint_exists(runtime_id: u64) -> bool {
    index_query_scratch_hints()
        .lock()
        .expect("index query scratch hints lock poisoned")
        .contains_key(&runtime_id)
}
