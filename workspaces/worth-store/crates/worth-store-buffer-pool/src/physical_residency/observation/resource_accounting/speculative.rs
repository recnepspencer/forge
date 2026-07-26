use crate::{PhysicalResidencyDimension, PhysicalSpeculativeWorkKind};

use super::super::speculative_index;
use super::PhysicalResidencyAccounting;

impl PhysicalResidencyAccounting {
    pub(crate) fn attempt_speculative(&mut self, kind: PhysicalSpeculativeWorkKind) {
        self.counters.speculative_attempts[speculative_index(kind)] += 1;
    }

    pub(crate) fn record_speculative_denial(&mut self, kind: PhysicalSpeculativeWorkKind) {
        self.counters.speculative_denials[speculative_index(kind)] += 1;
    }

    pub(crate) fn admit_speculative(&mut self, kind: PhysicalSpeculativeWorkKind, frames: u32) {
        let index = speculative_index(kind);
        self.counters.active_speculative_frames[index] += frames;
        self.counters.peak_speculative_frames[index] = self.counters.peak_speculative_frames[index]
            .max(self.counters.active_speculative_frames[index]);
        self.counters.speculative_admissions[index] += 1;
        self.events.admit(
            PhysicalResidencyDimension::SpeculativeFrames(kind),
            u64::from(frames),
        );
    }

    pub(crate) fn release_speculative(&mut self, kind: PhysicalSpeculativeWorkKind, frames: u32) {
        let index = speculative_index(kind);
        self.counters.active_speculative_frames[index] -= frames;
        self.counters.speculative_completions[index] += 1;
        self.events.release(
            PhysicalResidencyDimension::SpeculativeFrames(kind),
            u64::from(frames),
        );
    }

    pub(crate) fn claim_writeback(&mut self, frames: u32) {
        self.counters.active_writeback_claims += frames;
        self.counters.peak_writeback_claims = self
            .counters
            .peak_writeback_claims
            .max(self.counters.active_writeback_claims);
    }

    pub(crate) fn release_writeback(&mut self, frames: u32) {
        self.counters.active_writeback_claims -= frames;
    }
}
