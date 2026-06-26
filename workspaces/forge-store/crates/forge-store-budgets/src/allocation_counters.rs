use crate::AllocationScope;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ScopeAllocationCounters {
    requested_bytes: u64,
    admitted_bytes: u64,
    allocated_bytes: u64,
    copied_bytes: u64,
    denied_bytes: u64,
    denial_count: u32,
}

impl ScopeAllocationCounters {
    pub const fn requested_bytes(self) -> u64 {
        self.requested_bytes
    }

    pub const fn admitted_bytes(self) -> u64 {
        self.admitted_bytes
    }

    pub const fn allocated_bytes(self) -> u64 {
        self.allocated_bytes
    }

    pub const fn copied_bytes(self) -> u64 {
        self.copied_bytes
    }

    pub const fn denied_bytes(self) -> u64 {
        self.denied_bytes
    }

    pub const fn denial_count(self) -> u32 {
        self.denial_count
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AllocationCounterSnapshot {
    foreground: ScopeAllocationCounters,
    maintenance: ScopeAllocationCounters,
    recovery: ScopeAllocationCounters,
    scrub: ScopeAllocationCounters,
    import_export: ScopeAllocationCounters,
    streaming: ScopeAllocationCounters,
    fixed_metadata_bytes: u64,
    fixed_metadata_exemption_count: u32,
    fixed_metadata_denied_bytes: u64,
}

impl AllocationCounterSnapshot {
    pub const fn scope(self, scope: AllocationScope) -> ScopeAllocationCounters {
        match scope {
            AllocationScope::Foreground => self.foreground,
            AllocationScope::Maintenance => self.maintenance,
            AllocationScope::Recovery => self.recovery,
            AllocationScope::Scrub => self.scrub,
            AllocationScope::ImportExport => self.import_export,
            AllocationScope::Streaming => self.streaming,
        }
    }

    pub const fn fixed_metadata_bytes(self) -> u64 {
        self.fixed_metadata_bytes
    }

    pub const fn fixed_metadata_exemption_count(self) -> u32 {
        self.fixed_metadata_exemption_count
    }

    pub const fn fixed_metadata_denied_bytes(self) -> u64 {
        self.fixed_metadata_denied_bytes
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AllocationCounters {
    snapshot: AllocationCounterSnapshot,
}

impl AllocationCounters {
    pub const fn new() -> Self {
        Self {
            snapshot: AllocationCounterSnapshot {
                foreground: ScopeAllocationCounters {
                    requested_bytes: 0,
                    admitted_bytes: 0,
                    allocated_bytes: 0,
                    copied_bytes: 0,
                    denied_bytes: 0,
                    denial_count: 0,
                },
                maintenance: ScopeAllocationCounters {
                    requested_bytes: 0,
                    admitted_bytes: 0,
                    allocated_bytes: 0,
                    copied_bytes: 0,
                    denied_bytes: 0,
                    denial_count: 0,
                },
                recovery: ScopeAllocationCounters {
                    requested_bytes: 0,
                    admitted_bytes: 0,
                    allocated_bytes: 0,
                    copied_bytes: 0,
                    denied_bytes: 0,
                    denial_count: 0,
                },
                scrub: ScopeAllocationCounters {
                    requested_bytes: 0,
                    admitted_bytes: 0,
                    allocated_bytes: 0,
                    copied_bytes: 0,
                    denied_bytes: 0,
                    denial_count: 0,
                },
                import_export: ScopeAllocationCounters {
                    requested_bytes: 0,
                    admitted_bytes: 0,
                    allocated_bytes: 0,
                    copied_bytes: 0,
                    denied_bytes: 0,
                    denial_count: 0,
                },
                streaming: ScopeAllocationCounters {
                    requested_bytes: 0,
                    admitted_bytes: 0,
                    allocated_bytes: 0,
                    copied_bytes: 0,
                    denied_bytes: 0,
                    denial_count: 0,
                },
                fixed_metadata_bytes: 0,
                fixed_metadata_exemption_count: 0,
                fixed_metadata_denied_bytes: 0,
            },
        }
    }

    pub const fn snapshot(self) -> AllocationCounterSnapshot {
        self.snapshot
    }

    pub fn record_requested(&mut self, scope: AllocationScope, bytes: u64) {
        self.scope_mut(scope).requested_bytes += bytes;
    }

    pub fn record_admitted(&mut self, scope: AllocationScope, bytes: u64) {
        self.scope_mut(scope).admitted_bytes += bytes;
    }

    pub fn record_allocated(&mut self, scope: AllocationScope, bytes: u64) {
        self.scope_mut(scope).allocated_bytes += bytes;
    }

    pub fn record_copied(&mut self, scope: AllocationScope, bytes: u64) {
        self.scope_mut(scope).copied_bytes += bytes;
    }

    pub fn record_denied(&mut self, scope: AllocationScope, bytes: u64) {
        let counters = self.scope_mut(scope);
        counters.denied_bytes += bytes;
        counters.denial_count += 1;
    }

    pub fn record_fixed_metadata(&mut self, bytes: u64) {
        self.snapshot.fixed_metadata_bytes += bytes;
        self.snapshot.fixed_metadata_exemption_count += 1;
    }

    pub fn record_fixed_metadata_denial(&mut self, bytes: u64) {
        self.snapshot.fixed_metadata_denied_bytes += bytes;
    }

    fn scope_mut(&mut self, scope: AllocationScope) -> &mut ScopeAllocationCounters {
        match scope {
            AllocationScope::Foreground => &mut self.snapshot.foreground,
            AllocationScope::Maintenance => &mut self.snapshot.maintenance,
            AllocationScope::Recovery => &mut self.snapshot.recovery,
            AllocationScope::Scrub => &mut self.snapshot.scrub,
            AllocationScope::ImportExport => &mut self.snapshot.import_export,
            AllocationScope::Streaming => &mut self.snapshot.streaming,
        }
    }
}
