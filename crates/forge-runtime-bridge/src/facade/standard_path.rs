use crate::error::{BridgeDeliveryError, BridgeRouteError, BridgeSpeculationError};

use super::*;

#[derive(Debug, Clone)]
pub struct BridgeDiagnostics<'a> {
    inner: &'a BridgeDiagnosticsFacade,
}

impl<'a> BridgeDiagnostics<'a> {
    pub(crate) fn new(inner: &'a BridgeDiagnosticsFacade) -> Self {
        Self { inner }
    }

    pub fn raw(&self) -> &'a BridgeDiagnosticsFacade {
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
            .or_else(|| self.explain_last_route().map(BridgeStandardDiagnosticsExplanation::Route))
    }

    /// Explains the most recent routed truth change.
    pub fn explain_last_route(&self) -> Option<BridgeRouteExplanation> {
        self.inner.explain_last_route_record()
    }

    /// Explains one routed truth change by route identity.
    pub fn explain_route(&self, route_identity: &str) -> Option<BridgeRouteExplanation> {
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
        historical_record_identity: &str,
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
        preview_session_identity: &str,
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
        preview_session_identity: &str,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTruthViewEvaluationRequest {
    selector: BridgeTruthViewSelector,
    read_packet: SnapshotReadPacket,
    replay_mode: BridgeReplayMode,
    diagnostics_tier: BridgeDiagnosticsTier,
    delivery_intent: BridgeDeliveryIntent,
}

impl BridgeTruthViewEvaluationRequest {
    /// Creates a request for the current head of a truth branch.
    pub fn for_branch_head(branch_identity: TruthBranchIdentity) -> Self {
        Self::new(BridgeTruthViewSelector::branch_head(branch_identity))
    }

    /// Creates a request for a specific branch-local snapshot.
    pub fn for_branch_snapshot(
        branch_identity: TruthBranchIdentity,
        snapshot_identity: TruthSnapshotIdentity,
    ) -> Self {
        Self::new(BridgeTruthViewSelector::branch_snapshot(
            branch_identity,
            snapshot_identity,
        ))
    }

    /// Creates a request for a historical commit on a branch.
    pub fn for_historical_commit(
        branch_identity: TruthBranchIdentity,
        commit_identity: TruthCommitIdentity,
    ) -> Self {
        Self::new(BridgeTruthViewSelector::historical_commit(
            branch_identity,
            commit_identity,
        ))
    }

    /// Creates a request from an explicit truth-view selector.
    pub fn new(selector: BridgeTruthViewSelector) -> Self {
        Self {
            selector,
            read_packet: SnapshotReadPacket::new(vec![]),
            replay_mode: BridgeReplayMode::Enabled,
            diagnostics_tier: BridgeDiagnosticsTier::Standard,
            delivery_intent: BridgeDeliveryIntent::PrepareSignalEvaluation,
        }
    }

    /// Overrides the read packet used for truth-view materialization.
    pub fn with_read_packet(mut self, read_packet: SnapshotReadPacket) -> Self {
        self.read_packet = read_packet;
        self
    }

    /// Overrides replay-mode policy for this evaluation.
    pub fn with_replay_mode(mut self, replay_mode: BridgeReplayMode) -> Self {
        self.replay_mode = replay_mode;
        self
    }

    /// Overrides diagnostics-tier policy for this evaluation.
    pub fn with_diagnostics_tier(mut self, diagnostics_tier: BridgeDiagnosticsTier) -> Self {
        self.diagnostics_tier = diagnostics_tier;
        self
    }

    /// Overrides delivery intent for this evaluation.
    pub fn with_delivery_intent(mut self, delivery_intent: BridgeDeliveryIntent) -> Self {
        self.delivery_intent = delivery_intent;
        self
    }

    pub fn selector(&self) -> &BridgeTruthViewSelector {
        &self.selector
    }

    pub(crate) fn declaration(&self) -> HistoricalEvaluationDeclaration {
        HistoricalEvaluationDeclaration::new(
            self.selector.clone(),
            self.replay_mode,
            self.diagnostics_tier,
            self.delivery_intent,
        )
    }

    pub(crate) fn read_packet(&self) -> SnapshotReadPacket {
        self.read_packet.clone()
    }
}

pub struct BridgeTruthViewEvaluation {
    observation: crate::snapshot::MaterializedTruthViewObservation,
    canonical_record: BridgeCanonicalHistoricalEvaluationRecord,
}

impl BridgeTruthViewEvaluation {
    pub(crate) fn new(
        observation: crate::snapshot::MaterializedTruthViewObservation,
        canonical_record: BridgeCanonicalHistoricalEvaluationRecord,
    ) -> Self {
        Self {
            observation,
            canonical_record,
        }
    }

    /// Returns the materialized truth-view observation.
    pub fn observation(&self) -> &crate::snapshot::MaterializedTruthViewObservation {
        &self.observation
    }

    /// Returns the canonical historical evaluation record derived from the
    /// materialized observation.
    pub fn record(&self) -> &BridgeCanonicalHistoricalEvaluationRecord {
        &self.canonical_record
    }

    /// Returns the snapshot identity bound by the observation.
    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        self.observation.snapshot_identity()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeEvaluationTarget {
    planned_route: BridgePlannedRoute,
}

impl BridgeEvaluationTarget {
    pub(crate) fn new(planned_route: BridgePlannedRoute) -> Self {
        Self { planned_route }
    }

    /// Returns the route identity that produced this evaluation target.
    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        self.planned_route.route_identity()
    }

    /// Returns the planned route behind this target.
    pub fn planned_route(&self) -> &BridgePlannedRoute {
        &self.planned_route
    }

    pub(crate) fn into_planned_route(self) -> BridgePlannedRoute {
        self.planned_route
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRoute {
    target: BridgeEvaluationTarget,
    result: BridgeRouteResult,
}

impl BridgeRoute {
    pub(crate) fn new(planned_route: BridgePlannedRoute, result: BridgeRouteResult) -> Self {
        Self {
            target: BridgeEvaluationTarget::new(planned_route),
            result,
        }
    }

    /// Returns the route identity.
    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        self.target.route_identity()
    }

    /// Returns the evaluation target produced by this route.
    pub fn target(&self) -> BridgeEvaluationTarget {
        self.target.clone()
    }

    /// Returns the delivery result produced by routing.
    pub fn result(&self) -> &BridgeRouteResult {
        &self.result
    }
}

#[derive(Debug)]
pub enum BridgeStandardRouteError {
    Route(BridgeRouteError),
    Delivery(BridgeDeliveryError),
}

impl std::fmt::Display for BridgeStandardRouteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Route(error) => write!(f, "{error}"),
            Self::Delivery(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for BridgeStandardRouteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Route(error) => Some(error),
            Self::Delivery(error) => Some(error),
        }
    }
}

