#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceCategory {
    Lineage,
    Provenance,
    Receipt,
    SupportTruth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceLocality {
    Current,
    BranchLocal,
    Historical,
    ComparisonPaired,
    SnapshotBound,
    ReplayDerived,
    RestoredReadmitted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceExecutionPosture {
    Planned,
    Executed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceDescriptiveRole {
    AuthorityAdjacentDescription,
    SupportGrade,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryEvidenceFreshnessPosture {
    FreshRetained,
    StaleRetained,
    ReducedRetained,
    ReconstructedFromReplay,
    RestoredFromCheckpoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidencePrimitiveDefinition<T> {
    primitive: T,
    name: &'static str,
    intended_use: &'static str,
    must_not_mean: &'static str,
}

impl<T: Copy> FoundationalBoundaryEvidencePrimitiveDefinition<T> {
    pub const fn primitive(&self) -> T {
        self.primitive
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn intended_use(&self) -> &'static str {
        self.intended_use
    }

    pub const fn must_not_mean(&self) -> &'static str {
        self.must_not_mean
    }
}

pub(crate) const fn definition<T>(
    primitive: T,
    name: &'static str,
    intended_use: &'static str,
    must_not_mean: &'static str,
) -> FoundationalBoundaryEvidencePrimitiveDefinition<T> {
    FoundationalBoundaryEvidencePrimitiveDefinition {
        primitive,
        name,
        intended_use,
        must_not_mean,
    }
}

pub const fn foundational_boundary_evidence_category_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<FoundationalBoundaryEvidenceCategory>; 4] {
    [
        definition(
            FoundationalBoundaryEvidenceCategory::Lineage,
            "lineage",
            "continuity, divergence, successor/predecessor, and identity-survival meaning",
            "basis context, completed-boundary attestation, or support-grade evidence",
        ),
        definition(
            FoundationalBoundaryEvidenceCategory::Provenance,
            "provenance",
            "source basis, authority path, profile basis, comparison basis, and retention context",
            "continuity verdicts, completed-boundary truth, or support-grade closeout",
        ),
        definition(
            FoundationalBoundaryEvidenceCategory::Receipt,
            "receipt",
            "completed effectful or authority-bearing boundary attestation",
            "mere planning intent, continuity truth, or support-only explanation",
        ),
        definition(
            FoundationalBoundaryEvidenceCategory::SupportTruth,
            "support_truth",
            "support-grade evidence, parity, recovery, and residual-debt meaning",
            "stronger authority state, continuity truth, or completed-boundary attestation alone",
        ),
    ]
}

pub const fn foundational_boundary_evidence_locality_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<FoundationalBoundaryEvidenceLocality>; 7] {
    [
        definition(
            FoundationalBoundaryEvidenceLocality::Current,
            "current",
            "current retained boundary context",
            "historical, replay-derived, or restored/readmitted context",
        ),
        definition(
            FoundationalBoundaryEvidenceLocality::BranchLocal,
            "branch_local",
            "non-authoritative branch-local or pre-promotion context",
            "globally admitted authority truth",
        ),
        definition(
            FoundationalBoundaryEvidenceLocality::Historical,
            "historical",
            "retained historical context over earlier boundaries",
            "current hot state or replay-derived reconstruction",
        ),
        definition(
            FoundationalBoundaryEvidenceLocality::ComparisonPaired,
            "comparison_paired",
            "paired comparison context across two named bases or artifacts",
            "ordinary single-source current or replay-derived context",
        ),
        definition(
            FoundationalBoundaryEvidenceLocality::SnapshotBound,
            "snapshot_bound",
            "bounded to one retained snapshot or checkpoint slice",
            "live current observation or free replay reconstruction",
        ),
        definition(
            FoundationalBoundaryEvidenceLocality::ReplayDerived,
            "replay_derived",
            "derived from replay rather than directly retained observation",
            "fresh retained or directly attested current truth",
        ),
        definition(
            FoundationalBoundaryEvidenceLocality::RestoredReadmitted,
            "restored_readmitted",
            "restored or readmitted context after checkpoint or boundary reentry",
            "ordinary current hot context with no restoration seam",
        ),
    ]
}

pub const fn foundational_boundary_evidence_execution_posture_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<FoundationalBoundaryEvidenceExecutionPosture>;
       2] {
    [
        definition(
            FoundationalBoundaryEvidenceExecutionPosture::Planned,
            "planned",
            "intent, planning, or pre-execution boundary meaning",
            "completed execution or attested effectful boundary truth",
        ),
        definition(
            FoundationalBoundaryEvidenceExecutionPosture::Executed,
            "executed",
            "completed effectful or authority-bearing boundary meaning",
            "mere planning intent or blocked/denied non-execution",
        ),
    ]
}

pub const fn foundational_boundary_evidence_descriptive_role_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<FoundationalBoundaryEvidenceDescriptiveRole>; 2]
{
    [
        definition(
            FoundationalBoundaryEvidenceDescriptiveRole::AuthorityAdjacentDescription,
            "authority_adjacent_description",
            "descriptive context that sits next to stronger authority truth",
            "support-grade evidence claims or stronger authority ownership",
        ),
        definition(
            FoundationalBoundaryEvidenceDescriptiveRole::SupportGrade,
            "support_grade",
            "support-authoritative evidence for declared support obligations",
            "stronger authority state or direct authoritative ownership",
        ),
    ]
}

pub const fn foundational_boundary_evidence_freshness_posture_definitions(
) -> [FoundationalBoundaryEvidencePrimitiveDefinition<FoundationalBoundaryEvidenceFreshnessPosture>;
       5] {
    [
        definition(
            FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained,
            "fresh_retained",
            "fresh directly retained supporting context",
            "stale, reduced, replay-derived, or restored-only support",
        ),
        definition(
            FoundationalBoundaryEvidenceFreshnessPosture::StaleRetained,
            "stale_retained",
            "retained context that is no longer current-fresh",
            "replay reconstruction or fully fresh retained support",
        ),
        definition(
            FoundationalBoundaryEvidenceFreshnessPosture::ReducedRetained,
            "reduced_retained",
            "retained context with intentionally reduced support richness",
            "full retained coverage or replay-only reconstruction",
        ),
        definition(
            FoundationalBoundaryEvidenceFreshnessPosture::ReconstructedFromReplay,
            "reconstructed_from_replay",
            "support reconstructed from replay rather than directly retained evidence",
            "fresh retained or direct current observation",
        ),
        definition(
            FoundationalBoundaryEvidenceFreshnessPosture::RestoredFromCheckpoint,
            "restored_from_checkpoint",
            "support re-established from checkpoint or restore/readmission",
            "ordinary current-fresh retained observation",
        ),
    ]
}
