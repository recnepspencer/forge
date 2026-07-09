use super::yieldpoint::PhysicalBoundaryYieldpoint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YieldpointPauseReceipt {
    yieldpoint: PhysicalBoundaryYieldpoint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YieldpointResumeReceipt {
    yieldpoint: PhysicalBoundaryYieldpoint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YieldpointObservationReceipt {
    yieldpoint: PhysicalBoundaryYieldpoint,
}

impl YieldpointPauseReceipt {
    pub const fn yieldpoint(&self) -> &PhysicalBoundaryYieldpoint {
        &self.yieldpoint
    }
}

impl YieldpointResumeReceipt {
    pub const fn yieldpoint(&self) -> &PhysicalBoundaryYieldpoint {
        &self.yieldpoint
    }
}

impl YieldpointObservationReceipt {
    pub const fn yieldpoint(&self) -> &PhysicalBoundaryYieldpoint {
        &self.yieldpoint
    }
}
