use crate::lower_runtime_routing::WorthQueryLowerRuntimeSeamKey;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryLowerRuntimeSyntheticTailRow {
    seam_key: WorthQueryLowerRuntimeSeamKey,
    justification: &'static str,
}

impl WorthQueryLowerRuntimeSyntheticTailRow {
    pub(crate) fn seam_key(self) -> WorthQueryLowerRuntimeSeamKey {
        self.seam_key
    }

    pub(crate) fn justification(self) -> &'static str {
        self.justification
    }
}

pub(crate) fn required_phase_six_concrete_seams() -> &'static [WorthQueryLowerRuntimeSeamKey] {
    &[
        WorthQueryLowerRuntimeSeamKey::ComposeRead,
        WorthQueryLowerRuntimeSeamKey::ComposeReadWithInvariantPack,
        WorthQueryLowerRuntimeSeamKey::ExecuteReadFamily,
        WorthQueryLowerRuntimeSeamKey::ExecuteReadFamilyInBasisContext,
        WorthQueryLowerRuntimeSeamKey::ExecuteRuntimeCurrentReadGraph,
        WorthQueryLowerRuntimeSeamKey::ExecuteRuntimeBasisContextReadGraph,
        WorthQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
        WorthQueryLowerRuntimeSeamKey::LiveViewSourceDeclaration,
        WorthQueryLowerRuntimeSeamKey::PublicLiveViewDeclaration,
        WorthQueryLowerRuntimeSeamKey::RuntimeLiveInstallationOrchestration,
        WorthQueryLowerRuntimeSeamKey::SubscriptionActivation,
        WorthQueryLowerRuntimeSeamKey::SubscriptionContinuity,
        WorthQueryLowerRuntimeSeamKey::PreviewBasisAdmission,
        WorthQueryLowerRuntimeSeamKey::BasisReadmissionFromTruthViewEvidence,
        WorthQueryLowerRuntimeSeamKey::BasisReadmissionFromSubscriptionEvidence,
        WorthQueryLowerRuntimeSeamKey::HistoricalBridgeLowering,
        WorthQueryLowerRuntimeSeamKey::EffectBackedRelationalMutation,
        WorthQueryLowerRuntimeSeamKey::EffectBackedRelationalMerge,
        WorthQueryLowerRuntimeSeamKey::EffectBackedBridgeWriteback,
        WorthQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        WorthQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        WorthQueryLowerRuntimeSeamKey::RuntimeIntentAuthorityAdapter,
        WorthQueryLowerRuntimeSeamKey::IntentRuntimeExecution,
        WorthQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromQueryReceipts,
        WorthQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromRelationalArtifacts,
        WorthQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromBridgeArtifacts,
        WorthQueryLowerRuntimeSeamKey::CausalBridgeMaterialization,
        WorthQueryLowerRuntimeSeamKey::FrontierEvidenceIntake,
    ]
}

pub(crate) fn allowed_phase_six_synthetic_seams(
) -> &'static [WorthQueryLowerRuntimeSyntheticTailRow] {
    ALLOWED_PHASE_SIX_SYNTHETIC_SEAMS
}

const ALLOWED_PHASE_SIX_SYNTHETIC_SEAMS: &[WorthQueryLowerRuntimeSyntheticTailRow] = &[];
