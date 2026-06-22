#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopDecisionPhase {
    ContinuationIndexing,
    WalkOutcomeClassification,
    LoopCandidatePromotion,
    LoopProductAssembly,
    IslandPartition,
    SourceLoopAttribution,
    RoleClassification,
    DegeneracyClassification,
    IdentityMinting,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopDecisionKind {
    Indexed,
    Admitted,
    Denied,
    Preserved,
    Derived,
    PolicyRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopDecisionAffectedArtifact {
    Continuation,
    WalkOutcome,
    LoopCandidate,
    DeniedLoopCandidate,
    ReconstructedLoop,
    BornLoop,
    IslandRow,
    SplitAttributionRow,
    RoleOutcome,
    DegenerateOutcome,
    LoopIdentityRow,
    PersistentNameRow,
    SubshapeSignatureRow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopDecisionReason {
    IndexedFragmentContinuation,
    ClassifiedWalkOutcome,
    PromotedClosedWalk,
    RejectedLoopCandidate,
    BuiltReconstructedLoop,
    BuiltBornLoop,
    PartitionedLoopIsland,
    AttributedSourceLoopSplit,
    ClassifiedLoopRole,
    ClassifiedDegenerateLoop,
    MintedCanonicalLoopIdentity,
    PropagatedPersistentName,
    PropagatedSubshapeSignature,
}

impl PlanarBooleanLoopDecisionPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinuationIndexing => "continuation-indexing",
            Self::WalkOutcomeClassification => "walk-outcome-classification",
            Self::LoopCandidatePromotion => "loop-candidate-promotion",
            Self::LoopProductAssembly => "loop-product-assembly",
            Self::IslandPartition => "island-partition",
            Self::SourceLoopAttribution => "source-loop-attribution",
            Self::RoleClassification => "role-classification",
            Self::DegeneracyClassification => "degeneracy-classification",
            Self::IdentityMinting => "identity-minting",
        }
    }
}

impl PlanarBooleanLoopDecisionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Indexed => "indexed",
            Self::Admitted => "admitted",
            Self::Denied => "denied",
            Self::Preserved => "preserved",
            Self::Derived => "derived",
            Self::PolicyRequired => "policy-required",
        }
    }
}

impl PlanarBooleanLoopDecisionAffectedArtifact {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Continuation => "continuation",
            Self::WalkOutcome => "walk-outcome",
            Self::LoopCandidate => "loop-candidate",
            Self::DeniedLoopCandidate => "denied-loop-candidate",
            Self::ReconstructedLoop => "reconstructed-loop",
            Self::BornLoop => "born-loop",
            Self::IslandRow => "island-row",
            Self::SplitAttributionRow => "split-attribution-row",
            Self::RoleOutcome => "role-outcome",
            Self::DegenerateOutcome => "degenerate-outcome",
            Self::LoopIdentityRow => "loop-identity-row",
            Self::PersistentNameRow => "persistent-name-row",
            Self::SubshapeSignatureRow => "subshape-signature-row",
        }
    }
}
