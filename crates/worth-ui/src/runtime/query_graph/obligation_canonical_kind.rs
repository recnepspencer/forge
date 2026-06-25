use forge_query::facade::runtime::ForgeQueryGraphObligationKind;

use super::obligation_mapping::{
    WorthUiQueryGraphCanonicalObligationKind, WorthUiQueryGraphObligationSemantic,
};

impl WorthUiQueryGraphObligationSemantic {
    pub fn canonical_kind(self) -> WorthUiQueryGraphCanonicalObligationKind {
        match self {
            Self::SchemaAdmission => ForgeQueryGraphObligationKind::SchemaContractValidator,
            Self::CapabilitySupport => ForgeQueryGraphObligationKind::CapabilityGapScreen,
            Self::ActivationEligibility => ForgeQueryGraphObligationKind::BlockingInvariant,
            Self::CommandSupport => ForgeQueryGraphObligationKind::CapabilityGapScreen,
            Self::InteractionContainment => ForgeQueryGraphObligationKind::OperatingContextGate,
            Self::InteractionFocusability => ForgeQueryGraphObligationKind::CapabilityGapScreen,
            Self::OperatingContext => ForgeQueryGraphObligationKind::OperatingContextGate,
            Self::DependencyContract => {
                ForgeQueryGraphObligationKind::PreflightSequencingObligation
            }
            Self::EventRegionContract => {
                ForgeQueryGraphObligationKind::PreflightSequencingObligation
            }
            Self::EventContainment => ForgeQueryGraphObligationKind::OperatingContextGate,
            Self::EventDisabledBlock => ForgeQueryGraphObligationKind::BlockingInvariant,
            Self::EventCapturePolicy => ForgeQueryGraphObligationKind::CapabilityGapScreen,
            Self::EventCursorPosture => ForgeQueryGraphObligationKind::OperatingContextGate,
            Self::EventPropagation => ForgeQueryGraphObligationKind::PreflightSequencingObligation,
            Self::ContentSchemaAdmission => ForgeQueryGraphObligationKind::SchemaContractValidator,
            Self::ContentIconCapability => ForgeQueryGraphObligationKind::CapabilityGapScreen,
            Self::ContentVectorPosture => ForgeQueryGraphObligationKind::CapabilityGapScreen,
            Self::ContentAccessibilityParticipation => {
                ForgeQueryGraphObligationKind::OperatingContextGate
            }
            Self::ContentSlotParticipation => ForgeQueryGraphObligationKind::CapabilityGapScreen,
            Self::ContentDependencyContract => {
                ForgeQueryGraphObligationKind::PreflightSequencingObligation
            }
            Self::TargetMountedTopology => ForgeQueryGraphObligationKind::OperatingContextGate,
            Self::TargetArtifactBasis => {
                ForgeQueryGraphObligationKind::PreflightSequencingObligation
            }
            Self::TargetComponentIdentity => ForgeQueryGraphObligationKind::BlockingInvariant,
            Self::TargetOperationFamily => ForgeQueryGraphObligationKind::BlockingInvariant,
            Self::TargetBindingPosture => ForgeQueryGraphObligationKind::OperatingContextGate,
            Self::LiveViewDeclarationIdentity => ForgeQueryGraphObligationKind::BlockingInvariant,
            Self::LiveViewTargetBinding => ForgeQueryGraphObligationKind::OperatingContextGate,
            Self::LiveViewStateCompatibility => {
                ForgeQueryGraphObligationKind::SchemaContractValidator
            }
            Self::LiveViewReadPosture => ForgeQueryGraphObligationKind::CapabilityGapScreen,
            Self::LiveViewWritePosture => ForgeQueryGraphObligationKind::CapabilityGapScreen,
            Self::LiveViewEffectIntentAdmission => {
                ForgeQueryGraphObligationKind::PreflightSequencingObligation
            }
            Self::LiveViewProjectionConsumption => {
                ForgeQueryGraphObligationKind::PreflightSequencingObligation
            }
            Self::LiveViewControlProjectionKind => {
                ForgeQueryGraphObligationKind::SchemaContractValidator
            }
            Self::LiveViewControlOptionSource => ForgeQueryGraphObligationKind::CapabilityGapScreen,
            Self::LiveViewControlCompatibility => {
                ForgeQueryGraphObligationKind::SchemaContractValidator
            }
            Self::LiveViewConditionalExpression => {
                ForgeQueryGraphObligationKind::SchemaContractValidator
            }
            Self::LiveViewConditionalParticipation => {
                ForgeQueryGraphObligationKind::OperatingContextGate
            }
            Self::LiveViewRetainedStatePosture => {
                ForgeQueryGraphObligationKind::OperatingContextGate
            }
            Self::LiveViewExpressionOperator => {
                ForgeQueryGraphObligationKind::CapabilityGapScreen
            }
            Self::LiveViewExpressionArity => ForgeQueryGraphObligationKind::SchemaContractValidator,
            Self::LiveViewExpressionValueKind => {
                ForgeQueryGraphObligationKind::SchemaContractValidator
            }
            Self::LiveViewExpressionDependencyContract => {
                ForgeQueryGraphObligationKind::PreflightSequencingObligation
            }
            Self::LiveViewRequirednessDeclaration => {
                ForgeQueryGraphObligationKind::SchemaContractValidator
            }
            Self::LiveViewValuePresence => ForgeQueryGraphObligationKind::BlockingInvariant,
            Self::LiveViewReadinessPosture => ForgeQueryGraphObligationKind::BlockingInvariant,
            Self::LiveViewInteractionIntentKind => {
                ForgeQueryGraphObligationKind::SchemaContractValidator
            }
            Self::LiveViewInteractionEffect => ForgeQueryGraphObligationKind::CapabilityGapScreen,
            Self::LiveViewPayloadShape => ForgeQueryGraphObligationKind::SchemaContractValidator,
            Self::CompositionNodeKind => ForgeQueryGraphObligationKind::SchemaContractValidator,
            Self::CompositionParentEdge => {
                ForgeQueryGraphObligationKind::PreflightSequencingObligation
            }
            Self::CompositionSiblingOrder => ForgeQueryGraphObligationKind::BlockingInvariant,
            Self::CompositionParticipation => ForgeQueryGraphObligationKind::OperatingContextGate,
            Self::CompositionMountedTopology => ForgeQueryGraphObligationKind::OperatingContextGate,
            Self::CompositionAccessPlan => {
                ForgeQueryGraphObligationKind::PreflightSequencingObligation
            }
            Self::CompositionChildLookup => {
                ForgeQueryGraphObligationKind::PreflightSequencingObligation
            }
            Self::CompositionAncestorLookup => {
                ForgeQueryGraphObligationKind::PreflightSequencingObligation
            }
            Self::CompositionParticipationFilter => {
                ForgeQueryGraphObligationKind::OperatingContextGate
            }
            Self::CompositionAffectedConsumerLookup => {
                ForgeQueryGraphObligationKind::PreflightSequencingObligation
            }
            Self::CompositionContextPropagation => {
                ForgeQueryGraphObligationKind::OperatingContextGate
            }
            Self::CompositionContextOverrideEligibility => {
                ForgeQueryGraphObligationKind::BlockingInvariant
            }
            Self::CompositionContextDisabledSuppression => {
                ForgeQueryGraphObligationKind::BlockingInvariant
            }
            Self::CompositionContextValidationParticipation => {
                ForgeQueryGraphObligationKind::OperatingContextGate
            }
            Self::CompositionContextDependencyContract => {
                ForgeQueryGraphObligationKind::PreflightSequencingObligation
            }
            Self::CompositionAccessibilityRole => {
                ForgeQueryGraphObligationKind::SchemaContractValidator
            }
            Self::CompositionAccessibilityName => ForgeQueryGraphObligationKind::BlockingInvariant,
            Self::CompositionAccessibilityDescription => {
                ForgeQueryGraphObligationKind::BlockingInvariant
            }
            Self::CompositionAccessibilityAssociation => {
                ForgeQueryGraphObligationKind::PreflightSequencingObligation
            }
            Self::CompositionFocusScope => ForgeQueryGraphObligationKind::OperatingContextGate,
            Self::CompositionFocusOrder => ForgeQueryGraphObligationKind::BlockingInvariant,
            Self::CompositionVisibilityPosture => {
                ForgeQueryGraphObligationKind::OperatingContextGate
            }
        }
    }
}
