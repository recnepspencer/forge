use super::*;

#[test]
fn cancellation_after_each_possible_effect_boundary_remains_effectful() {
    for (index, checkpoint) in [
        CertificationPhysicalMutationCheckpoint::AfterGroupSeal,
        CertificationPhysicalMutationCheckpoint::AfterWalDurability,
        CertificationPhysicalMutationCheckpoint::DuringDataSettlement,
        CertificationPhysicalMutationCheckpoint::DuringRootPublication,
        CertificationPhysicalMutationCheckpoint::BeforeTerminalFinalization,
    ]
    .into_iter()
    .enumerate()
    {
        let parent = tempfile::tempdir().unwrap();
        let serving = serving_from_initialization(&parent.path().join("store"));
        let (_, placement, _) = configuration();
        let gate = serving.certification_pause_physical_mutation_at(checkpoint);
        let handle = prepare(
            &serving,
            placement,
            [200 + index as u8; 32],
            b"effectful-cancellation-matrix",
        )
        .start();
        assert!(
            gate.await_arrival(),
            "mutation did not reach {checkpoint:?}"
        );
        assert!(matches!(
            handle.request_cancellation(),
            PhysicalMutationCancellationOutcome::SettlementAlreadyEffectful { .. }
        ));
        gate.release();
        completed(handle.wait());
        let shutdown = serving.close().mutations();
        assert_eq!(shutdown.completed(), 1);
        assert_eq!(shutdown.cancellation_effectful(), 1);
    }
}

#[test]
fn deadline_elapsed_after_group_seal_cannot_rewrite_effectful_fate() {
    let parent = tempfile::tempdir().unwrap();
    let serving = serving_from_initialization(&parent.path().join("store"));
    let (_, placement, _) = configuration();
    let gate = serving.certification_pause_physical_mutation_at(
        CertificationPhysicalMutationCheckpoint::AfterGroupSeal,
    );
    let handle =
        prepare_with_deadline(&serving, placement, [210; 32], b"post-effect-deadline", 7).start();
    assert!(gate.await_arrival());
    serving
        .certification_advance_physical_signal_clock(
            worth_signal::facade::ClockAdvanceRequest::new(
                worth_signal::facade::ClockDomain::MonotonicExecution,
                worth_signal::facade::ClockTick::new(7),
            ),
        )
        .unwrap();
    gate.release();
    completed(handle.wait());
    let shutdown = serving.close().mutations();
    assert_eq!(shutdown.completed(), 1);
    assert_eq!(shutdown.proven_no_effect(), 0);
}
