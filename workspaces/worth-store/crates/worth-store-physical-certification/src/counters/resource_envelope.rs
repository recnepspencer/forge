use worth_store_buffer_pool::{
    AllocationByteBudget, AllocationEnvelopeDeclaration, AllocationEnvelopeSet,
    FixedMetadataReservation, ResidentByteCount,
};
use worth_store_io_scheduler::IoQueueResourceEnvelope;

use crate::PhysicalSimulationProfile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalResourceEnvelope {
    profile: PhysicalSimulationProfile,
    allocation: AllocationEnvelopeSet,
    resident_bytes: ResidentByteCount,
    max_pinned_pages: u32,
    max_dirty_pages: u32,
    io_queue: IoQueueResourceEnvelope,
}

impl PhysicalResourceEnvelope {
    pub fn for_profile(profile: PhysicalSimulationProfile) -> Self {
        let scale = profile_scale(profile);
        Self {
            profile,
            allocation: allocation_envelope(scale.allocation_bytes),
            resident_bytes: ResidentByteCount::from_observed_bytes(scale.resident_bytes),
            max_pinned_pages: scale.max_pinned_pages,
            max_dirty_pages: scale.max_dirty_pages,
            io_queue: IoQueueResourceEnvelope::bounded(
                scale.io_queue_depth,
                scale.io_interference_events,
            )
            .expect("static I/O queue envelopes are non-zero"),
        }
    }

    pub const fn profile(self) -> PhysicalSimulationProfile {
        self.profile
    }

    pub const fn allocation(self) -> AllocationEnvelopeSet {
        self.allocation
    }

    pub const fn resident_bytes(self) -> ResidentByteCount {
        self.resident_bytes
    }

    pub const fn max_pinned_pages(self) -> u32 {
        self.max_pinned_pages
    }

    pub const fn max_dirty_pages(self) -> u32 {
        self.max_dirty_pages
    }

    pub const fn io_queue(self) -> IoQueueResourceEnvelope {
        self.io_queue
    }
}

#[derive(Debug, Clone, Copy)]
struct ProfileEnvelopeScale {
    allocation_bytes: u64,
    resident_bytes: u64,
    max_pinned_pages: u32,
    max_dirty_pages: u32,
    io_queue_depth: u32,
    io_interference_events: u32,
}

const fn profile_scale(profile: PhysicalSimulationProfile) -> ProfileEnvelopeScale {
    match profile {
        PhysicalSimulationProfile::DeveloperSmoke => ProfileEnvelopeScale {
            allocation_bytes: 64 * 1024,
            resident_bytes: 256 * 1024,
            max_pinned_pages: 8,
            max_dirty_pages: 4,
            io_queue_depth: 4,
            io_interference_events: 1,
        },
        PhysicalSimulationProfile::CiCertification => ProfileEnvelopeScale {
            allocation_bytes: 512 * 1024,
            resident_bytes: 2 * 1024 * 1024,
            max_pinned_pages: 64,
            max_dirty_pages: 32,
            io_queue_depth: 16,
            io_interference_events: 4,
        },
        PhysicalSimulationProfile::LocalSoak => ProfileEnvelopeScale {
            allocation_bytes: 2 * 1024 * 1024,
            resident_bytes: 8 * 1024 * 1024,
            max_pinned_pages: 256,
            max_dirty_pages: 128,
            io_queue_depth: 64,
            io_interference_events: 16,
        },
        PhysicalSimulationProfile::ReleaseCertification => ProfileEnvelopeScale {
            allocation_bytes: 8 * 1024 * 1024,
            resident_bytes: 32 * 1024 * 1024,
            max_pinned_pages: 1024,
            max_dirty_pages: 512,
            io_queue_depth: 256,
            io_interference_events: 64,
        },
        PhysicalSimulationProfile::HardwareQualification => ProfileEnvelopeScale {
            allocation_bytes: 32 * 1024 * 1024,
            resident_bytes: 128 * 1024 * 1024,
            max_pinned_pages: 4096,
            max_dirty_pages: 2048,
            io_queue_depth: 1024,
            io_interference_events: 256,
        },
    }
}

fn allocation_envelope(bytes: u64) -> AllocationEnvelopeSet {
    let budget = AllocationByteBudget::bytes(bytes).expect("static allocation budget is non-zero");
    AllocationEnvelopeDeclaration::declare()
        .foreground(budget)
        .maintenance(budget)
        .recovery(budget)
        .scrub(budget)
        .import_export(budget)
        .streaming(budget)
        .fixed_metadata(
            FixedMetadataReservation::constant_bytes(4096)
                .expect("static metadata budget is non-zero"),
        )
        .seal()
        .expect("static allocation envelope is complete")
}
