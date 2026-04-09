use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::error::{BridgeErrorContext, BridgeReplayError, BridgeReplayErrorKind};
use crate::identity::{
    BridgeIdentity, StructuralBranchComparisonRecordIdentityTag, StructuralRemapRecordIdentityTag,
};
use crate::structural::{
    AdmittedStructuralComparisonContract, PlannedStructuralMatchPacketSet,
    PublishedBranchComparisonArtifact, PublishedStructuralRemapArtifact, ReducedStructuralMatchSet,
    StructuralComparisonMode, StructuralFingerprintFamily, StructuralMatchCandidateKind,
    StructuralMatchOutcomeClass, StructuralSchemaIdentity, StructuralTruthViewBasisKind,
};

pub const BRIDGE_CANONICAL_STRUCTURAL_REMAP_RECORD_SCHEMA_V1: &str =
    "forge-runtime-bridge.structural-remap-record.v1";
pub const BRIDGE_CANONICAL_STRUCTURAL_BRANCH_COMPARISON_RECORD_SCHEMA_V1: &str =
    "forge-runtime-bridge.structural-branch-comparison-record.v1";

pub type BridgeStructuralRemapRecordIdentity = BridgeIdentity<StructuralRemapRecordIdentityTag>;
pub type BridgeStructuralBranchComparisonRecordIdentity =
    BridgeIdentity<StructuralBranchComparisonRecordIdentityTag>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BridgeStructuralCounters {
    structural_declaration_count: usize,
    structural_contract_count: usize,
    structural_fingerprint_count: usize,
    structural_match_packet_count: usize,
    structural_candidate_count: usize,
    structural_candidate_cohort_count: usize,
    structural_exact_match_count: usize,
    structural_ambiguity_count: usize,
    structural_mismatch_count: usize,
    structural_identity_conflict_count: usize,
    structural_lineage_divergence_count: usize,
    structural_reuse_publication_count: usize,
    branch_comparison_count: usize,
    branch_comparison_diff_count: usize,
    branch_comparison_drift_rejection_count: usize,
    structural_widened_scan_count: usize,
    structural_replay_request_count: usize,
    structural_replay_mismatch_count: usize,
}

impl BridgeStructuralCounters {
    pub(crate) fn from_structural_outcome(
        contract: &AdmittedStructuralComparisonContract,
        planned_packet_set: &PlannedStructuralMatchPacketSet,
        reduced_match_set: &ReducedStructuralMatchSet,
    ) -> Self {
        let declaration = contract.validated_declaration().declaration();
        let identity_conflict_count = planned_packet_set
            .candidates()
            .iter()
            .filter(|candidate| {
                matches!(
                    candidate.candidate_kind(),
                    StructuralMatchCandidateKind::IdentityAuthorityConflict
                )
            })
            .count();
        let lineage_divergence_count = planned_packet_set
            .candidates()
            .iter()
            .filter(|candidate| {
                matches!(
                    candidate.candidate_kind(),
                    StructuralMatchCandidateKind::LineageStructuralDivergence
                )
            })
            .count();
        let mismatch_count =
            usize::from(reduced_match_set.outcome_class().mismatch_class().is_some());

        Self {
            structural_declaration_count: 1,
            structural_contract_count: 1,
            structural_fingerprint_count: planned_packet_set.target_fingerprint().iter().count()
                + planned_packet_set.comparison_fingerprint().iter().count()
                + planned_packet_set
                    .candidates()
                    .iter()
                    .filter(|candidate| candidate.fingerprint().is_some())
                    .count(),
            structural_match_packet_count: 1,
            structural_candidate_count: planned_packet_set.candidate_count(),
            structural_candidate_cohort_count: planned_packet_set.candidate_count(),
            structural_exact_match_count: reduced_match_set.exact_match_count(),
            structural_ambiguity_count: reduced_match_set.ambiguity_count(),
            structural_mismatch_count: mismatch_count,
            structural_identity_conflict_count: identity_conflict_count,
            structural_lineage_divergence_count: lineage_divergence_count,
            structural_reuse_publication_count: usize::from(matches!(
                reduced_match_set.outcome_class(),
                StructuralMatchOutcomeClass::AdvisoryReuseCandidate
            )),
            branch_comparison_count: usize::from(
                planned_packet_set.comparison_mode() == StructuralComparisonMode::BranchComparison,
            ),
            branch_comparison_diff_count: reduced_match_set.branch_diff_count(),
            branch_comparison_drift_rejection_count: 0,
            structural_widened_scan_count: usize::from(matches!(
                declaration.candidate_scope(),
                crate::structural::StructuralCandidateSearchScope::ExplicitWidenedDebtScan
            )),
            structural_replay_request_count: 0,
            structural_replay_mismatch_count: 0,
        }
    }

