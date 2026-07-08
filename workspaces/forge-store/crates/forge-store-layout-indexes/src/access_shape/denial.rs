use super::lane::S8AccessLaneClassification;
use super::shape::S8AccessShape;
use crate::maintenance::S8PhysicalMutationShape;
use crate::materialization::S8MaterializationDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8AccessShapeUnsupportedDenial {
    MaterializationDenied(S8MaterializationDenial),
    HiddenBroadScan {
        requested_shape: S8AccessShape,
    },
    LaneDoesNotSupportShape {
        shape: S8AccessShape,
        lane: S8AccessLaneClassification,
    },
    MutationShapeDoesNotSupportAccessShape {
        requested_shape: S8AccessShape,
        mutation_shape: S8PhysicalMutationShape,
    },
    ExplicitDegradedExactScanRequired {
        requested_shape: S8AccessShape,
    },
    DegradedExactScanBudgetRequired,
}
