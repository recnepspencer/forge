use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::error::{BridgeErrorContext, BridgeReplayError, BridgeReplayErrorKind};
use crate::identity::{
    BridgeIdentity, StructuralBranchComparisonRecordIdentityTag, StructuralRemapRecordIdentityTag,
};
use crate::structural::{
    AdmittedStructuralComparisonContract, PlannedStructuralMatchPacketSet,
    PublishedBranchComparisonArtifact, PublishedStructuralRemapArtifact, ReducedStructuralMatchSet,
};

use super::counters::BridgeStructuralCounters;

pub const BRIDGE_CANONICAL_STRUCTURAL_REMAP_RECORD_SCHEMA_V1: &str =
    "forge-runtime-bridge.structural-remap-record.v1";
pub const BRIDGE_CANONICAL_STRUCTURAL_BRANCH_COMPARISON_RECORD_SCHEMA_V1: &str =
    "forge-runtime-bridge.structural-branch-comparison-record.v1";

pub type BridgeStructuralRemapRecordIdentity = BridgeIdentity<StructuralRemapRecordIdentityTag>;
pub type BridgeStructuralBranchComparisonRecordIdentity =
    BridgeIdentity<StructuralBranchComparisonRecordIdentityTag>;

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
            record_identity: BridgeStructuralRemapRecordIdentity::admit_bridge_owned(format!(
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

    pub(crate) fn decode(&self) -> Result<BridgeStructuralRemapRecord, BridgeReplayError> {
        if self.schema_version() != BRIDGE_CANONICAL_STRUCTURAL_REMAP_RECORD_SCHEMA_V1 {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::CanonicalArtifactCoherenceFailure,
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
            record_identity: BridgeStructuralBranchComparisonRecordIdentity::admit_bridge_owned(
                format!("structural-branch-comparison-record:sha256:{digest:x}"),
            ),
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

    pub(crate) fn decode(
        &self,
    ) -> Result<BridgeStructuralBranchComparisonRecord, BridgeReplayError> {
        if self.schema_version() != BRIDGE_CANONICAL_STRUCTURAL_BRANCH_COMPARISON_RECORD_SCHEMA_V1 {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::CanonicalArtifactCoherenceFailure,
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
