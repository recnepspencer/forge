use super::super::candidates::WorthGraphReadDeclarationCandidate;
use super::super::capability_gaps::WorthGraphReadQueryAccessCapabilityGap;
use super::super::deletion_ledger::WorthGraphReadDeletionLedgerItem;
use super::super::inventory_lane::WorthGraphReadAccessClassification;
use super::counters::WorthGraphReadAccessPhaseSixCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessMilestoneSevenSeed {
    declaration_candidates: Vec<WorthGraphReadDeclarationCandidate>,
    capability_gaps: Vec<WorthGraphReadQueryAccessCapabilityGap>,
    deletion_items: Vec<WorthGraphReadDeletionLedgerItem>,
    counters: WorthGraphReadAccessPhaseSixCounters,
}

impl WorthGraphReadAccessMilestoneSevenSeed {
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
}
