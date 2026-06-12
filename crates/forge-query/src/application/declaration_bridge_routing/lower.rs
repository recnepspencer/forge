use forge_foundational::facade::{AspectKey, AspectValue};
use forge_runtime_bridge::facade::{
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

use crate::application::ForgeQueryDeclarationEnvelope;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::{
    query_truth_branch_identity, query_truth_commit_identity, query_truth_snapshot_identity,
};

use super::{
    artifact::{
        ForgeQueryDeclarationBridgeBinding, ForgeQueryPreviewPromotionContinuationBinding,
        ForgeQueryWritebackPreparationBinding,
    },
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
            ForgeQueryDeclarationBridgeBinding::PreviewPromotion(preview_promotion_binding(
                envelope, &lowering, request,
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
    runtime_surface_identity: ForgeQueryEvidenceIdentity,
}

impl BridgeLoweringContext {
    fn new<
        D: crate::application::ForgeQueryDomainEntryMarker,
        I: crate::application::ForgeQueryDeclarationInput<D>,
    >(
        envelope: &ForgeQueryDeclarationEnvelope<D, I>,
    ) -> Self {
        Self {
            runtime_surface_identity: runtime_surface_identity(envelope),
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
        let branch_identity = query_truth_branch_identity(
            "query-branch",
            envelope.operating_context_identity_digest(),
        );
        match truth_context {
            ForgeQueryDeclarationBridgeTruthContext::Current => {
                BridgeTruthViewSelector::branch_head(branch_identity)
            }
            ForgeQueryDeclarationBridgeTruthContext::Historical => {
                BridgeTruthViewSelector::historical_commit(
                    branch_identity,
                    query_truth_commit_identity("query-commit", envelope.declaration_digest()),
                )
            }
            ForgeQueryDeclarationBridgeTruthContext::Preview => {
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

    fn preview_session_basis<
        D: crate::application::ForgeQueryDomainEntryMarker,
        I: crate::application::ForgeQueryDeclarationInput<D>,
    >(
        &self,
        envelope: &ForgeQueryDeclarationEnvelope<D, I>,
        truth_context: ForgeQueryDeclarationBridgeTruthContext,
    ) -> BridgePreviewSessionBasis {
        BridgePreviewSessionBasis::new(
            self.truth_view_selector(envelope, truth_context),
            BridgeSourceCapabilitySet::new(vec![BridgeSourceCapability::SnapshotRead]),
            BridgePreviewRetainedArtifactSchema::PreviewLifecycleArtifactsV1,
        )
    }

    fn runtime_surface_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.runtime_surface_identity
    }
}

fn runtime_route_request<
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
) -> BridgeRouteRequest {
    BridgeRouteRequest::for_commit(query_truth_commit_identity(
        "query-bridge-route",
        envelope.declaration_digest(),
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
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
    lowering: &BridgeLoweringContext,
    request: ForgeQueryDeclarationBridgeContinuationRequest,
) -> ForgeQueryPreviewPromotionContinuationBinding {
    let preview_basis_digest = lowering.truth_view_basis_digest(envelope, request.truth_context());
    let promotion_continuation_digest = crate::evidence_identity::forge_query_evidence_identity(
        crate::evidence_identity::ForgeQueryEvidenceScope::PreviewPromotionContinuation,
    )
    .field_identity(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("preview_basis"),
        &preview_basis_digest,
    )
    .field_identity(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("declaration"),
        envelope.declaration_digest(),
    )
    .field_shape(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("basis_algorithm"),
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
    ForgeQueryPreviewPromotionContinuationBinding::new(
        preview_basis_digest,
        promotion_continuation_digest,
        envelope.declaration_digest().to_string(),
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
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
    lowering: &BridgeLoweringContext,
    request: ForgeQueryDeclarationBridgeContinuationRequest,
) -> ForgeQueryWritebackPreparationBinding {
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
            &BridgeIdentityEvidence::from_external_authority(lowering.runtime_surface_identity()),
        ),
        query_truth_snapshot_identity("evaluation", envelope.declaration_digest()),
        query_truth_snapshot_identity("truth-view-basis", basis_digest),
    );
    let effect_intent = BridgeWritebackEffectIntent::validated_scalar_patch(
        BridgeWritebackEffectClass::ProjectedStateDiff,
        AspectKey::new("query.writeback.preparation")
            .expect("static query writeback preparation aspect key is valid"),
        AspectValue::String(envelope.declaration_digest().to_string().into()),
    )
    .expect("query writeback preparation effect intent should validate");

    ForgeQueryWritebackPreparationBinding::new(declaration, causality, effect_intent)
}

fn preview_branch_binding<
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
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
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::DeclarationBridgeLoweringIdentity)
        .field_shape(ForgeQueryEvidenceTag::new("role"), "runtime-surface")
        .field_shape(
            ForgeQueryEvidenceTag::new("declaration_family"),
            envelope.declaration_family_key(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("handle"),
            envelope.handle_identity_digest(),
        )
        .seal()
}

fn bridge_lowering_evidence_identity(
    role: &'static str,
    evidence: impl AsRef<str>,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::DeclarationBridgeLoweringIdentity)
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_identity(ForgeQueryEvidenceTag::new("evidence"), evidence)
        .seal()
}

fn bridge_lowering_bridge_evidence_identity(
    role: &'static str,
    evidence: impl AsRef<str>,
) -> BridgeIdentityEvidence {
    BridgeIdentityEvidence::from_external_authority(bridge_lowering_evidence_identity(
        role, evidence,
    ))
}
