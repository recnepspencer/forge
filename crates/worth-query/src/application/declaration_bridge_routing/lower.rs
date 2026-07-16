use worth_foundational::facade::AspectKey;
use worth_runtime_bridge::facade::{
    BridgeDeliveryIntent, BridgeDiagnosticsTier, BridgeIdentityEvidence,
    BridgePreviewRetainedArtifactSchema, BridgePreviewSessionBasis,
    BridgePreviewSessionDeclaration, BridgePreviewSessionDeclarationIdentity,
    BridgePreviewSessionIdentity, BridgeReplayMode, BridgeRequestKind, BridgeRouteIdentity,
    BridgeRouteRequest, BridgeSignalBranchIdentity, BridgeSourceCapability,
    BridgeSourceCapabilitySet, BridgeSpeculativeBranchBinding,
    BridgeSpeculativeBranchBindingIdentity, BridgeSpeculativeSessionRequest,
    BridgeSubscriptionContinuationCandidateInput, BridgeTruthViewEvaluationRequest,
    BridgeTruthViewSelector, BridgeWritebackCausalityIdentity, BridgeWritebackDeclaration,
    BridgeWritebackDeclarationIdentity, BridgeWritebackEffectClass, BridgeWritebackEffectIntent,
    BridgeWritebackFamilyKind, BridgeWritebackIdempotenceClass,
    BridgeWritebackNativeCausalityInputs, BridgeWritebackStrategyClass,
    HistoricalEvaluationDeclaration,
};

use crate::application::WorthQueryDeclarationEnvelope;
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::{
    query_truth_branch_identity, query_truth_commit_identity, query_truth_snapshot_identity,
};

use super::{
    artifact::{
        WorthQueryDeclarationBridgeBinding, WorthQueryPreviewPromotionContinuationBinding,
        WorthQueryWritebackPreparationBinding,
    },
    contract::{
        WorthQueryDeclarationBridgeContinuationContract,
        WorthQueryDeclarationBridgeContinuationFamily,
    },
    request::{
        WorthQueryDeclarationBridgeContinuationMode,
        WorthQueryDeclarationBridgeContinuationRequest, WorthQueryDeclarationBridgeTruthContext,
    },
};

pub(crate) fn worth_query_lower_bridge_binding<
    D: crate::application::WorthQueryDomainEntryMarker,
    I: crate::application::WorthQueryDeclarationInput<D>,
>(
    envelope: &WorthQueryDeclarationEnvelope<D, I>,
    contract: WorthQueryDeclarationBridgeContinuationContract,
) -> (
    WorthQueryDeclarationBridgeContinuationRequest,
    WorthQueryDeclarationBridgeContinuationFamily,
    WorthQueryDeclarationBridgeBinding,
) {
    let request = contract.request();
    let lowering = BridgeLoweringContext::new(envelope);
    let binding = match request.mode() {
        WorthQueryDeclarationBridgeContinuationMode::RuntimeRoute => {
            WorthQueryDeclarationBridgeBinding::RuntimeRoute(runtime_route_request(envelope))
        }
        WorthQueryDeclarationBridgeContinuationMode::TruthView => {
            WorthQueryDeclarationBridgeBinding::TruthView(truth_view_request(
                &lowering, envelope, request,
            ))
        }
        WorthQueryDeclarationBridgeContinuationMode::PreviewSession => {
            WorthQueryDeclarationBridgeBinding::PreviewSession(preview_session_request(
                envelope, &lowering, request,
            ))
        }
        WorthQueryDeclarationBridgeContinuationMode::PreviewPromotion => {
            WorthQueryDeclarationBridgeBinding::PreviewPromotion(preview_promotion_binding(
                envelope, &lowering, request,
            ))
        }
        WorthQueryDeclarationBridgeContinuationMode::SubscriptionPreparation => {
            WorthQueryDeclarationBridgeBinding::SubscriptionPreparation(
                subscription_preparation_request(envelope, &lowering, request),
            )
        }
        WorthQueryDeclarationBridgeContinuationMode::WritebackPreparation => {
            WorthQueryDeclarationBridgeBinding::WritebackPreparation(writeback_preparation_request(
                envelope, &lowering, request,
            ))
        }
    };
    (request, contract.family(), binding)
}

