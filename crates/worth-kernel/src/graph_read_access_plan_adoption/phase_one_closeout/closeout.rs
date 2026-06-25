use crate::graph_read_access_declarations::{
    WorthGraphReadAccessDeclarationMilestoneEightSeed, WorthGraphReadAdmissionCapabilityGap,
    WorthGraphReadDeclarationCappedResidueReport, WorthGraphReadDeclarationDeletionLedgerReport,
    WorthGraphReadDeclarationReadFamilyIdentity, WorthGraphReadDeclarationSourceFirewallReport,
    WorthGraphReadRequirementDerivationCapabilityGap, WorthGraphReadRequirementRowDigestProjection,
};

use super::super::execution_folklore_inventory::WorthGraphReadAccessPlanAdoptionExecutionFolkloreInventory;
use super::super::query_surface_anchors::WorthGraphReadAccessPlanAdoptionQuerySurfaceAnchors;
use super::super::seed_admission::{
    admit_milestone_eight_seed, WorthGraphReadAccessPlanAdoptionAdmittedSeed,
};
use super::counters::WorthGraphReadAccessPlanAdoptionPhaseOneCounters;
use super::errors::WorthGraphReadAccessPlanAdoptionPhaseOneError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionPhaseOneCloseout {
    admitted_seed: WorthGraphReadAccessPlanAdoptionAdmittedSeed,
    execution_folklore_inventory: WorthGraphReadAccessPlanAdoptionExecutionFolkloreInventory,
    query_surface_anchors: WorthGraphReadAccessPlanAdoptionQuerySurfaceAnchors,
    counters: WorthGraphReadAccessPlanAdoptionPhaseOneCounters,
}

pub fn current_worth_graph_read_access_plan_adoption_phase_one_closeout(
    seed: &WorthGraphReadAccessDeclarationMilestoneEightSeed,
) -> Result<
    WorthGraphReadAccessPlanAdoptionPhaseOneCloseout,
    WorthGraphReadAccessPlanAdoptionPhaseOneError,
> {
    let admitted_seed = admit_milestone_eight_seed(seed)?;
    let execution_folklore_inventory =
        WorthGraphReadAccessPlanAdoptionExecutionFolkloreInventory::from_milestone_eight_seed(
            admitted_seed.seed(),
        )?;
    let counters = WorthGraphReadAccessPlanAdoptionPhaseOneCounters::from_parts(
        &admitted_seed,
        &execution_folklore_inventory,
    );
    Ok(WorthGraphReadAccessPlanAdoptionPhaseOneCloseout {
        admitted_seed,
        execution_folklore_inventory,
        query_surface_anchors: WorthGraphReadAccessPlanAdoptionQuerySurfaceAnchors::current(),
        counters,
    })
}

impl WorthGraphReadAccessPlanAdoptionPhaseOneCloseout {
    pub fn milestone_seven_closeout_digest(&self) -> &str {
        self.admitted_seed.seed().milestone_seven_closeout_digest()
    }

    pub fn declaration_catalog_digest(&self) -> &str {
        self.admitted_seed.seed().declaration_catalog_digest()
    }

    pub fn read_family_identities(&self) -> &[WorthGraphReadDeclarationReadFamilyIdentity] {
        self.admitted_seed.seed().read_family_identities()
    }

    pub fn requirement_row_evidence(&self) -> &[WorthGraphReadRequirementRowDigestProjection] {
        self.admitted_seed.seed().requirement_row_evidence()
    }

    pub fn admission_capability_gaps(&self) -> &[WorthGraphReadAdmissionCapabilityGap] {
        self.admitted_seed.seed().admission_capability_gaps()
    }

    pub fn carried_requirement_derivation_gaps(
        &self,
    ) -> &[WorthGraphReadRequirementDerivationCapabilityGap] {
        self.admitted_seed
            .seed()
            .carried_requirement_derivation_gaps()
    }

    pub fn deletion_ledger_report(&self) -> &WorthGraphReadDeclarationDeletionLedgerReport {
        self.admitted_seed.seed().deletion_ledger_report()
    }

    pub fn capped_residue_report(&self) -> &WorthGraphReadDeclarationCappedResidueReport {
        self.admitted_seed.seed().capped_residue_report()
    }

    pub fn source_firewall_report(&self) -> &WorthGraphReadDeclarationSourceFirewallReport {
        self.admitted_seed.seed().source_firewall_report()
    }

    pub const fn execution_folklore_inventory(
        &self,
    ) -> &WorthGraphReadAccessPlanAdoptionExecutionFolkloreInventory {
        &self.execution_folklore_inventory
    }

    pub const fn query_surface_anchors(
        &self,
    ) -> &WorthGraphReadAccessPlanAdoptionQuerySurfaceAnchors {
        &self.query_surface_anchors
    }

    pub const fn counters(&self) -> &WorthGraphReadAccessPlanAdoptionPhaseOneCounters {
        &self.counters
    }

    pub const fn claims_access_plan_admission(&self) -> bool {
        false
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        false
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        false
    }

    pub const fn claims_graph_read_receipts(&self) -> bool {
        false
    }

    pub const fn claims_validator_selection(&self) -> bool {
        false
    }
}
