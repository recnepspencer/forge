use crate::query_obligation_selection::public_facade::WorthQueryObligationSelectionMilestoneSixSeed;

use super::super::inventory_lane::{
    current_worth_graph_read_access_surface_inventory, WorthGraphReadAccessInventoryCloseout,
};
use super::super::phase_six_closeout::{
    WorthGraphReadAccessMilestoneSevenSeed, WorthGraphReadAccessPhaseSixCloseout,
};
use super::counters::WorthGraphReadAccessMilestoneSixCloseoutCounters;
use super::errors::WorthGraphReadAccessMilestoneSixError;
use super::later_milestone_claims::{
    reject_later_milestone_claims, WorthGraphReadAccessLaterMilestoneClaims,
};
use super::readiness::WorthGraphReadAccessMilestoneSixReadiness;
use super::seed_audit::audit_milestone_seven_seed;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessMilestoneSixCloseout {
    inventory_closeout: WorthGraphReadAccessInventoryCloseout,
    disposition_closeout: WorthGraphReadAccessPhaseSixCloseout,
    milestone_seven_seed: WorthGraphReadAccessMilestoneSevenSeed,
    counters: WorthGraphReadAccessMilestoneSixCloseoutCounters,
    readiness: WorthGraphReadAccessMilestoneSixReadiness,
}

pub fn current_worth_graph_read_access_milestone_six_closeout(
    milestone_five_seed: WorthQueryObligationSelectionMilestoneSixSeed,
) -> Result<WorthGraphReadAccessMilestoneSixCloseout, WorthGraphReadAccessMilestoneSixError> {
    let inventory_closeout =
        current_worth_graph_read_access_surface_inventory(milestone_five_seed)?;
    WorthGraphReadAccessMilestoneSixCloseout::from_inventory_closeout(inventory_closeout)
}

impl WorthGraphReadAccessMilestoneSixCloseout {
    pub fn from_inventory_closeout(
        inventory_closeout: WorthGraphReadAccessInventoryCloseout,
    ) -> Result<Self, WorthGraphReadAccessMilestoneSixError> {
        Self::from_inventory_closeout_with_later_milestone_claims(
            inventory_closeout,
            WorthGraphReadAccessLaterMilestoneClaims::absent(),
        )
    }

    pub(super) fn from_inventory_closeout_with_later_milestone_claims(
        inventory_closeout: WorthGraphReadAccessInventoryCloseout,
        later_milestone_claims: WorthGraphReadAccessLaterMilestoneClaims,
    ) -> Result<Self, WorthGraphReadAccessMilestoneSixError> {
        let disposition_closeout =
            WorthGraphReadAccessPhaseSixCloseout::from_inventory(&inventory_closeout)?;
        reject_later_milestone_claims(&disposition_closeout, later_milestone_claims)?;
        let counters = WorthGraphReadAccessMilestoneSixCloseoutCounters::from_closeouts(
            &inventory_closeout,
            &disposition_closeout,
        )?;
        let milestone_seven_seed = disposition_closeout.milestone_seven_seed();
        audit_milestone_seven_seed(&milestone_seven_seed)?;

        Ok(Self {
            inventory_closeout,
            disposition_closeout,
            milestone_seven_seed,
            counters,
            readiness: WorthGraphReadAccessMilestoneSixReadiness::ReadyForMilestoneSeven,
        })
    }

    pub const fn readiness(&self) -> WorthGraphReadAccessMilestoneSixReadiness {
        self.readiness
    }

    pub const fn counters(&self) -> &WorthGraphReadAccessMilestoneSixCloseoutCounters {
        &self.counters
    }

    pub const fn inventory_closeout(&self) -> &WorthGraphReadAccessInventoryCloseout {
        &self.inventory_closeout
    }

    pub(super) const fn disposition_closeout(&self) -> &WorthGraphReadAccessPhaseSixCloseout {
        &self.disposition_closeout
    }

    pub const fn milestone_seven_seed(&self) -> &WorthGraphReadAccessMilestoneSevenSeed {
        &self.milestone_seven_seed
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

    pub const fn claims_validator_derivation_complete(&self) -> bool {
        false
    }

    pub const fn claims_invalidation_complete(&self) -> bool {
        false
    }

    pub const fn claims_replay_complete(&self) -> bool {
        false
    }

    pub const fn claims_conflict_complete(&self) -> bool {
        false
    }

    pub const fn claims_cache_complete(&self) -> bool {
        false
    }

    pub const fn claims_public_diagnostics_complete(&self) -> bool {
        false
    }
}