struct BridgeLoweringContext {
    runtime_surface_identity: WorthQueryEvidenceIdentity,
}

impl BridgeLoweringContext {
    fn new<
        D: crate::application::WorthQueryDomainEntryMarker,
        I: crate::application::WorthQueryDeclarationInput<D>,
    >(
        envelope: &WorthQueryDeclarationEnvelope<D, I>,
    ) -> Self {
        Self {
            runtime_surface_identity: runtime_surface_identity(envelope),
        }
    }

    fn truth_view_selector<
        D: crate::application::WorthQueryDomainEntryMarker,
        I: crate::application::WorthQueryDeclarationInput<D>,
    >(
        &self,
        envelope: &WorthQueryDeclarationEnvelope<D, I>,
        truth_context: WorthQueryDeclarationBridgeTruthContext,
    ) -> BridgeTruthViewSelector {
        let branch_identity = query_truth_branch_identity(
            "query-branch",
            envelope.operating_context_identity_digest(),
        );
        match truth_context {
            WorthQueryDeclarationBridgeTruthContext::Current => {
                BridgeTruthViewSelector::branch_head(branch_identity)
            }
            WorthQueryDeclarationBridgeTruthContext::Historical => {
                BridgeTruthViewSelector::historical_commit(
                    branch_identity,
                    query_truth_commit_identity("query-commit", envelope.declaration_digest()),
                )
            }
            WorthQueryDeclarationBridgeTruthContext::Preview => {
                BridgeTruthViewSelector::branch_snapshot(
                    branch_identity,
                    query_truth_snapshot_identity(
                        "query-snapshot",
                        envelope.route_plan_digest().unwrap_or("none"),
                    ),
                )
            }
        }
    }

    fn truth_view_basis_digest<
        D: crate::application::WorthQueryDomainEntryMarker,
        I: crate::application::WorthQueryDeclarationInput<D>,
    >(
        &self,
        envelope: &WorthQueryDeclarationEnvelope<D, I>,
        truth_context: WorthQueryDeclarationBridgeTruthContext,
    ) -> String {
        let declaration = HistoricalEvaluationDeclaration::new(
            self.truth_view_selector(envelope, truth_context),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        declaration.digest().to_string()
    }

    fn preview_session_basis<
        D: crate::application::WorthQueryDomainEntryMarker,
        I: crate::application::WorthQueryDeclarationInput<D>,
    >(
        &self,
        envelope: &WorthQueryDeclarationEnvelope<D, I>,
        truth_context: WorthQueryDeclarationBridgeTruthContext,
    ) -> BridgePreviewSessionBasis {
        BridgePreviewSessionBasis::new(
            self.truth_view_selector(envelope, truth_context),
            BridgeSourceCapabilitySet::new(vec![BridgeSourceCapability::SnapshotRead]),
            BridgePreviewRetainedArtifactSchema::PreviewLifecycleArtifactsV1,
        )
    }

    fn runtime_surface_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.runtime_surface_identity
    }
}

fn runtime_route_request<
    D: crate::application::WorthQueryDomainEntryMarker,
    I: crate::application::WorthQueryDeclarationInput<D>,
>(
    envelope: &WorthQueryDeclarationEnvelope<D, I>,
) -> BridgeRouteRequest {
    BridgeRouteRequest::for_commit(query_truth_commit_identity(
        "query-bridge-route",
        envelope.declaration_digest(),
    ))
}

fn truth_view_request<
    D: crate::application::WorthQueryDomainEntryMarker,
    I: crate::application::WorthQueryDeclarationInput<D>,
>(
    lowering: &BridgeLoweringContext,
    envelope: &WorthQueryDeclarationEnvelope<D, I>,
    request: WorthQueryDeclarationBridgeContinuationRequest,
) -> BridgeTruthViewEvaluationRequest {
    let selector = lowering.truth_view_selector(envelope, request.truth_context());
    let evaluation = BridgeTruthViewEvaluationRequest::new(selector)
        .with_diagnostics_tier(BridgeDiagnosticsTier::Standard);
    if request.truth_context() == WorthQueryDeclarationBridgeTruthContext::Historical {
        evaluation
            .with_replay_mode(BridgeReplayMode::Required)
            .with_delivery_intent(BridgeDeliveryIntent::PrepareOnly)
    } else {
        evaluation.with_delivery_intent(BridgeDeliveryIntent::PrepareSignalEvaluation)
    }
}

