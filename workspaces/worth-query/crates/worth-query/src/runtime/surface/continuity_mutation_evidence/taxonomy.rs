use worth_runtime_bridge::facade::BridgeContinuityOutcomeClass;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryContinuityClass {
    SingleSuccessor,
    SplitSuccessors,
    TruthLoweredCanonicalMergeSuccessor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryContinuityRejectionClass {
    NoAuthoritativeSuccessor,
    AmbiguousSuccessor,
    UnsupportedContinuityClass,
    HistoricalResolutionFailure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryContinuityOutcomeClass {
    ContinuesAsSingleSuccessor,
    ContinuesAsSplitSuccessors,
    ContinuesViaTruthLoweredCanonicalMergeSuccessor,
    RejectedNoAuthoritativeSuccessor,
    RejectedAmbiguousSuccessor,
    RejectedUnsupportedContinuityClass,
    RejectedHistoricalResolutionFailure,
}

impl WorthQueryContinuityOutcomeClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ContinuesAsSingleSuccessor => "continues_as_single_successor",
            Self::ContinuesAsSplitSuccessors => "continues_as_split_successors",
            Self::ContinuesViaTruthLoweredCanonicalMergeSuccessor => {
                "continues_via_truth_lowered_canonical_merge_successor"
            }
            Self::RejectedNoAuthoritativeSuccessor => "rejected_no_authoritative_successor",
            Self::RejectedAmbiguousSuccessor => "rejected_ambiguous_successor",
            Self::RejectedUnsupportedContinuityClass => "rejected_unsupported_continuity_class",
            Self::RejectedHistoricalResolutionFailure => "rejected_historical_resolution_failure",
        }
    }

    pub fn continuity_class(self) -> Option<WorthQueryContinuityClass> {
        match self {
            Self::ContinuesAsSingleSuccessor => Some(WorthQueryContinuityClass::SingleSuccessor),
            Self::ContinuesAsSplitSuccessors => Some(WorthQueryContinuityClass::SplitSuccessors),
            Self::ContinuesViaTruthLoweredCanonicalMergeSuccessor => {
                Some(WorthQueryContinuityClass::TruthLoweredCanonicalMergeSuccessor)
            }
            _ => None,
        }
    }

    pub fn rejection_class(self) -> Option<WorthQueryContinuityRejectionClass> {
        match self {
            Self::RejectedNoAuthoritativeSuccessor => {
                Some(WorthQueryContinuityRejectionClass::NoAuthoritativeSuccessor)
            }
            Self::RejectedAmbiguousSuccessor => {
                Some(WorthQueryContinuityRejectionClass::AmbiguousSuccessor)
            }
            Self::RejectedUnsupportedContinuityClass => {
                Some(WorthQueryContinuityRejectionClass::UnsupportedContinuityClass)
            }
            Self::RejectedHistoricalResolutionFailure => {
                Some(WorthQueryContinuityRejectionClass::HistoricalResolutionFailure)
            }
            _ => None,
        }
    }
}

pub(super) fn map_outcome_class(
    outcome: BridgeContinuityOutcomeClass,
) -> WorthQueryContinuityOutcomeClass {
    match outcome {
        BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor => {
            WorthQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
        }
        BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors => {
            WorthQueryContinuityOutcomeClass::ContinuesAsSplitSuccessors
        }
        BridgeContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor => {
            WorthQueryContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor
        }
        BridgeContinuityOutcomeClass::RejectedNoAuthoritativeSuccessor => {
            WorthQueryContinuityOutcomeClass::RejectedNoAuthoritativeSuccessor
        }
        BridgeContinuityOutcomeClass::RejectedAmbiguousSuccessor => {
            WorthQueryContinuityOutcomeClass::RejectedAmbiguousSuccessor
        }
        BridgeContinuityOutcomeClass::RejectedUnsupportedContinuityClass => {
            WorthQueryContinuityOutcomeClass::RejectedUnsupportedContinuityClass
        }
        BridgeContinuityOutcomeClass::RejectedHistoricalResolutionFailure => {
            WorthQueryContinuityOutcomeClass::RejectedHistoricalResolutionFailure
        }
    }
}
