use worth_store_budgets::{
    AllocationBudgetDenial, AllocationCounterSnapshot, AllocationCounters, AllocationEnvelopeSet,
    AllocationScope, FixedMetadataReservation,
};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ALLOCATION_ADMISSION_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationRequestKind {
    UnboundedBuffer,
    CopiedPayload,
    RichDiagnostics,
    MaterializedRecordSet,
    BackgroundWorkMemory,
    StreamingWindow,
}

impl AllocationRequestKind {
    const fn is_copied_payload(self) -> bool {
        matches!(self, Self::CopiedPayload)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationRequest {
    scope: AllocationScope,
    kind: AllocationRequestKind,
    bytes: Option<u64>,
}

impl AllocationRequest {
    pub const fn unbounded_buffer(scope: AllocationScope) -> Self {
        Self {
            scope,
            kind: AllocationRequestKind::UnboundedBuffer,
            bytes: None,
        }
    }

    pub fn copied_payload(scope: AllocationScope, bytes: u64) -> Result<Self, AllocationDenial> {
        Self::exact(scope, AllocationRequestKind::CopiedPayload, bytes)
    }

    pub fn rich_diagnostics(scope: AllocationScope, bytes: u64) -> Result<Self, AllocationDenial> {
        Self::exact(scope, AllocationRequestKind::RichDiagnostics, bytes)
    }

    pub fn materialized_record_set(
        scope: AllocationScope,
        bytes: u64,
    ) -> Result<Self, AllocationDenial> {
        Self::exact(scope, AllocationRequestKind::MaterializedRecordSet, bytes)
    }

    pub fn background_work_memory(
        scope: AllocationScope,
        bytes: u64,
    ) -> Result<Self, AllocationDenial> {
        Self::exact(scope, AllocationRequestKind::BackgroundWorkMemory, bytes)
    }

    pub fn streaming_window(scope: AllocationScope, bytes: u64) -> Result<Self, AllocationDenial> {
        Self::exact(scope, AllocationRequestKind::StreamingWindow, bytes)
    }

    pub const fn scope(self) -> AllocationScope {
        self.scope
    }

    pub const fn kind(self) -> AllocationRequestKind {
        self.kind
    }

    pub const fn requested_bytes(self) -> Option<u64> {
        self.bytes
    }

    fn exact(
        scope: AllocationScope,
        kind: AllocationRequestKind,
        bytes: u64,
    ) -> Result<Self, AllocationDenial> {
        if bytes == 0 {
            return Err(AllocationDenial::RequestBytesAreZero { scope, kind });
        }
        Ok(Self {
            scope,
            kind,
            bytes: Some(bytes),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AllocationGrant {
    admission_id: AllocationAdmissionId,
    scope: AllocationScope,
    kind: AllocationRequestKind,
    bytes: u64,
}

impl AllocationGrant {
    pub const fn scope(&self) -> AllocationScope {
        self.scope
    }

    pub const fn kind(&self) -> AllocationRequestKind {
        self.kind
    }

    pub const fn bytes(&self) -> u64 {
        self.bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationAdmissionId {
    value: u64,
}

impl AllocationAdmissionId {
    fn next() -> Self {
        Self {
            value: NEXT_ALLOCATION_ADMISSION_ID.fetch_add(1, Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationReceipt {
    scope: AllocationScope,
    kind: AllocationRequestKind,
    bytes: u64,
    counters: AllocationCounterSnapshot,
}

impl AllocationReceipt {
    pub const fn scope(self) -> AllocationScope {
        self.scope
    }

    pub const fn kind(self) -> AllocationRequestKind {
        self.kind
    }

    pub const fn bytes(self) -> u64 {
        self.bytes
    }

    pub const fn counters(self) -> AllocationCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FixedMetadataGrant {
    admission_id: AllocationAdmissionId,
    bytes: u64,
}

impl FixedMetadataGrant {
    pub const fn bytes(&self) -> u64 {
        self.bytes
    }

    pub const fn constant_size_at_scale(
        &self,
        _store_bytes: u64,
        _page_count: u64,
        _payload_bytes: u64,
        _diagnostic_bytes: u64,
    ) -> u64 {
        self.bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationDenial {
    Budget(AllocationBudgetDenial),
    RequestBytesAreZero {
        scope: AllocationScope,
        kind: AllocationRequestKind,
    },
    UnboundedRequest {
        scope: AllocationScope,
    },
    EnvelopeExceeded {
        scope: AllocationScope,
        kind: AllocationRequestKind,
        requested_bytes: u64,
        remaining_bytes: u64,
    },
    FixedMetadataEnvelopeExceeded {
        requested_bytes: u64,
        remaining_bytes: u64,
    },
    VariableAllocationCannotUseFixedMetadata {
        scope: AllocationScope,
        kind: AllocationRequestKind,
    },
    GrantAuthorityMismatch {
        scope: AllocationScope,
        bytes: u64,
    },
}

impl AllocationDenial {
    pub const fn scope(self) -> Option<AllocationScope> {
        match self {
            Self::RequestBytesAreZero { scope, .. }
            | Self::UnboundedRequest { scope }
            | Self::EnvelopeExceeded { scope, .. }
            | Self::VariableAllocationCannotUseFixedMetadata { scope, .. }
            | Self::GrantAuthorityMismatch { scope, .. } => Some(scope),
            Self::Budget(_) | Self::FixedMetadataEnvelopeExceeded { .. } => None,
        }
    }
}

impl From<AllocationBudgetDenial> for AllocationDenial {
    fn from(denial: AllocationBudgetDenial) -> Self {
        Self::Budget(denial)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AllocationAdmission {
    admission_id: AllocationAdmissionId,
    envelopes: AllocationEnvelopeSet,
    counters: AllocationCounters,
    foreground_used: u64,
    maintenance_used: u64,
    recovery_used: u64,
    scrub_used: u64,
    import_export_used: u64,
    streaming_used: u64,
    fixed_metadata_used: u64,
}

impl AllocationAdmission {
    pub fn from_declaration(envelopes: AllocationEnvelopeSet) -> Self {
        Self {
            admission_id: AllocationAdmissionId::next(),
            envelopes,
            counters: AllocationCounters::new(),
            foreground_used: 0,
            maintenance_used: 0,
            recovery_used: 0,
            scrub_used: 0,
            import_export_used: 0,
            streaming_used: 0,
            fixed_metadata_used: 0,
        }
    }

    pub fn admit(
        &mut self,
        request: AllocationRequest,
    ) -> Result<AllocationGrant, AllocationDenial> {
        let bytes = match request.requested_bytes() {
            Some(bytes) => bytes,
            None => {
                self.counters.record_denied(request.scope(), 0);
                return Err(AllocationDenial::UnboundedRequest {
                    scope: request.scope(),
                });
            }
        };
        self.counters.record_requested(request.scope(), bytes);
        if bytes > self.remaining(request.scope()) {
            let remaining = self.remaining(request.scope());
            self.counters.record_denied(request.scope(), bytes);
            return Err(AllocationDenial::EnvelopeExceeded {
                scope: request.scope(),
                kind: request.kind(),
                requested_bytes: bytes,
                remaining_bytes: remaining,
            });
        }
        *self.used_mut(request.scope()) += bytes;
        self.counters.record_admitted(request.scope(), bytes);
        Ok(AllocationGrant {
            admission_id: self.admission_id,
            scope: request.scope(),
            kind: request.kind(),
            bytes,
        })
    }

    pub fn record_allocation(
        &mut self,
        grant: AllocationGrant,
    ) -> Result<AllocationReceipt, AllocationDenial> {
        if grant.admission_id != self.admission_id {
            self.counters.record_denied(grant.scope, grant.bytes);
            return Err(AllocationDenial::GrantAuthorityMismatch {
                scope: grant.scope,
                bytes: grant.bytes,
            });
        }
        self.counters.record_allocated(grant.scope, grant.bytes);
        if grant.kind.is_copied_payload() {
            self.counters.record_copied(grant.scope, grant.bytes);
        }
        Ok(AllocationReceipt {
            scope: grant.scope,
            kind: grant.kind,
            bytes: grant.bytes,
            counters: self.counters(),
        })
    }

    pub fn admit_fixed_metadata(
        &mut self,
        reservation: FixedMetadataReservation,
    ) -> Result<FixedMetadataGrant, AllocationDenial> {
        let bytes = reservation.as_bytes();
        if bytes > self.fixed_metadata_remaining() {
            let remaining = self.fixed_metadata_remaining();
            self.counters.record_fixed_metadata_denial(bytes);
            return Err(AllocationDenial::FixedMetadataEnvelopeExceeded {
                requested_bytes: bytes,
                remaining_bytes: remaining,
            });
        }
        self.fixed_metadata_used += bytes;
        self.counters.record_fixed_metadata(bytes);
        Ok(FixedMetadataGrant {
            admission_id: self.admission_id,
            bytes,
        })
    }

    pub fn owns_fixed_metadata_grant(&self, grant: &FixedMetadataGrant) -> bool {
        grant.admission_id == self.admission_id
    }

    pub fn reject_fixed_metadata_for_variable_request(
        &mut self,
        request: AllocationRequest,
    ) -> AllocationDenial {
        let bytes = request.requested_bytes().unwrap_or(0);
        self.counters.record_denied(request.scope(), bytes);
        AllocationDenial::VariableAllocationCannotUseFixedMetadata {
            scope: request.scope(),
            kind: request.kind(),
        }
    }

    pub const fn counters(&self) -> AllocationCounterSnapshot {
        self.counters.snapshot()
    }

    pub const fn remaining(&self, scope: AllocationScope) -> u64 {
        self.envelopes
            .budget(scope)
            .as_bytes()
            .saturating_sub(self.used(scope))
    }

    pub const fn fixed_metadata_remaining(&self) -> u64 {
        self.envelopes
            .fixed_metadata()
            .as_bytes()
            .saturating_sub(self.fixed_metadata_used)
    }

    const fn used(&self, scope: AllocationScope) -> u64 {
        match scope {
            AllocationScope::Foreground => self.foreground_used,
            AllocationScope::Maintenance => self.maintenance_used,
            AllocationScope::Recovery => self.recovery_used,
            AllocationScope::Scrub => self.scrub_used,
            AllocationScope::ImportExport => self.import_export_used,
            AllocationScope::Streaming => self.streaming_used,
        }
    }

    fn used_mut(&mut self, scope: AllocationScope) -> &mut u64 {
        match scope {
            AllocationScope::Foreground => &mut self.foreground_used,
            AllocationScope::Maintenance => &mut self.maintenance_used,
            AllocationScope::Recovery => &mut self.recovery_used,
            AllocationScope::Scrub => &mut self.scrub_used,
            AllocationScope::ImportExport => &mut self.import_export_used,
            AllocationScope::Streaming => &mut self.streaming_used,
        }
    }
}
