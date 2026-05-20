use crate::lower_runtime_routing::ForgeQueryLowerRuntimeSeamKey;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ForgeQueryLowerRuntimeSyntheticTailRow {
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    justification: &'static str,
}

impl ForgeQueryLowerRuntimeSyntheticTailRow {
    pub(crate) fn seam_key(self) -> ForgeQueryLowerRuntimeSeamKey {
        self.seam_key
    }

    pub(crate) fn justification(self) -> &'static str {
        self.justification
    }
}

pub(crate) fn required_phase_six_concrete_seams() -> &'static [ForgeQueryLowerRuntimeSeamKey] {
    &[
        ForgeQueryLowerRuntimeSeamKey::ComposeRead,
        ForgeQueryLowerRuntimeSeamKey::ComposeReadWithInvariantPack,
        ForgeQueryLowerRuntimeSeamKey::ExecuteReadFamily,
        ForgeQueryLowerRuntimeSeamKey::ExecuteReadFamilyInBasisContext,
        ForgeQueryLowerRuntimeSeamKey::ExecuteRuntimeCurrentReadGraph,
        ForgeQueryLowerRuntimeSeamKey::ExecuteRuntimeBasisContextReadGraph,
        ForgeQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
        ForgeQueryLowerRuntimeSeamKey::LiveViewSourceDeclaration,
        ForgeQueryLowerRuntimeSeamKey::PublicLiveViewDeclaration,
        ForgeQueryLowerRuntimeSeamKey::RuntimeLiveInstallationOrchestration,
        ForgeQueryLowerRuntimeSeamKey::SubscriptionActivation,
        ForgeQueryLowerRuntimeSeamKey::SubscriptionContinuity,
        ForgeQueryLowerRuntimeSeamKey::PreviewBasisAdmission,
        ForgeQueryLowerRuntimeSeamKey::BasisReadmissionFromTruthViewEvidence,
        ForgeQueryLowerRuntimeSeamKey::BasisReadmissionFromSubscriptionEvidence,
        ForgeQueryLowerRuntimeSeamKey::HistoricalBridgeLowering,
        ForgeQueryLowerRuntimeSeamKey::EffectBackedRelationalMutation,
        ForgeQueryLowerRuntimeSeamKey::EffectBackedRelationalMerge,
        ForgeQueryLowerRuntimeSeamKey::EffectBackedBridgeWriteback,
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        ForgeQueryLowerRuntimeSeamKey::RuntimeIntentAuthorityAdapter,
        ForgeQueryLowerRuntimeSeamKey::IntentRuntimeExecution,
        ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromQueryReceipts,
        ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromRelationalArtifacts,
        ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromBridgeArtifacts,
        ForgeQueryLowerRuntimeSeamKey::CausalBridgeMaterialization,
        ForgeQueryLowerRuntimeSeamKey::FrontierEvidenceIntake,
    ]
}

pub(crate) fn allowed_phase_six_synthetic_seams(
) -> &'static [ForgeQueryLowerRuntimeSyntheticTailRow] {
    ALLOWED_PHASE_SIX_SYNTHETIC_SEAMS
}

const ALLOWED_PHASE_SIX_SYNTHETIC_SEAMS: &[ForgeQueryLowerRuntimeSyntheticTailRow] = &[];
