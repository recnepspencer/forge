use super::closure::ForgeQueryIdentityBoundaryClosure;
use super::identity_boundary_hostile_matrix::identity_boundary_hostile_matrix_artifact;
use super::registry::{
    ForgeQueryCapabilityFamily, ForgeQueryCapabilityStatus, ForgeQuerySupportMatrix,
};
use crate::application::config::{
    ForgeQueryConfigSectionFamily, ForgeQueryConfigSectionResolution, ForgeQuerySubsystemOwner,
    ValidatedForgeQueryConfig,
};
use crate::composition::{
    runtime_backed_query_composition_support_profile, QueryCompositionSupportProfile,
};
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
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
    posture_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQuerySupportSectionPosture {
    fn from_resolution(resolution: ForgeQueryConfigSectionResolution) -> Self {
        let section = resolution.section();
        let owner = resolution.owner();
        let enabled = resolution.enabled();
        let config_digest = resolution.config_digest().to_string();
        let posture_identity = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::ApplicationSupportSectionPosture,
        )
        .field_shape(ForgeQueryEvidenceTag::new("section"), section.as_str())
        .field_shape(ForgeQueryEvidenceTag::new("owner"), owner.as_str())
        .field_bool(ForgeQueryEvidenceTag::new("enabled"), enabled)
        .field_identity(
            ForgeQueryEvidenceTag::new("config_digest"),
            config_digest.clone(),
        )
        .seal();
        Self {
            section,
            owner,
            enabled,
            config_digest,
            posture_identity,
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

    pub fn posture_digest(&self) -> &str {
        self.posture_identity.as_str()
    }

    pub fn posture_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.posture_identity
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
    identity_boundary_closure: ForgeQueryIdentityBoundaryClosure,
    validated_config_digest: String,
    counters: ForgeQuerySupportReportCounters,
    report_identity: ForgeQueryEvidenceIdentity,
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
        let query_read_surface_available = support_matrix
            .descriptor(ForgeQueryCapabilityFamily::QueryRead)
            .is_some_and(|descriptor| descriptor.status() == ForgeQueryCapabilityStatus::Admitted);
        let preview_session_surface_available = support_matrix
            .descriptor(ForgeQueryCapabilityFamily::PreviewSession)
            .is_some_and(|descriptor| descriptor.status() == ForgeQueryCapabilityStatus::Admitted);
        let hostile_matrix = identity_boundary_hostile_matrix_artifact();
        let identity_boundary_closure = ForgeQueryIdentityBoundaryClosure::derived(
            support_matrix.support_matrix_digest(),
            &hostile_matrix,
            query_read_surface_available,
            query_read_surface_available,
            preview_session_surface_available,
        );
        let validated_config_digest = config.validated_digest().to_string();
        let counters = ForgeQuerySupportReportCounters::generated_once();
        let report_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::ApplicationSupportReport)
                .field_identity(
                    ForgeQueryEvidenceTag::new("support_matrix_digest"),
                    support_matrix.support_matrix_digest(),
                )
                .field_identity(
                    ForgeQueryEvidenceTag::new("validated_config_digest"),
                    validated_config_digest.clone(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("admitted_capability_count"),
                    admitted_capability_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("deferred_capability_count"),
                    deferred_capability_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("unsupported_capability_count"),
                    unsupported_capability_count,
                )
                .field_identity_sequence(
                    ForgeQueryEvidenceTag::new("admitted_capability_family"),
                    admitted_capability_families
                        .iter()
                        .map(ForgeQueryCapabilityFamily::as_str),
                )
                .field_identity_sequence(
                    ForgeQueryEvidenceTag::new("deferred_capability_family"),
                    deferred_capability_families
                        .iter()
                        .map(ForgeQueryCapabilityFamily::as_str),
                )
                .field_identity_sequence(
                    ForgeQueryEvidenceTag::new("unsupported_capability_family"),
                    unsupported_capability_families
                        .iter()
                        .map(ForgeQueryCapabilityFamily::as_str),
                )
                .field_identity_sequence(
                    ForgeQueryEvidenceTag::new("section_posture_digest"),
                    section_postures
                        .iter()
                        .map(ForgeQuerySupportSectionPosture::posture_digest),
                )
                .optional_identity(
                    ForgeQueryEvidenceTag::new("query_composition_profile_digest"),
                    query_composition_support_profile
                        .as_ref()
                        .map(ForgeQueryQueryCompositionSupportProfile::profile_digest),
                )
                .optional_identity(
                    ForgeQueryEvidenceTag::new("query_context_profile_digest"),
                    query_context_support_profile
                        .as_ref()
                        .map(ForgeQueryQueryContextSupportProfile::profile_digest),
                )
                .optional_identity(
                    ForgeQueryEvidenceTag::new("identity_evolution_profile_digest"),
                    identity_evolution_support_profile
                        .as_ref()
                        .map(ForgeQueryIdentityEvolutionSupportProfile::profile_digest),
                )
                .field_identity(
                    ForgeQueryEvidenceTag::new("identity_boundary_closure_digest"),
                    identity_boundary_closure.closure_digest(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("support_report_generation_count"),
                    counters.support_report_generation_count(),
                )
                .seal();

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
            identity_boundary_closure,
            validated_config_digest,
            counters,
            report_identity,
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

    pub fn identity_boundary_closure(&self) -> &ForgeQueryIdentityBoundaryClosure {
        &self.identity_boundary_closure
    }

    pub fn validated_config_digest(&self) -> &str {
        &self.validated_config_digest
    }

    pub fn counters(&self) -> &ForgeQuerySupportReportCounters {
        &self.counters
    }

    pub fn report_digest(&self) -> &str {
        self.report_identity.as_str()
    }

    pub fn report_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.report_identity
    }
}
