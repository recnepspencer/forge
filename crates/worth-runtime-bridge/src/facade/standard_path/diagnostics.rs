use super::*;
use crate::diagnostics::BridgeHistoricalEvaluationRecordIdentity;
use crate::routing::BridgeRouteIdentity;
use crate::speculation::BridgePreviewSessionIdentity;

#[derive(Debug, Clone)]
pub struct BridgeDiagnostics<'a> {
    inner: &'a BridgeDiagnosticsFacade,
}

impl<'a> BridgeDiagnostics<'a> {
    pub(crate) fn new(inner: &'a BridgeDiagnosticsFacade) -> Self {
        Self { inner }
    }

    pub fn retained_artifacts(&self) -> &'a BridgeDiagnosticsFacade {
        self.inner
    }

    /// Returns the richest available explanation for the most recent
    /// standard-path bridge action.
    pub fn explain_last(&self) -> Option<BridgeStandardDiagnosticsExplanation> {
        self.explain_last_promotion()
            .map(BridgeStandardDiagnosticsExplanation::PreviewPromotion)
            .or_else(|| {
                self.explain_last_discard()
                    .map(BridgeStandardDiagnosticsExplanation::PreviewDiscard)
            })
            .or_else(|| {
                self.explain_last_session()
                    .map(BridgeStandardDiagnosticsExplanation::PreviewExecution)
            })
            .or_else(|| {
                self.explain_last_evaluation()
                    .map(BridgeStandardDiagnosticsExplanation::Evaluation)
            })
            .or_else(|| {
                self.explain_last_route()
                    .map(BridgeStandardDiagnosticsExplanation::Route)
            })
    }

    /// Explains the most recent routed truth change.
    pub fn explain_last_route(&self) -> Option<BridgeRouteExplanation> {
        self.inner.explain_last_route_record()
    }

    /// Explains one routed truth change by route identity.
    pub fn explain_route(
        &self,
        route_identity: &BridgeRouteIdentity,
    ) -> Option<BridgeRouteExplanation> {
        self.inner
            .route_record_for_route_identity(route_identity)
            .map(|record| self.inner.explain_route_record(&record))
    }

    /// Explains the most recent explicit or implicit truth-view evaluation.
    pub fn explain_last_evaluation(&self) -> Option<BridgeHistoricalEvaluationExplanation> {
        self.inner.explain_last_historical_evaluation_record()
    }

    /// Explains one truth-view evaluation by retained record identity.
    pub fn explain_evaluation(
        &self,
        historical_record_identity: &BridgeHistoricalEvaluationRecordIdentity,
    ) -> Option<BridgeHistoricalEvaluationExplanation> {
        self.inner
            .historical_record_for_record_identity(historical_record_identity)
            .map(|record| self.inner.explain_historical_evaluation_record(&record))
    }

    /// Explains the most recent speculative session execution.
    pub fn explain_last_session(&self) -> Option<BridgePreviewExecutionExplanation> {
        self.inner.explain_last_preview_execution_record()
    }

    /// Explains a speculative session by preview session identity.
    ///
    /// If the session progressed to discard or promotion, this returns the
    /// terminal explanation instead of only the original execution record.
    pub fn explain_session(
        &self,
        preview_session_identity: &BridgePreviewSessionIdentity,
    ) -> Option<BridgeStandardSessionExplanation> {
        self.inner
            .preview_promotion_record_for_session_identity(preview_session_identity)
            .map(|record| {
                BridgeStandardSessionExplanation::PreviewPromotion(
                    self.inner.explain_preview_promotion_record(&record),
                )
            })
            .or_else(|| {
                self.inner
                    .preview_discard_record_for_session_identity(preview_session_identity)
                    .map(|record| {
                        BridgeStandardSessionExplanation::PreviewDiscard(
                            self.inner.explain_preview_discard_record(&record),
                        )
                    })
            })
            .or_else(|| {
                self.inner
                    .preview_execution_record_for_session_identity(preview_session_identity)
                    .map(|record| {
                        BridgeStandardSessionExplanation::PreviewExecution(
                            self.inner.explain_preview_execution_record(&record),
                        )
                    })
            })
    }

    /// Explains the most recent discard record.
    pub fn explain_last_discard(&self) -> Option<BridgePreviewDiscardExplanation> {
        self.inner.explain_last_preview_discard_record()
    }

    /// Explains the most recent promotion record.
    pub fn explain_last_promotion(&self) -> Option<BridgePreviewPromotionExplanation> {
        self.inner.explain_last_preview_promotion_record()
    }

    /// Explains a promotion by preview session identity.
    pub fn explain_promotion(
        &self,
        preview_session_identity: &BridgePreviewSessionIdentity,
    ) -> Option<BridgePreviewPromotionExplanation> {
        self.inner
            .preview_promotion_record_for_session_identity(preview_session_identity)
            .map(|record| self.inner.explain_preview_promotion_record(&record))
    }
}

impl<'a> std::ops::Deref for BridgeDiagnostics<'a> {
    type Target = BridgeDiagnosticsFacade;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

#[derive(Debug, Clone)]
pub enum BridgeStandardDiagnosticsExplanation {
    Route(BridgeRouteExplanation),
    Evaluation(BridgeHistoricalEvaluationExplanation),
    PreviewExecution(BridgePreviewExecutionExplanation),
    PreviewDiscard(BridgePreviewDiscardExplanation),
    PreviewPromotion(BridgePreviewPromotionExplanation),
}

#[derive(Debug, Clone)]
pub enum BridgeStandardSessionExplanation {
    PreviewExecution(BridgePreviewExecutionExplanation),
    PreviewDiscard(BridgePreviewDiscardExplanation),
    PreviewPromotion(BridgePreviewPromotionExplanation),
}
