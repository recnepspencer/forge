#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TilingIterationCounters {
    query_declarations_checked: usize,
    query_readiness_rows: usize,
    required_checker_lanes: usize,
    eligible_actions: usize,
    blocked_actions: usize,
    stale_frontier_blocks: usize,
    suppression_blocks: usize,
    invariant_legality_blocks: usize,
    advisory_only_rows: usize,
    unsupported_rows: usize,
    equivalence_basis_rows: usize,
}

impl TilingIterationCounters {
    pub(crate) fn new(input: TilingIterationCounterInput) -> Self {
        Self {
            query_declarations_checked: input.query_declarations_checked,
            query_readiness_rows: input.query_readiness_rows,
            required_checker_lanes: input.required_checker_lanes,
            eligible_actions: input.eligible_actions,
            blocked_actions: input.blocked_actions,
            stale_frontier_blocks: input.stale_frontier_blocks,
            suppression_blocks: input.suppression_blocks,
            invariant_legality_blocks: input.invariant_legality_blocks,
            advisory_only_rows: input.advisory_only_rows,
            unsupported_rows: input.unsupported_rows,
            equivalence_basis_rows: input.equivalence_basis_rows,
        }
    }

    pub fn query_declarations_checked(&self) -> usize {
        self.query_declarations_checked
    }

    pub fn query_readiness_rows(&self) -> usize {
        self.query_readiness_rows
    }

    pub fn required_checker_lanes(&self) -> usize {
        self.required_checker_lanes
    }

    pub fn eligible_actions(&self) -> usize {
        self.eligible_actions
    }

    pub fn blocked_actions(&self) -> usize {
        self.blocked_actions
    }

    pub fn stale_frontier_blocks(&self) -> usize {
        self.stale_frontier_blocks
    }

    pub fn suppression_blocks(&self) -> usize {
        self.suppression_blocks
    }

    pub fn invariant_legality_blocks(&self) -> usize {
        self.invariant_legality_blocks
    }

    pub fn advisory_only_rows(&self) -> usize {
        self.advisory_only_rows
    }

    pub fn unsupported_rows(&self) -> usize {
        self.unsupported_rows
    }

    pub fn equivalence_basis_rows(&self) -> usize {
        self.equivalence_basis_rows
    }
}

pub(crate) struct TilingIterationCounterInput {
    pub(crate) query_declarations_checked: usize,
    pub(crate) query_readiness_rows: usize,
    pub(crate) required_checker_lanes: usize,
    pub(crate) eligible_actions: usize,
    pub(crate) blocked_actions: usize,
    pub(crate) stale_frontier_blocks: usize,
    pub(crate) suppression_blocks: usize,
    pub(crate) invariant_legality_blocks: usize,
    pub(crate) advisory_only_rows: usize,
    pub(crate) unsupported_rows: usize,
    pub(crate) equivalence_basis_rows: usize,
}
