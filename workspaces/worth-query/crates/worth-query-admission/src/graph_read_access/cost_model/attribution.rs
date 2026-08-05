use super::WorthQueryGraphReadMemoryByteEstimate;
use crate::graph_read_access::WorthQueryGraphReadAccessRequirementKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadIntrinsicCostContribution {
    frontier_breadth: usize,
    edge_touches: usize,
    candidate_roots: usize,
    intermediate_set_size: usize,
}

impl WorthQueryGraphReadIntrinsicCostContribution {
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
            "intrinsic_contribution:frontier:{}:edges:{}:roots:{}:intermediate:{}",
            self.frontier_breadth,
            self.edge_touches,
            self.candidate_roots,
            self.intermediate_set_size
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadSupportedCostContribution {
    memory: WorthQueryGraphReadMemoryByteEstimate,
    allocation_lifecycle_count: usize,
}

impl WorthQueryGraphReadSupportedCostContribution {
    pub fn memory(&self) -> &WorthQueryGraphReadMemoryByteEstimate {
        &self.memory
    }

    pub fn allocation_lifecycle_count(&self) -> usize {
        self.allocation_lifecycle_count
    }

    pub(crate) fn new(
        memory: WorthQueryGraphReadMemoryByteEstimate,
        allocation_lifecycle_count: usize,
    ) -> Self {
        Self {
            memory,
            allocation_lifecycle_count,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "supported_contribution:{}:allocation_lifecycle:{}",
            self.memory.digest_part(),
            self.allocation_lifecycle_count
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadCostAttributionRow {
    requirement_digest_part: String,
    requirement_kind: WorthQueryGraphReadAccessRequirementKind,
    intrinsic: WorthQueryGraphReadIntrinsicCostContribution,
    supported: WorthQueryGraphReadSupportedCostContribution,
}

impl WorthQueryGraphReadCostAttributionRow {
    pub fn requirement_digest_part(&self) -> &str {
        &self.requirement_digest_part
    }

    pub fn requirement_kind(&self) -> &WorthQueryGraphReadAccessRequirementKind {
        &self.requirement_kind
    }

    pub fn intrinsic(&self) -> &WorthQueryGraphReadIntrinsicCostContribution {
        &self.intrinsic
    }

    pub fn supported(&self) -> &WorthQueryGraphReadSupportedCostContribution {
        &self.supported
    }

    pub(crate) fn new(
        requirement_digest_part: impl Into<String>,
        requirement_kind: WorthQueryGraphReadAccessRequirementKind,
        intrinsic: WorthQueryGraphReadIntrinsicCostContribution,
        supported: WorthQueryGraphReadSupportedCostContribution,
    ) -> Self {
        Self {
            requirement_digest_part: requirement_digest_part.into(),
            requirement_kind,
            intrinsic,
            supported,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "attribution:{}:{}:{}:{}",
            self.requirement_digest_part,
            self.requirement_kind.as_str(),
            self.intrinsic.digest_part(),
            self.supported.digest_part()
        )
    }
}
