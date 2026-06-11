use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRuntimeMissingComponent {
    Backend,
    RuntimeBridge,
    SchemaAdapter,
    SourceAdapter,
    WriteAuthority,
    SignalSink,
    SubscriptionActivation,
    PreviewBasis,
    InspectorEvidence,
    IntentAuthority,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRuntimeLookupFailureKind {
    UnknownProgram,
    UnknownOperation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRuntimeMissingArtifactKind {
    LiveView,
    LiveSubscription,
    DerivedView,
    Effect,
    PendingWriteIntent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRuntimeDeclarationFailureKind {
    RetainedRowDecode,
    ComputedDeclaration,
    EffectDeclaration,
    LiveSubscriptionInstallation,
    InvariantRegistration,
    PreviewOperationEffectDenied,
}

#[derive(Clone, Copy, Debug)]
pub enum ForgeQueryStopClass<'a> {
    MissingRuntimeComponent {
        component: ForgeQueryRuntimeMissingComponent,
    },
    ExistingTruthAssertionDenied {
        denial: &'a ForgeQueryExistingTruthAssertionDenial,
    },
    ExistingTruthProbeDenied {
        denial: &'a ForgeQueryExistingTruthProbeDenial,
    },
    MutationBindingDenied {
        denial: &'a ForgeQueryExistingTruthBindingDenial,
    },
    MutationContinuityDenied {
        denial: &'a ForgeQueryContinuityMutationDenial,
    },
    GraphCompositionDenied {
        denial: &'a ForgeQueryGraphCompositionDenial,
    },
    GraphCompositionDomainInvariantDenied {
        denial: &'a ForgeQueryGraphCompositionDomainInvariantDenial,
    },
    MutationNamingDenied {
        denial: &'a ForgeQueryNamingMutationDenial,
    },
    MutationTargetReferenceDenied {
        denial: &'a ForgeQuerySymbolicTargetReferenceDenial,
    },
    ReadCompositionDenied {
        denial: &'a ForgeQueryReadDenial,
    },
    ReadCompositionDomainInvariantDenied {
        denial: &'a ForgeQueryReadDomainInvariantDenial,
    },
    Workspace {
        error: &'a ForgeQueryWorkspaceError,
    },
    Program {
        error: &'a ForgeQueryProgramError,
    },
    RuntimeLookupFailed {
        kind: ForgeQueryRuntimeLookupFailureKind,
        program_id: &'a str,
        operation_id: Option<&'a str>,
    },
    MissingRuntimeArtifact {
        kind: ForgeQueryRuntimeMissingArtifactKind,
        name: &'a str,
    },
    RuntimeDeclarationFailed {
        kind: ForgeQueryRuntimeDeclarationFailureKind,
        name: &'a str,
        stage: &'static str,
        message: &'a str,
    },
    SessionLabelCollision {
        authority_lane: ForgeQueryAuthorityLane,
        label: &'a ForgeQuerySessionLabel,
    },
    UnsupportedAuthority {
        authority: &'a str,
    },
    IntentCommitDenied {
        intent_name: &'a str,
        stage: &'static str,
        message: &'a str,
        evidence: &'a ForgeQueryIntentDenialEvidence,
    },
    IntentExecutionRoutingFailed {
        intent_name: &'a str,
        stage: &'static str,
        message: &'a str,
        evidence: &'a ForgeQueryIntentExecutionFailureEvidence,
        source: &'a ForgeQueryRuntimeError,
    },
    EffectPolicyDenied {
        denial: ForgeQueryEffectPolicyDenial,
    },
    PreviewPromotionDenied {
        kind: ForgeQueryPreviewPromotionDenialKind,
        evidence: &'a ForgeQueryPreviewPromotionDenialEvidence,
    },
    FamilyAdmissionDenied {
        family: ForgeQueryRuntimeFacadeFamily,
        status: ForgeQueryRuntimeFamilySupportStatus,
        teaching_posture: Option<ForgeQueryRuntimeFamilyTeachingPosture>,
        reason: &'a str,
    },
}
