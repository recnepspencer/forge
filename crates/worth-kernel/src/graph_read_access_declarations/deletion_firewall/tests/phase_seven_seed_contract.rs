use super::phase_chain_fixture::production_phase_six_seed;
use crate::graph_read_access_declarations::current_worth_graph_read_declaration_deletion_firewall_closeout;

#[test]
fn phase_seven_seed_preserves_gap_and_residue_visibility() {
    let phase_six_seed = production_phase_six_seed();
    let closeout = current_worth_graph_read_declaration_deletion_firewall_closeout(&phase_six_seed)
        .expect("Phase 6 closeout should build");
    let phase_seven_seed = closeout.phase_seven_seed();

    assert_eq!(
        phase_seven_seed.admission_closeout_digest(),
        phase_six_seed.admission_closeout_digest()
    );
    assert_eq!(
        phase_seven_seed.deletion_firewall_digest(),
        closeout.closeout_digest()
    );
    assert_eq!(
        phase_seven_seed.admission_capability_gaps(),
        phase_six_seed.admission_capability_gaps()
    );
    assert_eq!(
        phase_seven_seed.carried_requirement_derivation_gaps(),
        phase_six_seed.carried_requirement_derivation_gaps()
    );
    assert_eq!(phase_seven_seed.capped_residue_report().residue_count(), 0);
}

#[test]
fn phase_six_does_not_claim_execution_receipts_or_plan_consumption() {
    let phase_six_seed = production_phase_six_seed();
    let closeout = current_worth_graph_read_declaration_deletion_firewall_closeout(&phase_six_seed)
        .expect("Phase 6 closeout should build");
    let phase_seven_seed = closeout.phase_seven_seed();

    assert!(!closeout.claims_graph_read_execution());
    assert!(!closeout.claims_access_plan_consumption());
    assert!(!phase_seven_seed.claims_graph_read_execution());
    assert!(!phase_seven_seed.claims_access_plan_consumption());
}
