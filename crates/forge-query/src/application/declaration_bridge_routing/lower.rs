use forge_runtime_bridge::facade::{
    BridgeDeliveryIntent, BridgeDiagnosticsTier, BridgePreviewSessionDeclaration,
    BridgePreviewSessionDeclarationIdentity, BridgePreviewSessionIdentity, BridgeReplayMode,
    BridgeRequestKind, BridgeRouteRequest, BridgeSignalBranchIdentity,
    BridgeSpeculativeBranchBinding, BridgeSpeculativeBranchBindingIdentity,
    BridgeSpeculativePromotionRequest, BridgeSpeculativeSessionRequest,
    BridgeSubscriptionContinuationCandidateInput, BridgeTruthViewEvaluationRequest,
    BridgeTruthViewSelector, BridgeWritebackEffectClass, BridgeWritebackFamilyKind,
    BridgeWritebackIdempotenceClass, BridgeWritebackLoopDisposition, BridgeWritebackStrategyClass,
    HistoricalEvaluationDeclaration, TruthBranchIdentity, TruthCommitIdentity,
    TruthSnapshotIdentity, TruthWritebackRequest,
};

use crate::application::ForgeQueryDeclarationEnvelope;

use super::{
    artifact::ForgeQueryDeclarationBridgeBinding,
    contract::{
        ForgeQueryDeclarationBridgeContinuationContract,
        ForgeQueryDeclarationBridgeContinuationFamily,
    },
    request::{
        ForgeQueryDeclarationBridgeContinuationMode,
        ForgeQueryDeclarationBridgeContinuationRequest, ForgeQueryDeclarationBridgeTruthContext,
    },
};

pub(crate) fn forge_query_lower_bridge_binding<
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
    contract: ForgeQueryDeclarationBridgeContinuationContract,
) -> (
    ForgeQueryDeclarationBridgeContinuationRequest,
    ForgeQueryDeclarationBridgeContinuationFamily,
    ForgeQueryDeclarationBridgeBinding,
) {
    let request = contract.request();
    let lowering = BridgeLoweringContext::new(envelope);
    let binding = match request.mode() {
        ForgeQueryDeclarationBridgeContinuationMode::RuntimeRoute => {
            ForgeQueryDeclarationBridgeBinding::RuntimeRoute(runtime_route_request(envelope))
        }
        ForgeQueryDeclarationBridgeContinuationMode::TruthView => {
            ForgeQueryDeclarationBridgeBinding::TruthView(truth_view_request(
                &lowering, envelope, request,
            ))
        }
        ForgeQueryDeclarationBridgeContinuationMode::PreviewSession => {
            ForgeQueryDeclarationBridgeBinding::PreviewSession(preview_session_request(
                envelope, &lowering, request,
            ))
        }
        ForgeQueryDeclarationBridgeContinuationMode::PreviewPromotion => {
            ForgeQueryDeclarationBridgeBinding::PreviewPromotion(preview_promotion_request(
                envelope,
            ))
        }
        ForgeQueryDeclarationBridgeContinuationMode::SubscriptionPreparation => {
            ForgeQueryDeclarationBridgeBinding::SubscriptionPreparation(
                subscription_preparation_request(envelope, &lowering, request),
            )
        }
        ForgeQueryDeclarationBridgeContinuationMode::WritebackPreparation => {
            ForgeQueryDeclarationBridgeBinding::WritebackPreparation(writeback_preparation_request(
                envelope, &lowering, request,
            ))
        }
    };
    (request, contract.family(), binding)
}

struct BridgeLoweringContext {
    runtime_surface_digest: String,
}

impl BridgeLoweringContext {
    fn new<
        D: crate::application::ForgeQueryDomainEntryMarker,
        I: crate::application::ForgeQueryDeclarationInput<D>,
    >(
        envelope: &ForgeQueryDeclarationEnvelope<D, I>,
    ) -> Self {
        Self {
            runtime_surface_digest: runtime_surface_digest(envelope),
        }
    }

    fn truth_view_selector<
        D: crate::application::ForgeQueryDomainEntryMarker,
        I: crate::application::ForgeQueryDeclarationInput<D>,
    >(
        &self,
        envelope: &ForgeQueryDeclarationEnvelope<D, I>,
        truth_context: ForgeQueryDeclarationBridgeTruthContext,
    ) -> BridgeTruthViewSelector {
        let branch_identity = TruthBranchIdentity::new(format!(
            "query-branch:{}",
            envelope.operating_context_identity_digest()
        ));
        match truth_context {
            ForgeQueryDeclarationBridgeTruthContext::Current => {
                BridgeTruthViewSelector::branch_head(branch_identity)
            }
            ForgeQueryDeclarationBridgeTruthContext::Historical => {
                BridgeTruthViewSelector::historical_commit(
                    branch_identity,
                    TruthCommitIdentity::new(format!(
                        "query-commit:{}",
                        envelope.declaration_digest()
                    )),
                )
            }
            ForgeQueryDeclarationBridgeTruthContext::Preview => {
                BridgeTruthViewSelector::branch_snapshot(
                    branch_identity,
                    TruthSnapshotIdentity::new(format!(
                        "query-snapshot:{}",
                        envelope.route_plan_digest().unwrap_or("none")
                    )),
                )
            }
        }
    }

