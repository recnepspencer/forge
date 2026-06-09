use crate::construction::outcome::PrimitiveConstructionRejectionLocality;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PrimitiveConstructionBlockingBoundary {
    KernelIntent,
    SpatialBirth,
    TopologyLegality,
    PrimitiveClassAdmission,
}

impl PrimitiveConstructionBlockingBoundary {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::KernelIntent => "kernel_intent",
            Self::SpatialBirth => "spatial_birth",
            Self::TopologyLegality => "topology_legality",
            Self::PrimitiveClassAdmission => "primitive_class_admission",
        }
    }
}

pub(crate) fn blocking_boundary_for(
    rejection_locality: PrimitiveConstructionRejectionLocality,
) -> PrimitiveConstructionBlockingBoundary {
    match rejection_locality {
        PrimitiveConstructionRejectionLocality::Admission => {
            PrimitiveConstructionBlockingBoundary::PrimitiveClassAdmission
        }
        PrimitiveConstructionRejectionLocality::Scaffold => {
            PrimitiveConstructionBlockingBoundary::KernelIntent
        }
        PrimitiveConstructionRejectionLocality::SpatialBirth => {
            PrimitiveConstructionBlockingBoundary::SpatialBirth
        }
        PrimitiveConstructionRejectionLocality::Execution => {
            PrimitiveConstructionBlockingBoundary::TopologyLegality
        }
    }
}
