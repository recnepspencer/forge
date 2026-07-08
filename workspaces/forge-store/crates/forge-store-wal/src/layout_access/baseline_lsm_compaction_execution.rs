#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineLsmCompactionExecution {
    tombstone_older_sequence: u64,
    tombstone_newer_sequence: u64,
    tombstone_blocks_older: bool,
    older_precedes_newer_start: bool,
    newer_precedence_preserved: bool,
    input_generations: [u64; 3],
    output_generation: u64,
    stale_runs_retired: bool,
    bytes_in: u64,
    bytes_out: u64,
    rewritten_runs: u16,
}

impl BaselineLsmCompactionExecution {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        tombstone_older_sequence: u64,
        tombstone_newer_sequence: u64,
        tombstone_blocks_older: bool,
        older_precedes_newer_start: bool,
        newer_precedence_preserved: bool,
        input_generations: [u64; 3],
        output_generation: u64,
        stale_runs_retired: bool,
        bytes_in: u64,
        bytes_out: u64,
        rewritten_runs: u16,
    ) -> Self {
        Self {
            tombstone_older_sequence,
            tombstone_newer_sequence,
            tombstone_blocks_older,
            older_precedes_newer_start,
            newer_precedence_preserved,
            input_generations,
            output_generation,
            stale_runs_retired,
            bytes_in,
            bytes_out,
            rewritten_runs,
        }
    }

    pub const fn tombstone_older_sequence(self) -> u64 {
        self.tombstone_older_sequence
    }
    pub const fn tombstone_newer_sequence(self) -> u64 {
        self.tombstone_newer_sequence
    }
    pub const fn tombstone_blocks_older(self) -> bool {
        self.tombstone_blocks_older
    }
    pub const fn older_precedes_newer_start(self) -> bool {
        self.older_precedes_newer_start
    }
    pub const fn newer_precedence_preserved(self) -> bool {
        self.newer_precedence_preserved
    }
    pub const fn input_generations(self) -> [u64; 3] {
        self.input_generations
    }
    pub const fn output_generation(self) -> u64 {
        self.output_generation
    }
    pub const fn stale_runs_retired(self) -> bool {
        self.stale_runs_retired
    }
    pub const fn bytes_in(self) -> u64 {
        self.bytes_in
    }
    pub const fn bytes_out(self) -> u64 {
        self.bytes_out
    }
    pub const fn rewritten_runs(self) -> u16 {
        self.rewritten_runs
    }
}
