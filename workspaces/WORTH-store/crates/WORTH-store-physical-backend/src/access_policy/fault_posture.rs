#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MmapFaultHandling {
    LazyFaultsDeniedBeforeExecution,
    FaultsSurfaceAsTypedViolation,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MmapWritebackPosture {
    StoreTrackedDirtyWriteback,
    WritebackDeniedOutsideStore,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MmapVisibilityPosture {
    SharedVisibilityAdmitted,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MmapTruncatePosture {
    TypedFaultOnTruncate,
    TruncateDeniedWhileMapped,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MmapPunchHolePosture {
    TypedFaultOnPunchHole,
    PunchHoleDeniedWhileMapped,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MmapFaultPosture {
    fault: MmapFaultHandling,
    writeback: MmapWritebackPosture,
    visibility: MmapVisibilityPosture,
    truncate: MmapTruncatePosture,
    punch_hole: MmapPunchHolePosture,
    _seal: MmapFaultPostureSeal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MmapFaultPostureSeal;

impl MmapFaultPosture {
    pub(crate) const fn not_mmap() -> Self {
        Self::new(
            MmapFaultHandling::Unsupported,
            MmapWritebackPosture::Unsupported,
            MmapVisibilityPosture::Unsupported,
            MmapTruncatePosture::Unsupported,
            MmapPunchHolePosture::Unsupported,
        )
    }

    pub(crate) const fn new(
        fault: MmapFaultHandling,
        writeback: MmapWritebackPosture,
        visibility: MmapVisibilityPosture,
        truncate: MmapTruncatePosture,
        punch_hole: MmapPunchHolePosture,
    ) -> Self {
        Self {
            fault,
            writeback,
            visibility,
            truncate,
            punch_hole,
            _seal: MmapFaultPostureSeal,
        }
    }

    pub const fn admits_mmap(self) -> bool {
        !matches!(self.fault, MmapFaultHandling::Unsupported)
            && !matches!(self.writeback, MmapWritebackPosture::Unsupported)
            && !matches!(self.visibility, MmapVisibilityPosture::Unsupported)
            && !matches!(self.truncate, MmapTruncatePosture::Unsupported)
            && !matches!(self.punch_hole, MmapPunchHolePosture::Unsupported)
    }

    pub const fn fault(self) -> MmapFaultHandling {
        self.fault
    }

    pub const fn writeback(self) -> MmapWritebackPosture {
        self.writeback
    }

    pub const fn visibility(self) -> MmapVisibilityPosture {
        self.visibility
    }

    pub const fn truncate(self) -> MmapTruncatePosture {
        self.truncate
    }

    pub const fn punch_hole(self) -> MmapPunchHolePosture {
        self.punch_hole
    }
}
