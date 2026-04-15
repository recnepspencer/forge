use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorthCertificationRequiredOutput {
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthCertificationCanonicalRow {
    pub family: String,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthCertificationRejectionRow {
    pub family: String,
    pub role: String,
    pub rejection_class: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthCertificationParityRow {
    pub family: String,
    pub parity_kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthCertificationValidatorExpectation {
    pub family: String,
    pub validators: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthCertificationBridgeExpectation {
    pub family: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthCertificationSuiteRequirements {
    pub suite_name: String,
    pub required_family_rows: Vec<String>,
    pub required_rejection_rows: Vec<String>,
    pub validator_expectations: Vec<WorthCertificationValidatorExpectation>,
    pub required_parity_rows: Vec<String>,
    pub required_bridge_rows: Vec<WorthCertificationBridgeExpectation>,
    pub required_outputs: Vec<WorthCertificationRequiredOutput>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthCertificationSuiteDefinition {
    pub suite_name: String,
    pub canonical_rows: Vec<WorthCertificationCanonicalRow>,
    pub rejection_rows: Vec<WorthCertificationRejectionRow>,
    pub parity_rows: Vec<WorthCertificationParityRow>,
    pub required_outputs: Vec<WorthCertificationRequiredOutput>,
}
