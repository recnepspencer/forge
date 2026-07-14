use serde::Serialize;

use crate::data::resource::descriptor::ResourceDescriptorId;
use crate::data::resource::policy::{
    ResourceDiagnosticsDecisionClass, ResourceReplayDecisionClass, ResourceReplayDecisionPlan,
    ResourceRetentionDecisionClass,
};
use crate::data::resource::policy_registry::{
    FrozenResourcePolicyDescriptor, FrozenResourcePolicyDescriptorSet,
    FrozenResourcePolicyRegistry, LoweredResourcePolicyBundle, ResourcePolicyCompatibilityPosture,
    ResourcePolicyDescriptorId, ResourcePolicyDigest, ResourcePolicyKind,
    ResourcePolicyResolutionError, ResourcePolicySelectionBasis, ResourcePolicyVersion,
    ValidatedResourcePolicyDeclaration,
};
use crate::data::resource::request::ResourceNodeId;
use crate::data::resource::summary::ResourceBoundaryPerformanceEnvelope;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourcePolicyCompatibilityClass {
    ExactDescriptorMatch,
    CompatibleParameterExpansion,
    CompatibleRetentionNarrowing,
    CompatibleDiagnosticsRichnessChange,
    MissingDescriptor,
    VersionIncompatible,
    ParameterDigestDrift,
    DecisionSemanticsDrift,
}

