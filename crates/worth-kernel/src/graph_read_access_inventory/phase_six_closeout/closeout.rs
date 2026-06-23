use super::super::candidates::WorthGraphReadDeclarationCandidate;
use super::super::capability_gaps::WorthGraphReadQueryAccessCapabilityGap;
use super::super::deletion_ledger::WorthGraphReadDeletionLedgerItem;
use super::super::inventory_lane::{
    WorthGraphReadAccessClassification, WorthGraphReadAccessInventoryCloseout,
};
use super::collector::WorthGraphReadAccessPhaseSixCollector;
use super::counters::WorthGraphReadAccessPhaseSixCounters;
use super::errors::WorthGraphReadAccessPhaseSixError;
use super::inventory_conversion::{
    plan_phase_six_disposition_for_row, WorthGraphReadAccessPhaseSixPlannedDisposition,
};
use super::milestone_seven_seed::WorthGraphReadAccessMilestoneSevenSeed;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPhaseSixCloseout {
    declaration_candidates: Vec<WorthGraphReadDeclarationCandidate>,
    capability_gaps: Vec<WorthGraphReadQueryAccessCapabilityGap>,
    deletion_items: Vec<WorthGraphReadDeletionLedgerItem>,
    counters: WorthGraphReadAccessPhaseSixCounters,
}

impl WorthGraphReadAccessPhaseSixCloseout {
    pub fn from_inventory(
        inventory: &WorthGraphReadAccessInventoryCloseout,
    ) -> Result<Self, WorthGraphReadAccessPhaseSixError> {
        let mut collector = WorthGraphReadAccessPhaseSixCollector::from_inventory(inventory);
        for row in inventory.rows() {
            collector = match plan_phase_six_disposition_for_row(row)? {
                WorthGraphReadAccessPhaseSixPlannedDisposition::DeclarationCandidate(candidate) => {
                    collector.admit_declaration_candidate(candidate)?
                }
                WorthGraphReadAccessPhaseSixPlannedDisposition::CapabilityGap(gap) => {
                    collector.admit_capability_gap(gap)?
                }
                WorthGraphReadAccessPhaseSixPlannedDisposition::DeletionItem(item) => {
                    collector.admit_deletion_item(item)?
                }
                WorthGraphReadAccessPhaseSixPlannedDisposition::Excluded => collector,
            };
        }
        collector.closeout()
    }

    pub(crate) fn new(
        declaration_candidates: Vec<WorthGraphReadDeclarationCandidate>,
        capability_gaps: Vec<WorthGraphReadQueryAccessCapabilityGap>,
        deletion_items: Vec<WorthGraphReadDeletionLedgerItem>,
        counters: WorthGraphReadAccessPhaseSixCounters,
    ) -> Self {
        Self {
            declaration_candidates,
            capability_gaps,
            deletion_items,
            counters,
        }
    }

    pub fn declaration_candidates(&self) -> &[WorthGraphReadDeclarationCandidate] {
        &self.declaration_candidates
    }

    pub fn capability_gaps(&self) -> &[WorthGraphReadQueryAccessCapabilityGap] {
        &self.capability_gaps
    }

    pub fn deletion_items(&self) -> &[WorthGraphReadDeletionLedgerItem] {
        &self.deletion_items
    }

    pub fn counters(&self) -> &WorthGraphReadAccessPhaseSixCounters {
        &self.counters
    }

    pub const fn claims_execution_authority(&self) -> bool {
        false
    }

    pub fn contains_uncapped_old_graph_read_folklore_as_declaration_or_gap(&self) -> bool {
        self.declaration_candidates.iter().any(|candidate| {
            candidate
                .inventory_row_context()
                .scope_binding()
                .adoption_manifest_digest()
                .is_some()
        }) || self.capability_gaps.iter().any(|gap| {
            let context = gap.inventory_row_context();
            context.scope_binding().adoption_manifest_digest().is_some()
                && context.classification() != WorthGraphReadAccessClassification::CappedResidue
        })
    }

    pub const fn claims_later_milestone_completion(&self) -> bool {
        false
    }

    pub fn milestone_seven_seed(&self) -> WorthGraphReadAccessMilestoneSevenSeed {
        WorthGraphReadAccessMilestoneSevenSeed::new(
            self.declaration_candidates.clone(),
            self.capability_gaps.clone(),
            self.deletion_items.clone(),
            self.counters,
        )
    }
}
