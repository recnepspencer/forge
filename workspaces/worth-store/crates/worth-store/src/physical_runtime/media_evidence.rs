use worth_foundational::performance_api::lower_lane::{basis, receipts};
use worth_foundational::{
    FoundationalAuthoritativePerformanceClaim, FoundationalCounterBackedPerformanceReceipt,
    FoundationalCounterBackedPerformanceReceiptConstructionDenial,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBundleConstructionDenial,
    FoundationalPerformanceClaimConstructionDenial, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
};
use worth_store_physical_backend::{MediaCounterSnapshot, MediaOperationRole};

type CounterProjection = (&'static str, fn(MediaCounterSnapshot) -> u64);
type RoleCounterProjection = (
    &'static str,
    fn(MediaCounterSnapshot, MediaOperationRole) -> u64,
);

const COUNTERS: &[CounterProjection] = &[
    (
        "store.media.operations.attempted",
        MediaCounterSnapshot::attempted_operations,
    ),
    (
        "store.media.operations.completed",
        MediaCounterSnapshot::completed_operations,
    ),
    (
        "store.media.operations.denied",
        MediaCounterSnapshot::denied_before_effect,
    ),
    (
        "store.media.operations.partial",
        MediaCounterSnapshot::partial_effects,
    ),
    (
        "store.media.operations.indeterminate",
        MediaCounterSnapshot::indeterminate_effects,
    ),
    (
        "store.media.bytes.requested",
        MediaCounterSnapshot::requested_bytes,
    ),
    (
        "store.media.bytes.completed",
        MediaCounterSnapshot::completed_bytes,
    ),
    (
        "store.media.allocations.explicit",
        MediaCounterSnapshot::explicit_heap_allocation_events,
    ),
    (
        "store.media.allocations.requested_capacity_bytes",
        MediaCounterSnapshot::requested_heap_capacity_bytes,
    ),
    ("store.media.eof", MediaCounterSnapshot::eof_observations),
    ("store.media.retries", MediaCounterSnapshot::retry_attempts),
    (
        "store.media.listing.batches",
        MediaCounterSnapshot::listing_batches,
    ),
    (
        "store.media.listing.entries",
        MediaCounterSnapshot::listing_entries,
    ),
    (
        "store.media.qualification.transactions",
        MediaCounterSnapshot::qualification_transactions,
    ),
    (
        "store.media.ownership.attempts",
        MediaCounterSnapshot::ownership_attempts,
    ),
    (
        "store.media.ownership.acquisitions",
        MediaCounterSnapshot::ownership_acquisitions,
    ),
    (
        "store.media.ownership.contentions",
        MediaCounterSnapshot::ownership_contentions,
    ),
    (
        "store.media.ownership.releases",
        MediaCounterSnapshot::ownership_releases,
    ),
    ("store.media.sync.file", MediaCounterSnapshot::file_syncs),
    (
        "store.media.sync.directory",
        MediaCounterSnapshot::directory_syncs,
    ),
    (
        "store.media.replacements",
        MediaCounterSnapshot::replacements,
    ),
    ("store.media.deletions", MediaCounterSnapshot::deletions),
    ("store.media.files.opened", MediaCounterSnapshot::file_opens),
    (
        "store.media.files.created",
        MediaCounterSnapshot::file_creates,
    ),
    (
        "store.media.files.closed",
        MediaCounterSnapshot::file_closes,
    ),
    (
        "store.media.files.live",
        MediaCounterSnapshot::live_file_handles,
    ),
    (
        "store.media.files.peak",
        MediaCounterSnapshot::peak_file_handles,
    ),
    (
        "store.media.directories.opened",
        MediaCounterSnapshot::directory_opens,
    ),
    (
        "store.media.directories.closed",
        MediaCounterSnapshot::directory_closes,
    ),
    (
        "store.media.directories.live",
        MediaCounterSnapshot::live_directory_handles,
    ),
    (
        "store.media.directories.peak",
        MediaCounterSnapshot::peak_directory_handles,
    ),
    (
        "store.media.denials.confinement",
        MediaCounterSnapshot::confinement_denials,
    ),
    (
        "store.media.denials.stale_handle",
        MediaCounterSnapshot::stale_handle_denials,
    ),
    (
        "store.media.denials.unsupported_capability",
        MediaCounterSnapshot::unsupported_capabilities,
    ),
    (
        "store.media.cleanup.actions",
        MediaCounterSnapshot::cleanup_actions,
    ),
    (
        "store.media.cleanup.residue",
        MediaCounterSnapshot::preserved_residue,
    ),
    (
        "store.media.request.peak_width_bytes",
        MediaCounterSnapshot::peak_request_width_bytes,
    ),
];

const ROLE_COUNTERS: [RoleCounterProjection; 7] = [
    ("attempted", MediaCounterSnapshot::attempts_for),
    ("completed", MediaCounterSnapshot::completed_operations_for),
    ("denied", MediaCounterSnapshot::denied_before_effect_for),
    ("partial", MediaCounterSnapshot::partial_effects_for),
    (
        "indeterminate",
        MediaCounterSnapshot::indeterminate_effects_for,
    ),
    ("bytes.requested", MediaCounterSnapshot::requested_bytes_for),
    ("bytes.completed", MediaCounterSnapshot::completed_bytes_for),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaOperationSummary {
    counters: MediaCounterSnapshot,
    store_identity: [u8; 16],
    owner_identity: [u8; 16],
}

impl MediaOperationSummary {
    pub(super) fn from_qualified_media(
        media: &worth_store_physical_backend::QualifiedFilesystemMedia,
    ) -> Result<Self, MediaEvidenceLoweringDenial> {
        let counters = media.counters();
        if !counters.is_conserved() {
            return Err(MediaEvidenceLoweringDenial::UnconservedStoreCounters);
        }
        if counters.completed_operations_for(MediaOperationRole::AtomicReplace) == 0
            || counters
                .completed_operations_for(MediaOperationRole::SynchronizeDirectoryPublication)
                == 0
            || counters.completed_operations_for(MediaOperationRole::SynchronizeFileState) == 0
        {
            return Err(MediaEvidenceLoweringDenial::NoCompletedDurablePublication);
        }
        Ok(Self {
            counters,
            store_identity: media.store_identity().bytes(),
            owner_identity: media.mutation_owner().owner().bytes(),
        })
    }

    pub const fn counters(self) -> MediaCounterSnapshot {
        self.counters
    }

    pub const fn store_identity(self) -> [u8; 16] {
        self.store_identity
    }

    pub const fn owner_identity(self) -> [u8; 16] {
        self.owner_identity
    }
}

pub type StoreMediaPerformanceReceipt =
    FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaEvidenceLoweringDenial {
    UnconservedStoreCounters,
    NoCompletedDurablePublication,
    Claim(FoundationalPerformanceClaimConstructionDenial),
    Bundle(FoundationalPerformanceBundleConstructionDenial),
    Receipt(FoundationalCounterBackedPerformanceReceiptConstructionDenial),
}

pub fn lower_media_operation_summary(
    summary: MediaOperationSummary,
) -> Result<StoreMediaPerformanceReceipt, MediaEvidenceLoweringDenial> {
    let claim = worth_foundational::performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::PointLocal)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::PointLookup)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::AuthoritativeMutation)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .map_err(MediaEvidenceLoweringDenial::Claim)?;

    let mut bundle = basis::performance_bundle(claim);
    for &(name, read) in COUNTERS {
        let name = basis::FoundationalPerformanceCounterName::new(name)
            .expect("Store-owned counter names are static and valid");
        bundle = bundle.attach_counter_spec(FoundationalPerformanceCounterSpec::new(
            name,
            FoundationalPerformanceWorkClass::AuthoritativeMutation,
            read(summary.counters),
        ));
    }
    for role in MediaOperationRole::ALL {
        for (suffix, read) in ROLE_COUNTERS {
            let name = role_counter_name(role, suffix);
            bundle = bundle.attach_counter_spec(FoundationalPerformanceCounterSpec::new(
                name,
                FoundationalPerformanceWorkClass::AuthoritativeMutation,
                read(summary.counters, role),
            ));
        }
    }
    let bundle = bundle
        .finish()
        .map_err(MediaEvidenceLoweringDenial::Bundle)?;
    let mut receipt = receipts::counter_backed_performance_receipt(bundle);
    for &(name, read) in COUNTERS {
        let name = basis::FoundationalPerformanceCounterName::new(name)
            .expect("Store-owned counter names are static and valid");
        receipt = receipt.attach_counter_row(FoundationalPerformanceCounterRow::new(
            name,
            read(summary.counters),
        ));
    }
    for role in MediaOperationRole::ALL {
        for (suffix, read) in ROLE_COUNTERS {
            receipt = receipt.attach_counter_row(FoundationalPerformanceCounterRow::new(
                role_counter_name(role, suffix),
                read(summary.counters, role),
            ));
        }
    }
    receipt
        .finish()
        .map_err(MediaEvidenceLoweringDenial::Receipt)
}

fn role_counter_name(
    role: MediaOperationRole,
    suffix: &str,
) -> basis::FoundationalPerformanceCounterName {
    basis::FoundationalPerformanceCounterName::new(format!(
        "store.media.role.{}.{}",
        role.metric_name(),
        suffix
    ))
    .expect("Store-owned role counter names are static and valid")
}
