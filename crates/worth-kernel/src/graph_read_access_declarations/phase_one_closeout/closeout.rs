use crate::graph_read_access_inventory::{
    WorthGraphReadAccessMilestoneSevenSeed, WorthGraphReadAccessMilestoneSixCloseout,
    WorthGraphReadDeclarationCandidate, WorthGraphReadDeletionLedgerItem,
    WorthGraphReadQueryAccessCapabilityGap,
};

use super::super::seed_contract::{
    admit_milestone_seven_seed, WorthGraphReadAccessDeclarationAdmittedSeed,
};
use super::counters::WorthGraphReadAccessDeclarationPhaseOneCounters;
use super::errors::WorthGraphReadAccessDeclarationPhaseOneError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessDeclarationPhaseOneCloseout {
    admitted_seed: WorthGraphReadAccessDeclarationAdmittedSeed,
    counters: WorthGraphReadAccessDeclarationPhaseOneCounters,
}

pub fn current_worth_graph_read_access_declaration_phase_one_closeout_from_milestone_six(
    milestone_six_closeout: &WorthGraphReadAccessMilestoneSixCloseout,
) -> Result<
    WorthGraphReadAccessDeclarationPhaseOneCloseout,
    WorthGraphReadAccessDeclarationPhaseOneError,
> {
    closeout_from_milestone_seven_seed(milestone_six_closeout.milestone_seven_seed())
}

pub(crate) fn closeout_from_milestone_seven_seed(
    seed: &WorthGraphReadAccessMilestoneSevenSeed,
) -> Result<
    WorthGraphReadAccessDeclarationPhaseOneCloseout,
    WorthGraphReadAccessDeclarationPhaseOneError,
> {
    let admitted_seed = admit_milestone_seven_seed(seed)?;
    Ok(WorthGraphReadAccessDeclarationPhaseOneCloseout {
        counters: WorthGraphReadAccessDeclarationPhaseOneCounters::from_admitted_seed(
            &admitted_seed,
        ),
        admitted_seed,
    })
}

#[cfg(test)]
pub(crate) fn phase_one_closeout_from_milestone_seven_seed_for_tests(
    seed: &WorthGraphReadAccessMilestoneSevenSeed,
) -> Result<
    WorthGraphReadAccessDeclarationPhaseOneCloseout,
    WorthGraphReadAccessDeclarationPhaseOneError,
> {
    closeout_from_milestone_seven_seed(seed)
}

impl WorthGraphReadAccessDeclarationPhaseOneCloseout {
    pub fn declaration_candidates(&self) -> &[WorthGraphReadDeclarationCandidate] {
        self.admitted_seed.declaration_candidates()
    }

    pub fn capability_gaps(&self) -> &[WorthGraphReadQueryAccessCapabilityGap] {
        self.admitted_seed.capability_gaps()
    }

    pub fn deletion_items(&self) -> &[WorthGraphReadDeletionLedgerItem] {
        self.admitted_seed.deletion_items()
    }

    pub const fn counters(&self) -> &WorthGraphReadAccessDeclarationPhaseOneCounters {
        &self.counters
    }

    pub const fn claims_execution_authority(&self) -> bool {
        false
    }

    pub const fn claims_query_declarations_complete(&self) -> bool {
        false
    }

    pub const fn claims_admitted_access_plans_complete(&self) -> bool {
        false
    }

    pub const fn claims_graph_read_receipts_complete(&self) -> bool {
        false
    }
}
