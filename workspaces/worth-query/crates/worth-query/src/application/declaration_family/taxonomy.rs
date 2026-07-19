#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryDeclarationPrimaryAuthorityFamily {
    DescriptiveOnly,
    RelationalTruth,
    BridgeContinuation,
    MixedAuthority,
}

impl WorthQueryDeclarationPrimaryAuthorityFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DescriptiveOnly => "descriptive_only",
            Self::RelationalTruth => "relational_truth",
            Self::BridgeContinuation => "bridge_continuation",
            Self::MixedAuthority => "mixed_authority",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQuerySignalCompatibilityPosture {
    NotCompatible,
    Compatible,
    Deferred,
}

impl WorthQuerySignalCompatibilityPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotCompatible => "not_compatible",
            Self::Compatible => "compatible",
            Self::Deferred => "deferred",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryGroupedDeclarationPosture {
    SingleOnly,
    NeighborhoodCapable,
    BatchCapable,
    NeighborhoodAndBatchCapable,
}

impl WorthQueryGroupedDeclarationPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SingleOnly => "single_only",
            Self::NeighborhoodCapable => "neighborhood_capable",
            Self::BatchCapable => "batch_capable",
            Self::NeighborhoodAndBatchCapable => "neighborhood_and_batch_capable",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryDeclarationFamilyTaxonomy {
    primary_authority_family: WorthQueryDeclarationPrimaryAuthorityFamily,
    signal_compatibility: WorthQuerySignalCompatibilityPosture,
    grouped_posture: WorthQueryGroupedDeclarationPosture,
}

impl WorthQueryDeclarationFamilyTaxonomy {
    pub fn new(
        primary_authority_family: WorthQueryDeclarationPrimaryAuthorityFamily,
        signal_compatibility: WorthQuerySignalCompatibilityPosture,
        grouped_posture: WorthQueryGroupedDeclarationPosture,
    ) -> Self {
        Self {
            primary_authority_family,
            signal_compatibility,
            grouped_posture,
        }
    }

    pub fn primary_authority_family(&self) -> WorthQueryDeclarationPrimaryAuthorityFamily {
        self.primary_authority_family
    }

    pub fn signal_compatibility(&self) -> WorthQuerySignalCompatibilityPosture {
        self.signal_compatibility
    }

    pub fn grouped_posture(&self) -> WorthQueryGroupedDeclarationPosture {
        self.grouped_posture
    }

    pub fn from_type_tags<P, S, G>() -> Self
    where
        P: WorthQueryDeclarationPrimaryAuthorityTag,
        S: WorthQueryDeclarationSignalCompatibilityTag,
        G: WorthQueryDeclarationGroupedPostureTag,
    {
        Self::new(P::runtime_value(), S::runtime_value(), G::runtime_value())
    }
}
use crate::application::{
    WorthQueryDeclarationGroupedPostureTag, WorthQueryDeclarationPrimaryAuthorityTag,
    WorthQueryDeclarationSignalCompatibilityTag,
};
