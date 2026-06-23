#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanLoopIslandPartitionCounters {
    reconstructed_loops_consumed: usize,
    born_loops_consumed: usize,
    island_rows_emitted: usize,
}

impl PlanarBooleanLoopIslandPartitionCounters {
    pub(crate) fn consumed_reconstructed_loop(&mut self) {
        self.reconstructed_loops_consumed += 1;
    }

    pub(crate) fn consumed_born_loop(&mut self) {
        self.born_loops_consumed += 1;
    }

    pub(crate) fn emitted_island_row(&mut self) {
        self.island_rows_emitted += 1;
    }

    pub fn reconstructed_loops_consumed(self) -> usize {
        self.reconstructed_loops_consumed
    }

    pub fn born_loops_consumed(self) -> usize {
        self.born_loops_consumed
    }

    pub fn island_rows_emitted(self) -> usize {
        self.island_rows_emitted
    }
}
