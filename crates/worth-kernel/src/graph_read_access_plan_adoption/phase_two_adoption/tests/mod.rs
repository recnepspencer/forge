mod declare_once_routing;
mod execution_boundary;
mod pairing_uniqueness;
mod query_admission_postures;
mod source_firewall;
mod structured_seed_consumption;

use super::closeout::current_worth_graph_read_access_plan_adoption_phase_two_closeout;
use super::read_family_adoption::WorthGraphReadAccessPlanAdoptionLedger;
use crate::graph_read_access_declarations::{
    WorthGraphReadAdmissionCapabilityGap, WorthGraphReadDeclarationReadFamilyIdentity,
    WorthGraphReadRequirementRowDigestProjection,
};
use crate::graph_read_access_plan_adoption::phase_one_closeout::current_worth_graph_read_access_plan_adoption_phase_one_closeout;
use crate::graph_read_access_plan_adoption::test_fixtures::production_milestone_eight_seed;

fn production_phase_two_closeout(
) -> super::closeout::WorthGraphReadAccessPlanAdoptionPhaseTwoCloseout {
    let seed = production_milestone_eight_seed();
    let phase_one = current_worth_graph_read_access_plan_adoption_phase_one_closeout(&seed)
        .expect("Phase 1 should close from production seed");
    current_worth_graph_read_access_plan_adoption_phase_two_closeout(&phase_one)
        .expect("Phase 2 should close from Phase 1")
}

fn production_phase_one_closeout(
) -> crate::graph_read_access_plan_adoption::WorthGraphReadAccessPlanAdoptionPhaseOneCloseout {
    let seed = production_milestone_eight_seed();
    current_worth_graph_read_access_plan_adoption_phase_one_closeout(&seed)
        .expect("Phase 1 should close from production seed")
}

fn adoption_ledger_from_rows(
    read_family_identities: Vec<WorthGraphReadDeclarationReadFamilyIdentity>,
    requirement_rows: Vec<WorthGraphReadRequirementRowDigestProjection>,
    carried_gaps: &[WorthGraphReadAdmissionCapabilityGap],
) -> Result<
    WorthGraphReadAccessPlanAdoptionLedger,
    super::errors::WorthGraphReadAccessPlanAdoptionPhaseTwoError,
> {
    WorthGraphReadAccessPlanAdoptionLedger::from_phase_one_closeout(
        "phase-seven-closeout-test",
        "declaration-catalog-test",
        &read_family_identities,
        &requirement_rows,
        carried_gaps,
    )
}

fn read_family_row(
    catalog: &str,
    family: &str,
    touched_authority: &str,
) -> WorthGraphReadDeclarationReadFamilyIdentity {
    WorthGraphReadDeclarationReadFamilyIdentity::for_access_plan_adoption_test(
        catalog,
        format!("{family}_name"),
        family,
        touched_authority,
        format!("{family}_target"),
    )
}

fn requirement_row(
    requirement: &str,
    catalog: &str,
    family: &str,
) -> WorthGraphReadRequirementRowDigestProjection {
    WorthGraphReadRequirementRowDigestProjection::for_access_plan_adoption_test(
        requirement,
        catalog,
        family,
    )
}
