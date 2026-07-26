use crate::{
    PhysicalOperationAllocationScope, PhysicalResidencyDimension, PhysicalSpeculativeWorkKind,
};

use super::super::{speculative_index, PhysicalResidencyAllocationEventRecorder};
use super::PhysicalResidencyAccounting;

impl Drop for PhysicalResidencyAccounting {
    fn drop(&mut self) {
        self.release_bytes();
        self.release_counts();
        for scope in operation_scopes() {
            release_if_active(
                &self.events,
                PhysicalResidencyDimension::OperationScope(scope),
                self.counters.operation_scope_bytes[scope.index()],
            );
        }
        for kind in speculative_kinds() {
            release_if_active(
                &self.events,
                PhysicalResidencyDimension::SpeculativeFrames(kind),
                u64::from(self.counters.active_speculative_frames[speculative_index(kind)]),
            );
        }
        release_if_active(
            &self.events,
            PhysicalResidencyDimension::TotalBytes,
            self.counters.admitted_bytes(),
        );
    }
}

impl PhysicalResidencyAccounting {
    fn release_bytes(&self) {
        for (dimension, units) in [
            (
                PhysicalResidencyDimension::MetadataBytes,
                self.counters.metadata_bytes,
            ),
            (
                PhysicalResidencyDimension::ResidentBytes,
                self.counters.resident_bytes,
            ),
            (
                PhysicalResidencyDimension::DirtyReplacementBytes,
                self.counters.dirty_replacement_bytes,
            ),
            (
                PhysicalResidencyDimension::OperationBytes,
                self.counters.active_operation_bytes,
            ),
        ] {
            release_if_active(&self.events, dimension, units);
        }
    }

    fn release_counts(&self) {
        for (dimension, units) in [
            (
                PhysicalResidencyDimension::FrameEntries,
                u64::from(self.counters.frame_entries),
            ),
            (
                PhysicalResidencyDimension::PinnedFrames,
                u64::from(self.counters.pinned_frames),
            ),
            (
                PhysicalResidencyDimension::PinLeases,
                u64::from(self.counters.pin_leases),
            ),
            (
                PhysicalResidencyDimension::DirtyFrames,
                u64::from(self.counters.dirty_frames),
            ),
        ] {
            release_if_active(&self.events, dimension, units);
        }
    }
}

fn release_if_active(
    events: &PhysicalResidencyAllocationEventRecorder,
    dimension: PhysicalResidencyDimension,
    units: u64,
) {
    if units != 0 {
        events.release(dimension, units);
    }
}

const fn operation_scopes() -> [PhysicalOperationAllocationScope; 7] {
    use PhysicalOperationAllocationScope as Scope;
    [
        Scope::ForegroundRead,
        Scope::ForegroundWrite,
        Scope::Recovery,
        Scope::Scrub,
        Scope::Maintenance,
        Scope::Verification,
        Scope::Blob,
    ]
}

const fn speculative_kinds() -> [PhysicalSpeculativeWorkKind; 3] {
    use PhysicalSpeculativeWorkKind as Kind;
    [Kind::ReadAhead, Kind::Prefetch, Kind::WriteBehind]
}