impl ResourcePolicyCompatibilityClass {
    pub fn is_compatible(self) -> bool {
        matches!(
            self,
            Self::ExactDescriptorMatch
                | Self::CompatibleParameterExpansion
                | Self::CompatibleRetentionNarrowing
                | Self::CompatibleDiagnosticsRichnessChange
        )
    }
}

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
    fn classify(
        historical: &FrozenResourcePolicyDescriptor,
        current: &FrozenResourcePolicyDescriptor,
        registry: &FrozenResourcePolicyRegistry,
    ) -> Self {
        let (
            class,
            historical_retention_class,
            current_retention_class,
            historical_diagnostics_class,
            current_diagnostics_class,
            defaulted_parameter_names,
            canonical_truth_preserved,
            retained_history_unavailable,
            diagnostics_details_unavailable,
        ) = classify_family_compatibility(historical, current, registry);

        Self {
            kind: historical.descriptor().kind(),
            class,
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
            historical_retention_class,
            current_retention_class,
            historical_diagnostics_class,
            current_diagnostics_class,
            defaulted_parameter_names,
            canonical_truth_preserved,
            retained_history_unavailable,
            diagnostics_details_unavailable,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourcePolicyCompatibilityReport {
    descriptor_id: ResourceDescriptorId,
    node: ResourceNodeId,
    compared_width: u32,
    incompatible_width: u32,
    historical_registry_digest: ResourcePolicyDigest,
    current_registry_digest: ResourcePolicyDigest,
    families: Vec<ResourcePolicyCompatibilityFamilyReport>,
    compatibility_digest: ResourcePolicyDigest,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourcePolicyRestoreCompatibilityDenialClass {
    MissingDescriptor,
    VersionIncompatible,
    ParameterDigestDrift,
    DecisionSemanticsDrift,
    ReplayPolicyDisallowsCompatibleDrift,
    MultipleIncompatibilities,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourcePolicyRestoreCompatibilityProof {
    compatibility: ResourcePolicyCompatibilityReport,
    replay_decision_class: ResourceReplayDecisionClass,
    replay_decision_descriptor_id: ResourcePolicyDescriptorId,
    replay_decision_digest: ResourcePolicyDigest,
}

impl ResourcePolicyRestoreCompatibilityProof {
    pub(crate) fn from_compatibility(
        compatibility: ResourcePolicyCompatibilityReport,
        replay_decision_plan: &ResourceReplayDecisionPlan,
    ) -> Result<Self, ResourcePolicyCompatibilityReport> {
        if compatibility.is_compatible()
            && compatibility
                .families()
                .iter()
                .all(|family| replay_decision_plan.admits_compatible_class(family.class()))
        {
            Ok(Self {
                compatibility,
                replay_decision_class: replay_decision_plan.class(),
                replay_decision_descriptor_id: replay_decision_plan.descriptor_id(),
                replay_decision_digest: replay_decision_plan.decision_digest().clone(),
            })
        } else {
            Err(compatibility)
        }
    }

    pub fn compatibility(&self) -> &ResourcePolicyCompatibilityReport {
        &self.compatibility
    }

    pub fn compatibility_digest(&self) -> &ResourcePolicyDigest {
        self.compatibility.compatibility_digest()
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.compatibility.performance()
    }

    pub fn replay_decision_class(&self) -> ResourceReplayDecisionClass {
        self.replay_decision_class
    }

    pub fn replay_decision_descriptor_id(&self) -> ResourcePolicyDescriptorId {
        self.replay_decision_descriptor_id
    }

    pub fn replay_decision_digest(&self) -> &ResourcePolicyDigest {
        &self.replay_decision_digest
    }

    pub fn canonical_truth_preserved_width(&self) -> u32 {
        self.compatibility.canonical_truth_preserved_width()
    }

    pub fn retained_history_unavailable_width(&self) -> u32 {
        self.compatibility.retained_history_unavailable_width()
    }

    pub fn diagnostics_details_unavailable_width(&self) -> u32 {
        self.compatibility.diagnostics_details_unavailable_width()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeniedResourcePolicyRestoreCompatibility {
    class: ResourcePolicyRestoreCompatibilityDenialClass,
    primary_incompatible_kind: Option<ResourcePolicyKind>,
    compatibility: ResourcePolicyCompatibilityReport,
    replay_decision_class: ResourceReplayDecisionClass,
    replay_decision_descriptor_id: ResourcePolicyDescriptorId,
    replay_decision_digest: ResourcePolicyDigest,
}

impl DeniedResourcePolicyRestoreCompatibility {
    pub(crate) fn from_compatibility(
        compatibility: ResourcePolicyCompatibilityReport,
        replay_decision_plan: &ResourceReplayDecisionPlan,
    ) -> Self {
        let incompatibilities: Vec<_> = compatibility
            .families()
            .iter()
            .filter(|family| !family.class().is_compatible())
            .collect();
        debug_assert!(
            !incompatibilities.is_empty(),
            "restore compatibility denial requires at least one incompatible family"
        );
        let primary_incompatible_kind = incompatibilities.first().map(|family| family.kind());
        let class = if incompatibilities.len() > 1 {
            ResourcePolicyRestoreCompatibilityDenialClass::MultipleIncompatibilities
        } else {
            match incompatibilities.first().map(|family| family.class()) {
                Some(ResourcePolicyCompatibilityClass::MissingDescriptor) => {
                    ResourcePolicyRestoreCompatibilityDenialClass::MissingDescriptor
                }
                Some(ResourcePolicyCompatibilityClass::VersionIncompatible) => {
                    ResourcePolicyRestoreCompatibilityDenialClass::VersionIncompatible
                }
                Some(ResourcePolicyCompatibilityClass::CompatibleParameterExpansion)
                | Some(ResourcePolicyCompatibilityClass::CompatibleRetentionNarrowing)
                | Some(ResourcePolicyCompatibilityClass::CompatibleDiagnosticsRichnessChange) => {
                    unreachable!("restore compatibility denial constructed from compatible report")
                }
                Some(ResourcePolicyCompatibilityClass::ParameterDigestDrift) => {
                    ResourcePolicyRestoreCompatibilityDenialClass::ParameterDigestDrift
                }
                Some(ResourcePolicyCompatibilityClass::DecisionSemanticsDrift) => {
                    ResourcePolicyRestoreCompatibilityDenialClass::DecisionSemanticsDrift
                }
                Some(ResourcePolicyCompatibilityClass::ExactDescriptorMatch) | None => {
                    unreachable!("restore compatibility denial constructed from compatible report")
                }
            }
        };

        Self {
            class,
            primary_incompatible_kind,
            compatibility,
            replay_decision_class: replay_decision_plan.class(),
            replay_decision_descriptor_id: replay_decision_plan.descriptor_id(),
            replay_decision_digest: replay_decision_plan.decision_digest().clone(),
        }
    }

    pub(crate) fn from_replay_policy_gate(
        compatibility: ResourcePolicyCompatibilityReport,
        replay_decision_plan: &ResourceReplayDecisionPlan,
        primary_incompatible_kind: ResourcePolicyKind,
    ) -> Self {
        Self {
            class:
                ResourcePolicyRestoreCompatibilityDenialClass::ReplayPolicyDisallowsCompatibleDrift,
            primary_incompatible_kind: Some(primary_incompatible_kind),
            compatibility,
            replay_decision_class: replay_decision_plan.class(),
            replay_decision_descriptor_id: replay_decision_plan.descriptor_id(),
            replay_decision_digest: replay_decision_plan.decision_digest().clone(),
        }
    }

    pub fn class(&self) -> ResourcePolicyRestoreCompatibilityDenialClass {
        self.class
    }

    pub fn primary_incompatible_kind(&self) -> Option<ResourcePolicyKind> {
        self.primary_incompatible_kind
    }

    pub fn compatibility(&self) -> &ResourcePolicyCompatibilityReport {
        &self.compatibility
    }

    pub fn compatibility_digest(&self) -> &ResourcePolicyDigest {
        self.compatibility.compatibility_digest()
    }

    pub fn replay_decision_class(&self) -> ResourceReplayDecisionClass {
        self.replay_decision_class
    }

    pub fn replay_decision_descriptor_id(&self) -> ResourcePolicyDescriptorId {
        self.replay_decision_descriptor_id
    }

    pub fn replay_decision_digest(&self) -> &ResourcePolicyDigest {
        &self.replay_decision_digest
    }

    pub fn incompatible_width(&self) -> u32 {
        self.compatibility.incompatible_width()
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.compatibility.performance()
    }
}

impl ResourcePolicyCompatibilityReport {
    pub fn classify_against_validated_declaration(
        descriptor_id: ResourceDescriptorId,
        node: ResourceNodeId,
        historical: &LoweredResourcePolicyBundle,
        current: &ValidatedResourcePolicyDeclaration,
        registry: &FrozenResourcePolicyRegistry,
    ) -> Result<Self, ResourcePolicyResolutionError> {
        let current_frozen =
            FrozenResourcePolicyDescriptorSet::from_validated_declaration(current, registry)?;
        let families = vec![
            ResourcePolicyCompatibilityFamilyReport::classify(
                historical.retry(),
                current_frozen.retry(),
                registry,
            ),
            ResourcePolicyCompatibilityFamilyReport::classify(
                historical.timeout(),
                current_frozen.timeout(),
                registry,
            ),
            ResourcePolicyCompatibilityFamilyReport::classify(
                historical.cancellation(),
                current_frozen.cancellation(),
                registry,
            ),
            ResourcePolicyCompatibilityFamilyReport::classify(
                historical.stale_after(),
                current_frozen.stale_after(),
                registry,
            ),
            ResourcePolicyCompatibilityFamilyReport::classify(
                historical.supersession(),
                current_frozen.supersession(),
                registry,
            ),
            ResourcePolicyCompatibilityFamilyReport::classify(
                historical.revalidation(),
                current_frozen.revalidation(),
                registry,
            ),
            ResourcePolicyCompatibilityFamilyReport::classify(
                historical.observation(),
                current_frozen.observation(),
                registry,
            ),
            ResourcePolicyCompatibilityFamilyReport::classify(
                historical.output_continuity(),
                current_frozen.output_continuity(),
                registry,
            ),
            ResourcePolicyCompatibilityFamilyReport::classify(
                historical.retention(),
                current_frozen.retention(),
                registry,
            ),
            ResourcePolicyCompatibilityFamilyReport::classify(
                historical.diagnostics(),
                current_frozen.diagnostics(),
                registry,
            ),
        ];
        let compared_width = families.len() as u32;
        let incompatible_width = families
            .iter()
            .filter(|family| !family.class().is_compatible())
            .count() as u32;
        let compatibility_digest = compatibility_digest(
            historical.registry_digest(),
            current_frozen.registry_digest(),
            &families,
        );
        let performance = ResourceBoundaryPerformanceEnvelope::policy_compatibility(
            compared_width,
            incompatible_width,
        );

        Ok(Self {
            descriptor_id,
            node,
            compared_width,
            incompatible_width,
            historical_registry_digest: historical.registry_digest().clone(),
            current_registry_digest: current_frozen.registry_digest().clone(),
            families,
            compatibility_digest,
            performance,
        })
    }

    pub fn descriptor_id(&self) -> ResourceDescriptorId {
        self.descriptor_id
    }

    pub fn node(&self) -> ResourceNodeId {
        self.node
    }

    pub fn compared_width(&self) -> u32 {
        self.compared_width
    }

    pub fn incompatible_width(&self) -> u32 {
        self.incompatible_width
    }

    pub fn historical_registry_digest(&self) -> &ResourcePolicyDigest {
        &self.historical_registry_digest
    }

    pub fn current_registry_digest(&self) -> &ResourcePolicyDigest {
        &self.current_registry_digest
    }

    pub fn is_compatible(&self) -> bool {
        self.incompatible_width == 0
    }

    pub fn compatibility_digest(&self) -> &ResourcePolicyDigest {
        &self.compatibility_digest
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn families(&self) -> &[ResourcePolicyCompatibilityFamilyReport] {
        &self.families
    }

    pub fn family(
        &self,
        kind: ResourcePolicyKind,
    ) -> Option<&ResourcePolicyCompatibilityFamilyReport> {
        self.families.iter().find(|family| family.kind() == kind)
    }

    pub fn canonical_truth_preserved_width(&self) -> u32 {
        self.families
            .iter()
            .filter(|family| family.canonical_truth_preserved())
            .count() as u32
    }

    pub fn retained_history_unavailable_width(&self) -> u32 {
        self.families
            .iter()
            .filter(|family| family.retained_history_unavailable())
            .count() as u32
    }

    pub fn diagnostics_details_unavailable_width(&self) -> u32 {
        self.families
            .iter()
            .filter(|family| family.diagnostics_details_unavailable())
            .count() as u32
    }
}

fn classify_family_compatibility(
    historical: &FrozenResourcePolicyDescriptor,
    current: &FrozenResourcePolicyDescriptor,
    registry: &FrozenResourcePolicyRegistry,
) -> (
    ResourcePolicyCompatibilityClass,
    Option<ResourceRetentionDecisionClass>,
    Option<ResourceRetentionDecisionClass>,
    Option<ResourceDiagnosticsDecisionClass>,
    Option<ResourceDiagnosticsDecisionClass>,
    Vec<String>,
    bool,
    bool,
    bool,
) {
    let Some(registry_descriptor) = registry.resolve_by_id(historical.descriptor().id()) else {
        return (
            ResourcePolicyCompatibilityClass::MissingDescriptor,
            retention_class(historical),
            None,
            diagnostics_class(historical),
            None,
            Vec::new(),
            false,
            false,
            false,
        );
    };

    if registry_descriptor.descriptor_digest() != historical.descriptor().descriptor_digest() {
        return classify_descriptor_drift(
            historical,
            current,
            registry_descriptor.compatibility_posture(),
        );
    }
    if current.descriptor().descriptor_digest() != historical.descriptor().descriptor_digest() {
        return classify_descriptor_drift(
            historical,
            current,
            current.descriptor().compatibility_posture(),
        );
    }
    if current.frozen_digest() != historical.frozen_digest() {
        return (
            ResourcePolicyCompatibilityClass::ParameterDigestDrift,
            retention_class(historical),
            retention_class(current),
            diagnostics_class(historical),
            diagnostics_class(current),
            Vec::new(),
            false,
            false,
            false,
        );
    }
    (
        ResourcePolicyCompatibilityClass::ExactDescriptorMatch,
        retention_class(historical),
        retention_class(current),
        diagnostics_class(historical),
        diagnostics_class(current),
        Vec::new(),
        true,
        false,
        false,
    )
}

fn classify_descriptor_drift(
    historical: &FrozenResourcePolicyDescriptor,
    current: &FrozenResourcePolicyDescriptor,
    posture: ResourcePolicyCompatibilityPosture,
) -> (
    ResourcePolicyCompatibilityClass,
    Option<ResourceRetentionDecisionClass>,
    Option<ResourceRetentionDecisionClass>,
    Option<ResourceDiagnosticsDecisionClass>,
    Option<ResourceDiagnosticsDecisionClass>,
    Vec<String>,
    bool,
    bool,
    bool,
) {
    match posture {
        ResourcePolicyCompatibilityPosture::IncompatibleVersion => (
            ResourcePolicyCompatibilityClass::VersionIncompatible,
            retention_class(historical),
            retention_class(current),
            diagnostics_class(historical),
            diagnostics_class(current),
            Vec::new(),
            false,
            false,
            false,
        ),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch => (
            ResourcePolicyCompatibilityClass::DecisionSemanticsDrift,
            retention_class(historical),
            retention_class(current),
            diagnostics_class(historical),
            diagnostics_class(current),
            Vec::new(),
            false,
            false,
            false,
        ),
        ResourcePolicyCompatibilityPosture::CompatibleVersion => {
            classify_compatible_version_drift(historical, current)
        }
    }
}

fn classify_compatible_version_drift(
    historical: &FrozenResourcePolicyDescriptor,
    current: &FrozenResourcePolicyDescriptor,
) -> (
    ResourcePolicyCompatibilityClass,
    Option<ResourceRetentionDecisionClass>,
    Option<ResourceRetentionDecisionClass>,
    Option<ResourceDiagnosticsDecisionClass>,
    Option<ResourceDiagnosticsDecisionClass>,
    Vec<String>,
    bool,
    bool,
    bool,
) {
    match historical.descriptor().kind() {
        ResourcePolicyKind::Retention => classify_retention_compatible_drift(historical, current),
        ResourcePolicyKind::Diagnostics => {
            classify_diagnostics_parameter_or_richness_drift(historical, current)
        }
        _ => (
            ResourcePolicyCompatibilityClass::DecisionSemanticsDrift,
            retention_class(historical),
            retention_class(current),
            diagnostics_class(historical),
            diagnostics_class(current),
            Vec::new(),
            false,
            false,
            false,
        ),
    }
}

fn classify_diagnostics_parameter_or_richness_drift(
    historical: &FrozenResourcePolicyDescriptor,
    current: &FrozenResourcePolicyDescriptor,
) -> (
    ResourcePolicyCompatibilityClass,
    Option<ResourceRetentionDecisionClass>,
    Option<ResourceRetentionDecisionClass>,
    Option<ResourceDiagnosticsDecisionClass>,
    Option<ResourceDiagnosticsDecisionClass>,
    Vec<String>,
    bool,
    bool,
    bool,
) {
    if let Some(expansion) = classify_diagnostics_parameter_expansion(historical, current) {
        return expansion;
    }
    classify_diagnostics_compatible_drift(historical, current)
}

fn classify_diagnostics_parameter_expansion(
    historical: &FrozenResourcePolicyDescriptor,
    current: &FrozenResourcePolicyDescriptor,
) -> Option<(
    ResourcePolicyCompatibilityClass,
    Option<ResourceRetentionDecisionClass>,
    Option<ResourceRetentionDecisionClass>,
    Option<ResourceDiagnosticsDecisionClass>,
    Option<ResourceDiagnosticsDecisionClass>,
    Vec<String>,
    bool,
    bool,
    bool,
)> {
    let historical_name = historical.descriptor().semantic_name().as_str();
    let current_name = current.descriptor().semantic_name().as_str();
    if historical_name != "signal.resource.diagnostics.budgeted-expansion"
        || current_name != "signal.resource.diagnostics.forensic-expansion-budget"
    {
        return None;
    }

    let historical_replay_width = diagnostics_replay_width(historical)?;
    let current_replay_width = diagnostics_replay_width(current)?;
    let current_forensic_width = diagnostics_forensic_width(current)?;

    let historical_class = diagnostics_class(historical);
    let current_class = diagnostics_class(current);
    if historical_replay_width == current_replay_width
        && current_forensic_width == historical_replay_width
    {
        Some((
            ResourcePolicyCompatibilityClass::CompatibleParameterExpansion,
            retention_class(historical),
            retention_class(current),
            historical_class,
            current_class,
            vec!["max_forensic_reconstruction_width".to_owned()],
            true,
            false,
            false,
        ))
    } else {
        Some((
            ResourcePolicyCompatibilityClass::DecisionSemanticsDrift,
            retention_class(historical),
            retention_class(current),
            historical_class,
            current_class,
            Vec::new(),
            false,
            false,
            false,
        ))
    }
}

fn classify_retention_compatible_drift(
    historical: &FrozenResourcePolicyDescriptor,
    current: &FrozenResourcePolicyDescriptor,
) -> (
    ResourcePolicyCompatibilityClass,
    Option<ResourceRetentionDecisionClass>,
    Option<ResourceRetentionDecisionClass>,
    Option<ResourceDiagnosticsDecisionClass>,
    Option<ResourceDiagnosticsDecisionClass>,
    Vec<String>,
    bool,
    bool,
    bool,
) {
    let historical_class = retention_class(historical);
    let current_class = retention_class(current);
    if matches!(
        historical_class,
        Some(ResourceRetentionDecisionClass::RetainAllTransitions)
    ) && current_class.is_some()
        && current_class != historical_class
    {
        (
            ResourcePolicyCompatibilityClass::CompatibleRetentionNarrowing,
            historical_class,
            current_class,
            diagnostics_class(historical),
            diagnostics_class(current),
            Vec::new(),
            true,
            true,
            false,
        )
    } else {
        (
            ResourcePolicyCompatibilityClass::DecisionSemanticsDrift,
            historical_class,
            current_class,
            diagnostics_class(historical),
            diagnostics_class(current),
            Vec::new(),
            false,
            false,
            false,
        )
    }
}

fn classify_diagnostics_compatible_drift(
    historical: &FrozenResourcePolicyDescriptor,
    current: &FrozenResourcePolicyDescriptor,
) -> (
    ResourcePolicyCompatibilityClass,
    Option<ResourceRetentionDecisionClass>,
    Option<ResourceRetentionDecisionClass>,
    Option<ResourceDiagnosticsDecisionClass>,
    Option<ResourceDiagnosticsDecisionClass>,
    Vec<String>,
    bool,
    bool,
    bool,
) {
    let historical_class = diagnostics_class(historical);
    let current_class = diagnostics_class(current);
    if historical_class.is_some() && current_class.is_some() && historical_class != current_class {
        let diagnostics_details_unavailable = matches!(
            current_class,
            Some(
                ResourceDiagnosticsDecisionClass::RetainedOnly
                    | ResourceDiagnosticsDecisionClass::DenyColdExpansion
            )
        ) && !matches!(
            historical_class,
            Some(
                ResourceDiagnosticsDecisionClass::RetainedOnly
                    | ResourceDiagnosticsDecisionClass::DenyColdExpansion
            )
        );
        (
            ResourcePolicyCompatibilityClass::CompatibleDiagnosticsRichnessChange,
            retention_class(historical),
            retention_class(current),
            historical_class,
            current_class,
            Vec::new(),
            true,
            false,
            diagnostics_details_unavailable,
        )
    } else {
        (
            ResourcePolicyCompatibilityClass::DecisionSemanticsDrift,
            retention_class(historical),
            retention_class(current),
            historical_class,
            current_class,
            Vec::new(),
            false,
            false,
            false,
        )
    }
}

fn retention_class(
    frozen: &FrozenResourcePolicyDescriptor,
) -> Option<ResourceRetentionDecisionClass> {
    match frozen.descriptor().semantic_name().as_str() {
        "signal.resource.retention.retain-all-transitions" => {
            Some(ResourceRetentionDecisionClass::RetainAllTransitions)
        }
        "signal.resource.retention.terminal-summaries-only" => {
            Some(ResourceRetentionDecisionClass::TerminalSummariesOnly)
        }
        "signal.resource.retention.compact-superseded" => {
            Some(ResourceRetentionDecisionClass::CompactSuperseded)
        }
        "signal.resource.retention.compact-cancelled" => {
            Some(ResourceRetentionDecisionClass::CompactCancelled)
        }
        "signal.resource.retention.compact-timed-out" => {
            Some(ResourceRetentionDecisionClass::CompactTimedOut)
        }
        _ => None,
    }
}

fn diagnostics_class(
    frozen: &FrozenResourcePolicyDescriptor,
) -> Option<ResourceDiagnosticsDecisionClass> {
    match frozen.descriptor().semantic_name().as_str() {
        "signal.resource.diagnostics.retained-only" => {
            Some(ResourceDiagnosticsDecisionClass::RetainedOnly)
        }
        "signal.resource.diagnostics.budgeted-expansion" => {
            Some(ResourceDiagnosticsDecisionClass::BudgetedExpansion)
        }
        "signal.resource.diagnostics.forensic-expansion-budget" => {
            Some(ResourceDiagnosticsDecisionClass::ForensicExpansionBudget)
        }
        "signal.resource.diagnostics.deny-cold-expansion" => {
            Some(ResourceDiagnosticsDecisionClass::DenyColdExpansion)
        }
        _ => None,
    }
}

fn compatibility_digest(
    historical_registry_digest: &ResourcePolicyDigest,
    current_registry_digest: &ResourcePolicyDigest,
    families: &[ResourcePolicyCompatibilityFamilyReport],
) -> ResourcePolicyDigest {
    let joined = families
        .iter()
        .map(|family| {
            format!(
                "{:?}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
                family.kind(),
                family.class().as_str(),
                family.historical_descriptor_id().get(),
                family
                    .current_descriptor_id()
                    .map(ResourcePolicyDescriptorId::get)
                    .unwrap_or(u64::MAX),
                version_string(family.historical_version()),
                family
                    .current_version()
                    .map(version_string)
                    .unwrap_or_else(|| "missing".to_owned()),
                family.historical_frozen_digest().as_str(),
                family
                    .current_frozen_digest()
                    .map(ResourcePolicyDigest::as_str)
                    .unwrap_or("missing"),
                family
                    .historical_retention_class()
                    .map(retention_class_str)
                    .unwrap_or("none"),
                family
                    .current_retention_class()
                    .map(retention_class_str)
                    .unwrap_or("none"),
                family
                    .historical_diagnostics_class()
                    .map(diagnostics_class_str)
                    .unwrap_or("none"),
                family
                    .current_diagnostics_class()
                    .map(diagnostics_class_str)
                    .unwrap_or("none"),
                family.defaulted_parameter_names().join(","),
                family.canonical_truth_preserved(),
                family.retained_history_unavailable(),
                family.diagnostics_details_unavailable()
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    ResourcePolicyDigest::new(format!(
        "resource-policy-compatibility:{}:{}:{joined}",
        historical_registry_digest.as_str(),
        current_registry_digest.as_str()
    ))
}

impl ResourcePolicyCompatibilityClass {
    fn as_str(self) -> &'static str {
        match self {
            Self::ExactDescriptorMatch => "exact-descriptor-match",
            Self::CompatibleParameterExpansion => "compatible-parameter-expansion",
            Self::CompatibleRetentionNarrowing => "compatible-retention-narrowing",
            Self::CompatibleDiagnosticsRichnessChange => "compatible-diagnostics-richness-change",
            Self::MissingDescriptor => "missing-descriptor",
            Self::VersionIncompatible => "version-incompatible",
            Self::ParameterDigestDrift => "parameter-digest-drift",
            Self::DecisionSemanticsDrift => "decision-semantics-drift",
        }
    }
}

fn version_string(version: ResourcePolicyVersion) -> String {
    format!("{}.{}", version.major(), version.minor())
}

fn retention_class_str(class: ResourceRetentionDecisionClass) -> &'static str {
    match class {
        ResourceRetentionDecisionClass::RetainAllTransitions => "retain-all-transitions",
        ResourceRetentionDecisionClass::TerminalSummariesOnly => "terminal-summaries-only",
        ResourceRetentionDecisionClass::CompactSuperseded => "compact-superseded",
        ResourceRetentionDecisionClass::CompactCancelled => "compact-cancelled",
        ResourceRetentionDecisionClass::CompactTimedOut => "compact-timed-out",
    }
}

fn diagnostics_class_str(class: ResourceDiagnosticsDecisionClass) -> &'static str {
    match class {
        ResourceDiagnosticsDecisionClass::RetainedOnly => "retained-only",
        ResourceDiagnosticsDecisionClass::BudgetedExpansion => "budgeted-expansion",
        ResourceDiagnosticsDecisionClass::ForensicExpansionBudget => "forensic-expansion-budget",
        ResourceDiagnosticsDecisionClass::DenyColdExpansion => "deny-cold-expansion",
    }
}

fn diagnostics_replay_width(frozen: &FrozenResourcePolicyDescriptor) -> Option<u32> {
    let digest = frozen.parameter_digest().as_str();
    parse_diagnostics_width(digest, "max-replay-reconstruction-width:")
}

fn diagnostics_forensic_width(frozen: &FrozenResourcePolicyDescriptor) -> Option<u32> {
    let digest = frozen.parameter_digest().as_str();
    parse_diagnostics_width(digest, "max-forensic-reconstruction-width:")
}

fn parse_diagnostics_width(digest: &str, marker: &str) -> Option<u32> {
    let suffix = digest.split(marker).nth(1)?;
    let value = suffix.split(':').next()?;
    if value == "none" {
        None
    } else {
        value.parse().ok()
    }
}
