use super::{
    WorthServerQueryDependencyAuditRow, WorthServerQueryDependencyClosurePosture,
    WorthServerQueryDependencyRuntimeReadiness, WorthServerQueryDependencyScopePosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerQueryDependencySupportPosture {
    ordinary_row_count: usize,
    static_test_only_row_count: usize,
    blocked_row_count: usize,
    legacy_assumption_row_count: usize,
    unclassified_scope_row_count: usize,
    runtime_ready_for_phase_one: bool,
}

impl WorthServerQueryDependencySupportPosture {
    pub(crate) fn from_rows(rows: &[WorthServerQueryDependencyAuditRow]) -> Self {
        let ordinary_row_count = rows.iter().filter(|row| row.ordinary_path()).count();
        let static_test_only_row_count = rows.len().saturating_sub(ordinary_row_count);
        let blocked_row_count = rows
            .iter()
            .filter(|row| row.ordinary_path())
            .filter(|row| {
                row.closure_posture() == WorthServerQueryDependencyClosurePosture::Blocked
            })
            .count();
        let legacy_assumption_row_count = rows
            .iter()
            .filter(|row| row.ordinary_path())
            .filter(|row| {
                row.runtime_readiness()
                    == WorthServerQueryDependencyRuntimeReadiness::LegacyAssumption
            })
            .count();
        let unclassified_scope_row_count = rows
            .iter()
            .filter(|row| row.ordinary_path())
            .filter(|row| {
                row.scope_posture() == WorthServerQueryDependencyScopePosture::Unclassified
            })
            .count();
        let runtime_ready_for_phase_one = blocked_row_count == 0
            && legacy_assumption_row_count == 0
            && unclassified_scope_row_count == 0;

        Self {
            ordinary_row_count,
            static_test_only_row_count,
            blocked_row_count,
            legacy_assumption_row_count,
            unclassified_scope_row_count,
            runtime_ready_for_phase_one,
        }
    }

    pub fn ordinary_row_count(&self) -> usize {
        self.ordinary_row_count
    }

    pub fn static_test_only_row_count(&self) -> usize {
        self.static_test_only_row_count
    }

    pub fn blocked_row_count(&self) -> usize {
        self.blocked_row_count
    }

    pub fn legacy_assumption_row_count(&self) -> usize {
        self.legacy_assumption_row_count
    }

    pub fn unclassified_scope_row_count(&self) -> usize {
        self.unclassified_scope_row_count
    }

    pub fn runtime_ready_for_phase_one(&self) -> bool {
        self.runtime_ready_for_phase_one
    }
}
