#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiRuntimeChangeCounters {
    family_row_count: usize,
    changed_fact_count: usize,
    denied_family_count: usize,
}

impl WorthUiRuntimeChangeCounters {
    pub(crate) fn from_rows(rows: &[super::WorthUiRuntimeChangeFamilyRow]) -> Self {
        Self {
            family_row_count: rows.len(),
            changed_fact_count: rows.iter().map(|row| row.changed_facts().len()).sum(),
            denied_family_count: rows
                .iter()
                .filter(|row| row.status() == super::WorthUiRuntimeChangeFamilyStatus::Denied)
                .count(),
        }
    }

    pub fn family_row_count(self) -> usize {
        self.family_row_count
    }

    pub fn changed_fact_count(self) -> usize {
        self.changed_fact_count
    }

    pub fn denied_family_count(self) -> usize {
        self.denied_family_count
    }
}
