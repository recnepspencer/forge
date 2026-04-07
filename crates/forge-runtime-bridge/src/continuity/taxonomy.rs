#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeContinuityClass {
    SingleSuccessor,
    SplitSuccessors,
    TruthLoweredCanonicalMergeSuccessor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeContinuityRejectionClass {
    NoAuthoritativeSuccessor,
    AmbiguousSuccessor,
    UnsupportedContinuityClass,
    HistoricalResolutionFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeUnsupportedContinuityClass {
    MultiParentMerge,
    CorrespondenceCandidateSet,
    StructuralSimilarityOnly,
    UnplannedHistoricalResolution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeContinuityOutcomeClass {
    ContinuesAsSingleSuccessor,
    ContinuesAsSplitSuccessors,
    ContinuesViaTruthLoweredCanonicalMergeSuccessor,
    RejectedNoAuthoritativeSuccessor,
    RejectedAmbiguousSuccessor,
    RejectedUnsupportedContinuityClass,
    RejectedHistoricalResolutionFailure,
}

impl BridgeContinuityOutcomeClass {
    pub fn continued_class(self) -> Option<BridgeContinuityClass> {
        match self {
            Self::ContinuesAsSingleSuccessor => Some(BridgeContinuityClass::SingleSuccessor),
            Self::ContinuesAsSplitSuccessors => Some(BridgeContinuityClass::SplitSuccessors),
            Self::ContinuesViaTruthLoweredCanonicalMergeSuccessor => {
                Some(BridgeContinuityClass::TruthLoweredCanonicalMergeSuccessor)
            }
            Self::RejectedNoAuthoritativeSuccessor
            | Self::RejectedAmbiguousSuccessor
            | Self::RejectedUnsupportedContinuityClass
            | Self::RejectedHistoricalResolutionFailure => None,
        }
    }

    pub fn rejection_class(self) -> Option<BridgeContinuityRejectionClass> {
        match self {
            Self::ContinuesAsSingleSuccessor
            | Self::ContinuesAsSplitSuccessors
            | Self::ContinuesViaTruthLoweredCanonicalMergeSuccessor => None,
            Self::RejectedNoAuthoritativeSuccessor => {
                Some(BridgeContinuityRejectionClass::NoAuthoritativeSuccessor)
            }
            Self::RejectedAmbiguousSuccessor => {
                Some(BridgeContinuityRejectionClass::AmbiguousSuccessor)
            }
            Self::RejectedUnsupportedContinuityClass => {
                Some(BridgeContinuityRejectionClass::UnsupportedContinuityClass)
            }
            Self::RejectedHistoricalResolutionFailure => {
                Some(BridgeContinuityRejectionClass::HistoricalResolutionFailure)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BridgeContinuityClass, BridgeContinuityOutcomeClass, BridgeContinuityRejectionClass,
    };

    #[test]
    fn continuity_outcomes_are_closed_world_and_classified() {
        assert_eq!(
            BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor.continued_class(),
            Some(BridgeContinuityClass::SingleSuccessor)
        );
        assert_eq!(
            BridgeContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor
                .continued_class(),
            Some(BridgeContinuityClass::TruthLoweredCanonicalMergeSuccessor)
        );
        assert_eq!(
            BridgeContinuityOutcomeClass::RejectedAmbiguousSuccessor.rejection_class(),
            Some(BridgeContinuityRejectionClass::AmbiguousSuccessor)
        );
        assert_eq!(
            BridgeContinuityOutcomeClass::RejectedUnsupportedContinuityClass.continued_class(),
            None
        );
    }
}
