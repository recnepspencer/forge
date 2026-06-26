use crate::graph_read_access_plan_adoption::WorthGraphReadAccessSliceReceiptProjection;

use super::super::stable_digest;
use super::WorthGraphReadAccessCallerOwnedWorkBreakdown;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessSourceCounterProofKind {
    PhaseFourQueryReceipt,
    ReceiptRowCounterDigest,
    NoExecutionCounterRequired,
}

impl WorthGraphReadAccessSourceCounterProofKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PhaseFourQueryReceipt => "phase_four_query_receipt",
            Self::ReceiptRowCounterDigest => "receipt_row_counter_digest",
            Self::NoExecutionCounterRequired => "no_execution_counter_required",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessSourceCounterProof {
    kind: WorthGraphReadAccessSourceCounterProofKind,
    source_counter_digest: Option<String>,
    candidate_root_count: usize,
    touched_node_count: usize,
    touched_edge_count: usize,
    frontier_width: usize,
    visited_breadth: usize,
    dedup_breadth: usize,
    resident_byte_count: usize,
    fallback_count: usize,
    streaming_page_count: usize,
    streaming_emitted_row_count: usize,
    local_work_count: usize,
    caller_owned_breakdown_digest: String,
    proof_digest: String,
}

impl WorthGraphReadAccessSourceCounterProof {
    pub(crate) fn from_phase_four_receipt(
        receipt: &WorthGraphReadAccessSliceReceiptProjection,
        caller_owned_work: &WorthGraphReadAccessCallerOwnedWorkBreakdown,
    ) -> Self {
        Self::from_input(WorthGraphReadAccessSourceCounterProofInput {
            kind: WorthGraphReadAccessSourceCounterProofKind::PhaseFourQueryReceipt,
            source_counter_digest: receipt.plan_consumption_digest().map(str::to_string),
            candidate_root_count: receipt.candidate_root_count(),
            touched_node_count: receipt.touched_node_count(),
            touched_edge_count: receipt.touched_edge_count(),
            frontier_width: receipt.frontier_width(),
            visited_breadth: receipt.visited_breadth(),
            dedup_breadth: receipt.dedup_breadth(),
            resident_byte_count: receipt.resident_byte_count(),
            fallback_count: receipt.fallback_count(),
            streaming_page_count: 0,
            streaming_emitted_row_count: 0,
            local_work_count: caller_owned_work.total_count(),
            caller_owned_breakdown_digest: caller_owned_work.breakdown_digest().to_string(),
        })
    }

    pub(crate) fn from_receipt_counter_digest(
        source_counter_digest: Option<String>,
        missing_counter_for_claimed_execution_count: usize,
        caller_owned_work: &WorthGraphReadAccessCallerOwnedWorkBreakdown,
    ) -> Self {
        let kind = if source_counter_digest.is_some() {
            WorthGraphReadAccessSourceCounterProofKind::ReceiptRowCounterDigest
        } else {
            WorthGraphReadAccessSourceCounterProofKind::NoExecutionCounterRequired
        };
        Self::from_input(WorthGraphReadAccessSourceCounterProofInput {
            kind,
            source_counter_digest,
            candidate_root_count: usize::from(matches!(
                kind,
                WorthGraphReadAccessSourceCounterProofKind::ReceiptRowCounterDigest
            )),
            touched_node_count: 0,
            touched_edge_count: 0,
            frontier_width: 0,
            visited_breadth: 0,
            dedup_breadth: 0,
            resident_byte_count: 0,
            fallback_count: 0,
            streaming_page_count: 0,
            streaming_emitted_row_count: 0,
            local_work_count: missing_counter_for_claimed_execution_count,
            caller_owned_breakdown_digest: caller_owned_work.breakdown_digest().to_string(),
        })
    }

    fn from_input(input: WorthGraphReadAccessSourceCounterProofInput) -> Self {
        let proof_digest = stable_digest(&[
            "worth_graph_read_access_source_counter_proof_v1".to_string(),
            format!("kind:{}", input.kind.as_str()),
            format!(
                "source_counter:{}",
                input.source_counter_digest.as_deref().unwrap_or("none")
            ),
            format!("candidate_roots:{}", input.candidate_root_count),
            format!("touched_nodes:{}", input.touched_node_count),
            format!("touched_edges:{}", input.touched_edge_count),
            format!("frontier_width:{}", input.frontier_width),
            format!("visited_breadth:{}", input.visited_breadth),
            format!("dedup_breadth:{}", input.dedup_breadth),
            format!("resident_bytes:{}", input.resident_byte_count),
            format!("fallback_count:{}", input.fallback_count),
            format!("streaming_pages:{}", input.streaming_page_count),
            format!("streaming_rows:{}", input.streaming_emitted_row_count),
            format!("local_work:{}", input.local_work_count),
            format!("caller_work:{}", input.caller_owned_breakdown_digest),
        ]);
        Self {
            kind: input.kind,
            source_counter_digest: input.source_counter_digest,
            candidate_root_count: input.candidate_root_count,
            touched_node_count: input.touched_node_count,
            touched_edge_count: input.touched_edge_count,
            frontier_width: input.frontier_width,
            visited_breadth: input.visited_breadth,
            dedup_breadth: input.dedup_breadth,
            resident_byte_count: input.resident_byte_count,
            fallback_count: input.fallback_count,
            streaming_page_count: input.streaming_page_count,
            streaming_emitted_row_count: input.streaming_emitted_row_count,
            local_work_count: input.local_work_count,
            caller_owned_breakdown_digest: input.caller_owned_breakdown_digest,
            proof_digest,
        }
    }

    pub const fn kind(&self) -> WorthGraphReadAccessSourceCounterProofKind {
        self.kind
    }

    pub fn source_counter_digest(&self) -> Option<&str> {
        self.source_counter_digest.as_deref()
    }

    pub const fn candidate_root_count(&self) -> usize {
        self.candidate_root_count
    }

    pub const fn touched_node_count(&self) -> usize {
        self.touched_node_count
    }

    pub const fn touched_edge_count(&self) -> usize {
        self.touched_edge_count
    }

    pub const fn frontier_width(&self) -> usize {
        self.frontier_width
    }

    pub const fn visited_breadth(&self) -> usize {
        self.visited_breadth
    }

    pub const fn dedup_breadth(&self) -> usize {
        self.dedup_breadth
    }

    pub const fn resident_byte_count(&self) -> usize {
        self.resident_byte_count
    }

    pub const fn fallback_count(&self) -> usize {
        self.fallback_count
    }

    pub const fn streaming_page_count(&self) -> usize {
        self.streaming_page_count
    }

    pub const fn streaming_emitted_row_count(&self) -> usize {
        self.streaming_emitted_row_count
    }

    pub const fn local_work_count(&self) -> usize {
        self.local_work_count
    }

    pub fn caller_owned_breakdown_digest(&self) -> &str {
        &self.caller_owned_breakdown_digest
    }

    pub fn proof_digest(&self) -> &str {
        &self.proof_digest
    }
}

struct WorthGraphReadAccessSourceCounterProofInput {
    kind: WorthGraphReadAccessSourceCounterProofKind,
    source_counter_digest: Option<String>,
    candidate_root_count: usize,
    touched_node_count: usize,
    touched_edge_count: usize,
    frontier_width: usize,
    visited_breadth: usize,
    dedup_breadth: usize,
    resident_byte_count: usize,
    fallback_count: usize,
    streaming_page_count: usize,
    streaming_emitted_row_count: usize,
    local_work_count: usize,
    caller_owned_breakdown_digest: String,
}
