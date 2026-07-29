#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiRebindDecisionKey(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindDecisionDisposition {
    Changed,
    EvidenceOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindDecisionStopPoint {
    Published,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRebindStructuralCost {
    selected_decisions: usize,
    graph_and_mounted_entries: usize,
    measurement_and_allocation_entries: usize,
    binding_transitions: usize,
    effects: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRebindDecisionRecord {
    key: UiRebindDecisionKey,
    source_basis: u64,
    observation_count: usize,
    changed_fact_count: usize,
    affected_aspect_count: usize,
    consumer_count: usize,
    disposition: UiRebindDecisionDisposition,
    stop_point: UiRebindDecisionStopPoint,
    cost: UiRebindStructuralCost,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRebindDecisionRecordInput {
    pub key: u64,
    pub source_basis: u64,
    pub observation_count: usize,
    pub changed_fact_count: usize,
    pub affected_aspect_count: usize,
    pub consumer_count: usize,
    pub disposition: UiRebindDecisionDisposition,
    pub cost: [usize; 5],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindDecisionIndexDenial {
    EmptyCapacity,
    CapacityExceeded { configured: usize, required: usize },
    DuplicateKey(UiRebindDecisionKey),
}

#[derive(Debug)]
pub struct UiRebindDecisionIndex {
    records: Box<[UiRebindDecisionRecord]>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindDecisionLookup<'index> {
    Found(&'index UiRebindDecisionRecord),
    Expired,
    Unavailable,
}

impl UiRebindDecisionKey {
    #[doc(hidden)]
    pub const fn from_runtime_projection(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn diagnostic_value(self) -> u64 {
        self.0
    }
}

impl UiRebindStructuralCost {
    #[doc(hidden)]
    pub const fn from_runtime_projection(counters: [usize; 5]) -> Self {
        Self {
            selected_decisions: counters[0],
            graph_and_mounted_entries: counters[1],
            measurement_and_allocation_entries: counters[2],
            binding_transitions: counters[3],
            effects: counters[4],
        }
    }

    pub const fn counters(self) -> [usize; 5] {
        [
            self.selected_decisions,
            self.graph_and_mounted_entries,
            self.measurement_and_allocation_entries,
            self.binding_transitions,
            self.effects,
        ]
    }
}

impl UiRebindDecisionRecord {
    #[doc(hidden)]
    pub const fn from_runtime_projection(input: UiRebindDecisionRecordInput) -> Self {
        Self {
            key: UiRebindDecisionKey::from_runtime_projection(input.key),
            source_basis: input.source_basis,
            observation_count: input.observation_count,
            changed_fact_count: input.changed_fact_count,
            affected_aspect_count: input.affected_aspect_count,
            consumer_count: input.consumer_count,
            disposition: input.disposition,
            stop_point: UiRebindDecisionStopPoint::Published,
            cost: UiRebindStructuralCost::from_runtime_projection(input.cost),
        }
    }

    pub const fn key(self) -> UiRebindDecisionKey {
        self.key
    }

    pub const fn source_basis(self) -> u64 {
        self.source_basis
    }

    pub const fn observation_count(self) -> usize {
        self.observation_count
    }

    pub const fn changed_fact_count(self) -> usize {
        self.changed_fact_count
    }

    pub const fn affected_aspect_count(self) -> usize {
        self.affected_aspect_count
    }

    pub const fn consumer_count(self) -> usize {
        self.consumer_count
    }

    pub const fn disposition(self) -> UiRebindDecisionDisposition {
        self.disposition
    }

    pub const fn stop_point(self) -> UiRebindDecisionStopPoint {
        self.stop_point
    }

    pub const fn structural_cost(self) -> UiRebindStructuralCost {
        self.cost
    }
}

impl UiRebindDecisionIndex {
    #[doc(hidden)]
    pub fn from_runtime_projection(
        capacity: usize,
        mut records: Vec<UiRebindDecisionRecord>,
    ) -> Result<Self, UiRebindDecisionIndexDenial> {
        if capacity == 0 {
            return Err(UiRebindDecisionIndexDenial::EmptyCapacity);
        }
        if records.len() > capacity {
            return Err(UiRebindDecisionIndexDenial::CapacityExceeded {
                configured: capacity,
                required: records.len(),
            });
        }
        records.sort_unstable_by_key(|record| record.key);
        if let Some(pair) = records.windows(2).find(|pair| pair[0].key == pair[1].key) {
            return Err(UiRebindDecisionIndexDenial::DuplicateKey(pair[0].key));
        }
        Ok(Self {
            records: records.into_boxed_slice(),
        })
    }

    pub fn lookup(&self, key: UiRebindDecisionKey) -> UiRebindDecisionLookup<'_> {
        match self.records.binary_search_by_key(&key, |record| record.key) {
            Ok(index) => UiRebindDecisionLookup::Found(&self.records[index]),
            Err(_) if self.records.is_empty() => UiRebindDecisionLookup::Unavailable,
            Err(_) => UiRebindDecisionLookup::Expired,
        }
    }

    pub fn summary(&self) -> &[UiRebindDecisionRecord] {
        &self.records
    }
}

#[cfg(test)]
mod rebind_decision_tests {
    use super::*;

    fn record(key: u64) -> UiRebindDecisionRecord {
        UiRebindDecisionRecord::from_runtime_projection(UiRebindDecisionRecordInput {
            key,
            source_basis: 41,
            observation_count: 1,
            changed_fact_count: 1,
            affected_aspect_count: 1,
            consumer_count: 1,
            disposition: UiRebindDecisionDisposition::Changed,
            cost: [1, 2, 3, 4, 5],
        })
    }

    #[test]
    fn rebind_decision_index_is_bounded_sorted_and_exact_keyed() {
        let index = UiRebindDecisionIndex::from_runtime_projection(2, vec![record(9), record(4)])
            .expect("two exact records fit");
        assert_eq!(index.summary()[0].key().diagnostic_value(), 4);
        assert!(matches!(
            index.lookup(UiRebindDecisionKey::from_runtime_projection(9)),
            UiRebindDecisionLookup::Found(found)
                if found.structural_cost().counters() == [1, 2, 3, 4, 5]
        ));
        assert_eq!(
            index.lookup(UiRebindDecisionKey::from_runtime_projection(5)),
            UiRebindDecisionLookup::Expired
        );
    }

    #[test]
    fn rebind_decision_index_rejects_overload_and_duplicate_keys() {
        assert_eq!(
            UiRebindDecisionIndex::from_runtime_projection(1, vec![record(1), record(2)])
                .unwrap_err(),
            UiRebindDecisionIndexDenial::CapacityExceeded {
                configured: 1,
                required: 2,
            }
        );
        assert_eq!(
            UiRebindDecisionIndex::from_runtime_projection(2, vec![record(1), record(1)])
                .unwrap_err(),
            UiRebindDecisionIndexDenial::DuplicateKey(
                UiRebindDecisionKey::from_runtime_projection(1)
            )
        );
    }
}
