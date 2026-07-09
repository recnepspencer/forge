#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalFoundationEvidenceField {
    PhysicalLayoutReport,
    ArtifactDigest,
    FailureDigest,
    CounterSnapshot,
    ResourceEnvelopeReport,
    HardwareAssumptionReport,
    FoundationalCanonicalBasisBundle,
    FoundationalDiagnosticBundle,
    FoundationalProfileMaterializationPlan,
    FoundationalBoundaryEvidenceBundle,
    FoundationalProvenanceSupportTruth,
    FoundationalCounterBackedPerformanceReceipt,
}

impl PhysicalFoundationEvidenceField {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PhysicalLayoutReport => "physical_layout_report",
            Self::ArtifactDigest => "artifact_digest",
            Self::FailureDigest => "failure_digest",
            Self::CounterSnapshot => "counter_snapshot",
            Self::ResourceEnvelopeReport => "resource_envelope_report",
            Self::HardwareAssumptionReport => "hardware_assumption_report",
            Self::FoundationalCanonicalBasisBundle => "foundational_canonical_basis_bundle",
            Self::FoundationalDiagnosticBundle => "foundational_diagnostic_bundle",
            Self::FoundationalProfileMaterializationPlan => {
                "foundational_profile_materialization_plan"
            }
            Self::FoundationalBoundaryEvidenceBundle => "foundational_boundary_evidence_bundle",
            Self::FoundationalProvenanceSupportTruth => "foundational_provenance_support_truth",
            Self::FoundationalCounterBackedPerformanceReceipt => {
                "foundational_counter_backed_performance_receipt"
            }
        }
    }

    pub const fn required_for_s1() -> [Self; 12] {
        [
            Self::PhysicalLayoutReport,
            Self::ArtifactDigest,
            Self::FailureDigest,
            Self::CounterSnapshot,
            Self::ResourceEnvelopeReport,
            Self::HardwareAssumptionReport,
            Self::FoundationalCanonicalBasisBundle,
            Self::FoundationalDiagnosticBundle,
            Self::FoundationalProfileMaterializationPlan,
            Self::FoundationalBoundaryEvidenceBundle,
            Self::FoundationalProvenanceSupportTruth,
            Self::FoundationalCounterBackedPerformanceReceipt,
        ]
    }

    pub const fn required_for_s2_foundational_residency() -> [Self; 5] {
        [
            Self::CounterSnapshot,
            Self::PhysicalLayoutReport,
            Self::FoundationalProfileMaterializationPlan,
            Self::FoundationalProvenanceSupportTruth,
            Self::FoundationalCounterBackedPerformanceReceipt,
        ]
    }
}
