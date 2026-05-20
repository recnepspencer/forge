use crate::canonicalization::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisValue, CanonicalIntegerWidth, CanonicalizationRuleVersion,
};
use crate::performance::{
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceAllocationPosture,
    FoundationalPerformanceAttachmentTargetKind, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBudgetKind,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceLayoutIntent, FoundationalPerformanceReportMaterializationBoundary,
    FoundationalPerformanceReportSection, FoundationalPerformanceReportSectionDecisionCause,
    FoundationalPerformanceWorkClass,
};

pub fn performance_basis_rule_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("forge.performance.v1")
        .expect("performance basis version should be valid")
}

pub(super) fn claim_text_entry(locus: &str, value: &str) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Performance,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::PerformanceClaim,
        CanonicalBasisValue::ExactText(value.to_string().into()),
    )
}

pub(super) fn claim_bool_entry(locus: &str, value: bool) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Performance,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::PerformanceClaim,
        CanonicalBasisValue::Bool(value),
    )
}

pub(super) fn layout_text_entry(locus: &str, value: &str) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Performance,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::PerformanceLayout,
        CanonicalBasisValue::ExactText(value.to_string().into()),
    )
}

pub(super) fn layout_bool_entry(locus: &str, value: bool) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Performance,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::PerformanceLayout,
        CanonicalBasisValue::Bool(value),
    )
}

pub(super) fn counter_text_entry(locus: &str, value: &str) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Performance,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::PerformanceCounter,
        CanonicalBasisValue::ExactText(value.to_string().into()),
    )
}

pub(super) fn counter_integer_entry(locus: &str, value: u64) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Performance,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::PerformanceCounter,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: u128::from(value),
        },
    )
}

pub(super) fn support_text_entry(locus: &str, value: &str) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Performance,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::PerformanceSupport,
        CanonicalBasisValue::ExactText(value.to_string().into()),
    )
}

pub(super) fn boundary_token(value: FoundationalPerformanceBoundary) -> &'static str {
    match value {
        FoundationalPerformanceBoundary::AuthoritativeExecution => "authoritative-execution",
        FoundationalPerformanceBoundary::BoundaryMaterialization => "boundary-materialization",
        FoundationalPerformanceBoundary::ReplayReconstruction => "replay-reconstruction",
        FoundationalPerformanceBoundary::SupportAssembly => "support-assembly",
        FoundationalPerformanceBoundary::MaintenancePlanning => "maintenance-planning",
        FoundationalPerformanceBoundary::MaintenanceExecution => "maintenance-execution",
        FoundationalPerformanceBoundary::Publication => "publication",
        FoundationalPerformanceBoundary::Delivery => "delivery",
        FoundationalPerformanceBoundary::RetentionCompaction => "retention-compaction",
        FoundationalPerformanceBoundary::RestoreRecovery => "restore-recovery",
    }
}

pub(super) fn evidence_strength_token(
    value: FoundationalPerformanceEvidenceStrength,
) -> &'static str {
    match value {
        FoundationalPerformanceEvidenceStrength::CompileTimeContract => "compile-time-contract",
        FoundationalPerformanceEvidenceStrength::RuntimePolicyAdmission => {
            "runtime-policy-admission"
        }
        FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt => {
            "counter-backed-execution-receipt"
        }
        FoundationalPerformanceEvidenceStrength::SupportDerivedPerformanceClaim => {
            "support-derived-performance-claim"
        }
        FoundationalPerformanceEvidenceStrength::ExplicitDebtDeferredClaim => {
            "explicit-debt-deferred-claim"
        }
    }
}

pub(super) fn breadth_locality_token(
    value: FoundationalPerformanceBreadthLocalityPosture,
) -> &'static str {
    match value {
        FoundationalPerformanceBreadthLocalityPosture::PointLocal => "point-local",
        FoundationalPerformanceBreadthLocalityPosture::FamilyLocalBatch => "family-local-batch",
        FoundationalPerformanceBreadthLocalityPosture::BasisLocalBatch => "basis-local-batch",
        FoundationalPerformanceBreadthLocalityPosture::BranchLocal => "branch-local",
        FoundationalPerformanceBreadthLocalityPosture::SnapshotBound => "snapshot-bound",
        FoundationalPerformanceBreadthLocalityPosture::DeltaBound => "delta-bound",
        FoundationalPerformanceBreadthLocalityPosture::CrossPartitionOrCrossRegion => {
            "cross-partition-cross-region"
        }
        FoundationalPerformanceBreadthLocalityPosture::PortabilityScope => "portability-scope",
        FoundationalPerformanceBreadthLocalityPosture::OperatorOrGlobalDebt => {
            "operator-global-debt"
        }
    }
}

