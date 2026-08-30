use crate::domain_computation::primary_graph::WorthQueryApplicationPinnedBasisDenialKind;

#[test]
fn real_snapshot_capacity_reports_exact_public_limit_without_residue() {
    const MAXIMUM_ACTIVE_SNAPSHOTS: usize = 3;
    let world = crate::domain_computation::primary_graph::tests::fixture::installed_authorization_world_with_active_snapshot_limit(
        MAXIMUM_ACTIVE_SNAPSHOTS,
    );
    let handle = world
        .application
        .runtime
        .primary_graph()
        .expect("test world publishes a primary graph")
        .integration_handle();
    let (before, snapshots) = handle.with_runtime_mut(|runtime| {
        let identity = runtime.main_branch_identity();
        let (_, basis) = runtime.observe_branch(&identity).unwrap();
        let before = basis.descriptor().clone();
        let snapshots = (0..MAXIMUM_ACTIVE_SNAPSHOTS)
            .map(|_| {
                runtime
                    .snapshots()
                    .snapshot_for_observation(&basis.observation())
                    .unwrap()
            })
            .collect::<Vec<_>>();
        assert_eq!(
            runtime.retention().inspect_plan().active_snapshot_count,
            MAXIMUM_ACTIVE_SNAPSHOTS
        );
        (before, snapshots)
    });

    let denial = world
        .application
        .pin_current_application_query_basis(&super::live_scope())
        .err()
        .expect("the public application lane must preserve owner capacity exhaustion");
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationPinnedBasisDenialKind::ActiveSnapshotCapacityExhausted {
            maximum_active_snapshots: MAXIMUM_ACTIVE_SNAPSHOTS,
        }
    );

    handle.with_runtime_mut(|runtime| {
        for snapshot in snapshots {
            runtime.snapshots().release_snapshot(&snapshot).unwrap();
        }
        let identity = runtime.main_branch_identity();
        let (_, basis) = runtime.observe_branch(&identity).unwrap();
        assert_eq!(basis.descriptor(), &before);
        assert_eq!(runtime.retention().inspect_plan().active_snapshot_count, 0);
    });
}
