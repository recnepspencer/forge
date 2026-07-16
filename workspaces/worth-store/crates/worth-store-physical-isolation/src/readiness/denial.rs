use worth_foundational::FoundationalBoundaryEvidenceProvenanceConstructionDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalIsolationEntryDenial {
    CopiedRecoveryFields,
    LiveRuntimeState,
    TerminalProjection,
    SemanticSnapshot,
    JsonAuthority,
    FoundationalOrProofProjection,
    StaleRecoveryReadiness,
    FoundationalProvenanceConstructionDenied(
        FoundationalBoundaryEvidenceProvenanceConstructionDenial,
    ),
}

impl From<FoundationalBoundaryEvidenceProvenanceConstructionDenial>
    for PhysicalIsolationEntryDenial
{
    fn from(denial: FoundationalBoundaryEvidenceProvenanceConstructionDenial) -> Self {
        Self::FoundationalProvenanceConstructionDenied(denial)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalIsolationEntryRebindRequired {
    RecoveryReadinessMustBeRebound,
}
