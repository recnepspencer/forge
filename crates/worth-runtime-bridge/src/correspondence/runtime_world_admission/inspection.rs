use std::sync::atomic::{AtomicU64, Ordering};

/// Read-only evidence for the Runtime World correspondence hot path.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct RuntimeWorldCorrespondenceInspectionCounters {
    binding_index_lookups: u64,
    authoritative_registration_inspections: u64,
}

impl RuntimeWorldCorrespondenceInspectionCounters {
    pub const fn binding_index_lookups(self) -> u64 {
        self.binding_index_lookups
    }

    pub const fn authoritative_registration_inspections(self) -> u64 {
        self.authoritative_registration_inspections
    }
}

#[derive(Debug, Default)]
pub(crate) struct RuntimeWorldCorrespondenceInspectionLedger {
    binding_index_lookups: AtomicU64,
    authoritative_registration_inspections: AtomicU64,
}

impl RuntimeWorldCorrespondenceInspectionLedger {
    pub(crate) fn record_binding_index_lookup(&self) {
        self.binding_index_lookups.fetch_add(1, Ordering::Relaxed);
    }

    #[allow(dead_code)]
    pub(crate) fn record_authoritative_registration_inspection(&self) {
        self.authoritative_registration_inspections
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn snapshot(&self) -> RuntimeWorldCorrespondenceInspectionCounters {
        RuntimeWorldCorrespondenceInspectionCounters {
            binding_index_lookups: self.binding_index_lookups.load(Ordering::Relaxed),
            authoritative_registration_inspections: self
                .authoritative_registration_inspections
                .load(Ordering::Relaxed),
        }
    }
}
