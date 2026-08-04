use super::{
    ResourceBoundaryKind, ResourceBoundaryPerformanceEnvelope, ResourceCostContractId,
    ResourceCostPosture, ResourceDensityStrategy,
};

impl ResourceBoundaryPerformanceEnvelope {
    pub(crate) fn declaration_lowering(input_width: u32) -> Self {
        Self::new(
            ResourceBoundaryKind::DeclarationLowering,
            input_width,
            input_width,
            input_width,
            0,
            0,
            0,
            0,
            input_width,
            0,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(0),
            ResourceCostPosture::Verified,
        )
    }
}
