#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CorrespondenceCostPosture {
    LineageDirect,
    StructuralCandidateBounded,
    StructuralAmbiguityBounded,
    CorrespondenceDeniedByBreadth,
    CorrespondenceDeniedByTopology,
    CorrespondenceDeniedByUnsupportedEvidence,
}

impl CorrespondenceCostPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LineageDirect => "lineage_direct",
            Self::StructuralCandidateBounded => "structural_candidate_bounded",
            Self::StructuralAmbiguityBounded => "structural_ambiguity_bounded",
            Self::CorrespondenceDeniedByBreadth => "correspondence_denied_by_breadth",
            Self::CorrespondenceDeniedByTopology => "correspondence_denied_by_topology",
            Self::CorrespondenceDeniedByUnsupportedEvidence => {
                "correspondence_denied_by_unsupported_evidence"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StructuralCandidateDiscoveryPlan {
    IndexBackedBounded,
    FingerprintBucketBounded,
    RequiresBroadScanDenied,
}

impl StructuralCandidateDiscoveryPlan {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IndexBackedBounded => "index_backed_bounded",
            Self::FingerprintBucketBounded => "fingerprint_bucket_bounded",
            Self::RequiresBroadScanDenied => "requires_broad_scan_denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StructuralCandidateOrderingContract {
    StableFingerprintOrder,
    StableFingerprintThenLineageHintOrder,
}

impl StructuralCandidateOrderingContract {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::StableFingerprintOrder => "stable_fingerprint_order",
            Self::StableFingerprintThenLineageHintOrder => {
                "stable_fingerprint_then_lineage_hint_order"
            }
        }
    }
}
