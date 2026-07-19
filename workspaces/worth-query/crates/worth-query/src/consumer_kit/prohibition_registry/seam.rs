#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryProhibitedSeam {
    WorkspaceDirectWrite,
    WorkspaceDirectBatch,
    WorkspaceExistingTruthBindEntity,
    WorkspaceExistingTruthBindRelation,
    WorkspaceExistingTruthProbe,
    WorkspaceExistingTruthUpdate,
    WorkspaceExistingTruthAssert,
    WorkspaceExistingTruthVerify,
    WorkspaceExistingTruthUpdateVerified,
    WorkspaceExistingTruthDelete,
    WorkspaceExistingTruthDeleteWith,
    WorkspaceExistingTruthDeleteVerified,
    RawDigestMinting,
    RawBasisIdentity,
    UnscopedQueryContext,
    RawIntentAdmissionRequest,
    PostureAuthoredSubscription,
    ReceiptOnlyCausalInspection,
    LegacyPreviewExecution,
    DeepFacadeToolingImport,
    LegacyQueryBasisLifecycle,
    CrateRootPhaseMirror,
    DeepPhaseModuleImport,
    OrdinaryFacadePhaseReexport,
    PhaseArtifactAlias,
    GenericPhaseConversion,
}

impl WorthQueryProhibitedSeam {
    pub fn key(self) -> &'static str {
        match self {
            Self::WorkspaceDirectWrite => "workspace.direct-write",
            Self::WorkspaceDirectBatch => "workspace.direct-batch",
            Self::WorkspaceExistingTruthBindEntity => "workspace.existing-truth.bind-entity",
            Self::WorkspaceExistingTruthBindRelation => "workspace.existing-truth.bind-relation",
            Self::WorkspaceExistingTruthProbe => "workspace.existing-truth.probe",
            Self::WorkspaceExistingTruthUpdate => "workspace.existing-truth.update",
            Self::WorkspaceExistingTruthAssert => "workspace.existing-truth.assert",
            Self::WorkspaceExistingTruthVerify => "workspace.existing-truth.verify",
            Self::WorkspaceExistingTruthUpdateVerified => {
                "workspace.existing-truth.update-verified"
            }
            Self::WorkspaceExistingTruthDelete => "workspace.existing-truth.delete",
            Self::WorkspaceExistingTruthDeleteWith => "workspace.existing-truth.delete-with",
            Self::WorkspaceExistingTruthDeleteVerified => {
                "workspace.existing-truth.delete-verified"
            }
            Self::RawDigestMinting => "query.raw-digest-minting",
            Self::RawBasisIdentity => "query.raw-basis-identity",
            Self::UnscopedQueryContext => "query.unscoped-context",
            Self::RawIntentAdmissionRequest => "query.raw-intent-admission-request",
            Self::PostureAuthoredSubscription => "query.posture-authored-subscription",
            Self::ReceiptOnlyCausalInspection => "query.receipt-only-causal-inspection",
            Self::LegacyPreviewExecution => "query.legacy-preview-execution",
            Self::DeepFacadeToolingImport => "query.deep-facade-tooling-import",
            Self::LegacyQueryBasisLifecycle => "query.legacy-basis-lifecycle",
            Self::CrateRootPhaseMirror => "query.phase-api.crate-root-mirror",
            Self::DeepPhaseModuleImport => "query.phase-api.deep-module-import",
            Self::OrdinaryFacadePhaseReexport => "query.phase-api.ordinary-facade-reexport",
            Self::PhaseArtifactAlias => "query.phase-api.alias",
            Self::GenericPhaseConversion => "query.phase-api.generic-conversion",
        }
    }

    pub fn public_symbol(self) -> &'static str {
        match self {
            Self::WorkspaceDirectWrite => "WorthQueryWorkspace::write",
            Self::WorkspaceDirectBatch => "WorthQueryWorkspace::batch",
            Self::WorkspaceExistingTruthBindEntity => "WorthQueryWorkspace::bind_existing_entity",
            Self::WorkspaceExistingTruthBindRelation => {
                "WorthQueryWorkspace::bind_existing_relation"
            }
            Self::WorkspaceExistingTruthProbe => "WorthQueryWorkspace::probe_existing",
            Self::WorkspaceExistingTruthUpdate => "WorthQueryWorkspace::update_existing",
            Self::WorkspaceExistingTruthAssert => "WorthQueryWorkspace::assert_existing",
            Self::WorkspaceExistingTruthVerify => "WorthQueryWorkspace::verify_existing",
            Self::WorkspaceExistingTruthUpdateVerified => {
                "WorthQueryWorkspace::update_existing_verified"
            }
            Self::WorkspaceExistingTruthDelete => "WorthQueryWorkspace::delete_existing",
            Self::WorkspaceExistingTruthDeleteWith => "WorthQueryWorkspace::delete_existing_with",
            Self::WorkspaceExistingTruthDeleteVerified => {
                "WorthQueryWorkspace::delete_existing_verified"
            }
            Self::RawDigestMinting => "WorthQueryDigest::from_domain_parts",
            Self::RawBasisIdentity => "RawBasisIntent",
            Self::UnscopedQueryContext => "bind_query_basis_context",
            Self::RawIntentAdmissionRequest => "WorthQueryRawIntentAdmissionRequest",
            Self::PostureAuthoredSubscription => "QuerySubscriptionBasisPosture",
            Self::ReceiptOnlyCausalInspection => "CausalInspection::for_observation(receipt)",
            Self::LegacyPreviewExecution => "PreviewSessionPlanBinding",
            Self::DeepFacadeToolingImport => {
                "facade::certification tooling through ordinary facade"
            }
            Self::LegacyQueryBasisLifecycle => "query_basis_lifecycle",
            Self::CrateRootPhaseMirror => "worth_query::WorthQueryPreparedContinuation",
            Self::DeepPhaseModuleImport => "worth_query::planning::plan_validated_bundle",
            Self::OrdinaryFacadePhaseReexport => "worth_query::facade::read::plan_validated_bundle",
            Self::PhaseArtifactAlias => {
                "worth_query::WorthQueryPreparedContinuation as PreparedContinuation"
            }
            Self::GenericPhaseConversion => "WorthQueryReadRequest: Into<ExecutionPlanBundle>",
        }
    }
}
