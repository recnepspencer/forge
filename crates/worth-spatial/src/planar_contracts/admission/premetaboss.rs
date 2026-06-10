#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PlanarPremetabossInputFamily {
    CoplanarOverlapContractStorm,
    HighValencePlanarSingularityContract,
    ThinFeatureScaleSeparationContract,
    RetainedPlanarHistoryCancellationChain,
    DirtyPlanarInputCleanFailLocalization,
    UnboundedHalfSpacePlanarPosture,
    ProjectionConsumedPlanarFactParity,
    BooleanReadinessFinalBoss,
}

impl PlanarPremetabossInputFamily {
    pub const fn all() -> [Self; 8] {
        [
            Self::CoplanarOverlapContractStorm,
            Self::HighValencePlanarSingularityContract,
            Self::ThinFeatureScaleSeparationContract,
            Self::RetainedPlanarHistoryCancellationChain,
            Self::DirtyPlanarInputCleanFailLocalization,
            Self::UnboundedHalfSpacePlanarPosture,
            Self::ProjectionConsumedPlanarFactParity,
            Self::BooleanReadinessFinalBoss,
        ]
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoplanarOverlapContractStorm => "MB-M6-1",
            Self::HighValencePlanarSingularityContract => "MB-M6-2",
            Self::ThinFeatureScaleSeparationContract => "MB-M6-3",
            Self::RetainedPlanarHistoryCancellationChain => "MB-M6-4",
            Self::DirtyPlanarInputCleanFailLocalization => "MB-M6-5",
            Self::UnboundedHalfSpacePlanarPosture => "MB-M6-6",
            Self::ProjectionConsumedPlanarFactParity => "MB-M6-7",
            Self::BooleanReadinessFinalBoss => "MB-M6-8",
        }
    }
}
