use super::super::inventory_lane::WorthGraphReadAccessInventoryCloseout;
use super::super::phase_six_closeout::WorthGraphReadAccessPhaseSixCloseout;
use super::errors::{
    WorthGraphReadAccessMilestoneSixError, WorthGraphReadAccessMilestoneSixErrorKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessMilestoneSixCloseoutCounters {
    inventory_row_count: usize,
    declaration_candidate_count: usize,
    capability_gap_count: usize,
    deletion_target_count: usize,
    deletion_item_count: usize,
    capped_residue_count: usize,
    certification_only_count: usize,
    out_of_scope_count: usize,
    deleted_source_count: usize,
    existing_deleted_source_count: usize,
}

impl WorthGraphReadAccessMilestoneSixCloseoutCounters {
    pub(crate) fn from_closeouts(
        inventory: &WorthGraphReadAccessInventoryCloseout,
        disposition: &WorthGraphReadAccessPhaseSixCloseout,
    ) -> Result<Self, WorthGraphReadAccessMilestoneSixError> {
        let inventory_counters = inventory.counters();
        let disposition_counters = disposition.counters();

        require_count_match(
            inventory_counters.declaration_candidate_count(),
            disposition_counters.declaration_candidate_count(),
            WorthGraphReadAccessMilestoneSixErrorKind::DeclarationCandidateCountMismatch,
        )?;
        require_count_match(
            inventory_counters.capability_gap_count() + inventory_counters.capped_residue_count(),
            disposition_counters.capability_gap_count(),
            WorthGraphReadAccessMilestoneSixErrorKind::CapabilityGapCountMismatch,
        )?;
        require_count_match(
            inventory_counters.deletion_target_count(),
            disposition_counters.deletion_item_count(),
            WorthGraphReadAccessMilestoneSixErrorKind::DeletionItemCountMismatch,
        )?;
        require_count_match(
            inventory_counters.certification_only_count(),
            disposition_counters.excluded_certification_only_count(),
            WorthGraphReadAccessMilestoneSixErrorKind::CertificationOnlyCountMismatch,
        )?;
        require_count_match(
            inventory_counters.out_of_scope_count(),
            disposition_counters.excluded_out_of_scope_count(),
            WorthGraphReadAccessMilestoneSixErrorKind::OutOfScopeCountMismatch,
        )?;

        let required_disposition_count = inventory_counters.declaration_candidate_count()
            + inventory_counters.capability_gap_count()
            + inventory_counters.capped_residue_count()
            + inventory_counters.deletion_target_count();
        let disposition_count = disposition_counters.declaration_candidate_count()
            + disposition_counters.capability_gap_count()
            + disposition_counters.deletion_item_count();
        require_count_match(
            required_disposition_count,
            disposition_count,
            WorthGraphReadAccessMilestoneSixErrorKind::InventoryDispositionCountMismatch,
        )?;

        let deleted_source_report = inventory.deleted_source_report();
        if deleted_source_report.existing_deleted_source_count() != 0 {
            return Err(error(
                WorthGraphReadAccessMilestoneSixErrorKind::DeletedSourceStillExists,
            ));
        }

        Ok(Self {
            inventory_row_count: inventory_counters.total_row_count(),
            declaration_candidate_count: disposition_counters.declaration_candidate_count(),
            capability_gap_count: disposition_counters.capability_gap_count(),
            deletion_target_count: inventory_counters.deletion_target_count(),
            deletion_item_count: disposition_counters.deletion_item_count(),
            capped_residue_count: inventory_counters.capped_residue_count(),
            certification_only_count: inventory_counters.certification_only_count(),
            out_of_scope_count: inventory_counters.out_of_scope_count(),
            deleted_source_count: deleted_source_report.deleted_source_count(),
            existing_deleted_source_count: deleted_source_report.existing_deleted_source_count(),
        })
    }

    pub const fn inventory_row_count(&self) -> usize {
        self.inventory_row_count
    }

    pub const fn declaration_candidate_count(&self) -> usize {
        self.declaration_candidate_count
    }

    pub const fn capability_gap_count(&self) -> usize {
        self.capability_gap_count
    }

    pub const fn deletion_target_count(&self) -> usize {
        self.deletion_target_count
    }

    pub const fn deletion_item_count(&self) -> usize {
        self.deletion_item_count
    }

    pub const fn capped_residue_count(&self) -> usize {
        self.capped_residue_count
    }

    pub const fn certification_only_count(&self) -> usize {
        self.certification_only_count
    }

    pub const fn out_of_scope_count(&self) -> usize {
        self.out_of_scope_count
    }

    pub const fn deleted_source_count(&self) -> usize {
        self.deleted_source_count
    }

    pub const fn existing_deleted_source_count(&self) -> usize {
        self.existing_deleted_source_count
    }
}

fn require_count_match(
    left: usize,
    right: usize,
    kind: WorthGraphReadAccessMilestoneSixErrorKind,
) -> Result<(), WorthGraphReadAccessMilestoneSixError> {
    if left != right {
        return Err(error(kind));
    }
    Ok(())
}

const fn error(
    kind: WorthGraphReadAccessMilestoneSixErrorKind,
) -> WorthGraphReadAccessMilestoneSixError {
    WorthGraphReadAccessMilestoneSixError::new(kind)
}
