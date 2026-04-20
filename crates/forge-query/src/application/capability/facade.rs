use super::errors::{CapabilityAdmissionError, ForgeQueryFacadeCounters};
use super::resolution::{
    deny_capability, CapabilityAdmissionDecision, ForgeQueryCapabilityResolution,
};
use super::witnesses::{
    HistoricalEvaluationCapability, IdentityEvolutionCapability, LiveQueryCapability,
    PreviewSessionCapability, QueryCompositionCapability, QueryContextCapability, QueryReadCapability,
    WorkflowOrchestrationCapability,
};
use crate::application::config::{
    ConfigurationAdmissionError, ForgeQueryConfig, ForgeQueryConfigSectionFamily,
    ForgeQueryConfigSectionResolution, ValidatedForgeQueryConfig,
};
use crate::application::support::{
    ForgeQueryCapabilityFamily, ForgeQueryCapabilityRegistry, ForgeQueryCapabilityStatus,
    ForgeQuerySupportMatrix, ForgeQuerySupportReport,
};
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryApplicationFacade {
    config: ValidatedForgeQueryConfig,
    support_matrix: ForgeQuerySupportMatrix,
    facade_digest: String,
}

impl ForgeQueryApplicationFacade {
    pub fn new(config: ForgeQueryConfig) -> Result<Self, ConfigurationAdmissionError> {
        let config = config.validate()?;
        let registry = ForgeQueryCapabilityRegistry::from_validated_config(&config);
        let support_matrix = ForgeQuerySupportMatrix::new(registry);
        let facade_digest = hash_parts(&[
            format!("config:{}", config.validated_digest()),
            format!(
                "registry:{}",
                support_matrix.capability_registry().registry_digest()
            ),
            format!("support:{}", support_matrix.support_matrix_digest()),
        ]);
        Ok(Self {
            config,
            support_matrix,
            facade_digest,
        })
    }

    pub fn runtime_backed_default() -> Self {
        Self::new(ForgeQueryConfig::runtime_backed_default())
            .expect("runtime-backed default config should be valid")
    }

    pub fn validated_config(&self) -> &ValidatedForgeQueryConfig {
        &self.config
    }

    pub fn support_matrix(&self) -> ForgeQuerySupportMatrix {
        self.support_matrix.clone()
    }

    pub fn support_report(&self) -> ForgeQuerySupportReport {
        ForgeQuerySupportReport::from_validated_config_and_matrix(
            &self.config,
            self.support_matrix.clone(),
        )
    }

    pub fn resolve_config_section(
        &self,
        section: ForgeQueryConfigSectionFamily,
    ) -> (ForgeQueryConfigSectionResolution, ForgeQueryFacadeCounters) {
        (
            self.config.resolve_section(section),
            ForgeQueryFacadeCounters::config_resolution(),
        )
    }

    pub fn query_read_capability(
        &self,
    ) -> Result<ForgeQueryCapabilityResolution<QueryReadCapability>, CapabilityAdmissionError> {
        let admission = self.query_read_admission_decision()?;
        Ok(ForgeQueryCapabilityResolution::new(
            QueryReadCapability::new(self.facade_digest.clone()),
            admission,
        ))
    }

    pub(crate) fn query_read_admission_decision(
        &self,
    ) -> Result<CapabilityAdmissionDecision, CapabilityAdmissionError> {
        self.require_capability(ForgeQueryCapabilityFamily::QueryRead)
    }

    pub fn query_composition_capability(
        &self,
    ) -> Result<
        ForgeQueryCapabilityResolution<QueryCompositionCapability>,
        CapabilityAdmissionError,
    > {
        let admission = self.query_composition_admission_decision()?;
        Ok(ForgeQueryCapabilityResolution::new(
            QueryCompositionCapability::new(self.facade_digest.clone()),
            admission,
        ))
    }

    pub(crate) fn query_composition_admission_decision(
        &self,
    ) -> Result<CapabilityAdmissionDecision, CapabilityAdmissionError> {
        self.require_capability(ForgeQueryCapabilityFamily::QueryComposition)
    }

    pub fn live_query_capability(
        &self,
    ) -> Result<ForgeQueryCapabilityResolution<LiveQueryCapability>, CapabilityAdmissionError> {
        let admission = self.live_query_admission_decision()?;
        Ok(ForgeQueryCapabilityResolution::new(
            LiveQueryCapability::new(self.facade_digest.clone()),
            admission,
        ))
    }

