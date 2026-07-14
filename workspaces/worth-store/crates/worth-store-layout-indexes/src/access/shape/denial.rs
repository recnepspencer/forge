use super::kind::AccessShape;
use super::lane::AccessLaneClassification;
use crate::maintenance::PhysicalMutationShape;
use crate::materialization::MaterializationDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessShapeUnsupportedDenial {
    MaterializationDenied(MaterializationDenial),
    #[non_exhaustive]
    HiddenBroadScan {
        requested_shape: AccessShape,
    },
    LaneDoesNotSupportShape {
        shape: AccessShape,
        lane: AccessLaneClassification,
    },
    MutationShapeDoesNotSupportAccessShape {
        requested_shape: AccessShape,
        mutation_shape: PhysicalMutationShape,
    },
    ExplicitDegradedExactScanRequired {
        requested_shape: AccessShape,
    },
    DegradedExactScanBudgetRequired,
}
