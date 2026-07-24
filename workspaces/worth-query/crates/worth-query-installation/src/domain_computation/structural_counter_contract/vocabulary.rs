use worth_foundational::facade::FoundationalPerformanceCounterName;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryStructuralCounterRole {
    Bytes,
    Elements,
    StructuralWork,
    DomainWork,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryStructuralCounterUnit {
    Bytes,
    Elements,
    Operations,
    Comparisons,
    Iterations,
    Neighborhoods,
    Domain(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryStructuralCounterAggregation {
    Independent,
    SumOf(Vec<FoundationalPerformanceCounterName>),
    MaximumOf(Vec<FoundationalPerformanceCounterName>),
    MinimumOf(Vec<FoundationalPerformanceCounterName>),
}

impl WorthQueryStructuralCounterAggregation {
    pub fn sources(&self) -> &[FoundationalPerformanceCounterName] {
        match self {
            Self::Independent => &[],
            Self::SumOf(sources) | Self::MaximumOf(sources) | Self::MinimumOf(sources) => sources,
        }
    }

    pub(crate) fn canonicalize(&mut self) {
        let sources = match self {
            Self::Independent => return,
            Self::SumOf(sources) | Self::MaximumOf(sources) | Self::MinimumOf(sources) => sources,
        };
        sources.sort();
        sources.dedup();
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryStructuralCounterMonotonicity {
    Unconstrained,
    NonDecreasing,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryStructuralCounterScope {
    Operation,
    Run,
    Stage,
    Attempt,
    ArtifactOccurrence,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryStructuralCounterResetBoundary {
    Operation,
    Run,
    Stage,
    Attempt,
    ArtifactOccurrence,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryStructuralCounterRequiredness {
    RequiredCore,
    OptionalSidecar,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryStructuralCounterReplayPosture {
    Exact,
    NonDecreasing,
    ProviderCertified,
    NotCompared,
}
