use super::super::super::super::support::*;

const READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../_docs/forge-query/read-composition-phase1-closeout.md"
));

#[test]
fn runtime_public_read_composition_phase_gate_freezes_completion_and_blockers() {
    let workspace = stateful_bridge_task_runtime()
        .workspace("task.read-composition-phase-gate")
        .expect("task runtime should open a named workspace");
    let support = workspace.public_read_composition_support_report();
    let closeout = workspace.public_read_composition_phase_one_closeout();
    let gate = workspace.public_read_composition_phase_gate();

    assert_eq!(
        gate.backend_posture(),
        ForgeQueryRuntimeBackendPosture::Primary
    );
    assert_eq!(gate.read_support_digest(), support.support_digest());
    assert_eq!(gate.phase_one_closeout_digest(), closeout.closeout_digest());
    assert_eq!(
        gate.support_matrix_digest(),
        workspace.public_support_matrix().matrix_digest().as_str()
    );
    assert_eq!(gate.phase_two_start_family(), "loop_cycle_neighborhood");
    assert_gate_row(
        &gate,
        ForgeQueryReadCompositionPhaseGateFamily::PhaseOneKernelComplete,
        ForgeQueryReadCompositionPhaseGateStatus::Satisfied,
        "generic read kernel is frozen",
    );
    assert_gate_row(
        &gate,
        ForgeQueryReadCompositionPhaseGateFamily::PhaseTwoWorthAdoptionReady,
        ForgeQueryReadCompositionPhaseGateStatus::Satisfied,
        "Worth may begin domain adoption",
    );
    assert_gate_row(
        &gate,
        ForgeQueryReadCompositionPhaseGateFamily::PhaseThreeAggregateProofComplete,
        ForgeQueryReadCompositionPhaseGateStatus::Satisfied,
        "Worth topology now exposes aggregate",
    );
    assert_gate_row(
        &gate,
        ForgeQueryReadCompositionPhaseGateFamily::MilestoneThreeResumeReady,
        ForgeQueryReadCompositionPhaseGateStatus::Satisfied,
        "Milestone 3 may resume",
    );
    assert!(!gate.gate_digest().is_empty());
}

#[test]
fn runtime_public_read_composition_phase_gate_doc_matches_runtime_contract() {
    let workspace = stateful_bridge_task_runtime()
        .workspace("task.read-composition-phase-gate-doc")
        .expect("task runtime should open a named workspace");
    let gate = workspace.public_read_composition_phase_gate();

    assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains("Phase Gate"));
    assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains(gate.phase_two_start_family()));
    for row in gate.rows() {
        assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains(row.family().as_str()));
        assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains(row.status().as_str()));
        assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains(row.reason()));
    }
}

fn assert_gate_row(
    gate: &ForgeQueryReadCompositionPhaseGate,
    family: ForgeQueryReadCompositionPhaseGateFamily,
    status: ForgeQueryReadCompositionPhaseGateStatus,
    reason_fragment: &str,
) {
    assert!(gate.rows().iter().any(|row| {
        row.family() == family && row.status() == status && row.reason().contains(reason_fragment)
    }));
}
