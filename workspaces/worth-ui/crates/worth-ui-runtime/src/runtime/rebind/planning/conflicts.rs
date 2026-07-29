use super::{UiRebindPlanTarget, UiRebindSubsystemKind};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiRebindResourceAccess {
    subsystem: UiRebindSubsystemKind,
    target: UiRebindPlanTarget,
}

#[derive(Debug)]
pub struct UiRebindConflictFootprint {
    reads: Box<[UiRebindResourceAccess]>,
    writes: Box<[UiRebindResourceAccess]>,
    invalidations: Box<[UiRebindResourceAccess]>,
}

#[derive(Debug)]
pub struct UiRebindParallelAdmission {
    admitted_subsystems: Box<[UiRebindSubsystemKind]>,
}

impl UiRebindResourceAccess {
    pub(crate) const fn new(subsystem: UiRebindSubsystemKind, target: UiRebindPlanTarget) -> Self {
        Self { subsystem, target }
    }

    pub const fn subsystem(&self) -> UiRebindSubsystemKind {
        self.subsystem
    }

    pub const fn target(&self) -> &UiRebindPlanTarget {
        &self.target
    }
}

impl UiRebindConflictFootprint {
    pub(crate) fn new(
        reads: Vec<UiRebindResourceAccess>,
        writes: Vec<UiRebindResourceAccess>,
        invalidations: Vec<UiRebindResourceAccess>,
    ) -> Self {
        Self {
            reads: canonical(reads),
            writes: canonical(writes),
            invalidations: canonical(invalidations),
        }
    }

    pub fn reads(&self) -> &[UiRebindResourceAccess] {
        &self.reads
    }

    pub fn writes(&self) -> &[UiRebindResourceAccess] {
        &self.writes
    }

    pub fn invalidations(&self) -> &[UiRebindResourceAccess] {
        &self.invalidations
    }
}

impl UiRebindParallelAdmission {
    pub(crate) fn new(mut admitted_subsystems: Vec<UiRebindSubsystemKind>) -> Self {
        admitted_subsystems.sort();
        admitted_subsystems.dedup();
        Self {
            admitted_subsystems: admitted_subsystems.into_boxed_slice(),
        }
    }

    pub fn admitted_subsystems(&self) -> &[UiRebindSubsystemKind] {
        &self.admitted_subsystems
    }
}

fn canonical(mut accesses: Vec<UiRebindResourceAccess>) -> Box<[UiRebindResourceAccess]> {
    accesses.sort();
    accesses.dedup();
    accesses.into_boxed_slice()
}
