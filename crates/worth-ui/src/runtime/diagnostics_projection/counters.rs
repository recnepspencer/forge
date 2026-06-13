#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiDiagnosticsProjectionCounters {
    runtime_rows_consumed: usize,
    projected_rows: usize,
    reload_rows: usize,
    plan_rows: usize,
    query_rows: usize,
    frame_cost_rows: usize,
    hooks_applied: usize,
    rejected_rows: usize,
    authority_mutations: usize,
}

impl WorthUiDiagnosticsProjectionCounters {
    pub(crate) fn record_runtime_rows(&mut self, count: usize) {
        self.runtime_rows_consumed += count;
        self.projected_rows += count;
    }

    pub(crate) fn record_reload_row(&mut self) {
        self.reload_rows += 1;
    }

    pub(crate) fn record_plan_rows(&mut self, count: usize) {
        self.plan_rows += count;
        self.projected_rows += count;
    }

    pub(crate) fn record_query_rows(&mut self, count: usize) {
        self.query_rows += count;
        self.projected_rows += count;
    }

    pub(crate) fn record_frame_cost_rows(&mut self, count: usize) {
        self.frame_cost_rows += count;
        self.projected_rows += count;
    }

    pub(crate) fn record_hook(&mut self) {
        self.hooks_applied += 1;
    }

    pub fn runtime_rows_consumed(self) -> usize {
        self.runtime_rows_consumed
    }

    pub fn projected_rows(self) -> usize {
        self.projected_rows
    }

    pub fn reload_rows(self) -> usize {
        self.reload_rows
    }

    pub fn plan_rows(self) -> usize {
        self.plan_rows
    }

    pub fn query_rows(self) -> usize {
        self.query_rows
    }

    pub fn frame_cost_rows(self) -> usize {
        self.frame_cost_rows
    }

    pub fn hooks_applied(self) -> usize {
        self.hooks_applied
    }

    pub fn rejected_rows(self) -> usize {
        self.rejected_rows
    }

    pub fn authority_mutations(self) -> usize {
        self.authority_mutations
    }
}