    #[cfg(test)]
    pub(crate) fn with_replay_request(mut self) -> Self {
        self.structural_replay_request_count = 1;
        self
    }

    pub fn structural_declaration_count(&self) -> usize {
        self.structural_declaration_count
    }

    pub fn structural_contract_count(&self) -> usize {
        self.structural_contract_count
    }

    pub fn structural_fingerprint_count(&self) -> usize {
        self.structural_fingerprint_count
    }

    pub fn structural_match_packet_count(&self) -> usize {
        self.structural_match_packet_count
    }

    pub fn structural_candidate_count(&self) -> usize {
        self.structural_candidate_count
    }

    pub fn structural_candidate_cohort_count(&self) -> usize {
        self.structural_candidate_cohort_count
    }

    pub fn structural_exact_match_count(&self) -> usize {
        self.structural_exact_match_count
    }

    pub fn structural_ambiguity_count(&self) -> usize {
        self.structural_ambiguity_count
    }

    pub fn structural_mismatch_count(&self) -> usize {
        self.structural_mismatch_count
    }

    pub fn structural_identity_conflict_count(&self) -> usize {
        self.structural_identity_conflict_count
    }

    pub fn structural_lineage_divergence_count(&self) -> usize {
        self.structural_lineage_divergence_count
    }

    pub fn structural_reuse_publication_count(&self) -> usize {
        self.structural_reuse_publication_count
    }

    pub fn branch_comparison_count(&self) -> usize {
        self.branch_comparison_count
    }

    pub fn branch_comparison_diff_count(&self) -> usize {
        self.branch_comparison_diff_count
    }

    pub fn branch_comparison_drift_rejection_count(&self) -> usize {
        self.branch_comparison_drift_rejection_count
    }

    pub fn structural_widened_scan_count(&self) -> usize {
        self.structural_widened_scan_count
    }

    pub fn structural_replay_request_count(&self) -> usize {
        self.structural_replay_request_count
    }

