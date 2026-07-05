use forge_store_budgets::CounterEvidenceStrength;

use super::PhysicalIsolationCounterSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6IsolationInterferenceCounterName {
    LatchWait,
    BlockedMaintenance,
    ReclaimBlock,
    ProtectedByteFootprint,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6IsolationInterferenceSnapshotRow {
    name: S6IsolationInterferenceCounterName,
    value: u64,
    strength: CounterEvidenceStrength,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6IsolationInterferenceSnapshot {
    rows: [S6IsolationInterferenceSnapshotRow; 4],
}

impl S6IsolationInterferenceSnapshot {
    pub const fn from_s6_handoff_counters(counters: PhysicalIsolationCounterSnapshot) -> Self {
        Self {
            rows: [
                S6IsolationInterferenceSnapshotRow::exact(
                    S6IsolationInterferenceCounterName::LatchWait,
                    counters.latch_wait_count(),
                ),
                S6IsolationInterferenceSnapshotRow::exact(
                    S6IsolationInterferenceCounterName::BlockedMaintenance,
                    counters.blocked_maintenance_count(),
                ),
                S6IsolationInterferenceSnapshotRow::exact(
                    S6IsolationInterferenceCounterName::ReclaimBlock,
                    counters.reclaim_block_count(),
                ),
                S6IsolationInterferenceSnapshotRow::exact(
                    S6IsolationInterferenceCounterName::ProtectedByteFootprint,
                    counters.protected_byte_footprint(),
                ),
            ],
        }
    }

    pub const fn rows(self) -> [S6IsolationInterferenceSnapshotRow; 4] {
        self.rows
    }
}

impl S6IsolationInterferenceSnapshotRow {
    pub const fn exact(name: S6IsolationInterferenceCounterName, value: u64) -> Self {
        Self {
            name,
            value,
            strength: CounterEvidenceStrength::Exact,
        }
    }

    pub const fn name(self) -> S6IsolationInterferenceCounterName {
        self.name
    }

    pub const fn value(self) -> u64 {
        self.value
    }

    pub const fn strength(self) -> CounterEvidenceStrength {
        self.strength
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s6_handoff_interference_snapshot_declares_exact_store_counter_strength() {
        let counters = PhysicalIsolationCounterSnapshot::from_store_executed_counts(
            4, 3, 2, 1, 9, 1, 5, 6, 4096,
        )
        .expect("store executed counters should be valid");

        let snapshot = S6IsolationInterferenceSnapshot::from_s6_handoff_counters(counters);
        let rows = snapshot.rows();

        assert!(rows
            .iter()
            .all(|row| row.strength() == CounterEvidenceStrength::Exact));
        assert!(rows.iter().any(|row| {
            row.name() == S6IsolationInterferenceCounterName::ProtectedByteFootprint
                && row.value() == 4096
        }));
    }
}