    pub(crate) fn live_query_admission_decision(
        &self,
    ) -> Result<CapabilityAdmissionDecision, CapabilityAdmissionError> {
        self.require_capability(ForgeQueryCapabilityFamily::LiveQuery)
    }

    pub fn preview_query_capability(
        &self,
    ) -> Result<ForgeQueryCapabilityResolution<PreviewSessionCapability>, CapabilityAdmissionError>
    {
        let admission = self.preview_query_admission_decision()?;
        Ok(ForgeQueryCapabilityResolution::new(
            PreviewSessionCapability::new(self.facade_digest.clone()),
            admission,
        ))
    }

    pub(crate) fn preview_query_admission_decision(
        &self,
    ) -> Result<CapabilityAdmissionDecision, CapabilityAdmissionError> {
        self.require_capability(ForgeQueryCapabilityFamily::PreviewSession)
    }

    pub fn workflow_query_capability(
        &self,
    ) -> Result<
        ForgeQueryCapabilityResolution<WorkflowOrchestrationCapability>,
        CapabilityAdmissionError,
    > {
        let admission = self.workflow_query_admission_decision()?;
        Ok(ForgeQueryCapabilityResolution::new(
            WorkflowOrchestrationCapability::new(self.facade_digest.clone()),
            admission,
        ))
    }

    pub(crate) fn workflow_query_admission_decision(
        &self,
    ) -> Result<CapabilityAdmissionDecision, CapabilityAdmissionError> {
        self.require_capability(ForgeQueryCapabilityFamily::WorkflowOrchestration)
    }

    pub fn historical_query_capability(
        &self,
    ) -> Result<
        ForgeQueryCapabilityResolution<HistoricalEvaluationCapability>,
        CapabilityAdmissionError,
    > {
        let admission = self.historical_query_admission_decision()?;
        Ok(ForgeQueryCapabilityResolution::new(
            HistoricalEvaluationCapability::new(self.facade_digest.clone()),
            admission,
        ))
    }

    pub(crate) fn historical_query_admission_decision(
        &self,
    ) -> Result<CapabilityAdmissionDecision, CapabilityAdmissionError> {
        self.require_capability(ForgeQueryCapabilityFamily::HistoricalEvaluation)
    }

    pub fn query_context_capability(
        &self,
    ) -> Result<ForgeQueryCapabilityResolution<QueryContextCapability>, CapabilityAdmissionError>
    {
        let admission = self.query_context_admission_decision()?;
        Ok(ForgeQueryCapabilityResolution::new(
            QueryContextCapability::new(self.facade_digest.clone()),
            admission,
        ))
    }

    pub(crate) fn query_context_admission_decision(
        &self,
    ) -> Result<CapabilityAdmissionDecision, CapabilityAdmissionError> {
        self.require_capability(ForgeQueryCapabilityFamily::QueryContext)
    }

    pub fn identity_evolution_capability(
        &self,
    ) -> Result<
        ForgeQueryCapabilityResolution<IdentityEvolutionCapability>,
        CapabilityAdmissionError,
    > {
        let admission = self.identity_evolution_admission_decision()?;
        Ok(ForgeQueryCapabilityResolution::new(
            IdentityEvolutionCapability::new(self.facade_digest.clone()),
            admission,
        ))
    }

    pub(crate) fn identity_evolution_admission_decision(
        &self,
    ) -> Result<CapabilityAdmissionDecision, CapabilityAdmissionError> {
        self.require_capability(ForgeQueryCapabilityFamily::IdentityEvolution)
    }

    pub fn durable_artifact_capability(
        &self,
    ) -> Result<CapabilityAdmissionDecision, CapabilityAdmissionError> {
        self.require_capability(ForgeQueryCapabilityFamily::DurableArtifacts)
    }

    fn require_capability(
        &self,
        family: ForgeQueryCapabilityFamily,
    ) -> Result<CapabilityAdmissionDecision, CapabilityAdmissionError> {
        let descriptor = self
            .support_matrix
            .descriptor(family)
            .expect("capability registry should contain every known capability family")
            .clone();
        match descriptor.status() {
            ForgeQueryCapabilityStatus::Admitted => Ok(CapabilityAdmissionDecision::admitted(
                descriptor,
                self.config.validated_digest(),
            )),
            ForgeQueryCapabilityStatus::DeferredDebt | ForgeQueryCapabilityStatus::Unsupported => {
                Err(deny_capability(&descriptor, &self.config))
            }
        }
    }
}
