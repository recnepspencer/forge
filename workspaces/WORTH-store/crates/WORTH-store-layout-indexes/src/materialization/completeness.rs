use super::coverage::S8LayoutCoverageWitness;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8MaterializationCompleteness {
    ExactPointOnly,
    ExactRange,
    ExactPrefix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8RangeCompletenessWitness {
    coverage: S8LayoutCoverageWitness,
}

impl S8RangeCompletenessWitness {
    pub(crate) const fn new(coverage: S8LayoutCoverageWitness) -> Self {
        Self { coverage }
    }

    pub const fn coverage(self) -> S8LayoutCoverageWitness {
        self.coverage
    }

    pub const fn completeness(self) -> S8MaterializationCompleteness {
        S8MaterializationCompleteness::ExactRange
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8PrefixCompletenessWitness {
    coverage: S8LayoutCoverageWitness,
}

impl S8PrefixCompletenessWitness {
    pub(crate) const fn new(coverage: S8LayoutCoverageWitness) -> Self {
        Self { coverage }
    }

    pub const fn coverage(self) -> S8LayoutCoverageWitness {
        self.coverage
    }

    pub const fn completeness(self) -> S8MaterializationCompleteness {
        S8MaterializationCompleteness::ExactPrefix
    }
}
