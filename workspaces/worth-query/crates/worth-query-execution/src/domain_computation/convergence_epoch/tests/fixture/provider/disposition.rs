#[derive(Clone, Copy)]
pub(crate) enum FixtureFamilyMismatch {
    Universe,
    Termination,
    Feasibility,
    Comparison,
    Incumbent,
    Progress,
    Comparator,
    RepeatedState,
}

#[derive(Clone, Copy)]
pub(crate) enum FixtureDisposition {
    Continue,
    Converged,
    StableWithoutProof,
    FeasibleIncumbent,
    Oscillating,
    OscillatingSelected,
    DomainClassifiedOscillation,
    RepeatedContinue,
    Stalled,
    IndeterminateComparison,
    IncoherentStable,
    ComparatorFailure,
    ComparatorPanic,
    ProgressFailure,
    ProgressPanic,
    RepeatedStateFailure,
    RepeatedStatePanic,
    FamilyInspectionPanic,
    YieldThenConverged,
    ChunkedConverged(usize),
    StageQueueContractMismatch,
    ParetoReplacement,
    ParetoCollision,
    FamilyMismatch(FixtureFamilyMismatch),
}

impl FixtureDisposition {
    pub(super) fn mismatches(self, family: FixtureFamilyMismatch) -> bool {
        matches!(self, Self::FamilyMismatch(actual) if same_family(actual, family))
    }

    pub(super) const fn projection_width(self) -> Option<usize> {
        match self {
            Self::ChunkedConverged(width) => Some(width),
            _ => None,
        }
    }
}

fn same_family(left: FixtureFamilyMismatch, right: FixtureFamilyMismatch) -> bool {
    std::mem::discriminant(&left) == std::mem::discriminant(&right)
}
