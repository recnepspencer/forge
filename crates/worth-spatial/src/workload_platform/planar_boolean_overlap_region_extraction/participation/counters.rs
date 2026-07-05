#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanOverlapParticipationRecoveryCounters {
    loop_rows_recovered: usize,
    island_rows_recovered: usize,
    chain_rows_recovered: usize,
    denied_participation_rows: usize,
}

impl PlanarBooleanOverlapParticipationRecoveryCounters {
    pub(crate) fn recovered_loop_row(&mut self) {
        self.loop_rows_recovered += 1;
    }

    pub(crate) fn recovered_island_row(&mut self) {
        self.island_rows_recovered += 1;
    }

    pub(crate) fn recovered_chain_row(&mut self) {
        self.chain_rows_recovered += 1;
    }

    pub(crate) fn denied_participation(&mut self) {
        self.denied_participation_rows += 1;
    }

    pub fn loop_rows_recovered(self) -> usize {
        self.loop_rows_recovered
    }

    pub fn island_rows_recovered(self) -> usize {
        self.island_rows_recovered
    }

    pub fn chain_rows_recovered(self) -> usize {
        self.chain_rows_recovered
    }

    pub fn denied_participation_rows(self) -> usize {
        self.denied_participation_rows
    }
}
