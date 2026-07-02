#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadPlanScratchUsage {
    protected_reference_capacity: usize,
    protected_references: usize,
    scratch_allocations: u64,
    allocation_events: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadPlanAdmissionScratchArena {
    protected_reference_capacity: usize,
    references: Vec<super::ProtectedPhysicalReference>,
    ranges: Vec<super::ProtectedReferenceRange>,
    scratch_allocations: u64,
    allocation_events: u64,
}

pub(crate) struct ReadPlanScratchBuffers {
    pub(crate) references: Vec<super::ProtectedPhysicalReference>,
    pub(crate) ranges: Vec<super::ProtectedReferenceRange>,
    pub(crate) usage: ReadPlanScratchUsage,
}

impl ReadPlanAdmissionScratchArena {
    pub fn for_protected_reference_capacity(protected_reference_capacity: usize) -> Self {
        let scratch_allocations = if protected_reference_capacity == 0 {
            0
        } else {
            2
        };
        Self {
            protected_reference_capacity,
            references: Vec::with_capacity(protected_reference_capacity),
            ranges: Vec::with_capacity(protected_reference_capacity),
            scratch_allocations,
            allocation_events: scratch_allocations,
        }
    }

    pub const fn protected_reference_capacity(&self) -> usize {
        self.protected_reference_capacity
    }

    pub(crate) fn protect_current_generation_refs<I>(
        mut self,
        references: I,
    ) -> Result<ReadPlanScratchBuffers, super::PhysicalReadPlanAdmissionDenial>
    where
        I: IntoIterator<Item = crate::CurrentGenerationPhysicalReference>,
        I::IntoIter: ExactSizeIterator,
    {
        let iterator = references.into_iter();
        let protected_references = iterator.len();
        if protected_references > self.protected_reference_capacity {
            return Err(
                super::PhysicalReadPlanAdmissionDenial::UnboundedProtectedFootprint {
                    requested: protected_references,
                    capacity: self.protected_reference_capacity,
                },
            );
        }
        self.references.clear();
        self.references
            .extend(iterator.map(super::ProtectedPhysicalReference::from_current_generation));
        Ok(ReadPlanScratchUsage {
            protected_reference_capacity: self.protected_reference_capacity,
            protected_references,
            scratch_allocations: self.scratch_allocations,
            allocation_events: self.allocation_events,
        })
        .map(|usage| ReadPlanScratchBuffers {
            references: self.references,
            ranges: self.ranges,
            usage,
        })
    }

    pub(crate) fn protect_existing_refs<I>(
        mut self,
        references: I,
    ) -> Result<ReadPlanScratchBuffers, super::PhysicalReadPlanAdmissionDenial>
    where
        I: IntoIterator<Item = super::ProtectedPhysicalReference>,
        I::IntoIter: ExactSizeIterator,
    {
        let iterator = references.into_iter();
        let protected_references = iterator.len();
        if protected_references > self.protected_reference_capacity {
            return Err(
                super::PhysicalReadPlanAdmissionDenial::UnboundedProtectedFootprint {
                    requested: protected_references,
                    capacity: self.protected_reference_capacity,
                },
            );
        }
        self.references.clear();
        self.references.extend(iterator);
        Ok(ReadPlanScratchUsage {
            protected_reference_capacity: self.protected_reference_capacity,
            protected_references,
            scratch_allocations: self.scratch_allocations,
            allocation_events: self.allocation_events,
        })
        .map(|usage| ReadPlanScratchBuffers {
            references: self.references,
            ranges: self.ranges,
            usage,
        })
    }
}

impl ReadPlanScratchUsage {
    pub const fn protected_reference_capacity(self) -> usize {
        self.protected_reference_capacity
    }

    pub const fn protected_references(self) -> usize {
        self.protected_references
    }

    pub const fn scratch_allocations(self) -> u64 {
        self.scratch_allocations
    }

    pub const fn allocation_events(self) -> u64 {
        self.allocation_events
    }

    pub const fn with_proof_wrapper_construction(self) -> Self {
        Self {
            allocation_events: self.allocation_events + 3,
            ..self
        }
    }

    pub const fn with_range_compaction(self) -> Self {
        Self {
            allocation_events: self.allocation_events,
            ..self
        }
    }

    pub const fn with_latch_lowering(self) -> Self {
        Self {
            allocation_events: self.allocation_events + 1,
            ..self
        }
    }
}
