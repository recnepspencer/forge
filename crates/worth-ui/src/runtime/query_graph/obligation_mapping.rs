#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthUiQueryGraphObligationSemantic {
    SchemaAdmission,
    CapabilitySupport,
    ActivationEligibility,
    CommandSupport,
    InteractionContainment,
    InteractionFocusability,
    OperatingContext,
    DependencyContract,
    EventRegionContract,
    EventContainment,
    EventDisabledBlock,
    EventCapturePolicy,
    EventCursorPosture,
    EventPropagation,
    ContentSchemaAdmission,
    ContentIconCapability,
    ContentVectorPosture,
    ContentAccessibilityParticipation,
    ContentSlotParticipation,
    ContentDependencyContract,
    TargetMountedTopology,
    TargetArtifactBasis,
    TargetComponentIdentity,
    TargetOperationFamily,
    TargetBindingPosture,
    LiveViewDeclarationIdentity,
    LiveViewTargetBinding,
    LiveViewStateCompatibility,
    LiveViewReadPosture,
    LiveViewWritePosture,
    LiveViewEffectIntentAdmission,
    LiveViewProjectionConsumption,
    LiveViewControlProjectionKind,
    LiveViewControlOptionSource,
    LiveViewControlCompatibility,
    LiveViewConditionalExpression,
    LiveViewConditionalParticipation,
    LiveViewRetainedStatePosture,
    LiveViewExpressionOperator,
    LiveViewExpressionArity,
    LiveViewExpressionValueKind,
    LiveViewExpressionDependencyContract,
    LiveViewRequirednessDeclaration,
    LiveViewValuePresence,
    LiveViewReadinessPosture,
    LiveViewInteractionIntentKind,
    LiveViewInteractionEffect,
    LiveViewPayloadShape,
    CompositionNodeKind,
    CompositionParentEdge,
    CompositionSiblingOrder,
    CompositionParticipation,
    CompositionMountedTopology,
    CompositionAccessPlan,
    CompositionChildLookup,
    CompositionAncestorLookup,
    CompositionParticipationFilter,
    CompositionAffectedConsumerLookup,
    CompositionContextPropagation,
    CompositionContextOverrideEligibility,
    CompositionContextDisabledSuppression,
    CompositionContextValidationParticipation,
    CompositionContextDependencyContract,
    CompositionAccessibilityRole,
    CompositionAccessibilityName,
    CompositionAccessibilityDescription,
    CompositionAccessibilityAssociation,
    CompositionFocusScope,
    CompositionFocusOrder,
    CompositionVisibilityPosture,
}

pub type WorthUiQueryGraphCanonicalObligationKind =
    forge_query::facade::runtime::ForgeQueryGraphObligationKind;

impl WorthUiQueryGraphObligationSemantic {
    pub const PRIMITIVE_CONSTRUCTION: [Self; 4] = [
        Self::SchemaAdmission,
        Self::CapabilitySupport,
        Self::OperatingContext,
        Self::DependencyContract,
    ];

    pub const MOUNTED_INTERACTION_ACTIVATION: [Self; 8] = [
        Self::SchemaAdmission,
        Self::CapabilitySupport,
        Self::ActivationEligibility,
        Self::CommandSupport,
        Self::InteractionContainment,
        Self::InteractionFocusability,
        Self::OperatingContext,
        Self::DependencyContract,
    ];

    pub const PRIMITIVE_EVENT_DISPATCH: [Self; 6] = [
        Self::EventRegionContract,
        Self::EventContainment,
        Self::EventDisabledBlock,
        Self::EventCapturePolicy,
        Self::EventCursorPosture,
        Self::EventPropagation,
    ];

    pub const PRIMITIVE_CONTENT_ANATOMY: [Self; 6] = [
        Self::ContentSchemaAdmission,
        Self::ContentIconCapability,
        Self::ContentVectorPosture,
        Self::ContentAccessibilityParticipation,
        Self::ContentSlotParticipation,
        Self::ContentDependencyContract,
    ];

    pub const USER_INTENT_TARGET_BINDING: [Self; 5] = [
        Self::TargetMountedTopology,
        Self::TargetArtifactBasis,
        Self::TargetComponentIdentity,
        Self::TargetOperationFamily,
        Self::TargetBindingPosture,
    ];

    pub const LIVE_VIEW_STATE_BINDING: [Self; 7] = [
        Self::LiveViewDeclarationIdentity,
        Self::LiveViewTargetBinding,
        Self::LiveViewStateCompatibility,
        Self::LiveViewReadPosture,
        Self::LiveViewWritePosture,
        Self::LiveViewEffectIntentAdmission,
        Self::LiveViewProjectionConsumption,
    ];

