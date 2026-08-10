use serde::Serialize;

use crate::data::resource::descriptor::ResourceDescriptorId;
use crate::data::resource::policy_registry::{
    FrozenResourcePolicyDescriptorSet, FrozenResourcePolicyRegistry, LoweredResourcePolicyBundle,
    ResourcePolicyDigest, ResourcePolicyKind, ResourcePolicyResolutionError,
    ValidatedResourcePolicyDeclaration,
};
use crate::data::resource::request::ResourceNodeId;
use crate::data::resource::summary::ResourceBoundaryPerformanceEnvelope;

use super::classification::ResourcePolicyCompatibilityFamilyReport;
use super::digest;
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
        let compatibility_digest = digest::compatibility_digest(
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
