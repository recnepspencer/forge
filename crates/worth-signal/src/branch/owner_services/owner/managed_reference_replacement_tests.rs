use crate::branch::ManagedSignalBranchReferenceAdmissionDenial;
use std::sync::Arc;

use super::super::tests::runtime_root::runtime_with_two_branches;
use super::super::SignalBranchCellState;

#[test]
fn managed_reference_clones_do_not_strongly_own_the_owner_lifecycle() {
    let (mut runtime, _, _, basis) = runtime_with_two_branches();
    let (port, _, _) = runtime.owner_port_slots().expect("runtime seals");
    let owner = port.upgrade_owner().expect("sealed owner remains live");
    let admission = owner.admit().expect("cell inspection admits");
    let cell = owner
        .lookup_cell(&admission, basis.owner_branch_id())
        .expect("the real registry supplies the referenced cell");
    let strong_before = Arc::strong_count(&owner.lifecycle);
    let cell_strong_before = Arc::strong_count(&cell);
    let reference = owner
        .issue_managed_branch_reference(&basis)
        .expect("a real admitted basis issues its branch reference");
    let cloned = reference.clone();

    assert_eq!(Arc::strong_count(&owner.lifecycle), strong_before);
    assert_eq!(Arc::strong_count(&cell), cell_strong_before);
    drop(reference);
    drop(cloned);
    assert_eq!(Arc::strong_count(&owner.lifecycle), strong_before);
    assert_eq!(Arc::strong_count(&cell), cell_strong_before);
}

#[test]
fn replaced_cell_incarnation_cannot_reauthorize_an_owner_issued_reference() {
    let (mut runtime, _, branch, basis) = runtime_with_two_branches();
    let (port, _, _) = runtime.owner_port_slots().expect("runtime seals");
    let owner = port.upgrade_owner().expect("sealed owner remains live");
    let reference = owner
        .issue_managed_branch_reference(&basis)
        .expect("a real admitted basis issues its branch reference");
    let admission = owner.admit().expect("replacement setup admits");
    let original = owner
        .lookup_cell(&admission, branch.id)
        .expect("the real registry owns the referenced cell");
    let (handle, state, generation, restore_snapshot_id) = original
        .with_state(&admission, |state, _| {
            (
                state.handle().clone(),
                state.state().clone(),
                state.head_generation(),
                state.restore_snapshot_id(),
            )
        })
        .expect("the canonical cell supplies replacement state");

    owner
        .registry
        .begin_retirement(&admission, branch.id)
        .expect("the real cell enters retirement")
        .execute(|_, _| Ok::<(), ()>(()))
        .expect("registry admission remains valid")
        .expect("the old cell retires");
    owner
        .registry
        .reserve(&admission, branch.id)
        .expect("the retired branch number becomes vacant")
        .install(SignalBranchCellState::new(
            handle,
            owner.runtime_instance_id,
            owner.definition_basis,
            state,
            generation,
            restore_snapshot_id,
        ))
        .expect("the registry installs a distinct cell incarnation");

    assert!(matches!(
        owner.admit_managed_branch_reference(&reference),
        Err(ManagedSignalBranchReferenceAdmissionDenial::BranchIncarnationReplaced)
    ));
}