impl From<BridgeRouteError> for BridgeStandardRouteError {
    fn from(value: BridgeRouteError) -> Self {
        Self::Route(value)
    }
}

impl From<BridgeDeliveryError> for BridgeStandardRouteError {
    fn from(value: BridgeDeliveryError) -> Self {
        Self::Delivery(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSpeculativeSessionRequest {
    session_identity: BridgePreviewSessionIdentity,
    declaration: BridgePreviewSessionDeclaration,
    preview_artifact_count: usize,
    destroyable_artifact_count: usize,
    retained_non_authoritative_artifact_count: usize,
}

impl BridgeSpeculativeSessionRequest {
    /// Creates a speculative session request.
    pub fn new(
        session_identity: BridgePreviewSessionIdentity,
        declaration: BridgePreviewSessionDeclaration,
        preview_artifact_count: usize,
        destroyable_artifact_count: usize,
        retained_non_authoritative_artifact_count: usize,
    ) -> Self {
        Self {
            session_identity,
            declaration,
            preview_artifact_count,
            destroyable_artifact_count,
            retained_non_authoritative_artifact_count,
        }
    }

    /// Returns the preview session identity.
    pub fn session_identity(&self) -> &BridgePreviewSessionIdentity {
        &self.session_identity
    }

    /// Returns the preview declaration.
    pub fn declaration(&self) -> &BridgePreviewSessionDeclaration {
        &self.declaration
    }

    pub(crate) fn preview_artifact_count(&self) -> usize {
        self.preview_artifact_count
    }

    pub(crate) fn destroyable_artifact_count(&self) -> usize {
        self.destroyable_artifact_count
    }

    pub(crate) fn retained_non_authoritative_artifact_count(&self) -> usize {
        self.retained_non_authoritative_artifact_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSpeculativeComparison {
    preview_session_identity: BridgePreviewSessionIdentity,
    truth_branch_identity: TruthBranchIdentity,
    signal_branch_identity: BridgeSignalBranchIdentity,
    truth_view_basis_digest: std::sync::Arc<str>,
}

impl BridgeSpeculativeComparison {
    pub(crate) fn from_active_session(session: &BridgePreviewSession<PreviewActive>) -> Self {
        let declaration = session.declaration().declaration();
        let binding = declaration.branch_binding();
        Self {
            preview_session_identity: session.session_identity().clone(),
            truth_branch_identity: binding.truth_branch_identity().clone(),
            signal_branch_identity: binding.signal_branch_identity().clone(),
            truth_view_basis_digest: declaration.truth_view_basis_digest().into(),
        }
    }

    /// Returns the preview session identity being compared.
    pub fn preview_session_identity(&self) -> &BridgePreviewSessionIdentity {
        &self.preview_session_identity
    }

    /// Returns the speculative truth branch identity.
    pub fn truth_branch_identity(&self) -> &TruthBranchIdentity {
        &self.truth_branch_identity
    }

    /// Returns the speculative signal branch identity.
    pub fn signal_branch_identity(&self) -> &BridgeSignalBranchIdentity {
        &self.signal_branch_identity
    }

    /// Returns the shared truth-view basis digest for the comparison.
    pub fn truth_view_basis_digest(&self) -> &str {
        self.truth_view_basis_digest.as_ref()
    }

    /// Builds the truth-view request for the speculative side.
    pub fn speculative_evaluation_request(&self) -> BridgeTruthViewEvaluationRequest {
        BridgeTruthViewEvaluationRequest::for_branch_head(self.truth_branch_identity.clone())
    }

    /// Builds the truth-view request for the main side.
    pub fn main_evaluation_request(
        &self,
        main_branch_identity: TruthBranchIdentity,
    ) -> BridgeTruthViewEvaluationRequest {
        BridgeTruthViewEvaluationRequest::for_branch_head(main_branch_identity)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSpeculativePromotionRequest {
    authoritative_commit_boundary_digest: std::sync::Arc<str>,
    authoritative_artifact_digest: std::sync::Arc<str>,
}

impl BridgeSpeculativePromotionRequest {
    /// Creates a promotion request from authoritative boundary digests.
    pub fn new(
        authoritative_commit_boundary_digest: impl Into<std::sync::Arc<str>>,
        authoritative_artifact_digest: impl Into<std::sync::Arc<str>>,
    ) -> Self {
        Self {
            authoritative_commit_boundary_digest: authoritative_commit_boundary_digest.into(),
            authoritative_artifact_digest: authoritative_artifact_digest.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BridgeSpeculativeDiscardOutcome {
    session: BridgePreviewSession<PreviewDiscarded>,
    record: BridgePreviewDiscardRecord,
}

impl BridgeSpeculativeDiscardOutcome {
    pub(crate) fn new(
        session: BridgePreviewSession<PreviewDiscarded>,
        record: BridgePreviewDiscardRecord,
    ) -> Self {
        Self { session, record }
    }

    /// Returns the discarded terminal session state.
    pub fn session(&self) -> &BridgePreviewSession<PreviewDiscarded> {
        &self.session
    }

    /// Returns the canonical discard record.
    pub fn record(&self) -> &BridgePreviewDiscardRecord {
        &self.record
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BridgeSpeculativePromotionOutcome {
    session: BridgePreviewSession<PreviewPromoted>,
    record: BridgePreviewPromotionRecord,
}

impl BridgeSpeculativePromotionOutcome {
    pub(crate) fn new(
        session: BridgePreviewSession<PreviewPromoted>,
        record: BridgePreviewPromotionRecord,
    ) -> Self {
        Self { session, record }
    }

    /// Returns the promoted terminal session state.
    pub fn session(&self) -> &BridgePreviewSession<PreviewPromoted> {
        &self.session
    }

    /// Returns the canonical promotion record.
    pub fn record(&self) -> &BridgePreviewPromotionRecord {
        &self.record
    }
}

#[derive(Debug)]
pub struct BridgeSpeculativeSessionHandle {
    runtime: RuntimeBridge,
    session: BridgePreviewSession<PreviewActive>,
    execution_record: BridgePreviewExecutionRecord,
}

impl BridgeSpeculativeSessionHandle {
    pub(crate) fn new(
        runtime: RuntimeBridge,
        session: BridgePreviewSession<PreviewActive>,
        execution_record: BridgePreviewExecutionRecord,
    ) -> Self {
        Self {
            runtime,
            session,
            execution_record,
        }
    }

    /// Returns the preview session identity.
    pub fn session_identity(&self) -> &BridgePreviewSessionIdentity {
        self.session.session_identity()
    }

    /// Builds a comparison handle between this speculative session and main.
    pub fn compare_to_main(&self) -> BridgeSpeculativeComparison {
        BridgeSpeculativeComparison::from_active_session(&self.session)
    }

    /// Returns the raw diagnostics facade.
    pub fn diagnostics(&self) -> &BridgeDiagnosticsFacade {
        self.runtime.diagnostics().raw()
    }

    /// Returns the standard-path diagnostics wrapper.
    pub fn inspect(&self) -> BridgeDiagnostics<'_> {
        self.runtime.diagnostics()
    }

    /// Returns the retained execution record for this active session.
    pub fn execution_record(&self) -> &BridgePreviewExecutionRecord {
        &self.execution_record
    }

    /// Discards this speculative session with an explicit residue report.
    pub fn discard(
        self,
        residue_classes: Vec<BridgePreviewResidueClass>,
    ) -> Result<BridgeSpeculativeDiscardOutcome, BridgeSpeculationError> {
        let (session, record) = self.runtime.discard_preview_session(
            self.session,
            &self.execution_record,
            residue_classes,
        )?;
        Ok(BridgeSpeculativeDiscardOutcome::new(session, record))
    }

    /// Promotes this speculative session across the authoritative boundary.
    pub fn promote(
        self,
        request: BridgeSpeculativePromotionRequest,
    ) -> Result<BridgeSpeculativePromotionOutcome, BridgeSpeculationError> {
        let proof = self.session.promotion_admissibility_proof();
        let (session, record) = self.runtime.promote_preview_session(
            self.session,
            &self.execution_record,
            &proof,
            request.authoritative_commit_boundary_digest,
            request.authoritative_artifact_digest,
        )?;
        Ok(BridgeSpeculativePromotionOutcome::new(session, record))
    }
}
