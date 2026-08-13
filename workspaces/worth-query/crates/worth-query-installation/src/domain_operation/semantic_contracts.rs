use super::{
    WorthQueryOperationDecisionFactContract, WorthQueryOperationNativeProjectionContract,
    WorthQueryOperationWorkflowContract,
};
use worth_foundational::facade::{AspectIdentity, AspectKey};
use worth_query_declaration::facade::canonicalization::CanonicalQueryBundle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainOperationSemanticClosure {
    pub parameters: WorthQueryOperationParameterContract,
    pub native_projection: WorthQueryOperationNativeProjectionContract,
    pub canonical_query: CanonicalQueryBundle,
    pub collection: WorthQueryOperationCollectionContract,
    pub required_capabilities: Vec<WorthQueryOperationCapabilityRequirement>,
    pub required_domains: Vec<WorthQueryOperationRequiredDomainRole>,
    pub workflow: WorthQueryOperationWorkflowContract,
    pub evidence: super::WorthQueryDomainEvidenceContract,
    pub conditional_nodes: Vec<super::WorthQueryPortableConditionalNodeDeclaration>,
    pub graph_reads: WorthQueryOperationGraphReadContract,
    pub decision_facts: WorthQueryOperationDecisionFactContract,
    pub touches: WorthQueryOperationTouchContract,
    pub effects: WorthQueryOperationEffectContract,
    pub invariants: WorthQueryOperationInvariantContract,
    pub invariant_execution: super::WorthQueryInvariantExecutionContract,
    pub replay: WorthQueryOperationReplayContract,
    /// Installed aftermath classification for mutation operations.
    ///
    /// `None` means the operation carries no aftermath contract (not a
    /// mutation, or provisional-only work). Absence is not a posture variant.
    pub aftermath: Option<crate::application_aftermath::WorthQueryInstalledAftermathContract>,
    pub lineage: WorthQueryOperationLineageContract,
    pub promotion: WorthQueryOperationPromotionContract,
    pub publication: WorthQueryOperationPublicationContract,
    pub projection_consumption: WorthQueryOperationProjectionConsumptionContract,
    pub terminal: WorthQueryOperationTerminalContract,
    pub cost: WorthQueryOperationCostContract,
    pub resources: crate::domain_computation::WorthQueryOperationExecutionResourceContract,
    pub support: WorthQueryOperationSupportRequirements,
    pub lowering: WorthQueryOperationLoweringContract,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryOperationCapabilityRequirement {
    QueryRead,
    QueryComposition,
    QueryContext,
    IdentityEvolution,
    LiveQuery,
    PreviewSession,
    WorkflowOrchestration,
    HistoricalEvaluation,
    DurableArtifacts,
}

impl WorthQueryOperationCapabilityRequirement {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueryRead => "query_read",
            Self::QueryComposition => "query_composition",
            Self::QueryContext => "query_context",
            Self::IdentityEvolution => "identity_evolution",
            Self::LiveQuery => "live_query",
            Self::PreviewSession => "preview_session",
            Self::WorkflowOrchestration => "workflow_orchestration",
            Self::HistoricalEvaluation => "historical_evaluation",
            Self::DurableArtifacts => "durable_artifacts",
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryOperationRequiredDomainRole(String);

impl WorthQueryOperationRequiredDomainRole {
    pub fn new(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("empty-required-domain-role");
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationParameterContract {
    NotRequired,
    Declared {
        fields: Vec<WorthQueryOperationParameterField>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthQueryOperationParameterField {
    pub name: String,
    pub value_family: WorthQueryOperationValueFamily,
    pub required: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryOperationValueFamily {
    Bool,
    I64,
    U64,
    Text,
    EntityIdentity,
    NativeAspect {
        key: AspectKey,
        identity: AspectIdentity,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationCollectionContract {
    NotCollection,
    Collection {
        row_identity_field: WorthQueryOperationCollectionField,
        ordering_fields: Vec<WorthQueryOperationCollectionField>,
        grouping: WorthQueryOperationGroupingContract,
        window: WorthQueryOperationWindowPolicy,
        continuation: WorthQueryOperationContinuationPosture,
    },
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryOperationCollectionField {
    aspect_key: AspectKey,
    field_path: worth_foundational::facade::CanonicalFieldPath,
}

impl WorthQueryOperationCollectionField {
    pub fn new(
        aspect_key: AspectKey,
        field_path: worth_foundational::facade::CanonicalFieldPath,
    ) -> Self {
        Self {
            aspect_key,
            field_path,
        }
    }

    pub fn from_dotted(value: &str) -> Option<Self> {
        let mut parts = value.split('.');
        let aspect_key = AspectKey::new(parts.next()?.to_owned())?;
        let fields = parts
            .map(|part| worth_foundational::facade::FieldKey::new(part.to_owned()))
            .collect::<Option<Vec<_>>>()?;
        let field_path = worth_foundational::facade::CanonicalFieldPath::new(fields)?;
        Some(Self::new(aspect_key, field_path))
    }

    pub fn aspect_key(&self) -> &AspectKey {
        &self.aspect_key
    }

    pub fn field_path(&self) -> &worth_foundational::facade::CanonicalFieldPath {
        &self.field_path
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationGroupingContract {
    Ungrouped,
    Grouped {
        grouping_fields: Vec<WorthQueryOperationCollectionField>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationWindowPolicy {
    CompleteCollection,
    ContinuationBounded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationContinuationPosture {
    NotRequired,
    SnapshotCursor,
    LiveCursor,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationGraphReadContract {
    NotRequired,
    Declared {
        roles: Vec<WorthQueryOperationGraphReadRole>,
    },
}

impl WorthQueryOperationGraphReadContract {
    pub fn roles(&self) -> &[WorthQueryOperationGraphReadRole] {
        match self {
            Self::NotRequired => &[],
            Self::Declared { roles } => roles,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOperationGraphReadRole {
    pub role: String,
    pub participation: WorthQueryOperationGraphParticipation,
    pub access: WorthQueryOperationGraphAccess,
    pub semantic_reads: Vec<WorthQueryOperationNativeProjectionContract>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryOperationGraphParticipation {
    PrimaryLogicalGraph,
    SeparateAuthority { role: String },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryOperationGraphAccess {
    Observe,
    Project,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationTouchContract {
    NotRequired,
    Declared {
        graph_roles: Vec<String>,
        scopes: Vec<String>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationEffectContract {
    NotRequired,
    Declared {
        effect_families: Vec<WorthQueryOperationEffectFamily>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryOperationEffectFamily {
    Mutation,
    Merge,
    Writeback,
}

impl WorthQueryOperationEffectFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mutation => "mutation",
            Self::Merge => "merge",
            Self::Writeback => "writeback",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationInvariantContract {
    NotRequired,
    Declared { invariant_slots: Vec<String> },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationLineageContract {
    NotRequired,
    Preserve,
    Evolve,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationPromotionContract {
    NotRequired,
    OnDurableReference,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationPublicationContract {
    NotRequired,
    DerivedProjection {
        projection_role: WorthQueryOperationProjectionRole,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthQueryOperationProjectionRole(String);

impl WorthQueryOperationProjectionRole {
    pub fn new(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("empty-operation-projection-role");
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationProjectionConsumptionContract {
    NotRequired,
    QueryReadAuthority,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOperationTerminalContract {
    pub result_states: Vec<WorthQueryOperationResultState>,
    pub failure_classes: Vec<WorthQueryOperationFailureClass>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryOperationResultState {
    Ready,
    Advisory,
    Pending,
    Partial,
    Violation,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryOperationFailureClass {
    InvalidInput,
    Unsupported,
    Conflict,
    Dependency,
    Indeterminate,
    Domain(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryOperationCostContract {
    pub lookup: WorthQueryOperationCostClass,
    pub execution: WorthQueryOperationCostClass,
    pub result_width: WorthQueryOperationCostClass,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationCostClass {
    Constant,
    DeclaredWidth,
    GraphBreadth,
    ExternalBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryOperationSupportRequirements {
    pub live: WorthQuerySupportRequirement,
    pub continuation: WorthQuerySupportRequirement,
    pub async_result_state: WorthQuerySupportRequirement,
    pub recovery: WorthQuerySupportRequirement,
    pub inspection: WorthQuerySupportRequirement,
    pub projection_consumption: WorthQuerySupportRequirement,
    pub dependency_impact: WorthQuerySupportRequirement,
    pub sharing: WorthQuerySupportRequirement,
    pub invalidation: WorthQuerySupportRequirement,
    pub collection_delivery: WorthQuerySupportRequirement,
    pub conditional_evaluation: WorthQuerySupportRequirement,
    pub conditional_comparator: WorthQuerySupportRequirement,
    pub conditional_trigger: WorthQuerySupportRequirement,
    pub conditional_temporal_or_on_demand: WorthQuerySupportRequirement,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQuerySupportRequirement {
    NotRequired,
    Required,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOperationLoweringContract {
    pub family: String,
    pub deterministic: bool,
}
use super::replay_contract::WorthQueryOperationReplayContract;
