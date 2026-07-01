use crate::{PhysicalBoundarySeam, PhysicalDriverKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverAdmissionDenial {
    EmptyYieldpointName,
    YieldpointSeamNameMismatch {
        actual: String,
        expected: &'static str,
    },
    DuplicateYieldpointName(String),
    DuplicateDriverKind(PhysicalDriverKind),
    IrrelevantYieldpointForDriver {
        driver: PhysicalDriverKind,
        seam: PhysicalBoundarySeam,
    },
    MissingRelevantYieldpoint {
        driver: PhysicalDriverKind,
        yieldpoint: &'static str,
    },
    NoYieldpointsDeclared(PhysicalDriverKind),
    PrivateMutationDriverDenied,
    FakeInMemoryOnlyDriverDenied,
    TestSupportVerdictDriverDenied,
    SleepBasedSchedulingDenied,
}
