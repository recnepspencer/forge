use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertificationRequiredOutput {
    TopologyTruthDigest,
    NamingTruthDigest,
    TopologyValidationDigest,
    TopologyValidationReport,
    TopologyLocalizationReport,
    NamingAttachmentReport,
    PrimitiveFamilyCoverageMatrix,
    PrimitiveCorpusParityReport,
    AdmittedRangeSweepReport,
    ValidatorCoverageReport,
    BranchLocalTopologyReport,
    ReplayParityReport,
    RejectionClassReport,
    FailureLocalityReport,
    BridgeFamilyCoverageReport,
    BridgeProofReport,
    CounterReport,
    MaterializedTopologyDigest,
    InterpretedTopologyDigest,
    DerivedValidationDigest,
    DerivedTruthBasisDigest,
    BridgeRoutingDigest,
    BridgeHistoricalEvaluationDigest,
    DerivedFamilyCoverageMatrix,
    DerivedFamilyParityMatrix,
    DerivedValidatorCoverageReport,
    DerivedInvalidationReport,
    DerivedRebuildReport,
    DerivedEquivalenceContractReport,
    DerivedFallbackReport,
    DerivedFailureLocalityReport,
    DerivedBranchLocalParityReport,
    DerivedReplayParityReport,
    DerivedBridgeFamilyCoverageReport,
    MilestoneTwoCounterReport,
    MilestoneThreeHostileSuiteReport,
    MilestoneThreeHostileCoverageRows,
    MilestoneThreeHostileFamilyCoverageRows,
    MilestoneThreeRejectionDistributionRows,
    MilestoneThreeNamingDistributionRows,
    MilestoneThreeTopologyEditDigestRows,
    MilestoneThreeNamingContinuityMatrixRows,
    MilestoneThreeRejectedEditScopeReportRows,
    MilestoneThreeEditReplayParityRows,
    MilestoneThreeChangedScopeCoverageRows,
    MilestoneThreeDerivedRegionCoverageRows,
    MilestoneThreeEditBreadthCounterRows,
    MilestoneThreeFailureLocalityRows,
    MilestoneThreeSideQuestCloseoutReport,
    MilestoneThreeReturnGateReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationCanonicalRow {
    pub family: String,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationRejectionRow {
    pub family: String,
    pub role: String,
    pub rejection_class: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationParityRow {
    pub family: String,
    pub parity_kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationValidatorExpectation {
    pub family: String,
    pub validators: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationBridgeExpectation {
    pub family: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationSuiteRequirements {
    pub suite_name: String,
    pub required_family_rows: Vec<String>,
    pub required_rejection_rows: Vec<String>,
    pub validator_expectations: Vec<CertificationValidatorExpectation>,
    pub required_parity_rows: Vec<String>,
    pub required_bridge_rows: Vec<CertificationBridgeExpectation>,
    pub required_outputs: Vec<CertificationRequiredOutput>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationSuiteDefinition {
    pub suite_name: String,
    pub canonical_rows: Vec<CertificationCanonicalRow>,
    pub rejection_rows: Vec<CertificationRejectionRow>,
    pub parity_rows: Vec<CertificationParityRow>,
    pub required_outputs: Vec<CertificationRequiredOutput>,
}