    pub const LIVE_VIEW_CONTROL_PROJECTION: [Self; 4] = [
        Self::LiveViewProjectionConsumption,
        Self::LiveViewControlProjectionKind,
        Self::LiveViewControlOptionSource,
        Self::LiveViewControlCompatibility,
    ];

    pub const LIVE_VIEW_CONDITIONAL_PROJECTION: [Self; 4] = [
        Self::LiveViewProjectionConsumption,
        Self::LiveViewConditionalExpression,
        Self::LiveViewConditionalParticipation,
        Self::LiveViewRetainedStatePosture,
    ];

    pub const LIVE_VIEW_READINESS_PROJECTION: [Self; 5] = [
        Self::LiveViewProjectionConsumption,
        Self::LiveViewRequirednessDeclaration,
        Self::LiveViewValuePresence,
        Self::LiveViewReadinessPosture,
        Self::LiveViewTargetBinding,
    ];

    pub const LIVE_VIEW_EXPRESSION_PROJECTION: [Self; 5] = [
        Self::LiveViewProjectionConsumption,
        Self::LiveViewExpressionOperator,
        Self::LiveViewExpressionArity,
        Self::LiveViewExpressionValueKind,
        Self::LiveViewExpressionDependencyContract,
    ];

    pub const LIVE_VIEW_INTERACTION_INTENT: [Self; 5] = [
        Self::LiveViewProjectionConsumption,
        Self::LiveViewInteractionIntentKind,
        Self::LiveViewInteractionEffect,
        Self::LiveViewReadinessPosture,
        Self::LiveViewPayloadShape,
    ];

    pub const LIVE_VIEW_PAYLOAD_PROJECTION: [Self; 3] = [
        Self::LiveViewProjectionConsumption,
        Self::LiveViewPayloadShape,
        Self::LiveViewStateCompatibility,
    ];

    pub const COMPOSITION_TOPOLOGY: [Self; 5] = [
        Self::CompositionNodeKind,
        Self::CompositionParentEdge,
        Self::CompositionSiblingOrder,
        Self::CompositionParticipation,
        Self::CompositionMountedTopology,
    ];

    pub const COMPOSITION_GRAPH_ACCESS: [Self; 5] = [
        Self::CompositionAccessPlan,
        Self::CompositionChildLookup,
        Self::CompositionAncestorLookup,
        Self::CompositionParticipationFilter,
        Self::CompositionAffectedConsumerLookup,
    ];

    pub const COMPOSITION_CONTEXT: [Self; 5] = [
        Self::CompositionContextPropagation,
        Self::CompositionContextOverrideEligibility,
        Self::CompositionContextDisabledSuppression,
        Self::CompositionContextValidationParticipation,
        Self::CompositionContextDependencyContract,
    ];

