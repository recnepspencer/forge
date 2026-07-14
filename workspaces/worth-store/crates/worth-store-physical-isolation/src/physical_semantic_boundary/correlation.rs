use worth_proof::{CapabilityMarker, CapabilityWitness};

use super::{PhysicalReadStabilityCorrelationBasis, SemanticVisibilityReference};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticCorrelationCapability(());

impl CapabilityMarker for SemanticCorrelationCapability {}

#[derive(Debug, Clone)]
pub struct PhysicalSnapshotCorrelation {
    semantic: SemanticVisibilityReference,
    physical: PhysicalReadStabilityCorrelationBasis,
}

pub fn correlate_semantic_visibility_with_physical_snapshot(
    semantic: SemanticVisibilityReference,
    physical: PhysicalReadStabilityCorrelationBasis,
) -> Result<PhysicalSnapshotCorrelation, core::convert::Infallible> {
    let capability = CapabilityWitness::from_capability_marker(SemanticCorrelationCapability(()));
    Ok(PhysicalSnapshotCorrelation::new(
        semantic, physical, capability,
    ))
}

impl PhysicalSnapshotCorrelation {
    fn new(
        semantic: SemanticVisibilityReference,
        physical: PhysicalReadStabilityCorrelationBasis,
        _capability: CapabilityWitness<SemanticCorrelationCapability>,
    ) -> Self {
        Self { semantic, physical }
    }

    pub const fn semantic(&self) -> &SemanticVisibilityReference {
        &self.semantic
    }

    pub const fn physical(&self) -> &PhysicalReadStabilityCorrelationBasis {
        &self.physical
    }

    pub const fn is_diagnostic_only(&self) -> bool {
        true
    }

    pub const fn is_store_physical_stability_authority(&self) -> bool {
        false
    }
}
