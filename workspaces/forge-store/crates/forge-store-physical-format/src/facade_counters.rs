#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PlatformPhysicalFacadeCounterSnapshot {
    opens: u32,
    appends: u32,
    reads: u32,
    locates: u32,
    scans: u32,
    root_publications: u32,
    reopens: u32,
    writes: u32,
    flushes: u32,
    renames: u32,
    full_store_materialization_rejections: u32,
    backend_residue_guess_rejections: u32,
}

impl PlatformPhysicalFacadeCounterSnapshot {
    pub const fn empty() -> Self {
        Self {
            opens: 0,
            appends: 0,
            reads: 0,
            locates: 0,
            scans: 0,
            root_publications: 0,
            reopens: 0,
            writes: 0,
            flushes: 0,
            renames: 0,
            full_store_materialization_rejections: 0,
            backend_residue_guess_rejections: 0,
        }
    }

    pub const fn with_open(mut self) -> Self {
        self.opens += 1;
        self
    }

    pub const fn with_append(mut self) -> Self {
        self.appends += 1;
        self
    }

    pub const fn with_read(mut self) -> Self {
        self.reads += 1;
        self
    }

    pub const fn with_locate(mut self) -> Self {
        self.locates += 1;
        self
    }

    pub const fn with_scan(mut self) -> Self {
        self.scans += 1;
        self
    }

    pub const fn with_root_publication(mut self) -> Self {
        self.root_publications += 1;
        self
    }

    pub const fn with_reopen(mut self) -> Self {
        self.reopens += 1;
        self
    }

    pub const fn with_write(mut self) -> Self {
        self.writes += 1;
        self
    }

    pub const fn with_flush(mut self) -> Self {
        self.flushes += 1;
        self
    }

    pub const fn with_rename(mut self) -> Self {
        self.renames += 1;
        self
    }

    pub const fn with_full_store_materialization_rejection(mut self) -> Self {
        self.full_store_materialization_rejections += 1;
        self
    }

    pub const fn with_backend_residue_guess_rejection(mut self) -> Self {
        self.backend_residue_guess_rejections += 1;
        self
    }

    pub const fn opens(self) -> u32 {
        self.opens
    }

    pub const fn appends(self) -> u32 {
        self.appends
    }

    pub const fn reads(self) -> u32 {
        self.reads
    }

    pub const fn locates(self) -> u32 {
        self.locates
    }

    pub const fn scans(self) -> u32 {
        self.scans
    }

    pub const fn root_publications(self) -> u32 {
        self.root_publications
    }

    pub const fn reopens(self) -> u32 {
        self.reopens
    }

    pub const fn writes(self) -> u32 {
        self.writes
    }

    pub const fn flushes(self) -> u32 {
        self.flushes
    }

    pub const fn renames(self) -> u32 {
        self.renames
    }

    pub const fn full_store_materialization_rejections(self) -> u32 {
        self.full_store_materialization_rejections
    }

    pub const fn backend_residue_guess_rejections(self) -> u32 {
        self.backend_residue_guess_rejections
    }
}
