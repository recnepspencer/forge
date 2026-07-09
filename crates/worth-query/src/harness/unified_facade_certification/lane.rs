use crate::facade::{
    WorthQueryCapabilityFamily, WorthQueryCapabilityStatus, WorthQueryConfigSectionFamily,
    WorthQueryFacadeCounters, WorthQueryFacadeFailureClass,
};
use crate::harness::certification::{digest_parts, CertificationMatrix};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UnifiedFacadePerturbationClass {
    ApplicationCapability,
    QueryContextCapability,
    ConfigurationSection,
    SupportMetadata,
    UnsupportedComposition,
    DeferredComposition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnifiedFacadeFailureClass {
    UnsupportedCapability,
    MissingOwningSection,
    InvalidComposedSupportPosture,
    DeferredCapability,
    QueryContextBroadeningDenied,
    InvalidConfiguration,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnifiedFacadeLane {
    pub query_digest: String,
    pub plan_digest: String,
    pub support_matrix_digest: String,
    pub capability_registry_digest: String,
    pub support_report_digest: String,
    pub counter_snapshot_digest: String,
    pub capability_lookup_count: usize,
    pub configuration_section_resolution_count: usize,
    pub unsupported_composition_denial_count: usize,
    pub deferred_capability_denial_count: usize,
    pub support_report_generation_count: usize,
    pub capability_family: String,
    pub capability_status: String,
    pub config_section: String,
    pub query_context_support_profile_digest: String,
    pub query_context_basis_families: Vec<String>,
    pub query_context_comparison_families: Vec<String>,
    pub query_context_deferred_scope_markers: Vec<String>,
    pub identity_evolution_support_profile_digest: String,
    pub identity_evolution_traversal_families: Vec<String>,
    pub identity_evolution_comparison_basis_families: Vec<String>,
    pub identity_evolution_inspector_consumable_classifications: Vec<String>,
    pub identity_evolution_deferred_scope_markers: Vec<String>,
    pub identity_evolution_result_digest: String,
    pub identity_evolution_branch_locality_digest: String,
    pub basis_result_digest: String,
    pub diff_result_digest: String,
    pub query_context_replay_digest: String,
}

impl UnifiedFacadeLane {
    pub fn new(
        query_digest: String,
        plan_digest: String,
        support_matrix_digest: String,
        capability_registry_digest: String,
        counters: &WorthQueryFacadeCounters,
        capability_family: WorthQueryCapabilityFamily,
        capability_status: WorthQueryCapabilityStatus,
        config_section: WorthQueryConfigSectionFamily,
    ) -> Self {
        Self {
            query_digest,
            plan_digest,
            support_matrix_digest,
            capability_registry_digest,
            support_report_digest: String::new(),
            capability_lookup_count: counters.capability_lookup_count(),
            configuration_section_resolution_count: counters
                .configuration_section_resolution_count(),
            unsupported_composition_denial_count: counters.unsupported_composition_denial_count(),
            deferred_capability_denial_count: counters.deferred_capability_denial_count(),
            support_report_generation_count: 0,
            counter_snapshot_digest: digest_parts(&[
                format!("lookups:{}", counters.capability_lookup_count()),
                format!(
                    "section_resolutions:{}",
                    counters.configuration_section_resolution_count()
                ),
                format!(
                    "unsupported_denials:{}",
                    counters.unsupported_composition_denial_count()
                ),
                format!(
                    "deferred_denials:{}",
                    counters.deferred_capability_denial_count()
                ),
            ]),
            capability_family: capability_family.as_str().to_string(),
            capability_status: capability_status.as_str().to_string(),
            config_section: config_section.as_str().to_string(),
            query_context_support_profile_digest: String::new(),
            query_context_basis_families: Vec::new(),
            query_context_comparison_families: Vec::new(),
            query_context_deferred_scope_markers: Vec::new(),
            identity_evolution_support_profile_digest: String::new(),
            identity_evolution_traversal_families: Vec::new(),
            identity_evolution_comparison_basis_families: Vec::new(),
            identity_evolution_inspector_consumable_classifications: Vec::new(),
            identity_evolution_deferred_scope_markers: Vec::new(),
            identity_evolution_result_digest: String::new(),
            identity_evolution_branch_locality_digest: String::new(),
            basis_result_digest: String::new(),
            diff_result_digest: String::new(),
            query_context_replay_digest: String::new(),
        }
    }

    pub fn with_report_digest(
        mut self,
        support_report_digest: String,
        support_report_generation_count: usize,
    ) -> Self {
        self.support_report_digest = support_report_digest;
        self.support_report_generation_count = support_report_generation_count;
        self.counter_snapshot_digest = digest_parts(&[
            format!("lookups:{}", self.capability_lookup_count),
            format!(
                "section_resolutions:{}",
                self.configuration_section_resolution_count
            ),
            format!(
                "unsupported_denials:{}",
                self.unsupported_composition_denial_count
            ),
            format!("deferred_denials:{}", self.deferred_capability_denial_count),
            format!(
                "support_report_generation:{}",
                self.support_report_generation_count
            ),
        ]);
        self
    }

    pub fn with_query_context_support_profile(
        mut self,
        profile_digest: String,
        basis_families: Vec<String>,
        comparison_families: Vec<String>,
        deferred_scope_markers: Vec<String>,
    ) -> Self {
        self.query_context_support_profile_digest = profile_digest;
        self.query_context_basis_families = basis_families;
        self.query_context_comparison_families = comparison_families;
        self.query_context_deferred_scope_markers = deferred_scope_markers;
        self
    }

    pub fn with_identity_evolution_support_profile(
        mut self,
        profile_digest: String,
        traversal_families: Vec<String>,
        comparison_basis_families: Vec<String>,
        inspector_consumable_classifications: Vec<String>,
        deferred_scope_markers: Vec<String>,
    ) -> Self {
        self.identity_evolution_support_profile_digest = profile_digest;
        self.identity_evolution_traversal_families = traversal_families;
        self.identity_evolution_comparison_basis_families = comparison_basis_families;
        self.identity_evolution_inspector_consumable_classifications =
            inspector_consumable_classifications;
        self.identity_evolution_deferred_scope_markers = deferred_scope_markers;
        self
    }

    pub fn with_identity_evolution_result_digests(
        mut self,
        result_digest: String,
        branch_locality_digest: String,
    ) -> Self {
        self.identity_evolution_result_digest = result_digest;
        self.identity_evolution_branch_locality_digest = branch_locality_digest;
        self
    }

    pub fn with_query_context_result_digests(
        mut self,
        basis_result_digest: String,
        diff_result_digest: String,
        replay_digest: String,
    ) -> Self {
        self.basis_result_digest = basis_result_digest;
        self.diff_result_digest = diff_result_digest;
        self.query_context_replay_digest = replay_digest;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnifiedFacadeRejection {
    pub failure_class: UnifiedFacadeFailureClass,
    pub counter_snapshot_digest: String,
    pub capability_lookup_count: usize,
    pub configuration_section_resolution_count: usize,
    pub unsupported_composition_denial_count: usize,
    pub deferred_capability_denial_count: usize,
    pub config_validation_denial_count: usize,
    pub query_context_denial_width: usize,
    pub query_context_broadening_denial_count: usize,
}

impl UnifiedFacadeRejection {
    pub fn from_error(error: &crate::facade::WorthQueryFacadeError) -> Self {
        Self {
            failure_class: match error.failure_class() {
                WorthQueryFacadeFailureClass::UnsupportedCapabilityFamily => {
                    UnifiedFacadeFailureClass::UnsupportedCapability
                }
                WorthQueryFacadeFailureClass::MissingOwningSection => {
                    UnifiedFacadeFailureClass::MissingOwningSection
                }
                WorthQueryFacadeFailureClass::InvalidComposedSupportPosture => {
                    UnifiedFacadeFailureClass::InvalidComposedSupportPosture
                }
                WorthQueryFacadeFailureClass::DeferredCapabilityFamily => {
                    UnifiedFacadeFailureClass::DeferredCapability
                }
            },
            capability_lookup_count: error.counters().capability_lookup_count(),
            configuration_section_resolution_count: error
                .counters()
                .configuration_section_resolution_count(),
            unsupported_composition_denial_count: error
                .counters()
                .unsupported_composition_denial_count(),
            deferred_capability_denial_count: error.counters().deferred_capability_denial_count(),
            config_validation_denial_count: 0,
            query_context_denial_width: 0,
            query_context_broadening_denial_count: 0,
            counter_snapshot_digest: digest_parts(&[
                format!("lookups:{}", error.counters().capability_lookup_count()),
                format!(
                    "section_resolutions:{}",
                    error.counters().configuration_section_resolution_count()
                ),
                format!(
                    "unsupported_denials:{}",
                    error.counters().unsupported_composition_denial_count()
                ),
                format!(
                    "deferred_denials:{}",
                    error.counters().deferred_capability_denial_count()
                ),
                "query_context_denial_width:0".to_string(),
                "query_context_broadening_denials:0".to_string(),
            ]),
        }
    }

    pub fn from_config_error(error: &crate::facade::ConfigurationAdmissionError) -> Self {
        Self {
            failure_class: UnifiedFacadeFailureClass::InvalidConfiguration,
            capability_lookup_count: 0,
            configuration_section_resolution_count: error
                .counters()
                .config_section_resolution_count(),
            unsupported_composition_denial_count: 0,
            deferred_capability_denial_count: 0,
            config_validation_denial_count: error.counters().config_validation_denial_count(),
            query_context_denial_width: 0,
            query_context_broadening_denial_count: 0,
            counter_snapshot_digest: digest_parts(&[
                format!(
                    "validation_denials:{}",
                    error.counters().config_validation_denial_count()
                ),
                format!(
                    "section_resolutions:{}",
                    error.counters().config_section_resolution_count()
                ),
                "lookups:0".to_string(),
                "unsupported_denials:0".to_string(),
                "deferred_denials:0".to_string(),
                "query_context_denial_width:0".to_string(),
                "query_context_broadening_denials:0".to_string(),
            ]),
        }
    }

    pub fn from_query_context_error(
        facade_counters: &crate::facade::WorthQueryFacadeCounters,
        error: &crate::query_context::QueryContextAdmissionError,
    ) -> Self {
        Self {
            failure_class: match error.failure_class() {
                crate::query_context::QueryContextAdmissionFailureClass::ComparisonBroadeningRequired => {
                    UnifiedFacadeFailureClass::QueryContextBroadeningDenied
                }
                other => panic!("unexpected query-context rejection for unified facade row: {other:?}"),
            },
            capability_lookup_count: facade_counters.capability_lookup_count(),
            configuration_section_resolution_count: facade_counters
                .configuration_section_resolution_count(),
            unsupported_composition_denial_count: facade_counters
                .unsupported_composition_denial_count(),
            deferred_capability_denial_count: facade_counters.deferred_capability_denial_count(),
            config_validation_denial_count: 0,
            query_context_denial_width: error.counters().denial_width(),
            query_context_broadening_denial_count: error
                .counters()
                .comparison_broadening_denial_count(),
            counter_snapshot_digest: digest_parts(&[
                format!("lookups:{}", facade_counters.capability_lookup_count()),
                format!(
                    "section_resolutions:{}",
                    facade_counters.configuration_section_resolution_count()
                ),
                format!(
                    "unsupported_denials:{}",
                    facade_counters.unsupported_composition_denial_count()
                ),
                format!(
                    "deferred_denials:{}",
                    facade_counters.deferred_capability_denial_count()
                ),
                format!("query_context_denial_width:{}", error.counters().denial_width()),
                format!(
                    "query_context_broadening_denials:{}",
                    error.counters().comparison_broadening_denial_count()
                ),
            ]),
        }
    }
}

pub type UnifiedFacadeCertificationMatrix =
    CertificationMatrix<UnifiedFacadePerturbationClass, UnifiedFacadeLane, UnifiedFacadeRejection>;