    pub const COMPOSITION_PARTICIPATION: [Self; 7] = [
        Self::CompositionAccessibilityRole,
        Self::CompositionAccessibilityName,
        Self::CompositionAccessibilityDescription,
        Self::CompositionAccessibilityAssociation,
        Self::CompositionFocusScope,
        Self::CompositionFocusOrder,
        Self::CompositionVisibilityPosture,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::SchemaAdmission => "worth.ui.schema-admission",
            Self::CapabilitySupport => "worth.ui.capability-support",
            Self::ActivationEligibility => "worth.ui.activation-eligibility",
            Self::CommandSupport => "worth.ui.command-support",
            Self::InteractionContainment => "worth.ui.interaction-containment",
            Self::InteractionFocusability => "worth.ui.interaction-focusability",
            Self::OperatingContext => "worth.ui.operating-context",
            Self::DependencyContract => "worth.ui.dependency-contract",
            Self::EventRegionContract => "worth.ui.event-region-contract",
            Self::EventContainment => "worth.ui.event-containment",
            Self::EventDisabledBlock => "worth.ui.event-disabled-block",
            Self::EventCapturePolicy => "worth.ui.event-capture-policy",
            Self::EventCursorPosture => "worth.ui.event-cursor-posture",
            Self::EventPropagation => "worth.ui.event-propagation",
            Self::ContentSchemaAdmission => "worth.ui.content-schema-admission",
            Self::ContentIconCapability => "worth.ui.content-icon-capability",
            Self::ContentVectorPosture => "worth.ui.content-vector-posture",
            Self::ContentAccessibilityParticipation => {
                "worth.ui.content-accessibility-participation"
            }
            Self::ContentSlotParticipation => "worth.ui.content-slot-participation",
            Self::ContentDependencyContract => "worth.ui.content-dependency-contract",
            Self::TargetMountedTopology => "worth.ui.target-mounted-topology",
            Self::TargetArtifactBasis => "worth.ui.target-artifact-basis",
            Self::TargetComponentIdentity => "worth.ui.target-component-identity",
            Self::TargetOperationFamily => "worth.ui.target-operation-family",
            Self::TargetBindingPosture => "worth.ui.target-binding-posture",
            Self::LiveViewDeclarationIdentity => "worth.ui.live-view-declaration-identity",
            Self::LiveViewTargetBinding => "worth.ui.live-view-target-binding",
            Self::LiveViewStateCompatibility => "worth.ui.live-view-state-compatibility",
            Self::LiveViewReadPosture => "worth.ui.live-view-read-posture",
            Self::LiveViewWritePosture => "worth.ui.live-view-write-posture",
            Self::LiveViewEffectIntentAdmission => "worth.ui.live-view-effect-intent-admission",
            Self::LiveViewProjectionConsumption => "worth.ui.live-view-projection-consumption",
            Self::LiveViewControlProjectionKind => "worth.ui.live-view-control-projection-kind",
            Self::LiveViewControlOptionSource => "worth.ui.live-view-control-option-source",
            Self::LiveViewControlCompatibility => "worth.ui.live-view-control-compatibility",
            Self::LiveViewConditionalExpression => "worth.ui.live-view-conditional-expression",
            Self::LiveViewConditionalParticipation => {
                "worth.ui.live-view-conditional-participation"
            }
            Self::LiveViewRetainedStatePosture => "worth.ui.live-view-retained-state-posture",
            Self::LiveViewExpressionOperator => "worth.ui.live-view-expression-operator",
            Self::LiveViewExpressionArity => "worth.ui.live-view-expression-arity",
            Self::LiveViewExpressionValueKind => "worth.ui.live-view-expression-value-kind",
            Self::LiveViewExpressionDependencyContract => {
                "worth.ui.live-view-expression-dependency-contract"
            }
            Self::LiveViewRequirednessDeclaration => "worth.ui.live-view-requiredness-declaration",
            Self::LiveViewValuePresence => "worth.ui.live-view-value-presence",
            Self::LiveViewReadinessPosture => "worth.ui.live-view-readiness-posture",
            Self::LiveViewInteractionIntentKind => "worth.ui.live-view-interaction-intent-kind",
            Self::LiveViewInteractionEffect => "worth.ui.live-view-interaction-effect",
            Self::LiveViewPayloadShape => "worth.ui.live-view-payload-shape",
            Self::CompositionNodeKind => "worth.ui.composition-node-kind",
            Self::CompositionParentEdge => "worth.ui.composition-parent-edge",
            Self::CompositionSiblingOrder => "worth.ui.composition-sibling-order",
            Self::CompositionParticipation => "worth.ui.composition-participation",
            Self::CompositionMountedTopology => "worth.ui.composition-mounted-topology",
            Self::CompositionAccessPlan => "worth.ui.composition-access-plan",
            Self::CompositionChildLookup => "worth.ui.composition-child-lookup",
            Self::CompositionAncestorLookup => "worth.ui.composition-ancestor-lookup",
            Self::CompositionParticipationFilter => "worth.ui.composition-participation-filter",
            Self::CompositionAffectedConsumerLookup => {
                "worth.ui.composition-affected-consumer-lookup"
            }
            Self::CompositionContextPropagation => "worth.ui.composition-context-propagation",
            Self::CompositionContextOverrideEligibility => {
                "worth.ui.composition-context-override-eligibility"
            }
            Self::CompositionContextDisabledSuppression => {
                "worth.ui.composition-context-disabled-suppression"
            }
            Self::CompositionContextValidationParticipation => {
                "worth.ui.composition-context-validation-participation"
            }
            Self::CompositionContextDependencyContract => {
                "worth.ui.composition-context-dependency-contract"
            }
            Self::CompositionAccessibilityRole => "worth.ui.composition-accessibility-role",
            Self::CompositionAccessibilityName => "worth.ui.composition-accessibility-name",
            Self::CompositionAccessibilityDescription => {
                "worth.ui.composition-accessibility-description"
            }
            Self::CompositionAccessibilityAssociation => {
                "worth.ui.composition-accessibility-association"
            }
            Self::CompositionFocusScope => "worth.ui.composition-focus-scope",
            Self::CompositionFocusOrder => "worth.ui.composition-focus-order",
            Self::CompositionVisibilityPosture => "worth.ui.composition-visibility-posture",
        }
    }

    pub fn rule_name(self) -> &'static str {
        match self {
            Self::SchemaAdmission => "schema-admission",
            Self::CapabilitySupport => "capability-support",
            Self::ActivationEligibility => "activation-eligibility",
            Self::CommandSupport => "command-support",
            Self::InteractionContainment => "interaction-containment",
            Self::InteractionFocusability => "interaction-focusability",
            Self::OperatingContext => "operating-context",
            Self::DependencyContract => "dependency-contract",
            Self::EventRegionContract => "primitive-event-region-contract",
            Self::EventContainment => "primitive-event-containment",
            Self::EventDisabledBlock => "primitive-event-disabled-block",
            Self::EventCapturePolicy => "primitive-event-capture-policy",
            Self::EventCursorPosture => "primitive-event-cursor-posture",
            Self::EventPropagation => "primitive-event-propagation",
            Self::ContentSchemaAdmission => "primitive-content-schema-admission",
            Self::ContentIconCapability => "primitive-content-icon-capability",
            Self::ContentVectorPosture => "primitive-content-vector-posture",
            Self::ContentAccessibilityParticipation => "primitive-content-accessibility",
            Self::ContentSlotParticipation => "primitive-content-slot-participation",
            Self::ContentDependencyContract => "primitive-content-dependency-contract",
            Self::TargetMountedTopology => "target-mounted-topology",
            Self::TargetArtifactBasis => "target-artifact-basis",
            Self::TargetComponentIdentity => "target-component-identity",
            Self::TargetOperationFamily => "target-operation-family",
            Self::TargetBindingPosture => "target-binding-posture",
            Self::LiveViewDeclarationIdentity => "live-view-declaration-identity",
            Self::LiveViewTargetBinding => "live-view-target-binding",
            Self::LiveViewStateCompatibility => "live-view-state-compatibility",
            Self::LiveViewReadPosture => "live-view-read-posture",
            Self::LiveViewWritePosture => "live-view-write-posture",
            Self::LiveViewEffectIntentAdmission => "live-view-effect-intent-admission",
            Self::LiveViewProjectionConsumption => "live-view-projection-consumption",
            Self::LiveViewControlProjectionKind => "live-view-control-projection-kind",
            Self::LiveViewControlOptionSource => "live-view-control-option-source",
            Self::LiveViewControlCompatibility => "live-view-control-compatibility",
            Self::LiveViewConditionalExpression => "live-view-conditional-expression",
            Self::LiveViewConditionalParticipation => "live-view-conditional-participation",
            Self::LiveViewRetainedStatePosture => "live-view-retained-state-posture",
            Self::LiveViewExpressionOperator => "live-view-expression-operator",
            Self::LiveViewExpressionArity => "live-view-expression-arity",
            Self::LiveViewExpressionValueKind => "live-view-expression-value-kind",
            Self::LiveViewExpressionDependencyContract => {
                "live-view-expression-dependency-contract"
            }
            Self::LiveViewRequirednessDeclaration => "live-view-requiredness-declaration",
            Self::LiveViewValuePresence => "live-view-value-presence",
            Self::LiveViewReadinessPosture => "live-view-readiness-posture",
            Self::LiveViewInteractionIntentKind => "live-view-interaction-intent-kind",
            Self::LiveViewInteractionEffect => "live-view-interaction-effect",
            Self::LiveViewPayloadShape => "live-view-payload-shape",
            Self::CompositionNodeKind => "composition-node-kind",
            Self::CompositionParentEdge => "composition-parent-edge",
            Self::CompositionSiblingOrder => "composition-sibling-order",
            Self::CompositionParticipation => "composition-participation",
            Self::CompositionMountedTopology => "composition-mounted-topology",
            Self::CompositionAccessPlan => "composition-access-plan",
            Self::CompositionChildLookup => "composition-child-lookup",
            Self::CompositionAncestorLookup => "composition-ancestor-lookup",
            Self::CompositionParticipationFilter => "composition-participation-filter",
            Self::CompositionAffectedConsumerLookup => "composition-affected-consumer-lookup",
            Self::CompositionContextPropagation => "composition-context-propagation",
            Self::CompositionContextOverrideEligibility => {
                "composition-context-override-eligibility"
            }
            Self::CompositionContextDisabledSuppression => {
                "composition-context-disabled-suppression"
            }
            Self::CompositionContextValidationParticipation => {
                "composition-context-validation-participation"
            }
            Self::CompositionContextDependencyContract => "composition-context-dependency-contract",
            Self::CompositionAccessibilityRole => "composition-accessibility-role",
            Self::CompositionAccessibilityName => "composition-accessibility-name",
            Self::CompositionAccessibilityDescription => "composition-accessibility-description",
            Self::CompositionAccessibilityAssociation => "composition-accessibility-association",
            Self::CompositionFocusScope => "composition-focus-scope",
            Self::CompositionFocusOrder => "composition-focus-order",
            Self::CompositionVisibilityPosture => "composition-visibility-posture",
        }
    }
}