fn preview_session_request<
    D: crate::application::WorthQueryDomainEntryMarker,
    I: crate::application::WorthQueryDeclarationInput<D>,
>(
    envelope: &WorthQueryDeclarationEnvelope<D, I>,
    lowering: &BridgeLoweringContext,
    request: WorthQueryDeclarationBridgeContinuationRequest,
) -> BridgeSpeculativeSessionRequest {
    let binding = preview_branch_binding(envelope);
    let declaration = BridgePreviewSessionDeclaration::new(
        BridgePreviewSessionDeclarationIdentity::from_bridge_evidence(
            &bridge_lowering_bridge_evidence_identity(
                "preview-session-declaration",
                envelope
                    .envelope_digest()
                    .metadata()
                    .algorithm()
                    .id()
                    .as_str(),
            ),
        ),
        BridgeRequestKind::Preview,
        binding,
        lowering.preview_session_basis(envelope, request.truth_context()),
    );
    BridgeSpeculativeSessionRequest::new(
        BridgePreviewSessionIdentity::from_bridge_evidence(
            &bridge_lowering_bridge_evidence_identity(
                "preview-session",
                envelope.declaration_digest(),
            ),
        ),
        declaration,
        1,
        1,
        1,
    )
}

fn preview_promotion_binding<
    D: crate::application::WorthQueryDomainEntryMarker,
    I: crate::application::WorthQueryDeclarationInput<D>,
>(
    envelope: &WorthQueryDeclarationEnvelope<D, I>,
    lowering: &BridgeLoweringContext,
    request: WorthQueryDeclarationBridgeContinuationRequest,
) -> WorthQueryPreviewPromotionContinuationBinding {
    let preview_basis_digest = lowering.truth_view_basis_digest(envelope, request.truth_context());
    let promotion_continuation_digest = crate::evidence_identity::worth_query_evidence_identity(
        crate::evidence_identity::WorthQueryEvidenceScope::PreviewPromotionContinuation,
    )
    .field_value(
        crate::evidence_identity::WorthQueryEvidenceTag::new("preview_basis"),
        &preview_basis_digest,
    )
    .field_value(
        crate::evidence_identity::WorthQueryEvidenceTag::new("declaration"),
        envelope.declaration_digest(),
    )
    .field_shape(
        crate::evidence_identity::WorthQueryEvidenceTag::new("basis_algorithm"),
        envelope
            .envelope_digest()
            .metadata()
            .algorithm()
            .id()
            .as_str(),
    )
    .seal()
    .as_str()
    .to_string();
    WorthQueryPreviewPromotionContinuationBinding::new(
        preview_basis_digest,
        promotion_continuation_digest,
        envelope.declaration_digest().to_string(),
    )
}

fn subscription_preparation_request<
    D: crate::application::WorthQueryDomainEntryMarker,
    I: crate::application::WorthQueryDeclarationInput<D>,
>(
    envelope: &WorthQueryDeclarationEnvelope<D, I>,
    lowering: &BridgeLoweringContext,
    request: WorthQueryDeclarationBridgeContinuationRequest,
) -> BridgeSubscriptionContinuationCandidateInput {
    let authority = bridge_lowering_bridge_evidence_identity(
        "subscription-authority",
        lowering.truth_view_basis_digest(envelope, request.truth_context()),
    );
    let locality = bridge_lowering_bridge_evidence_identity(
        "subscription-locality",
        envelope.declaration_family_key(),
    );
    let child_basis = bridge_lowering_bridge_evidence_identity(
        "subscription-child-basis",
        envelope.declaration_digest(),
    );
    BridgeSubscriptionContinuationCandidateInput::branch_local_continue_from_evidence(
        &authority,
        &locality,
        &child_basis,
    )
}

fn writeback_preparation_request<
    D: crate::application::WorthQueryDomainEntryMarker,
    I: crate::application::WorthQueryDeclarationInput<D>,
