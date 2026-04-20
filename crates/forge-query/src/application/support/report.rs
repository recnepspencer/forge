use super::registry::{
    ForgeQueryCapabilityFamily, ForgeQueryCapabilityStatus, ForgeQuerySupportMatrix,
};
use crate::application::config::{
    ForgeQueryConfigSectionFamily, ForgeQueryConfigSectionResolution, ForgeQuerySubsystemOwner,
    ValidatedForgeQueryConfig,
};
use crate::identity::hash_parts;
use crate::composition::{
    runtime_backed_query_composition_support_profile, QueryCompositionSupportProfile,
};
use crate::identity_evolution::{
    runtime_backed_direct_identity_evolution_support_profile, IdentityEvolutionSupportProfile,
};
use crate::query_context::{
    runtime_backed_narrow_query_context_support_profile, QueryContextSupportProfile,
};

pub type ForgeQueryQueryContextSupportProfile = QueryContextSupportProfile;
pub type ForgeQueryIdentityEvolutionSupportProfile = IdentityEvolutionSupportProfile;
pub type ForgeQueryQueryCompositionSupportProfile = QueryCompositionSupportProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySupportReportCounters {
    support_report_generation_count: usize,
}

impl ForgeQuerySupportReportCounters {
    fn generated_once() -> Self {
        Self {
            support_report_generation_count: 1,
        }
    }

