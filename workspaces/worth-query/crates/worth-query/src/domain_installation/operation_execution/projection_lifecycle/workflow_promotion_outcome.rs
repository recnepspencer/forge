use crate::basis_lifecycle::BasisOperationLane;

use super::{
    WorthQueryAuthorityRevalidationWorkflowProjection, WorthQueryCurrentWorkflowProjection,
    WorthQueryLiveBoundWorkflowProjection, WorthQueryProjectionPromotionCounters,
    WorthQueryProjectionPromotionDenialKind, WorthQueryRebindRequiredWorkflowProjection,
    WorthQueryStaleReadableWorkflowProjection,
};

#[must_use = "workflow promotion retains either the managed resource or workflow projection state"]
pub enum WorthQueryWorkflowProjectionPromotionOutcome<D, O, F, L: BasisOperationLane> {
    Promoted(WorthQueryLiveBoundWorkflowProjection<D, O, F, L>),
    Denied(WorthQueryWorkflowProjectionPromotionStop<D, O, F, L>),
    Deferred(WorthQueryWorkflowProjectionPromotionStop<D, O, F, L>),
    Stale(WorthQueryStaleReadableWorkflowProjection<D, O, F, L>),
    RebindRequired(WorthQueryRebindRequiredWorkflowProjection<D, O, F, L>),
    AuthorityRevalidationRequired(WorthQueryAuthorityRevalidationWorkflowProjection<D, O, F, L>),
    Failed(WorthQueryWorkflowProjectionPromotionStop<D, O, F, L>),
}

pub struct WorthQueryWorkflowProjectionPromotionStop<D, O, F, L: BasisOperationLane> {
    current: WorthQueryCurrentWorkflowProjection<D, O, F, L>,
    kind: WorthQueryProjectionPromotionDenialKind,
    detail: String,
    counters: WorthQueryProjectionPromotionCounters,
}

impl<D, O, F, L: BasisOperationLane> WorthQueryWorkflowProjectionPromotionStop<D, O, F, L> {
    pub(super) fn new(
        current: WorthQueryCurrentWorkflowProjection<D, O, F, L>,
        kind: WorthQueryProjectionPromotionDenialKind,
        detail: impl Into<String>,
        counters: WorthQueryProjectionPromotionCounters,
    ) -> Self {
        Self {
            current,
            kind,
            detail: detail.into(),
            counters,
        }
    }

    pub fn kind(&self) -> WorthQueryProjectionPromotionDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn counters(&self) -> WorthQueryProjectionPromotionCounters {
        self.counters
    }

    pub fn into_current(self) -> WorthQueryCurrentWorkflowProjection<D, O, F, L> {
        self.current
    }
}
