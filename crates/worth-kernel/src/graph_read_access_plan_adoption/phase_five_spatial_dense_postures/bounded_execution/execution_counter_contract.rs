use super::super::query_posture_projection::WorthGraphReadAccessSpatialDensePostureProjection;
use super::super::slice_classification::WorthGraphReadAccessUnresolvedSliceKind;
use super::super::stable_digest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessBoundedExecutionContractStatus {
    NoBoundedExecutionClaimed,
    BoundedExecutionRequiresCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessBoundedExecutionContract {
    status: WorthGraphReadAccessBoundedExecutionContractStatus,
    dense_or_broad_row_count: usize,
    unbounded_ephemeral_index_count: usize,
    lifecycle_scoped_index_count: usize,
    streaming_contract_count: usize,
    contract_digest: String,
}

pub(crate) fn build_bounded_execution_contract(
    projections: &[WorthGraphReadAccessSpatialDensePostureProjection],
) -> WorthGraphReadAccessBoundedExecutionContract {
    let dense_or_broad_row_count = projections
        .iter()
        .filter(|projection| projection.slice_kind().is_dense_or_broad())
        .count();
    let lifecycle_scoped_index_count = projections
        .iter()
        .filter(|projection| projection.query_posture() == "bounded_ephemeral_index")
        .filter(|projection| {
            projection.slice_kind() != WorthGraphReadAccessUnresolvedSliceKind::DenseFrontierRead
                && projection.slice_kind()
                    != WorthGraphReadAccessUnresolvedSliceKind::BroadBooleanPredicateRead
        })
        .count();
    let streaming_contract_count = projections
        .iter()
        .filter(|projection| projection.query_posture() == "admitted_paged_streaming")
        .count();
    let unbounded_ephemeral_index_count = projections
        .iter()
        .filter(|projection| projection.slice_kind().is_dense_or_broad())
        .filter(|projection| projection.query_posture() == "bounded_ephemeral_index")
        .count();
    let status = if lifecycle_scoped_index_count == 0 && streaming_contract_count == 0 {
        WorthGraphReadAccessBoundedExecutionContractStatus::NoBoundedExecutionClaimed
    } else {
        WorthGraphReadAccessBoundedExecutionContractStatus::BoundedExecutionRequiresCounters
    };
    WorthGraphReadAccessBoundedExecutionContract::new(
        status,
        dense_or_broad_row_count,
        unbounded_ephemeral_index_count,
        lifecycle_scoped_index_count,
        streaming_contract_count,
    )
}

impl WorthGraphReadAccessBoundedExecutionContract {
    fn new(
        status: WorthGraphReadAccessBoundedExecutionContractStatus,
        dense_or_broad_row_count: usize,
        unbounded_ephemeral_index_count: usize,
        lifecycle_scoped_index_count: usize,
        streaming_contract_count: usize,
    ) -> Self {
        let contract_digest = stable_digest(&[
            "worth_graph_read_access_bounded_execution_contract_v1".to_string(),
            format!("status:{}", status.as_str()),
            format!("dense_or_broad:{dense_or_broad_row_count}"),
            format!("unbounded_ephemeral:{unbounded_ephemeral_index_count}"),
            format!("lifecycle_ephemeral:{lifecycle_scoped_index_count}"),
            format!("streaming:{streaming_contract_count}"),
        ]);
        Self {
            status,
            dense_or_broad_row_count,
            unbounded_ephemeral_index_count,
            lifecycle_scoped_index_count,
            streaming_contract_count,
            contract_digest,
        }
    }

    pub const fn status(&self) -> WorthGraphReadAccessBoundedExecutionContractStatus {
        self.status
    }

    pub const fn dense_or_broad_row_count(&self) -> usize {
        self.dense_or_broad_row_count
    }

    pub const fn unbounded_ephemeral_index_count(&self) -> usize {
        self.unbounded_ephemeral_index_count
    }

    pub const fn lifecycle_scoped_index_count(&self) -> usize {
        self.lifecycle_scoped_index_count
    }

    pub const fn streaming_contract_count(&self) -> usize {
        self.streaming_contract_count
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }
}

impl WorthGraphReadAccessBoundedExecutionContractStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoBoundedExecutionClaimed => "no_bounded_execution_claimed",
            Self::BoundedExecutionRequiresCounters => "bounded_execution_requires_counters",
        }
    }
}
