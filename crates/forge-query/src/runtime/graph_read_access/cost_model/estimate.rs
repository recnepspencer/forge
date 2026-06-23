use super::{
    ForgeQueryGraphReadComplexityContract, ForgeQueryGraphReadCostAttributionRow,
    ForgeQueryGraphReadCostEstimateCounters, ForgeQueryGraphReadCostEstimateStatus,
    ForgeQueryGraphReadCostEvidence, ForgeQueryGraphReadMemoryByteEstimate,
};
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadAccessCostEstimateDigest(String);

impl ForgeQueryGraphReadAccessCostEstimateDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadIntrinsicCostEstimate {
    frontier_breadth: usize,
    edge_touches: usize,
    candidate_roots: usize,
    intermediate_set_size: usize,
}

impl ForgeQueryGraphReadIntrinsicCostEstimate {
    pub fn frontier_breadth(&self) -> usize {
        self.frontier_breadth
    }

    pub fn edge_touches(&self) -> usize {
        self.edge_touches
    }

    pub fn candidate_roots(&self) -> usize {
        self.candidate_roots
    }

    pub fn intermediate_set_size(&self) -> usize {
        self.intermediate_set_size
    }

    pub(crate) fn new(
        frontier_breadth: usize,
        edge_touches: usize,
        candidate_roots: usize,
        intermediate_set_size: usize,
    ) -> Self {
        Self {
            frontier_breadth,
            edge_touches,
            candidate_roots,
            intermediate_set_size,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "intrinsic:frontier:{}:edges:{}:roots:{}:intermediate:{}",
            self.frontier_breadth,
            self.edge_touches,
            self.candidate_roots,
            self.intermediate_set_size
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadSupportedCostEstimate {
    memory: ForgeQueryGraphReadMemoryByteEstimate,
    allocation_lifecycle_count: usize,
}

impl ForgeQueryGraphReadSupportedCostEstimate {
    pub fn memory(&self) -> &ForgeQueryGraphReadMemoryByteEstimate {
        &self.memory
    }

    pub fn index_bytes(&self) -> usize {
        self.memory.index_bytes()
    }

    pub fn result_bytes(&self) -> usize {
        self.memory.result_bytes()
    }

    pub fn proof_bytes(&self) -> usize {
        self.memory.proof_bytes()
    }

    pub fn allocation_lifecycle_count(&self) -> usize {
        self.allocation_lifecycle_count
    }

    pub(crate) fn new(
        memory: ForgeQueryGraphReadMemoryByteEstimate,
        allocation_lifecycle_count: usize,
    ) -> Self {
        Self {
            memory,
            allocation_lifecycle_count,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "supported:{}:allocation_lifecycle:{}",
            self.memory.digest_part(),
            self.allocation_lifecycle_count
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadAccessCostEstimate {
    digest: ForgeQueryGraphReadAccessCostEstimateDigest,
    requirement_set_digest: String,
    status: ForgeQueryGraphReadCostEstimateStatus,
    complexity_contract: ForgeQueryGraphReadComplexityContract,
    intrinsic: ForgeQueryGraphReadIntrinsicCostEstimate,
    supported: ForgeQueryGraphReadSupportedCostEstimate,
    counters: ForgeQueryGraphReadCostEstimateCounters,
    attribution_rows: Vec<ForgeQueryGraphReadCostAttributionRow>,
}

impl ForgeQueryGraphReadAccessCostEstimate {
    pub fn digest(&self) -> &ForgeQueryGraphReadAccessCostEstimateDigest {
        &self.digest
    }

    pub fn requirement_set_digest(&self) -> &str {
        &self.requirement_set_digest
    }

    pub fn status(&self) -> &ForgeQueryGraphReadCostEstimateStatus {
        &self.status
    }

    pub fn complexity_contract(&self) -> &ForgeQueryGraphReadComplexityContract {
        &self.complexity_contract
    }

    pub fn intrinsic(&self) -> &ForgeQueryGraphReadIntrinsicCostEstimate {
        &self.intrinsic
    }

    pub fn supported(&self) -> &ForgeQueryGraphReadSupportedCostEstimate {
        &self.supported
    }

    pub fn counters(&self) -> &ForgeQueryGraphReadCostEstimateCounters {
        &self.counters
    }

    pub fn attribution_rows(&self) -> &[ForgeQueryGraphReadCostAttributionRow] {
        &self.attribution_rows
    }

    pub(crate) fn new(
        requirement_set_digest: impl Into<String>,
        evidence: &ForgeQueryGraphReadCostEvidence,
        intrinsic: ForgeQueryGraphReadIntrinsicCostEstimate,
        supported: ForgeQueryGraphReadSupportedCostEstimate,
        counters: ForgeQueryGraphReadCostEstimateCounters,
        attribution_rows: Vec<ForgeQueryGraphReadCostAttributionRow>,
    ) -> Self {
        let requirement_set_digest = requirement_set_digest.into();
        let complexity_contract = ForgeQueryGraphReadComplexityContract::from_cost_dimensions(
            supported.index_bytes(),
            supported.result_bytes(),
            intrinsic.intermediate_set_size(),
        );
        let status = evidence.status().clone();
        let parts = vec![
            format!("requirements:{requirement_set_digest}"),
            evidence.digest_part(),
            status.digest_part(),
            complexity_contract.digest_part(),
            intrinsic.digest_part(),
            supported.digest_part(),
            counters.digest_part(),
            attribution_rows_digest_part(&attribution_rows),
        ];
        Self {
            digest: ForgeQueryGraphReadAccessCostEstimateDigest::from_parts(&parts),
            requirement_set_digest,
            status,
            complexity_contract,
            intrinsic,
            supported,
            counters,
            attribution_rows,
        }
    }
}

fn attribution_rows_digest_part(rows: &[ForgeQueryGraphReadCostAttributionRow]) -> String {
    format!(
        "attribution_rows:{}",
        rows.iter()
            .map(|row| row.digest_part())
            .collect::<Vec<_>>()
            .join("|")
    )
}
