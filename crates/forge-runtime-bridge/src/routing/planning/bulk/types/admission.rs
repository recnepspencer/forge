#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeParallelLegalityClass {
    SerialOnly,
    ParallelPreparationLegal,
    ParallelPreparationIllegal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeParallelLegalityReason {
    BelowMinWorkloadWidth,
    SharedTruthViewMaterializationTarget,
    ContinuityRemapRequiresSerialPreparation,
    DisjointPacketRegionsCertified,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeParallelLegalityDecision {
    class: BridgeParallelLegalityClass,
    reason: BridgeParallelLegalityReason,
    digest: Arc<str>,
}

impl BridgeParallelLegalityDecision {
    pub(crate) fn new(
        class: BridgeParallelLegalityClass,
        reason: BridgeParallelLegalityReason,
    ) -> Self {
        let basis = format!(
            "bridge-parallel-legality-decision|class={}|reason={}",
            super::super::planner::parallel_legality_class_label(class),
            super::super::planner::parallel_legality_reason_label(reason),
        );
        Self {
            class,
            reason,
            digest: digest_string("bridge-parallel-legality-decision", &basis),
        }
    }

    pub fn class(&self) -> BridgeParallelLegalityClass {
        self.class
    }

    pub fn reason(&self) -> BridgeParallelLegalityReason {
        self.reason
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeParallelProfitabilityClass {
    NotApplicable,
    Profitable,
    Unprofitable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeParallelProfitabilityReason {
    SerialOnlyWorkload,
    SharedPublicationReductionTarget,
    AdmittedOperational,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeParallelProfitabilityDecision {
    class: BridgeParallelProfitabilityClass,
    reason: BridgeParallelProfitabilityReason,
    digest: Arc<str>,
}

impl BridgeParallelProfitabilityDecision {
    pub(crate) fn new(
        class: BridgeParallelProfitabilityClass,
        reason: BridgeParallelProfitabilityReason,
    ) -> Self {
        let basis = format!(
            "bridge-parallel-profitability-decision|class={}|reason={}",
            super::super::planner::parallel_profitability_class_label(class),
            super::super::planner::parallel_profitability_reason_label(reason),
        );
        Self {
            class,
            reason,
            digest: digest_string("bridge-parallel-profitability-decision", &basis),
        }
    }

    pub fn class(&self) -> BridgeParallelProfitabilityClass {
        self.class
    }

    pub fn reason(&self) -> BridgeParallelProfitabilityReason {
        self.reason
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeParallelAdmissionClass {
    SerialRequired,
    ParallelPreparationAdmitted,
    ParallelPreparationRejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeParallelAdmissionReason {
    SerialExecutor,
    BelowMinWorkloadWidth,
    SharedPublicationReductionTarget,
    SharedTruthViewMaterializationTarget,
    ContinuityRemapRequiresSerialPreparation,
    AdmittedOperational,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgePreparationMode {
    Serial,
    ParallelPreparation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisjointPacketRegionSet {
    regions: Arc<[Arc<str>]>,
    digest: Arc<str>,
}

impl DisjointPacketRegionSet {
    pub(crate) fn new(regions: Vec<Arc<str>>) -> Self {
        let mut basis = format!("disjoint-packet-region-set|region-count={}", regions.len());
        for region in &regions {
            basis.push_str("|region=");
            basis.push_str(region);
        }
        Self {
            regions: regions.into(),
            digest: digest_string("disjoint-packet-region-set", &basis),
        }
    }

    pub fn regions(&self) -> &[Arc<str>] {
        &self.regions
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedPreparationPartitionSet {
    partitions: Arc<[Arc<str>]>,
    digest: Arc<str>,
}

impl AdmittedPreparationPartitionSet {
    pub(crate) fn new(partitions: Vec<Arc<str>>) -> Self {
        let mut basis = format!(
            "admitted-preparation-partition-set|partition-count={}",
            partitions.len()
        );
        for partition in &partitions {
            basis.push_str("|partition=");
            basis.push_str(partition);
        }
        Self {
            partitions: partitions.into(),
            digest: digest_string("admitted-preparation-partition-set", &basis),
        }
    }

    pub fn partitions(&self) -> &[Arc<str>] {
        &self.partitions
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParallelPreparationLegalityProof {
    canonical_planning_identity: BridgeCanonicalPlanningIdentity,
    disjoint_packet_regions: DisjointPacketRegionSet,
    admitted_partitions: AdmittedPreparationPartitionSet,
    digest: Arc<str>,
}

impl ParallelPreparationLegalityProof {
    pub(crate) fn new(
        canonical_planning_identity: BridgeCanonicalPlanningIdentity,
        disjoint_packet_regions: DisjointPacketRegionSet,
        admitted_partitions: AdmittedPreparationPartitionSet,
    ) -> Self {
        let basis = format!(
            "parallel-preparation-legality-proof|planning={}|regions={}|partitions={}",
            canonical_planning_identity.as_str(),
            disjoint_packet_regions.digest(),
            admitted_partitions.digest(),
        );
        Self {
            canonical_planning_identity,
            disjoint_packet_regions,
            admitted_partitions,
            digest: digest_string("parallel-preparation-legality-proof", &basis),
        }
    }

    pub fn canonical_planning_identity(&self) -> &BridgeCanonicalPlanningIdentity {
        &self.canonical_planning_identity
    }

    pub fn disjoint_packet_regions(&self) -> &DisjointPacketRegionSet {
        &self.disjoint_packet_regions
    }

    pub fn admitted_partitions(&self) -> &AdmittedPreparationPartitionSet {
        &self.admitted_partitions
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeParallelAdmission {
    class: BridgeParallelAdmissionClass,
    reason: BridgeParallelAdmissionReason,
    digest: Arc<str>,
}

impl BridgeParallelAdmission {
    pub(crate) fn new(
        class: BridgeParallelAdmissionClass,
        reason: BridgeParallelAdmissionReason,
    ) -> Self {
        let basis = format!(
            "bridge-parallel-admission|class={}|reason={}",
            super::super::planner::parallel_admission_class_label(class),
            super::super::planner::parallel_admission_reason_label(reason),
        );
        Self {
            class,
            reason,
            digest: digest_string("bridge-parallel-admission", &basis),
        }
    }

    pub fn class(&self) -> BridgeParallelAdmissionClass {
        self.class
    }

    pub fn reason(&self) -> BridgeParallelAdmissionReason {
        self.reason
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

use super::*;
