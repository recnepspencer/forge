use crate::capability::{
    FrozenCommandCapabilities, FrozenComponentCapabilities, FrozenMosaicRegionCapabilities,
    FrozenSurfaceCapabilities, RegisteredCapabilitySet,
};

use super::{CapabilitySnapshotDigest, SnapshotMetrics};

/// Immutable capability snapshot consumed by later lowering phases.
#[derive(Debug, Eq, PartialEq)]
pub struct CapabilitySnapshot {
    registered_capabilities: RegisteredCapabilitySet,
    commands: FrozenCommandCapabilities,
    components: FrozenComponentCapabilities,
    surfaces: FrozenSurfaceCapabilities,
    mosaic_regions: FrozenMosaicRegionCapabilities,
    digest: CapabilitySnapshotDigest,
    metrics: SnapshotMetrics,
}

impl CapabilitySnapshot {
    #[allow(dead_code)]
    pub(crate) fn from_registered_capabilities(
        registered_capabilities: RegisteredCapabilitySet,
    ) -> Self {
        Self::from_registered_capabilities_and_commands(
            registered_capabilities,
            FrozenCommandCapabilities::empty(),
        )
    }

    pub(crate) fn from_registered_capabilities_and_commands(
        registered_capabilities: RegisteredCapabilitySet,
        commands: FrozenCommandCapabilities,
    ) -> Self {
        Self::from_registered_capabilities_commands_and_components(
            registered_capabilities,
            commands,
            FrozenComponentCapabilities::empty(),
        )
    }

    pub(crate) fn from_registered_capabilities_commands_and_components(
        registered_capabilities: RegisteredCapabilitySet,
        commands: FrozenCommandCapabilities,
        components: FrozenComponentCapabilities,
    ) -> Self {
        Self::from_registered_capabilities_commands_components_and_surfaces(
            registered_capabilities,
            commands,
            components,
            FrozenSurfaceCapabilities::empty(),
        )
    }

    pub(crate) fn from_registered_capabilities_commands_components_and_surfaces(
        registered_capabilities: RegisteredCapabilitySet,
        commands: FrozenCommandCapabilities,
        components: FrozenComponentCapabilities,
        surfaces: FrozenSurfaceCapabilities,
    ) -> Self {
        Self::from_registered_capabilities_commands_components_surfaces_and_mosaic_regions(
            registered_capabilities,
            commands,
            components,
            surfaces,
            FrozenMosaicRegionCapabilities::empty(),
        )
    }

    pub(crate) fn from_registered_capabilities_commands_components_surfaces_and_mosaic_regions(
        registered_capabilities: RegisteredCapabilitySet,
        commands: FrozenCommandCapabilities,
        components: FrozenComponentCapabilities,
        surfaces: FrozenSurfaceCapabilities,
        mosaic_regions: FrozenMosaicRegionCapabilities,
    ) -> Self {
        let metrics = registered_capabilities.snapshot_metrics();
        let digest = CapabilitySnapshotDigest::from_metrics_and_registry_bases(
            metrics,
            commands.digest_basis(),
            components.digest_basis(),
            surfaces.digest_basis(),
            mosaic_regions.digest_basis(),
        );
        Self {
            registered_capabilities,
            commands,
            components,
            surfaces,
            mosaic_regions,
            digest,
            metrics,
        }
    }

    /// Frozen registered capability meaning.
    pub fn registered_capabilities(&self) -> &RegisteredCapabilitySet {
        &self.registered_capabilities
    }

    /// Frozen command capabilities admitted at registration freeze.
    pub fn commands(&self) -> &FrozenCommandCapabilities {
        &self.commands
    }

    /// Frozen component capabilities admitted at registration freeze.
    pub fn components(&self) -> &FrozenComponentCapabilities {
        &self.components
    }

    /// Frozen surface capabilities admitted at registration freeze.
    pub fn surfaces(&self) -> &FrozenSurfaceCapabilities {
        &self.surfaces
    }

    /// Frozen mosaic region kind capabilities admitted at registration freeze.
    pub fn mosaic_regions(&self) -> &FrozenMosaicRegionCapabilities {
        &self.mosaic_regions
    }

    /// Deterministic digest for this frozen capability meaning.
    pub fn digest(&self) -> CapabilitySnapshotDigest {
        self.digest
    }

    /// Structural counters computed at freeze.
    pub fn metrics(&self) -> SnapshotMetrics {
        self.metrics
    }
}
