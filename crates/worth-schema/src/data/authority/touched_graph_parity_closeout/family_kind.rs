#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TouchedGraphParityFamilyKind {
    ReadRouting,
    ValidatorInvariantRouting,
    Invalidation,
    EvidenceLookup,
    RetainedSpatial,
    ReplayUndo,
    ConflictIndependenceBatchAdmission,
    CompiledProductReuse,
    PublicProof,
    DerivedDiagnostics,
}

impl TouchedGraphParityFamilyKind {
    pub const ALL: [Self; 10] = [
        Self::ReadRouting,
        Self::ValidatorInvariantRouting,
        Self::Invalidation,
        Self::EvidenceLookup,
        Self::RetainedSpatial,
        Self::ReplayUndo,
        Self::ConflictIndependenceBatchAdmission,
        Self::CompiledProductReuse,
        Self::PublicProof,
        Self::DerivedDiagnostics,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadRouting => "read-routing",
            Self::ValidatorInvariantRouting => "validator-invariant-routing",
            Self::Invalidation => "invalidation",
            Self::EvidenceLookup => "evidence-lookup",
            Self::RetainedSpatial => "retained-spatial",
            Self::ReplayUndo => "replay-undo",
            Self::ConflictIndependenceBatchAdmission => "conflict-independence-batch-admission",
            Self::CompiledProductReuse => "compiled-product-reuse",
            Self::PublicProof => "public-proof",
            Self::DerivedDiagnostics => "derived-diagnostics",
        }
    }
}