pub(super) fn access_pattern_token(
    value: FoundationalPerformanceAccessPatternPosture,
) -> &'static str {
    match value {
        FoundationalPerformanceAccessPatternPosture::ScanHeavy => "scan-heavy",
        FoundationalPerformanceAccessPatternPosture::PointLookup => "point-lookup",
        FoundationalPerformanceAccessPatternPosture::TraversalLocal => "traversal-local",
        FoundationalPerformanceAccessPatternPosture::AppendHeavy => "append-heavy",
        FoundationalPerformanceAccessPatternPosture::RebuildCapable => "rebuild-capable",
        FoundationalPerformanceAccessPatternPosture::DensityAdaptive => "density-adaptive",
    }
}

pub(super) fn execution_temperature_token(
    value: FoundationalPerformanceExecutionTemperature,
) -> &'static str {
    match value {
        FoundationalPerformanceExecutionTemperature::HotPath => "hot-path",
        FoundationalPerformanceExecutionTemperature::WarmPath => "warm-path",
        FoundationalPerformanceExecutionTemperature::ColdPath => "cold-path",
        FoundationalPerformanceExecutionTemperature::RecoveryOnly => "recovery-only",
        FoundationalPerformanceExecutionTemperature::SupportOnly => "support-only",
    }
}

pub(super) fn freshness_retention_token(
    value: FoundationalPerformanceFreshnessRetentionPosture,
) -> &'static str {
    match value {
        FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent => {
            "exact-basis-current"
        }
        FoundationalPerformanceFreshnessRetentionPosture::HistoricalRetained => {
            "historical-retained"
        }
        FoundationalPerformanceFreshnessRetentionPosture::ReplayDerived => "replay-derived",
        FoundationalPerformanceFreshnessRetentionPosture::RestoredReadmitted => {
            "restored-readmitted"
        }
        FoundationalPerformanceFreshnessRetentionPosture::StaleSupport => "stale-support",
        FoundationalPerformanceFreshnessRetentionPosture::ReducedRetention => "reduced-retention",
    }
}

pub(super) fn fallback_debt_token(
    value: FoundationalPerformanceFallbackDebtPosture,
) -> &'static str {
    match value {
        FoundationalPerformanceFallbackDebtPosture::Verified => "verified",
        FoundationalPerformanceFallbackDebtPosture::Deferred => "deferred",
        FoundationalPerformanceFallbackDebtPosture::Debt => "debt",
        FoundationalPerformanceFallbackDebtPosture::Rejected => "rejected",
        FoundationalPerformanceFallbackDebtPosture::WidenedWithExplicitDisclosure => {
            "widened-with-explicit-disclosure"
        }
        FoundationalPerformanceFallbackDebtPosture::FreshFreezeRebuildReadmissionRequired => {
            "fresh-freeze-rebuild-readmission-required"
        }
    }
}

pub(super) fn layout_intent_token(value: FoundationalPerformanceLayoutIntent) -> &'static str {
    match value {
        FoundationalPerformanceLayoutIntent::AoS => "aos",
        FoundationalPerformanceLayoutIntent::SoA => "soa",
        FoundationalPerformanceLayoutIntent::AoSoA => "aosoa",
        FoundationalPerformanceLayoutIntent::Sparse => "sparse",
        FoundationalPerformanceLayoutIntent::Packed => "packed",
        FoundationalPerformanceLayoutIntent::Custom => "custom",
    }
}