    pub fn structural_replay_mismatch_count(&self) -> usize {
        self.structural_replay_mismatch_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeStructuralRemapRecord {
    record_identity: BridgeStructuralRemapRecordIdentity,
    contract: AdmittedStructuralComparisonContract,
    planned_packet_set: PlannedStructuralMatchPacketSet,
    reduced_match_set: ReducedStructuralMatchSet,
    artifact: PublishedStructuralRemapArtifact,
    counters: BridgeStructuralCounters,
    canonical_basis: Arc<str>,
}

impl BridgeStructuralRemapRecord {
    pub(crate) fn new(
        contract: AdmittedStructuralComparisonContract,
        planned_packet_set: PlannedStructuralMatchPacketSet,
        reduced_match_set: ReducedStructuralMatchSet,
        artifact: PublishedStructuralRemapArtifact,
        counters: BridgeStructuralCounters,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "structural-remap-record|contract={}|planned={}|reduced={}|artifact={}",
            contract.digest(),
            planned_packet_set.digest(),
            reduced_match_set.digest(),
            artifact.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            record_identity: BridgeStructuralRemapRecordIdentity::new(format!(
                "structural-remap-record:sha256:{digest:x}"
            )),
            contract,
            planned_packet_set,
            reduced_match_set,
            artifact,
            counters,
            canonical_basis,
        }
    }

    pub fn record_identity(&self) -> &BridgeStructuralRemapRecordIdentity {
        &self.record_identity
    }

    pub fn contract(&self) -> &AdmittedStructuralComparisonContract {
        &self.contract
    }

    pub fn planned_packet_set(&self) -> &PlannedStructuralMatchPacketSet {
        &self.planned_packet_set
    }

    pub fn reduced_match_set(&self) -> &ReducedStructuralMatchSet {
        &self.reduced_match_set
    }

    pub fn artifact(&self) -> &PublishedStructuralRemapArtifact {
        &self.artifact
    }

    pub fn counters(&self) -> &BridgeStructuralCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeCanonicalStructuralRemapRecord {
    schema_version: Arc<str>,
    record: BridgeStructuralRemapRecord,
}

impl BridgeCanonicalStructuralRemapRecord {
    pub(crate) fn new(record: BridgeStructuralRemapRecord) -> Self {
        Self {
            schema_version: Arc::from(BRIDGE_CANONICAL_STRUCTURAL_REMAP_RECORD_SCHEMA_V1),
            record,
        }
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version.as_ref()
    }

    pub fn record_identity(&self) -> &BridgeStructuralRemapRecordIdentity {
        self.record.record_identity()
    }

    pub fn contract(&self) -> &AdmittedStructuralComparisonContract {
        self.record.contract()
    }

    pub fn planned_packet_set(&self) -> &PlannedStructuralMatchPacketSet {
        self.record.planned_packet_set()
    }

    pub fn reduced_match_set(&self) -> &ReducedStructuralMatchSet {
        self.record.reduced_match_set()
    }

    pub fn artifact(&self) -> &PublishedStructuralRemapArtifact {
        self.record.artifact()
    }

    pub fn counters(&self) -> &BridgeStructuralCounters {
        self.record.counters()
    }

    #[cfg(test)]
    pub(crate) fn with_schema_version_for_test(
        mut self,
        schema_version: impl Into<Arc<str>>,
    ) -> Self {
        self.schema_version = schema_version.into();
        self
    }

    pub(crate) fn decode(&self) -> Result<BridgeStructuralRemapRecord, BridgeReplayError> {
        if self.schema_version() != BRIDGE_CANONICAL_STRUCTURAL_REMAP_RECORD_SCHEMA_V1 {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::CanonicalArtifactCompatibilityFailure,
                format!(
                    "Bridge canonical structural remap record schema `{}` is not supported; expected `{}`.",
                    self.schema_version(),
                    BRIDGE_CANONICAL_STRUCTURAL_REMAP_RECORD_SCHEMA_V1
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }

        Ok(self.record.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeStructuralBranchComparisonRecord {
    record_identity: BridgeStructuralBranchComparisonRecordIdentity,
    contract: AdmittedStructuralComparisonContract,
    planned_packet_set: PlannedStructuralMatchPacketSet,
    reduced_match_set: ReducedStructuralMatchSet,
    artifact: PublishedBranchComparisonArtifact,
    counters: BridgeStructuralCounters,
    canonical_basis: Arc<str>,
}

impl BridgeStructuralBranchComparisonRecord {
    pub(crate) fn new(
        contract: AdmittedStructuralComparisonContract,
        planned_packet_set: PlannedStructuralMatchPacketSet,
        reduced_match_set: ReducedStructuralMatchSet,
        artifact: PublishedBranchComparisonArtifact,
        counters: BridgeStructuralCounters,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "structural-branch-comparison-record|contract={}|planned={}|reduced={}|artifact={}",
            contract.digest(),
            planned_packet_set.digest(),
            reduced_match_set.digest(),
            artifact.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            record_identity: BridgeStructuralBranchComparisonRecordIdentity::new(format!(
                "structural-branch-comparison-record:sha256:{digest:x}"
            )),
            contract,
            planned_packet_set,
            reduced_match_set,
            artifact,
            counters,
            canonical_basis,
        }
    }

    pub fn record_identity(&self) -> &BridgeStructuralBranchComparisonRecordIdentity {
        &self.record_identity
    }

    pub fn contract(&self) -> &AdmittedStructuralComparisonContract {
        &self.contract
    }

    pub fn planned_packet_set(&self) -> &PlannedStructuralMatchPacketSet {
        &self.planned_packet_set
    }

    pub fn reduced_match_set(&self) -> &ReducedStructuralMatchSet {
        &self.reduced_match_set
    }

    pub fn artifact(&self) -> &PublishedBranchComparisonArtifact {
        &self.artifact
    }

    pub fn counters(&self) -> &BridgeStructuralCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeCanonicalStructuralBranchComparisonRecord {
    schema_version: Arc<str>,
    record: BridgeStructuralBranchComparisonRecord,
}

impl BridgeCanonicalStructuralBranchComparisonRecord {
    pub(crate) fn new(record: BridgeStructuralBranchComparisonRecord) -> Self {
        Self {
            schema_version: Arc::from(
                BRIDGE_CANONICAL_STRUCTURAL_BRANCH_COMPARISON_RECORD_SCHEMA_V1,
            ),
            record,
        }
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version.as_ref()
    }

    pub fn record_identity(&self) -> &BridgeStructuralBranchComparisonRecordIdentity {
        self.record.record_identity()
    }

    pub fn contract(&self) -> &AdmittedStructuralComparisonContract {
        self.record.contract()
    }

    pub fn planned_packet_set(&self) -> &PlannedStructuralMatchPacketSet {
        self.record.planned_packet_set()
    }

    pub fn reduced_match_set(&self) -> &ReducedStructuralMatchSet {
        self.record.reduced_match_set()
    }

    pub fn artifact(&self) -> &PublishedBranchComparisonArtifact {
        self.record.artifact()
    }

    pub fn counters(&self) -> &BridgeStructuralCounters {
        self.record.counters()
    }

    #[cfg(test)]
    pub(crate) fn with_schema_version_for_test(
        mut self,
        schema_version: impl Into<Arc<str>>,
    ) -> Self {
        self.schema_version = schema_version.into();
        self
    }

    pub(crate) fn decode(
        &self,
    ) -> Result<BridgeStructuralBranchComparisonRecord, BridgeReplayError> {
        if self.schema_version() != BRIDGE_CANONICAL_STRUCTURAL_BRANCH_COMPARISON_RECORD_SCHEMA_V1 {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::CanonicalArtifactCompatibilityFailure,
                format!(
                    "Bridge canonical structural branch comparison record schema `{}` is not supported; expected `{}`.",
                    self.schema_version(),
                    BRIDGE_CANONICAL_STRUCTURAL_BRANCH_COMPARISON_RECORD_SCHEMA_V1
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }

        Ok(self.record.clone())
    }
}

pub type BridgeStructuralRemapReplaySummary = PublishedStructuralRemapArtifact;
pub type BridgeStructuralBranchComparisonReplaySummary = PublishedBranchComparisonArtifact;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeStructuralRemapExplanation {
    record_identity: BridgeStructuralRemapRecordIdentity,
    declaration_identity: String,
    schema_identity: StructuralSchemaIdentity,
    fingerprint_family: StructuralFingerprintFamily,
    semantics_version: Arc<str>,
    truth_view_basis_kind: StructuralTruthViewBasisKind,
    candidate_count: usize,
    outcome_class: StructuralMatchOutcomeClass,
    counters: BridgeStructuralCounters,
    artifact_digest: Arc<str>,
}

impl BridgeStructuralRemapExplanation {
    pub fn from_canonical_record(record: &BridgeCanonicalStructuralRemapRecord) -> Self {
        let declaration = record.contract().validated_declaration().declaration();
        let equivalence = declaration.equivalence_contract();
        Self {
            record_identity: record.record_identity().clone(),
            declaration_identity: declaration.declaration_identity().as_str().to_owned(),
            schema_identity: declaration.schema_identity().clone(),
            fingerprint_family: equivalence.fingerprint_family(),
            semantics_version: Arc::from(equivalence.semantics_version()),
            truth_view_basis_kind: declaration.truth_view_basis().basis_kind(),
            candidate_count: record.planned_packet_set().candidate_count(),
            outcome_class: record.reduced_match_set().outcome_class(),
            counters: *record.counters(),
            artifact_digest: Arc::from(record.artifact().digest()),
        }
    }

    pub fn record_identity(&self) -> &BridgeStructuralRemapRecordIdentity {
        &self.record_identity
    }

    pub fn declaration_identity(&self) -> &str {
        &self.declaration_identity
    }

    pub fn schema_identity(&self) -> &StructuralSchemaIdentity {
        &self.schema_identity
    }

    pub fn fingerprint_family(&self) -> StructuralFingerprintFamily {
        self.fingerprint_family
    }

    pub fn semantics_version(&self) -> &str {
        self.semantics_version.as_ref()
    }

    pub fn truth_view_basis_kind(&self) -> StructuralTruthViewBasisKind {
        self.truth_view_basis_kind
    }

    pub fn candidate_count(&self) -> usize {
        self.candidate_count
    }

    pub fn outcome_class(&self) -> StructuralMatchOutcomeClass {
        self.outcome_class
    }

    pub fn counters(&self) -> &BridgeStructuralCounters {
        &self.counters
    }

    pub fn artifact_digest(&self) -> &str {
        self.artifact_digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeStructuralBranchComparisonExplanation {
    record_identity: BridgeStructuralBranchComparisonRecordIdentity,
    declaration_identity: String,
    schema_identity: StructuralSchemaIdentity,
    fingerprint_family: StructuralFingerprintFamily,
    semantics_version: Arc<str>,
    branch_diff_count: usize,
    candidate_count: usize,
    counters: BridgeStructuralCounters,
    artifact_digest: Arc<str>,
}

impl BridgeStructuralBranchComparisonExplanation {
    pub fn from_canonical_record(record: &BridgeCanonicalStructuralBranchComparisonRecord) -> Self {
        let declaration = record.contract().validated_declaration().declaration();
        let equivalence = declaration.equivalence_contract();
        Self {
            record_identity: record.record_identity().clone(),
            declaration_identity: declaration.declaration_identity().as_str().to_owned(),
            schema_identity: declaration.schema_identity().clone(),
            fingerprint_family: equivalence.fingerprint_family(),
            semantics_version: Arc::from(equivalence.semantics_version()),
            branch_diff_count: record.reduced_match_set().branch_diff_count(),
            candidate_count: record.planned_packet_set().candidate_count(),
            counters: *record.counters(),
            artifact_digest: Arc::from(record.artifact().digest()),
        }
    }

    pub fn record_identity(&self) -> &BridgeStructuralBranchComparisonRecordIdentity {
        &self.record_identity
    }

    pub fn declaration_identity(&self) -> &str {
        &self.declaration_identity
    }

    pub fn schema_identity(&self) -> &StructuralSchemaIdentity {
        &self.schema_identity
    }

    pub fn fingerprint_family(&self) -> StructuralFingerprintFamily {
        self.fingerprint_family
    }

    pub fn semantics_version(&self) -> &str {
        self.semantics_version.as_ref()
    }

    pub fn branch_diff_count(&self) -> usize {
        self.branch_diff_count
    }

    pub fn candidate_count(&self) -> usize {
        self.candidate_count
    }

    pub fn counters(&self) -> &BridgeStructuralCounters {
        &self.counters
    }

    pub fn artifact_digest(&self) -> &str {
        self.artifact_digest.as_ref()
    }
}

pub(crate) fn validate_structural_replay_contract(
    original: &AdmittedStructuralComparisonContract,
    reconstructed: &AdmittedStructuralComparisonContract,
) -> Result<(), BridgeReplayError> {
    if reconstructed.digest() != original.digest() {
        return Err(BridgeReplayError::new(
            BridgeReplayErrorKind::PlanningContractMismatch,
            format!(
                "Bridge structural replay reconstructed contract `{}` but original contract was `{}`.",
                reconstructed.contract_identity().as_str(),
                original.contract_identity().as_str()
            ),
        )
        .with_context(BridgeErrorContext::default()));
    }

    Ok(())
}

pub(crate) fn validate_structural_replay_outcome(
    planned: &PlannedStructuralMatchPacketSet,
    reduced: &ReducedStructuralMatchSet,
    expected_mode: StructuralComparisonMode,
) -> Result<(), BridgeReplayError> {
    if planned.comparison_mode() != expected_mode {
        return Err(BridgeReplayError::new(
            BridgeReplayErrorKind::PlanningContractMismatch,
            format!(
                "Bridge structural replay reconstructed comparison mode `{:?}` but expected `{:?}`.",
                planned.comparison_mode(),
                expected_mode
            ),
        )
        .with_context(BridgeErrorContext::default()));
    }

    if reduced.planned_packet_set().digest() != planned.digest() {
        return Err(BridgeReplayError::new(
            BridgeReplayErrorKind::DigestMismatch,
            format!(
                "Bridge structural replay reconstructed reduced packet basis `{}` but planned packet set digest was `{}`.",
                reduced.planned_packet_set().digest(),
                planned.digest()
            ),
        )
        .with_context(BridgeErrorContext::default()));
    }

    Ok(())
}
