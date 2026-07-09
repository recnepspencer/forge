use super::{
    FoundationalPerformanceBoundaryDefinition, FoundationalPerformanceEvidenceStrengthDefinition,
    FoundationalPerformanceLayoutIntentDefinition, FoundationalPerformancePrimitiveDefinition,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceLayoutIntent {
    AoS,
    SoA,
    AoSoA,
    Sparse,
    Packed,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceBoundary {
    AuthoritativeExecution,
    BoundaryMaterialization,
    ReplayReconstruction,
    SupportAssembly,
    MaintenancePlanning,
    MaintenanceExecution,
    Publication,
    Delivery,
    RetentionCompaction,
    RestoreRecovery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceEvidenceStrength {
    CompileTimeContract,
    RuntimePolicyAdmission,
    CounterBackedExecutionReceipt,
    SupportDerivedPerformanceClaim,
    ExplicitDebtDeferredClaim,
}

pub fn foundational_performance_layout_intent_definitions(
) -> [FoundationalPerformanceLayoutIntentDefinition; 6] {
    [
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceLayoutIntent::AoS,
            "aos",
            "row-oriented layouts optimized around whole-record locality",
            "cost equivalence with columnar, sparse, or packed layouts",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceLayoutIntent::SoA,
            "soa",
            "column-oriented layouts optimized around field-family access",
            "proof that adjacent record traversal or packed storage is equally cheap",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceLayoutIntent::AoSoA,
            "aosoa",
            "chunked hybrid layouts tuned around grouped vectorized access",
            "a claim that row and column locality collapse into one cost story",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceLayoutIntent::Sparse,
            "sparse",
            "layouts that prioritize omission-aware storage and lookup",
            "a claim that dense scans or packed traversal are equivalent by default",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceLayoutIntent::Packed,
            "packed",
            "layouts optimized around compact contiguous materialization",
            "a promise that maintenance, mutation, or sparse rebuild costs disappear",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceLayoutIntent::Custom,
            "custom",
            "crate-local representations that still lower into shared boundary meaning",
            "permission to bypass the shared vocabulary or comparison law",
        ),
    ]
}

pub fn foundational_performance_boundary_definitions(
) -> [FoundationalPerformanceBoundaryDefinition; 10] {
    [
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBoundary::AuthoritativeExecution,
            "authoritative_execution",
            "current-basis operational work that mutates, validates, or serves authority",
            "report assembly, replay, or support-derived reconstruction cost",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBoundary::BoundaryMaterialization,
            "boundary_materialization",
            "explicit report or artifact assembly at a boundary seam",
            "authoritative hot-path execution cost",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBoundary::ReplayReconstruction,
            "replay_reconstruction",
            "work that replays, reconstructs, or rehydrates prior state",
            "current-basis execution truth",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBoundary::SupportAssembly,
            "support_assembly",
            "support and forensic rows assembled for operators or diagnostics",
            "narrow hot-path operational work",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBoundary::MaintenancePlanning,
            "maintenance_planning",
            "pre-execution planning or budgeting for wider work",
            "proof that execution has already happened",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBoundary::MaintenanceExecution,
            "maintenance_execution",
            "maintenance work that actually executes after admission",
            "the same semantics as foreground delivery or publication",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBoundary::Publication,
            "publication",
            "publication work that exposes committed or derived results",
            "validation, replay, or report-materialization cost by default",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBoundary::Delivery,
            "delivery",
            "delivery work that moves already-shaped output across a boundary",
            "mutation authority or storage-retention work",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBoundary::RetentionCompaction,
            "retention_compaction",
            "retention, compaction, or storage-shape maintenance work",
            "foreground execution truth for reads or commits",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBoundary::RestoreRecovery,
            "restore_recovery",
            "restore, recovery, or readmission-oriented work",
            "fresh current-basis hot-path execution",
        ),
    ]
}

pub fn foundational_performance_evidence_strength_definitions(
) -> [FoundationalPerformanceEvidenceStrengthDefinition; 5] {
    [
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceEvidenceStrength::CompileTimeContract,
            "compile_time_contract",
            "compile-time or static contract law with no runtime execution evidence",
            "counter-backed execution or support-materialized proof",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceEvidenceStrength::RuntimePolicyAdmission,
            "runtime_policy_admission",
            "runtime policy outcomes that admit, defer, widen, or reject work before execution",
            "proof that counted execution already happened",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt,
            "counter_backed_execution_receipt",
            "executed work backed by shared structural counters",
            "stronger proof-lane certification or replay/support derivation",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceEvidenceStrength::SupportDerivedPerformanceClaim,
            "support_derived_performance_claim",
            "support, replay, or forensic cost claims derived from wider evidence",
            "authoritative current-basis execution truth",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceEvidenceStrength::ExplicitDebtDeferredClaim,
            "explicit_debt_deferred_claim",
            "admitted debt or deferred disclosure when strong evidence is not available yet",
            "verified narrow execution evidence",
        ),
    ]
}