    fn truth_view_basis_digest<
        D: crate::application::ForgeQueryDomainEntryMarker,
        I: crate::application::ForgeQueryDeclarationInput<D>,
    >(
        &self,
        envelope: &ForgeQueryDeclarationEnvelope<D, I>,
        truth_context: ForgeQueryDeclarationBridgeTruthContext,
    ) -> String {
        let declaration = HistoricalEvaluationDeclaration::new(
            self.truth_view_selector(envelope, truth_context),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        declaration.digest().to_string()
    }

    fn runtime_surface_digest(&self) -> &str {
        &self.runtime_surface_digest
    }
}

fn runtime_route_request<
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
) -> BridgeRouteRequest {
    BridgeRouteRequest::for_commit(format!(
        "query.bridge.route:{}",
        envelope.declaration_digest()
    ))
}

fn truth_view_request<
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    lowering: &BridgeLoweringContext,
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
    request: ForgeQueryDeclarationBridgeContinuationRequest,
) -> BridgeTruthViewEvaluationRequest {
    let selector = lowering.truth_view_selector(envelope, request.truth_context());
    let evaluation = BridgeTruthViewEvaluationRequest::new(selector)
        .with_diagnostics_tier(BridgeDiagnosticsTier::Standard);
    if request.truth_context() == ForgeQueryDeclarationBridgeTruthContext::Historical {
        evaluation
            .with_replay_mode(BridgeReplayMode::Required)
            .with_delivery_intent(BridgeDeliveryIntent::PrepareOnly)
    } else {
        evaluation.with_delivery_intent(BridgeDeliveryIntent::PrepareSignalEvaluation)
    }
}

fn preview_session_request<
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
    lowering: &BridgeLoweringContext,
    request: ForgeQueryDeclarationBridgeContinuationRequest,
) -> BridgeSpeculativeSessionRequest {
    let binding = preview_branch_binding(envelope);
    let declaration = BridgePreviewSessionDeclaration::new(
        BridgePreviewSessionDeclarationIdentity::new(format!(
            "query.preview.declaration:{}",
            envelope
                .envelope_digest()
                .metadata()
                .algorithm()
                .id()
                .as_str()
        )),
        BridgeRequestKind::Preview,
        binding,
        lowering.truth_view_basis_digest(envelope, request.truth_context()),
        lowering.runtime_surface_digest().to_string(),
        format!("request-shape:{}", envelope.declaration_family_key()),
        format!(
            "artifact-schema:{}",
            envelope.route_plan_digest().unwrap_or("none")
        ),
    );
    BridgeSpeculativeSessionRequest::new(
        BridgePreviewSessionIdentity::new(format!(
            "query.preview.session:{}",
            envelope.declaration_digest()
        )),
        declaration,
        1,
        1,
        1,
    )
}

fn preview_promotion_request<
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
) -> BridgeSpeculativePromotionRequest {
    BridgeSpeculativePromotionRequest::new(
        format!(
            "boundary:{}",
            envelope
                .envelope_digest()
                .metadata()
                .algorithm()
                .id()
                .as_str()
        ),
        format!("artifact:{}", envelope.declaration_digest()),
    )
}

fn subscription_preparation_request<
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
    lowering: &BridgeLoweringContext,
    request: ForgeQueryDeclarationBridgeContinuationRequest,
) -> BridgeSubscriptionContinuationCandidateInput {
    BridgeSubscriptionContinuationCandidateInput::branch_local_continue(
        lowering.truth_view_basis_digest(envelope, request.truth_context()),
        format!(
            "subscription-locality:{}",
            envelope.declaration_family_key()
        ),
        format!("child-basis:{}", envelope.declaration_digest()),
    )
}

fn writeback_preparation_request<
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
    lowering: &BridgeLoweringContext,
    request: ForgeQueryDeclarationBridgeContinuationRequest,
) -> TruthWritebackRequest {
    let basis_digest = lowering.truth_view_basis_digest(envelope, request.truth_context());
    TruthWritebackRequest::new(
        BridgeWritebackFamilyKind::ProjectedStateDiff,
        format!("contract:{}", envelope.declaration_family_key()),
        format!("candidate:{}", envelope.declaration_digest()),
        format!("mapped-input:{}", envelope.declaration_digest()),
        format!("mapper-witness:{basis_digest}"),
        format!("derived-effect:{basis_digest}"),
        format!(
            "proposed-effect:{}",
            envelope.route_plan_digest().unwrap_or("none")
        ),
        BridgeWritebackEffectClass::ProjectedStateDiff,
        BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
        format!("feedback:{}", envelope.handle_identity_digest()),
        format!(
            "loop-prevention:{}",
            envelope.operating_context_identity_digest()
        ),
        BridgeWritebackLoopDisposition::AllowAuthoritativeAttempt,
        format!(
            "strategy-compatibility:{}",
            lowering.runtime_surface_digest()
        ),
        format!("causality:{basis_digest}"),
        format!("idempotence:{}", envelope.declaration_digest()),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
        format!("strategy-descriptor:{}", envelope.declaration_family_key()),
    )
}

fn preview_branch_binding<
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
) -> BridgeSpeculativeBranchBinding {
    BridgeSpeculativeBranchBinding::new(
        BridgeSpeculativeBranchBindingIdentity::new(format!(
            "query-preview-binding:{}",
            envelope.declaration_digest()
        )),
        TruthBranchIdentity::new(format!(
            "query-preview-truth-branch:{}",
            envelope.operating_context_identity_digest()
        )),
        BridgeSignalBranchIdentity::new(format!(
            "query-preview-signal-branch:{}",
            envelope.handle_identity_digest()
        )),
    )
}

fn runtime_surface_digest<
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
) -> String {
    format!(
        "runtime-surface:{}:{}",
        envelope.declaration_family_key(),
        envelope.handle_identity_digest()
    )
}
