#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryDeclarationPrimaryAuthorityFamily {
    DescriptiveOnly,
    RelationalTruth,
    BridgeContinuation,
    MixedAuthority,
}

impl ForgeQueryDeclarationPrimaryAuthorityFamily {
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
pub enum ForgeQuerySignalCompatibilityPosture {
    NotCompatible,
    Compatible,
    Deferred,
}

impl ForgeQuerySignalCompatibilityPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotCompatible => "not_compatible",
            Self::Compatible => "compatible",
            Self::Deferred => "deferred",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGroupedDeclarationPosture {
    SingleOnly,
    NeighborhoodCapable,
    BatchCapable,
    NeighborhoodAndBatchCapable,
}

impl ForgeQueryGroupedDeclarationPosture {
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
pub struct ForgeQueryDeclarationFamilyTaxonomy {
    primary_authority_family: ForgeQueryDeclarationPrimaryAuthorityFamily,
    signal_compatibility: ForgeQuerySignalCompatibilityPosture,
    grouped_posture: ForgeQueryGroupedDeclarationPosture,
}

impl ForgeQueryDeclarationFamilyTaxonomy {
    pub fn new(
        primary_authority_family: ForgeQueryDeclarationPrimaryAuthorityFamily,
        signal_compatibility: ForgeQuerySignalCompatibilityPosture,
        grouped_posture: ForgeQueryGroupedDeclarationPosture,
    ) -> Self {
        Self {
            primary_authority_family,
            signal_compatibility,
            grouped_posture,
        }
    }

    pub fn primary_authority_family(&self) -> ForgeQueryDeclarationPrimaryAuthorityFamily {
        self.primary_authority_family
    }

    pub fn signal_compatibility(&self) -> ForgeQuerySignalCompatibilityPosture {
        self.signal_compatibility
    }

    pub fn grouped_posture(&self) -> ForgeQueryGroupedDeclarationPosture {
        self.grouped_posture
    }

    pub fn from_type_tags<P, S, G>() -> Self
    where
        P: ForgeQueryDeclarationPrimaryAuthorityTag,
        S: ForgeQueryDeclarationSignalCompatibilityTag,
        G: ForgeQueryDeclarationGroupedPostureTag,
    {
        Self::new(P::runtime_value(), S::runtime_value(), G::runtime_value())
    }
}
use crate::application::{
    ForgeQueryDeclarationGroupedPostureTag, ForgeQueryDeclarationPrimaryAuthorityTag,
    ForgeQueryDeclarationSignalCompatibilityTag,
};
