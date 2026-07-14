use super::ForegroundIoLaneKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForegroundFairnessClass {
    PointRead,
    RangeRead,
    CommitCriticalWalWrite,
    OrdinaryPageWrite,
    InteractiveRead,
    InternalForegroundRead,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForegroundFairnessDenial {
    PriorityLaundering {
        declared: ForegroundIoLaneKind,
        attempted: ForegroundIoLaneKind,
    },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ForegroundArbitrationPolicy;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForegroundArbitrationDeclaration {
    declared_lane: ForegroundIoLaneKind,
    fairness_class: ForegroundFairnessClass,
}

impl ForegroundArbitrationDeclaration {
    pub const fn for_lane(declared_lane: ForegroundIoLaneKind) -> Self {
        Self {
            declared_lane,
            fairness_class: ForegroundArbitrationPolicy::class_for(declared_lane),
        }
    }

    pub const fn declared_lane(self) -> ForegroundIoLaneKind {
        self.declared_lane
    }

    pub const fn fairness_class(self) -> ForegroundFairnessClass {
        self.fairness_class
    }
}

impl ForegroundArbitrationPolicy {
    pub const fn class_for(lane: ForegroundIoLaneKind) -> ForegroundFairnessClass {
        match lane {
            ForegroundIoLaneKind::PointRead => ForegroundFairnessClass::PointRead,
            ForegroundIoLaneKind::RangeRead => ForegroundFairnessClass::RangeRead,
            ForegroundIoLaneKind::CommitCriticalWalWrite => {
                ForegroundFairnessClass::CommitCriticalWalWrite
            }
            ForegroundIoLaneKind::OrdinaryPageWrite => ForegroundFairnessClass::OrdinaryPageWrite,
            ForegroundIoLaneKind::InteractiveRead => ForegroundFairnessClass::InteractiveRead,
            ForegroundIoLaneKind::InternalForegroundRead => {
                ForegroundFairnessClass::InternalForegroundRead
            }
        }
    }

    pub const fn reject_priority_laundering(
        declared: ForegroundIoLaneKind,
        attempted: ForegroundIoLaneKind,
    ) -> Result<(), ForegroundFairnessDenial> {
        if declared as u8 == attempted as u8 {
            Ok(())
        } else {
            Err(ForegroundFairnessDenial::PriorityLaundering {
                declared,
                attempted,
            })
        }
    }
}
