#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PhysicalReferenceValidationCounterSnapshot {
    validation_attempt_count: u32,
    page_slot_validation_count: u32,
    extent_validation_count: u32,
    free_space_reuse_validation_count: u32,
    root_publication_validation_count: u32,
    segment_id_check_count: u32,
    page_id_check_count: u32,
    extent_id_check_count: u32,
    slot_check_count: u32,
    root_reference_check_count: u32,
    allocation_class_check_count: u32,
    generation_check_count: u32,
    wrong_kind_rejection_count: u32,
    placement_mismatch_rejection_count: u32,
    stale_generation_rejection_count: u32,
}

impl PhysicalReferenceValidationCounterSnapshot {
    pub const fn for_page_slot_attempt() -> Self {
        Self {
            validation_attempt_count: 1,
            page_slot_validation_count: 1,
            segment_id_check_count: 1,
            page_id_check_count: 1,
            slot_check_count: 1,
            ..Self::zero()
        }
    }

    pub const fn for_extent_attempt() -> Self {
        Self {
            validation_attempt_count: 1,
            extent_validation_count: 1,
            segment_id_check_count: 1,
            extent_id_check_count: 1,
            ..Self::zero()
        }
    }

    pub const fn for_free_space_slot_attempt() -> Self {
        Self {
            validation_attempt_count: 1,
            free_space_reuse_validation_count: 1,
            segment_id_check_count: 1,
            page_id_check_count: 1,
            slot_check_count: 1,
            allocation_class_check_count: 1,
            ..Self::zero()
        }
    }

    pub const fn for_free_space_extent_attempt() -> Self {
        Self {
            validation_attempt_count: 1,
            free_space_reuse_validation_count: 1,
            segment_id_check_count: 1,
            extent_id_check_count: 1,
            allocation_class_check_count: 1,
            ..Self::zero()
        }
    }

    pub const fn for_root_publication_attempt() -> Self {
        Self {
            validation_attempt_count: 1,
            root_publication_validation_count: 1,
            root_reference_check_count: 1,
            ..Self::zero()
        }
    }

    pub const fn with_generation_check(mut self) -> Self {
        self.generation_check_count = 1;
        self
    }

    pub const fn with_wrong_kind_rejection(mut self) -> Self {
        self.wrong_kind_rejection_count = 1;
        self
    }

    pub const fn with_placement_mismatch_rejection(mut self) -> Self {
        self.placement_mismatch_rejection_count = 1;
        self
    }

    pub const fn with_stale_generation_rejection(mut self) -> Self {
        self.stale_generation_rejection_count = 1;
        self
    }

    pub const fn validation_attempt_count(self) -> u32 {
        self.validation_attempt_count
    }

    pub const fn page_slot_validation_count(self) -> u32 {
        self.page_slot_validation_count
    }

    pub const fn extent_validation_count(self) -> u32 {
        self.extent_validation_count
    }

    pub const fn free_space_reuse_validation_count(self) -> u32 {
        self.free_space_reuse_validation_count
    }

    pub const fn root_publication_validation_count(self) -> u32 {
        self.root_publication_validation_count
    }

    pub const fn segment_id_check_count(self) -> u32 {
        self.segment_id_check_count
    }

    pub const fn page_id_check_count(self) -> u32 {
        self.page_id_check_count
    }

    pub const fn extent_id_check_count(self) -> u32 {
        self.extent_id_check_count
    }

    pub const fn slot_check_count(self) -> u32 {
        self.slot_check_count
    }

    pub const fn root_reference_check_count(self) -> u32 {
        self.root_reference_check_count
    }

    pub const fn allocation_class_check_count(self) -> u32 {
        self.allocation_class_check_count
    }

    pub const fn generation_check_count(self) -> u32 {
        self.generation_check_count
    }

    pub const fn wrong_kind_rejection_count(self) -> u32 {
        self.wrong_kind_rejection_count
    }

    pub const fn placement_mismatch_rejection_count(self) -> u32 {
        self.placement_mismatch_rejection_count
    }

    pub const fn stale_generation_rejection_count(self) -> u32 {
        self.stale_generation_rejection_count
    }

    const fn zero() -> Self {
        Self {
            validation_attempt_count: 0,
            page_slot_validation_count: 0,
            extent_validation_count: 0,
            free_space_reuse_validation_count: 0,
            root_publication_validation_count: 0,
            segment_id_check_count: 0,
            page_id_check_count: 0,
            extent_id_check_count: 0,
            slot_check_count: 0,
            root_reference_check_count: 0,
            allocation_class_check_count: 0,
            generation_check_count: 0,
            wrong_kind_rejection_count: 0,
            placement_mismatch_rejection_count: 0,
            stale_generation_rejection_count: 0,
        }
    }
}
