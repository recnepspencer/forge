use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarBooleanCommonPlaneReducedOperandPairOrderingContract {
    first_slot_side: PlanarBooleanCommonPlaneOperandSide,
    second_slot_side: PlanarBooleanCommonPlaneOperandSide,
}

impl PlanarBooleanCommonPlaneReducedOperandPairOrderingContract {
    pub fn semantic_left_right() -> Self {
        Self {
            first_slot_side: PlanarBooleanCommonPlaneOperandSide::Left,
            second_slot_side: PlanarBooleanCommonPlaneOperandSide::Right,
        }
    }

    pub fn first_slot_side(self) -> PlanarBooleanCommonPlaneOperandSide {
        self.first_slot_side
    }

    pub fn second_slot_side(self) -> PlanarBooleanCommonPlaneOperandSide {
        self.second_slot_side
    }

    pub fn semantic_left_side(self) -> PlanarBooleanCommonPlaneOperandSide {
        PlanarBooleanCommonPlaneOperandSide::Left
    }

    pub fn semantic_right_side(self) -> PlanarBooleanCommonPlaneOperandSide {
        PlanarBooleanCommonPlaneOperandSide::Right
    }
}
