use crate::application::config::{
    ForgeQueryConfigSectionFamily, ForgeQuerySubsystemOwner, ValidatedForgeQueryConfig,
};
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryCapabilityFamily {
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

impl ForgeQueryCapabilityFamily {
    pub fn as_str(&self) -> &'static str {
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

    pub fn config_section(&self) -> ForgeQueryConfigSectionFamily {
        match self {
            Self::QueryRead
            | Self::QueryComposition
            | Self::QueryContext
            | Self::IdentityEvolution => ForgeQueryConfigSectionFamily::Query,
            Self::LiveQuery => ForgeQueryConfigSectionFamily::Signal,
            Self::PreviewSession => ForgeQueryConfigSectionFamily::RuntimeBridge,
            Self::WorkflowOrchestration | Self::HistoricalEvaluation => {
                ForgeQueryConfigSectionFamily::Relational
            }
            Self::DurableArtifacts => ForgeQueryConfigSectionFamily::Store,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryCapabilityStatus {
    Admitted,
    DeferredDebt,
    Unsupported,
}

pub type ForgeQueryCapabilitySupportStatus = ForgeQueryCapabilityStatus;

impl ForgeQueryCapabilityStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::DeferredDebt => "deferred_debt",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryCapabilityDescriptor {
    family: ForgeQueryCapabilityFamily,
    status: ForgeQueryCapabilityStatus,
    owner: ForgeQuerySubsystemOwner,
    config_section: ForgeQueryConfigSectionFamily,
    reason: &'static str,
}

impl ForgeQueryCapabilityDescriptor {
    pub(crate) fn new(
        family: ForgeQueryCapabilityFamily,
        status: ForgeQueryCapabilityStatus,
        owner: ForgeQuerySubsystemOwner,
        reason: &'static str,
    ) -> Self {
        Self {
            family,
            status,
            owner,
            config_section: family.config_section(),
            reason,
        }
    }

    pub fn family(&self) -> ForgeQueryCapabilityFamily {
        self.family
    }

    pub fn status(&self) -> ForgeQueryCapabilityStatus {
        self.status
    }

    pub fn owner(&self) -> ForgeQuerySubsystemOwner {
        self.owner
    }

    pub fn config_section(&self) -> ForgeQueryConfigSectionFamily {
        self.config_section
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryCapabilityRegistry {
    descriptors: Vec<ForgeQueryCapabilityDescriptor>,
    registry_digest: String,
}

impl ForgeQueryCapabilityRegistry {
    pub(crate) fn from_validated_config(config: &ValidatedForgeQueryConfig) -> Self {
        let descriptors = vec![
            ForgeQueryCapabilityDescriptor::new(
                ForgeQueryCapabilityFamily::QueryRead,
                if config.query().runtime_backed_reads_enabled() {
                    ForgeQueryCapabilityStatus::Admitted
                } else {
                    ForgeQueryCapabilityStatus::Unsupported
                },
                ForgeQuerySubsystemOwner::Query,
                if config.query().runtime_backed_reads_enabled() {
                    "runtime-backed query execution is admitted through the daily-driver facade"
                } else {
                    "runtime-backed query execution is disabled by the query config section"
                },
            ),
            ForgeQueryCapabilityDescriptor::new(
                ForgeQueryCapabilityFamily::QueryComposition,
                if config.query().runtime_backed_reads_enabled() {
                    ForgeQueryCapabilityStatus::Admitted
                } else {
                    ForgeQueryCapabilityStatus::Unsupported
                },
                ForgeQuerySubsystemOwner::Query,
                if config.query().runtime_backed_reads_enabled() {
                    "query composition is admitted through the query config section"
                } else {
                    "query composition is disabled by the query config section"
                },
            ),
            ForgeQueryCapabilityDescriptor::new(
                ForgeQueryCapabilityFamily::QueryContext,
                if config.query().runtime_backed_reads_enabled() {
                    ForgeQueryCapabilityStatus::Admitted
                } else {
                    ForgeQueryCapabilityStatus::Unsupported
                },
                ForgeQuerySubsystemOwner::Query,
                if config.query().runtime_backed_reads_enabled() {
                    "query context binding is admitted through the query config section"
                } else {
                    "query context binding is disabled by the query config section"
                },
            ),
            ForgeQueryCapabilityDescriptor::new(
                ForgeQueryCapabilityFamily::IdentityEvolution,
                if config.query().runtime_backed_reads_enabled() {
                    ForgeQueryCapabilityStatus::Admitted
                } else {
                    ForgeQueryCapabilityStatus::Unsupported
                },
                ForgeQuerySubsystemOwner::Query,
                if config.query().runtime_backed_reads_enabled() {
                    "identity evolution is admitted through the query config section"
                } else {
                    "identity evolution is disabled by the query config section"
                },
            ),
            ForgeQueryCapabilityDescriptor::new(
                ForgeQueryCapabilityFamily::LiveQuery,
                if config.query().runtime_backed_reads_enabled()
                    && config.signal().live_promotion_enabled()
                {
                    ForgeQueryCapabilityStatus::Admitted
                } else {
                    ForgeQueryCapabilityStatus::Unsupported
                },
                ForgeQuerySubsystemOwner::Signal,
                if config.signal().live_promotion_enabled() {
                    "live query promotion is admitted through the signal config section"
                } else {
                    "live query promotion is disabled by the signal config section"
                },
            ),
            ForgeQueryCapabilityDescriptor::new(
                ForgeQueryCapabilityFamily::PreviewSession,
                if config.query().runtime_backed_reads_enabled()
                    && config.runtime_bridge().preview_session_enabled()
                {
                    ForgeQueryCapabilityStatus::Admitted
                } else {
                    ForgeQueryCapabilityStatus::Unsupported
                },
                ForgeQuerySubsystemOwner::RuntimeBridge,
                if config.runtime_bridge().preview_session_enabled() {
                    "preview session composition is admitted through the runtime bridge config section"
                } else {
                    "preview session composition is disabled by the runtime bridge config section"
                },
            ),
            ForgeQueryCapabilityDescriptor::new(
                ForgeQueryCapabilityFamily::WorkflowOrchestration,
                if config.query().runtime_backed_reads_enabled()
                    && config.relational().workflow_orchestration_enabled()
                {
                    ForgeQueryCapabilityStatus::Admitted
                } else {
                    ForgeQueryCapabilityStatus::Unsupported
                },
                ForgeQuerySubsystemOwner::Relational,
                if config.relational().workflow_orchestration_enabled() {
                    "workflow orchestration is admitted through the relational config section"
                } else {
                    "workflow orchestration is disabled by the relational config section"
                },
            ),
            ForgeQueryCapabilityDescriptor::new(
                ForgeQueryCapabilityFamily::HistoricalEvaluation,
                if config.query().runtime_backed_reads_enabled()
                    && config.relational().historical_evaluation_enabled()
                {
                    ForgeQueryCapabilityStatus::Admitted
                } else {
                    ForgeQueryCapabilityStatus::Unsupported
                },
                ForgeQuerySubsystemOwner::Relational,
                if config.relational().historical_evaluation_enabled() {
                    "historical evaluation is admitted through the relational config section"
                } else {
                    "historical evaluation is disabled by the relational config section"
                },
            ),
            ForgeQueryCapabilityDescriptor::new(
                ForgeQueryCapabilityFamily::DurableArtifacts,
                ForgeQueryCapabilityStatus::DeferredDebt,
                ForgeQuerySubsystemOwner::Store,
                "durable artifacts remain store-gated debt until the later durability milestones close",
            ),
        ];

        let descriptor_identities = descriptors
            .iter()
            .map(capability_descriptor_identity)
            .collect::<Vec<_>>();
        let registry_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::ApplicationSupportReport)
                .field_shape(ForgeQueryEvidenceTag::new("role"), "capability-registry")
                .field_identity_sequence(
                    ForgeQueryEvidenceTag::new("descriptors"),
                    descriptor_identities.iter().map(String::as_str),
                )
                .seal()
                .as_str()
                .to_string();

        Self {
            descriptors,
            registry_digest,
        }
    }

    pub fn descriptors(&self) -> &[ForgeQueryCapabilityDescriptor] {
        &self.descriptors
    }

    pub fn descriptor(
        &self,
        family: ForgeQueryCapabilityFamily,
    ) -> Option<&ForgeQueryCapabilityDescriptor> {
        self.descriptors
            .iter()
            .find(|descriptor| descriptor.family == family)
    }

    pub fn registry_digest(&self) -> &str {
        &self.registry_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySupportMatrix {
    registry: ForgeQueryCapabilityRegistry,
    support_matrix_digest: String,
}

impl ForgeQuerySupportMatrix {
    pub(crate) fn new(registry: ForgeQueryCapabilityRegistry) -> Self {
        let admitted = registry
            .descriptors()
            .iter()
            .filter(|descriptor| descriptor.status() == ForgeQueryCapabilityStatus::Admitted)
            .count();
        let deferred = registry
            .descriptors()
            .iter()
            .filter(|descriptor| descriptor.status() == ForgeQueryCapabilityStatus::DeferredDebt)
            .count();
        let unsupported = registry
            .descriptors()
            .iter()
            .filter(|descriptor| descriptor.status() == ForgeQueryCapabilityStatus::Unsupported)
            .count();
        let support_matrix_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::ApplicationSupportReport)
                .field_shape(ForgeQueryEvidenceTag::new("role"), "support-matrix")
                .field_identity(
                    ForgeQueryEvidenceTag::new("registry"),
                    registry.registry_digest(),
                )
                .field_usize(ForgeQueryEvidenceTag::new("admitted"), admitted)
                .field_usize(ForgeQueryEvidenceTag::new("deferred"), deferred)
                .field_usize(ForgeQueryEvidenceTag::new("unsupported"), unsupported)
                .seal()
                .as_str()
                .to_string();
        Self {
            registry,
            support_matrix_digest,
        }
    }

    pub fn descriptor(
        &self,
        family: ForgeQueryCapabilityFamily,
    ) -> Option<&ForgeQueryCapabilityDescriptor> {
        self.registry.descriptor(family)
    }

    pub fn capability_registry(&self) -> &ForgeQueryCapabilityRegistry {
        &self.registry
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }

    pub fn admitted_capability_count(&self) -> usize {
        self.registry
            .descriptors()
            .iter()
            .filter(|descriptor| descriptor.status() == ForgeQueryCapabilityStatus::Admitted)
            .count()
    }

    pub fn deferred_capability_count(&self) -> usize {
        self.registry
            .descriptors()
            .iter()
            .filter(|descriptor| descriptor.status() == ForgeQueryCapabilityStatus::DeferredDebt)
            .count()
    }

    pub fn unsupported_capability_count(&self) -> usize {
        self.registry
            .descriptors()
            .iter()
            .filter(|descriptor| descriptor.status() == ForgeQueryCapabilityStatus::Unsupported)
            .count()
    }
}

fn capability_descriptor_identity(descriptor: &ForgeQueryCapabilityDescriptor) -> String {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ApplicationSupportReport)
        .field_shape(ForgeQueryEvidenceTag::new("role"), "capability-descriptor")
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            descriptor.family().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("status"),
            descriptor.status().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("owner"),
            descriptor.owner().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("config_section"),
            descriptor.config_section().as_str(),
        )
        .field_value(ForgeQueryEvidenceTag::new("reason"), descriptor.reason())
        .seal()
        .as_str()
        .to_string()
}
