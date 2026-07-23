#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConvergenceIncumbentPosture {
    NoIncumbent,
    FirstFeasible,
    BestObserved,
    ParetoFrontier,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConvergenceOscillationPosture {
    Impossible,
    DetectAndDeny,
    DetectAndSelectIncumbent,
    DomainClassified,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryConvergenceContract {
    NotIterative,
    Iterative {
        progress_measure_family: String,
        comparator_family: String,
        incumbent: WorthQueryConvergenceIncumbentPosture,
        iteration_bound: usize,
        oscillation: WorthQueryConvergenceOscillationPosture,
    },
}

impl WorthQueryConvergenceContract {
    pub fn bounded(
        progress_measure_family: impl Into<String>,
        comparator_family: impl Into<String>,
        iteration_bound: usize,
    ) -> Self {
        Self::Iterative {
            progress_measure_family: progress_measure_family.into(),
            comparator_family: comparator_family.into(),
            incumbent: WorthQueryConvergenceIncumbentPosture::BestObserved,
            iteration_bound,
            oscillation: WorthQueryConvergenceOscillationPosture::DetectAndDeny,
        }
    }
}
