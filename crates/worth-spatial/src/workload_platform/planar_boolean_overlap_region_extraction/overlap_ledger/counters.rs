#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionLedgerAssemblyCounters {
    identity_rows_examined: usize,
    decision_rows_admitted: usize,
    ledger_rows_admitted: usize,
    rows_denied: usize,
}

impl PlanarBooleanOverlapRegionLedgerAssemblyCounters {
    pub(crate) fn examined_identity_row(&mut self) {
        self.identity_rows_examined += 1;
    }

    pub(crate) fn admitted_decision_rows(&mut self, count: usize) {
        self.decision_rows_admitted += count;
    }

    pub(crate) fn admitted_ledger_row(&mut self) {
        self.ledger_rows_admitted += 1;
    }

    pub(crate) fn denied_row(&mut self) {
        self.rows_denied += 1;
    }

    pub fn identity_rows_examined(self) -> usize {
        self.identity_rows_examined
    }

    pub fn decision_rows_admitted(self) -> usize {
        self.decision_rows_admitted
    }

    pub fn ledger_rows_admitted(self) -> usize {
        self.ledger_rows_admitted
    }

    pub fn rows_denied(self) -> usize {
        self.rows_denied
    }
}