    pub fn support_report_generation_count(&self) -> usize {
        self.support_report_generation_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySupportSectionPosture {
    section: ForgeQueryConfigSectionFamily,
    owner: ForgeQuerySubsystemOwner,
    enabled: bool,
    config_digest: String,
}

impl ForgeQuerySupportSectionPosture {
    fn from_resolution(resolution: ForgeQueryConfigSectionResolution) -> Self {
        Self {
            section: resolution.section(),
            owner: resolution.owner(),
            enabled: resolution.enabled(),
            config_digest: resolution.config_digest().to_string(),
        }
    }

    pub fn section(&self) -> ForgeQueryConfigSectionFamily {
        self.section
    }

    pub fn owner(&self) -> ForgeQuerySubsystemOwner {
        self.owner
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn config_digest(&self) -> &str {
        &self.config_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySupportReport {
    support_matrix: ForgeQuerySupportMatrix,
    admitted_capability_count: usize,
    deferred_capability_count: usize,
    unsupported_capability_count: usize,
    admitted_capability_families: Vec<ForgeQueryCapabilityFamily>,
    deferred_capability_families: Vec<ForgeQueryCapabilityFamily>,
    unsupported_capability_families: Vec<ForgeQueryCapabilityFamily>,
    section_postures: Vec<ForgeQuerySupportSectionPosture>,
    query_context_support_profile: Option<ForgeQueryQueryContextSupportProfile>,
    query_composition_support_profile: Option<ForgeQueryQueryCompositionSupportProfile>,
    identity_evolution_support_profile: Option<ForgeQueryIdentityEvolutionSupportProfile>,
    validated_config_digest: String,
    counters: ForgeQuerySupportReportCounters,
    report_digest: String,
}

impl ForgeQuerySupportReport {
    pub(crate) fn from_validated_config_and_matrix(
        config: &ValidatedForgeQueryConfig,
        support_matrix: ForgeQuerySupportMatrix,
    ) -> Self {
        let admitted_capability_families = support_matrix
            .capability_registry()
            .descriptors()
            .iter()
            .filter(|descriptor| descriptor.status() == ForgeQueryCapabilityStatus::Admitted)
            .map(|descriptor| descriptor.family())
            .collect::<Vec<_>>();
        let deferred_capability_families = support_matrix
            .capability_registry()
            .descriptors()
            .iter()
            .filter(|descriptor| descriptor.status() == ForgeQueryCapabilityStatus::DeferredDebt)
            .map(|descriptor| descriptor.family())
            .collect::<Vec<_>>();
        let unsupported_capability_families = support_matrix
            .capability_registry()
            .descriptors()
            .iter()
            .filter(|descriptor| descriptor.status() == ForgeQueryCapabilityStatus::Unsupported)
            .map(|descriptor| descriptor.family())
            .collect::<Vec<_>>();
        let admitted_capability_count = admitted_capability_families.len();
        let deferred_capability_count = deferred_capability_families.len();
        let unsupported_capability_count = unsupported_capability_families.len();
        let section_postures = vec![
            ForgeQuerySupportSectionPosture::from_resolution(
                config.resolve_section(ForgeQueryConfigSectionFamily::Query),
            ),
            ForgeQuerySupportSectionPosture::from_resolution(
                config.resolve_section(ForgeQueryConfigSectionFamily::Relational),
            ),
            ForgeQuerySupportSectionPosture::from_resolution(
                config.resolve_section(ForgeQueryConfigSectionFamily::Signal),
            ),
            ForgeQuerySupportSectionPosture::from_resolution(
                config.resolve_section(ForgeQueryConfigSectionFamily::RuntimeBridge),
            ),
            ForgeQuerySupportSectionPosture::from_resolution(
                config.resolve_section(ForgeQueryConfigSectionFamily::Store),
            ),
        ];
        let query_composition_support_profile = support_matrix
            .descriptor(ForgeQueryCapabilityFamily::QueryComposition)
            .filter(|descriptor| descriptor.status() == ForgeQueryCapabilityStatus::Admitted)
            .map(|_| runtime_backed_query_composition_support_profile());
        let query_context_support_profile = support_matrix
            .descriptor(ForgeQueryCapabilityFamily::QueryContext)
            .filter(|descriptor| descriptor.status() == ForgeQueryCapabilityStatus::Admitted)
            .map(|_| runtime_backed_narrow_query_context_support_profile());
        let identity_evolution_support_profile = support_matrix
            .descriptor(ForgeQueryCapabilityFamily::IdentityEvolution)
            .filter(|descriptor| descriptor.status() == ForgeQueryCapabilityStatus::Admitted)
            .map(|_| runtime_backed_direct_identity_evolution_support_profile());
        let validated_config_digest = config.validated_digest().to_string();
        let counters = ForgeQuerySupportReportCounters::generated_once();
        let report_digest = hash_parts(&[
            format!("support:{}", support_matrix.support_matrix_digest()),
            format!("validated_config:{validated_config_digest}"),
            format!("admitted:{admitted_capability_count}"),
            format!("deferred:{deferred_capability_count}"),
            format!("unsupported:{unsupported_capability_count}"),
            format!(
                "admitted_families:{}",
                admitted_capability_families
                    .iter()
                    .map(ForgeQueryCapabilityFamily::as_str)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            format!(
                "deferred_families:{}",
                deferred_capability_families
                    .iter()
                    .map(ForgeQueryCapabilityFamily::as_str)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            format!(
                "unsupported_families:{}",
                unsupported_capability_families
                    .iter()
                    .map(ForgeQueryCapabilityFamily::as_str)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            format!(
                "sections:{}",
                section_postures
                    .iter()
                    .map(|posture| format!(
                        "{}:{}:{}:{}",
                        posture.section().as_str(),
                        posture.owner().as_str(),
                        posture.enabled(),
                        posture.config_digest()
                    ))
                    .collect::<Vec<_>>()
                    .join("|")
            ),
            format!(
                "query_composition_profile:{}",
                query_composition_support_profile
                    .as_ref()
                    .map(ForgeQueryQueryCompositionSupportProfile::profile_digest)
                    .unwrap_or("none")
            ),
            format!(
                "query_context_profile:{}",
                query_context_support_profile
                    .as_ref()
                    .map(ForgeQueryQueryContextSupportProfile::profile_digest)
                    .unwrap_or("none")
            ),
            format!(
                "identity_evolution_profile:{}",
                identity_evolution_support_profile
                    .as_ref()
                    .map(ForgeQueryIdentityEvolutionSupportProfile::profile_digest)
                    .unwrap_or("none")
            ),
            format!(
                "report_generation:{}",
                counters.support_report_generation_count()
            ),
        ]);

        Self {
            support_matrix,
            admitted_capability_count,
            deferred_capability_count,
            unsupported_capability_count,
            admitted_capability_families,
            deferred_capability_families,
            unsupported_capability_families,
            section_postures,
            query_context_support_profile,
            query_composition_support_profile,
            identity_evolution_support_profile,
            validated_config_digest,
            counters,
            report_digest,
        }
    }

    pub fn support_matrix(&self) -> &ForgeQuerySupportMatrix {
        &self.support_matrix
    }

    pub fn admitted_capability_count(&self) -> usize {
        self.admitted_capability_count
    }

    pub fn deferred_capability_count(&self) -> usize {
        self.deferred_capability_count
    }

    pub fn unsupported_capability_count(&self) -> usize {
        self.unsupported_capability_count
    }

    pub fn admitted_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        &self.admitted_capability_families
    }

    pub fn deferred_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        &self.deferred_capability_families
    }

    pub fn unsupported_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        &self.unsupported_capability_families
    }

    pub fn section_postures(&self) -> &[ForgeQuerySupportSectionPosture] {
        &self.section_postures
    }

    pub fn query_context_support_profile(&self) -> Option<&ForgeQueryQueryContextSupportProfile> {
        self.query_context_support_profile.as_ref()
    }

    pub fn query_composition_support_profile(
        &self,
    ) -> Option<&ForgeQueryQueryCompositionSupportProfile> {
        self.query_composition_support_profile.as_ref()
    }

    pub fn identity_evolution_support_profile(
        &self,
    ) -> Option<&ForgeQueryIdentityEvolutionSupportProfile> {
        self.identity_evolution_support_profile.as_ref()
    }

    pub fn validated_config_digest(&self) -> &str {
        &self.validated_config_digest
    }

    pub fn counters(&self) -> &ForgeQuerySupportReportCounters {
        &self.counters
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}
