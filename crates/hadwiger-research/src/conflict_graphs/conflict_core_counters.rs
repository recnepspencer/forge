#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictCoreExtractionCounters {
    vertices_inspected: usize,
    edges_inspected: usize,
    deletion_candidates: usize,
    deletion_checks_attempted: usize,
    deletion_checks_admitted: usize,
    unsupported_checks: usize,
    solver_runs: usize,
    query_declarations_performed: usize,
}

impl ConflictCoreExtractionCounters {
    pub(crate) fn new(
        vertices_inspected: usize,
        edges_inspected: usize,
        deletion_candidates: usize,
        deletion_checks_attempted: usize,
        deletion_checks_admitted: usize,
        unsupported_checks: usize,
        solver_runs: usize,
        query_declarations_performed: usize,
    ) -> Self {
        Self {
            vertices_inspected,
            edges_inspected,
            deletion_candidates,
            deletion_checks_attempted,
            deletion_checks_admitted,
            unsupported_checks,
            solver_runs,
            query_declarations_performed,
        }
    }

    pub fn vertices_inspected(&self) -> usize {
        self.vertices_inspected
    }

    pub fn edges_inspected(&self) -> usize {
        self.edges_inspected
    }

    pub fn deletion_candidates(&self) -> usize {
        self.deletion_candidates
    }

    pub fn deletion_checks_attempted(&self) -> usize {
        self.deletion_checks_attempted
    }

    pub fn deletion_checks_admitted(&self) -> usize {
        self.deletion_checks_admitted
    }

    pub fn unsupported_checks(&self) -> usize {
        self.unsupported_checks
    }

    pub fn query_declarations_performed(&self) -> usize {
        self.query_declarations_performed
    }

    pub(crate) fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:{}:{}:{}",
            self.vertices_inspected,
            self.edges_inspected,
            self.deletion_candidates,
            self.deletion_checks_attempted,
            self.deletion_checks_admitted,
            self.unsupported_checks,
            self.solver_runs,
            self.query_declarations_performed
        )
    }
}
