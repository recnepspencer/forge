use worth_store_physical_format::RootSelectorIdentity;

use super::super::{
    PhysicalArtifactScope, PhysicalIntegrityValidationDigest, PhysicalIntegrityValidationMechanism,
    PhysicalIntegrityValidationRecord, UntrustedPhysicalArtifact,
};

#[derive(Debug)]
pub struct IntegrityValidatedCurrentRootSelector<'media> {
    scope: PhysicalArtifactScope,
    selector_identity: RootSelectorIdentity,
    root_generation: u64,
    linked_selector: Option<RootSelectorIdentity>,
    linked_root_generation: Option<u64>,
    validation_record: PhysicalIntegrityValidationRecord,
    inspected: UntrustedPhysicalArtifact<'media>,
}

impl<'media> IntegrityValidatedCurrentRootSelector<'media> {
    pub(crate) fn new(
        scope: PhysicalArtifactScope,
        selector_identity: RootSelectorIdentity,
        root_generation: u64,
        linked_selector: Option<RootSelectorIdentity>,
        linked_root_generation: Option<u64>,
        exact_scope_digest: u32,
        validated_range_checksum: u32,
        inspected: UntrustedPhysicalArtifact<'media>,
    ) -> Option<Self> {
        if !scope.is_current_selector()
            || root_generation == 0
            || linked_selector.is_some() != linked_root_generation.is_some()
            || inspected.byte_count() != scope.byte_range().length()
        {
            return None;
        }
        let validation_record = PhysicalIntegrityValidationRecord::from_validated_scope(
            scope,
            PhysicalIntegrityValidationDigest::crc32c(exact_scope_digest),
            PhysicalIntegrityValidationDigest::crc32c(validated_range_checksum),
            PhysicalIntegrityValidationMechanism::Crc32cV1,
        )?;
        Some(Self {
            scope,
            selector_identity,
            root_generation,
            linked_selector,
            linked_root_generation,
            validation_record,
            inspected,
        })
    }

    pub const fn scope(&self) -> PhysicalArtifactScope {
        self.scope
    }

    pub const fn selector_identity(&self) -> RootSelectorIdentity {
        self.selector_identity
    }

    pub const fn root_generation(&self) -> u64 {
        self.root_generation
    }

    pub const fn linked_selector(&self) -> Option<RootSelectorIdentity> {
        self.linked_selector
    }

    pub const fn linked_root_generation(&self) -> Option<u64> {
        self.linked_root_generation
    }

    pub const fn into_validation_record(self) -> PhysicalIntegrityValidationRecord {
        self.validation_record
    }

    /// Matches the exact immutable slice incarnation inspected by validation.
    /// It exposes no bytes and grants no decoder authority.
    pub fn matches_input(&self, input: UntrustedPhysicalArtifact<'media>) -> bool {
        self.inspected.same_incarnation(input)
    }
}
