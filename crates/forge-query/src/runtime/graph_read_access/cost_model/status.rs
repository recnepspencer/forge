use super::budget::{
    DEFAULT_INLINE_EPHEMERAL_INDEX_BYTES, DEFAULT_INLINE_EPHEMERAL_INTERMEDIATE_SET_SIZE,
    DEFAULT_INLINE_EPHEMERAL_RESULT_BYTES,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadCostEstimateStatusKind {
    Measured,
    Estimated,
    UnknownConservative,
    RequiresCapabilityRegistration,
}

impl ForgeQueryGraphReadCostEstimateStatusKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Measured => "measured",
            Self::Estimated => "estimated",
            Self::UnknownConservative => "unknown_conservative",
            Self::RequiresCapabilityRegistration => "requires_capability_registration",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadCostEstimateStatus {
    kind: ForgeQueryGraphReadCostEstimateStatusKind,
}

impl ForgeQueryGraphReadCostEstimateStatus {
    pub fn kind(&self) -> &ForgeQueryGraphReadCostEstimateStatusKind {
        &self.kind
    }

    pub fn as_str(&self) -> &'static str {
        self.kind.as_str()
    }

    pub(crate) fn unknown_conservative() -> Self {
        Self {
            kind: ForgeQueryGraphReadCostEstimateStatusKind::UnknownConservative,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!("status:{}", self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadComplexityContractKind {
    InlineEphemeralCandidate,
    BroadTraversalCandidate,
    CapabilityRequired,
}

impl ForgeQueryGraphReadComplexityContractKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InlineEphemeralCandidate => "inline_ephemeral_candidate",
            Self::BroadTraversalCandidate => "broad_traversal_candidate",
            Self::CapabilityRequired => "capability_required",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadComplexityContract {
    kind: ForgeQueryGraphReadComplexityContractKind,
}

impl ForgeQueryGraphReadComplexityContract {
    pub fn kind(&self) -> &ForgeQueryGraphReadComplexityContractKind {
        &self.kind
    }

    pub fn as_str(&self) -> &'static str {
        self.kind.as_str()
    }

    pub(crate) fn from_cost_dimensions(
        index_bytes: usize,
        result_bytes: usize,
        intermediate_set_size: usize,
    ) -> Self {
        let kind = if index_bytes > DEFAULT_INLINE_EPHEMERAL_INDEX_BYTES
            || result_bytes > DEFAULT_INLINE_EPHEMERAL_RESULT_BYTES
            || intermediate_set_size > DEFAULT_INLINE_EPHEMERAL_INTERMEDIATE_SET_SIZE
        {
            ForgeQueryGraphReadComplexityContractKind::BroadTraversalCandidate
        } else {
            ForgeQueryGraphReadComplexityContractKind::InlineEphemeralCandidate
        };
        Self { kind }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!("complexity_contract:{}", self.as_str())
    }
}