pub(super) fn allocation_posture_token(
    value: FoundationalPerformanceAllocationPosture,
) -> &'static str {
    match value {
        FoundationalPerformanceAllocationPosture::NoAllocation => "no-allocation",
        FoundationalPerformanceAllocationPosture::ActionLocal => "action-local",
        FoundationalPerformanceAllocationPosture::ArenaLocal => "arena-local",
        FoundationalPerformanceAllocationPosture::BatchLocal => "batch-local",
        FoundationalPerformanceAllocationPosture::ManifestScoped => "manifest-scoped",
        FoundationalPerformanceAllocationPosture::RebuildScoped => "rebuild-scoped",
    }
}

pub(super) fn work_class_token(value: FoundationalPerformanceWorkClass) -> &'static str {
    match value {
        FoundationalPerformanceWorkClass::AuthoritativeMutation => "authoritative-mutation",
        FoundationalPerformanceWorkClass::ValidationPlanning => "validation-planning",
        FoundationalPerformanceWorkClass::PublicationDelivery => "publication-delivery",
        FoundationalPerformanceWorkClass::ReplayReconstruction => "replay-reconstruction",
        FoundationalPerformanceWorkClass::SupportReportAssembly => "support-report-assembly",
        FoundationalPerformanceWorkClass::ForensicParity => "forensic-parity",
    }
}

pub(super) fn report_section_token(value: FoundationalPerformanceReportSection) -> &'static str {
    match value {
        FoundationalPerformanceReportSection::Claim => "claim",
        FoundationalPerformanceReportSection::LayoutIntent => "layout-intent",
        FoundationalPerformanceReportSection::ContractNames => "contract-names",
        FoundationalPerformanceReportSection::CounterSpecs => "counter-specs",
        FoundationalPerformanceReportSection::CounterRows => "counter-rows",
        FoundationalPerformanceReportSection::SupportingEvidenceRows => "supporting-evidence-rows",
        FoundationalPerformanceReportSection::BudgetDecisions => "budget-decisions",
        FoundationalPerformanceReportSection::DeniedWork => "denied-work",
        FoundationalPerformanceReportSection::WidenedWork => "widened-work",
    }
}

pub(super) fn report_decision_cause_token(
    value: FoundationalPerformanceReportSectionDecisionCause,
) -> &'static str {
    match value {
        FoundationalPerformanceReportSectionDecisionCause::AlwaysPresent => "always-present",
        FoundationalPerformanceReportSectionDecisionCause::Requested => "requested",
        FoundationalPerformanceReportSectionDecisionCause::NotRequested => "not-requested",
        FoundationalPerformanceReportSectionDecisionCause::UnavailableFromSource => {
            "unavailable-from-source"
        }
        FoundationalPerformanceReportSectionDecisionCause::ProfileElided => "profile-elided",
    }
}

pub(super) fn report_materialization_boundary_token(
    value: FoundationalPerformanceReportMaterializationBoundary,
) -> &'static str {
    match value {
        FoundationalPerformanceReportMaterializationBoundary::ClaimInspectionOnly => {
            "claim-inspection-only"
        }
        FoundationalPerformanceReportMaterializationBoundary::ReportAssembly => "report-assembly",
        FoundationalPerformanceReportMaterializationBoundary::SupportExpansion => {
            "support-expansion"
        }
    }
}

pub(super) fn attachment_target_token(
    value: FoundationalPerformanceAttachmentTargetKind,
) -> &'static str {
    match value {
        FoundationalPerformanceAttachmentTargetKind::BoundarySummary => "boundary-summary",
        FoundationalPerformanceAttachmentTargetKind::BoundaryReceipt => "boundary-receipt",
        FoundationalPerformanceAttachmentTargetKind::BoundaryReport => "boundary-report",
        FoundationalPerformanceAttachmentTargetKind::BoundaryArtifact => "boundary-artifact",
        FoundationalPerformanceAttachmentTargetKind::SupportBundle => "support-bundle",
        FoundationalPerformanceAttachmentTargetKind::CertificationBundle => "certification-bundle",
    }
}

pub(super) fn budget_kind_token(value: FoundationalPerformanceBudgetKind) -> &'static str {
    match value {
        FoundationalPerformanceBudgetKind::Breadth => "breadth",
        FoundationalPerformanceBudgetKind::Density => "density",
        FoundationalPerformanceBudgetKind::Locality => "locality",
        FoundationalPerformanceBudgetKind::FreshnessSensitive => "freshness-sensitive",
    }
}
