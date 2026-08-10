use serde::Serialize;

use crate::data::resource::policy::{
    ResourceDiagnosticsDecisionClass, ResourceRetentionDecisionClass,
};
use crate::data::resource::policy_registry::{
    FrozenResourcePolicyDescriptor, FrozenResourcePolicyRegistry,
    ResourcePolicyCompatibilityPosture, ResourcePolicyDescriptorId, ResourcePolicyDigest,
    ResourcePolicyKind, ResourcePolicySelectionBasis, ResourcePolicyVersion,
};

use super::migration;
use super::vocabulary::ResourcePolicyCompatibilityClass;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourcePolicyCompatibilityFamilyReport {
    kind: ResourcePolicyKind,
    class: ResourcePolicyCompatibilityClass,
    historical_descriptor_id: ResourcePolicyDescriptorId,
    current_descriptor_id: Option<ResourcePolicyDescriptorId>,
    historical_semantic_name: String,
    current_semantic_name: Option<String>,
    historical_version: ResourcePolicyVersion,
    current_version: Option<ResourcePolicyVersion>,
    historical_descriptor_digest: ResourcePolicyDigest,
    current_descriptor_digest: Option<ResourcePolicyDigest>,
    historical_frozen_digest: ResourcePolicyDigest,
    current_frozen_digest: Option<ResourcePolicyDigest>,
    historical_selection_basis: ResourcePolicySelectionBasis,
    current_selection_basis: Option<ResourcePolicySelectionBasis>,
    current_compatibility_posture: Option<ResourcePolicyCompatibilityPosture>,
    historical_retention_class: Option<ResourceRetentionDecisionClass>,
    current_retention_class: Option<ResourceRetentionDecisionClass>,
    historical_diagnostics_class: Option<ResourceDiagnosticsDecisionClass>,
    current_diagnostics_class: Option<ResourceDiagnosticsDecisionClass>,
    defaulted_parameter_names: Vec<String>,
    canonical_truth_preserved: bool,
    retained_history_unavailable: bool,
    diagnostics_details_unavailable: bool,
}

impl ResourcePolicyCompatibilityFamilyReport {
    pub(super) fn classify(
        historical: &FrozenResourcePolicyDescriptor,
        current: &FrozenResourcePolicyDescriptor,
        registry: &FrozenResourcePolicyRegistry,
    ) -> Self {
        let evidence = migration::classify_family_compatibility(historical, current, registry);
        Self {
            kind: historical.descriptor().kind(),
            class: evidence.class,
            historical_descriptor_id: historical.descriptor().id(),
            current_descriptor_id: Some(current.descriptor().id()),
            historical_semantic_name: historical.descriptor().semantic_name().as_str().to_owned(),
            current_semantic_name: Some(current.descriptor().semantic_name().as_str().to_owned()),
            historical_version: historical.descriptor().version(),
            current_version: Some(current.descriptor().version()),
            historical_descriptor_digest: historical.descriptor().descriptor_digest().clone(),
            current_descriptor_digest: Some(current.descriptor().descriptor_digest().clone()),
            historical_frozen_digest: historical.frozen_digest().clone(),
            current_frozen_digest: Some(current.frozen_digest().clone()),
            historical_selection_basis: historical.selection_basis(),
            current_selection_basis: Some(current.selection_basis()),
            current_compatibility_posture: Some(current.descriptor().compatibility_posture()),
            historical_retention_class: evidence.historical_retention_class,
            current_retention_class: evidence.current_retention_class,
            historical_diagnostics_class: evidence.historical_diagnostics_class,
            current_diagnostics_class: evidence.current_diagnostics_class,
            defaulted_parameter_names: evidence.defaulted_parameter_names,
            canonical_truth_preserved: evidence.canonical_truth_preserved,
            retained_history_unavailable: evidence.retained_history_unavailable,
            diagnostics_details_unavailable: evidence.diagnostics_details_unavailable,
        }
    }

    pub fn kind(&self) -> ResourcePolicyKind {
        self.kind
    }
    pub fn class(&self) -> ResourcePolicyCompatibilityClass {
        self.class
    }
    pub fn historical_descriptor_id(&self) -> ResourcePolicyDescriptorId {
        self.historical_descriptor_id
    }
    pub fn current_descriptor_id(&self) -> Option<ResourcePolicyDescriptorId> {
        self.current_descriptor_id
    }
    pub fn historical_semantic_name(&self) -> &str {
        &self.historical_semantic_name
    }
    pub fn current_semantic_name(&self) -> Option<&str> {
        self.current_semantic_name.as_deref()
    }
    pub fn historical_version(&self) -> ResourcePolicyVersion {
        self.historical_version
    }
    pub fn current_version(&self) -> Option<ResourcePolicyVersion> {
        self.current_version
    }
    pub fn historical_descriptor_digest(&self) -> &ResourcePolicyDigest {
        &self.historical_descriptor_digest
    }
    pub fn current_descriptor_digest(&self) -> Option<&ResourcePolicyDigest> {
        self.current_descriptor_digest.as_ref()
    }
    pub fn historical_frozen_digest(&self) -> &ResourcePolicyDigest {
        &self.historical_frozen_digest
    }
    pub fn current_frozen_digest(&self) -> Option<&ResourcePolicyDigest> {
        self.current_frozen_digest.as_ref()
    }
    pub fn historical_selection_basis(&self) -> ResourcePolicySelectionBasis {
        self.historical_selection_basis
    }
    pub fn current_selection_basis(&self) -> Option<ResourcePolicySelectionBasis> {
        self.current_selection_basis
    }
    pub fn current_compatibility_posture(&self) -> Option<ResourcePolicyCompatibilityPosture> {
        self.current_compatibility_posture
    }
    pub fn historical_retention_class(&self) -> Option<ResourceRetentionDecisionClass> {
        self.historical_retention_class
    }
    pub fn current_retention_class(&self) -> Option<ResourceRetentionDecisionClass> {
        self.current_retention_class
    }
    pub fn historical_diagnostics_class(&self) -> Option<ResourceDiagnosticsDecisionClass> {
        self.historical_diagnostics_class
    }
    pub fn current_diagnostics_class(&self) -> Option<ResourceDiagnosticsDecisionClass> {
        self.current_diagnostics_class
    }
    pub fn defaulted_parameter_names(&self) -> &[String] {
        &self.defaulted_parameter_names
    }
    pub fn canonical_truth_preserved(&self) -> bool {
        self.canonical_truth_preserved
    }
    pub fn retained_history_unavailable(&self) -> bool {
        self.retained_history_unavailable
    }
    pub fn diagnostics_details_unavailable(&self) -> bool {
        self.diagnostics_details_unavailable
    }
}
