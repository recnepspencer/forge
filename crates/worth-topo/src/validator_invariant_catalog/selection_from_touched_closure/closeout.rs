use crate::validator_invariant_catalog::selection_from_touched_closure::{
    WorthTopologyLegalitySelectionPhaseFourSeed, WorthTopologySelectedLegalityObligationPlan,
    WorthTopologyValidatorRoutingClosure,
};
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogCloseout, WorthTopologyLegalityCatalogError,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologyLegalitySelectionCloseout {
    phase_two_catalog_closeout_digest: String,
    selected_plan: WorthTopologySelectedLegalityObligationPlan,
    phase_four_seed: WorthTopologyLegalitySelectionPhaseFourSeed,
    closeout_digest: String,
}

impl WorthTopologyLegalitySelectionCloseout {
    pub fn from_phase_two_closeout_and_routing_closure(
        phase_two_closeout: &WorthTopologyLegalityCatalogCloseout,
        routing_closure: &WorthTopologyValidatorRoutingClosure,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let selected_plan =
            WorthTopologySelectedLegalityObligationPlan::select_from_catalog_and_routing_closure(
                phase_two_closeout.catalog(),
                routing_closure,
            )?;
        let phase_four_seed = selected_plan.phase_four_seed().clone();
        let closeout_digest = [
            "worth-topo-legality-selection-closeout-v1",
            phase_two_closeout.closeout_digest(),
            selected_plan.selected_plan_digest(),
            phase_four_seed.seed_digest(),
        ]
        .join("|");
        Ok(Self {
            phase_two_catalog_closeout_digest: phase_two_closeout.closeout_digest().to_string(),
            selected_plan,
            phase_four_seed,
            closeout_digest,
        })
    }

    pub fn phase_two_catalog_closeout_digest(&self) -> &str {
        &self.phase_two_catalog_closeout_digest
    }

    pub const fn selected_plan(&self) -> &WorthTopologySelectedLegalityObligationPlan {
        &self.selected_plan
    }

    pub const fn phase_four_seed(&self) -> &WorthTopologyLegalitySelectionPhaseFourSeed {
        &self.phase_four_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub const fn claims_enforcement_receipts(&self) -> bool {
        false
    }
}