>(
    envelope: &WorthQueryDeclarationEnvelope<D, I>,
    lowering: &BridgeLoweringContext,
    request: WorthQueryDeclarationBridgeContinuationRequest,
) -> WorthQueryWritebackPreparationBinding {
    let basis_digest = lowering.truth_view_basis_digest(envelope, request.truth_context());
    let declaration = BridgeWritebackDeclaration::writeback_capable(
        BridgeWritebackDeclarationIdentity::from_bridge_evidence(
            &bridge_lowering_bridge_evidence_identity(
                "writeback-declaration",
                envelope.declaration_digest(),
            ),
        ),
        BridgeRequestKind::Authoritative,
        BridgeWritebackFamilyKind::ProjectedStateDiff,
        BridgeWritebackEffectClass::ProjectedStateDiff,
        BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let causality = BridgeWritebackNativeCausalityInputs::new(
        BridgeWritebackCausalityIdentity::from_bridge_evidence(
            &bridge_lowering_bridge_evidence_identity("writeback-causality", &basis_digest),
        ),
        query_truth_commit_identity("truth-trigger", envelope.handle_identity_digest()),
        BridgeRouteIdentity::from_bridge_evidence(
            &lowering
                .runtime_surface_identity()
                .bridge_external_identity_evidence(),
        ),
        query_truth_snapshot_identity("evaluation", envelope.declaration_digest()),
        query_truth_snapshot_identity("truth-view-basis", basis_digest),
    );
    let effect_intent = BridgeWritebackEffectIntent::validated_scalar_patch(
        BridgeWritebackEffectClass::ProjectedStateDiff,
        AspectKey::new("query.writeback.preparation")
            .expect("static query writeback preparation aspect key is valid"),
        crate::runtime::WorthQueryAuthoredAspectMutation::native_string_value(
            envelope.declaration_digest().to_string(),
        ),
    )
    .expect("query writeback preparation effect intent should validate");

    WorthQueryWritebackPreparationBinding::new(declaration, causality, effect_intent)
}

fn preview_branch_binding<
    D: crate::application::WorthQueryDomainEntryMarker,
    I: crate::application::WorthQueryDeclarationInput<D>,
>(
    envelope: &WorthQueryDeclarationEnvelope<D, I>,
) -> BridgeSpeculativeBranchBinding {
    BridgeSpeculativeBranchBinding::new(
        BridgeSpeculativeBranchBindingIdentity::from_bridge_evidence(
            &bridge_lowering_bridge_evidence_identity(
                "preview-branch-binding",
                envelope.declaration_digest(),
            ),
        ),
        query_truth_branch_identity(
            "query-preview-truth-branch",
            envelope.operating_context_identity_digest(),
        ),
        BridgeSignalBranchIdentity::from_bridge_evidence(
            &bridge_lowering_bridge_evidence_identity(
                "preview-signal-branch",
                envelope.handle_identity_digest(),
            ),
        ),
    )
}

fn runtime_surface_identity<
    D: crate::application::WorthQueryDomainEntryMarker,
    I: crate::application::WorthQueryDeclarationInput<D>,
>(
    envelope: &WorthQueryDeclarationEnvelope<D, I>,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::DeclarationBridgeLoweringIdentity)
        .field_shape(WorthQueryEvidenceTag::new("role"), "runtime-surface")
        .field_shape(
            WorthQueryEvidenceTag::new("declaration_family"),
            envelope.declaration_family_key(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("handle"),
            envelope.handle_identity_digest(),
        )
        .seal()
}

fn bridge_lowering_evidence_identity(
    role: &'static str,
    evidence: impl AsRef<str>,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::DeclarationBridgeLoweringIdentity)
        .field_shape(WorthQueryEvidenceTag::new("role"), role)
        .field_value(WorthQueryEvidenceTag::new("evidence"), evidence)
        .seal()
}

fn bridge_lowering_bridge_evidence_identity(
    role: &'static str,
    evidence: impl AsRef<str>,
) -> BridgeIdentityEvidence {
    bridge_lowering_evidence_identity(role, evidence).bridge_external_identity_evidence()
}
