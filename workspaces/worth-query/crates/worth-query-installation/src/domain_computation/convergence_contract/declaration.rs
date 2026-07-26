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
        repeated_state_family: String,
        incumbent: WorthQueryConvergenceIncumbentPosture,
        iteration_bound: usize,
        oscillation: WorthQueryConvergenceOscillationPosture,
    },
}

impl WorthQueryConvergenceContract {
    pub fn bounded(
        progress_measure_family: impl Into<String>,
        comparator_family: impl Into<String>,
        repeated_state_family: impl Into<String>,
        iteration_bound: usize,
    ) -> Self {
        Self::Iterative {
            progress_measure_family: progress_measure_family.into(),
            comparator_family: comparator_family.into(),
            repeated_state_family: repeated_state_family.into(),
            incumbent: WorthQueryConvergenceIncumbentPosture::BestObserved,
            iteration_bound,
            oscillation: WorthQueryConvergenceOscillationPosture::DetectAndDeny,
        }
    }

    pub fn progress_measure_family(&self) -> Option<&str> {
        match self {
            Self::NotIterative => None,
            Self::Iterative {
                progress_measure_family,
                ..
            } => Some(progress_measure_family),
        }
    }

    pub fn comparator_family(&self) -> Option<&str> {
        match self {
            Self::NotIterative => None,
            Self::Iterative {
                comparator_family, ..
            } => Some(comparator_family),
        }
    }

    pub fn repeated_state_family(&self) -> Option<&str> {
        match self {
            Self::NotIterative => None,
            Self::Iterative {
                repeated_state_family,
                ..
            } => Some(repeated_state_family),
        }
    }

    pub const fn incumbent_posture(&self) -> Option<WorthQueryConvergenceIncumbentPosture> {
        match self {
            Self::NotIterative => None,
            Self::Iterative { incumbent, .. } => Some(*incumbent),
        }
    }

    pub const fn iteration_bound(&self) -> Option<usize> {
        match self {
            Self::NotIterative => None,
            Self::Iterative {
                iteration_bound, ..
            } => Some(*iteration_bound),
        }
    }

    pub const fn oscillation_posture(&self) -> Option<WorthQueryConvergenceOscillationPosture> {
        match self {
            Self::NotIterative => None,
            Self::Iterative { oscillation, .. } => Some(*oscillation),
        }
    }
}
