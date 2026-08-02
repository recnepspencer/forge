use super::*;

const DROP_BOUNDARIES: [CertificationPhysicalMutationCheckpoint; 6] = [
    CertificationPhysicalMutationCheckpoint::BeforeEffectCutover,
    CertificationPhysicalMutationCheckpoint::AfterGroupSeal,
    CertificationPhysicalMutationCheckpoint::AfterWalDurability,
    CertificationPhysicalMutationCheckpoint::DuringDataSettlement,
    CertificationPhysicalMutationCheckpoint::DuringRootPublication,
    CertificationPhysicalMutationCheckpoint::BeforeTerminalFinalization,
];

#[test]
fn dropping_the_only_handle_never_cancels_at_any_effect_boundary() {
    for (index, checkpoint) in DROP_BOUNDARIES.into_iter().enumerate() {
        let parent = tempfile::tempdir().unwrap();
        let serving = serving_from_initialization(&parent.path().join("store"));
        let (_, placement, _) = configuration();
        let gate = serving.certification_pause_physical_mutation_at(checkpoint);
        let handle = prepare(
            &serving,
            placement,
            [220 + index as u8; 32],
            b"drop-boundary-matrix",
        )
        .start();
        assert!(
            gate.await_arrival(),
            "mutation did not reach {checkpoint:?}"
        );
        drop(handle);
        gate.release();

        let settlement_deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while serving.physical_mutation_observation().completed() == 0
            && std::time::Instant::now() < settlement_deadline
        {
            std::thread::yield_now();
        }
        assert_eq!(serving.physical_mutation_observation().completed(), 1);
        let shutdown = serving.close().mutations();
        assert_eq!(shutdown.started(), 1);
        assert_eq!(shutdown.completed(), 1);
        assert_eq!(shutdown.completed_unobserved(), 1);
        assert_eq!(shutdown.proven_no_effect(), 0);
        assert_eq!(shutdown.indeterminate(), 0);
    }
}

#[test]
fn close_drains_dropped_handle_work_at_every_effect_boundary() {
    for (index, checkpoint) in DROP_BOUNDARIES.into_iter().enumerate() {
        let parent = tempfile::tempdir().unwrap();
        let serving = serving_from_initialization(&parent.path().join("store"));
        let (_, placement, _) = configuration();
        let gate = serving.certification_pause_physical_mutation_at(checkpoint);
        let closing_gate = (checkpoint
            == CertificationPhysicalMutationCheckpoint::BeforeEffectCutover)
            .then(|| {
                serving.certification_pause_physical_mutation_at(
                    CertificationPhysicalMutationCheckpoint::RuntimeClosingMarked,
                )
            });
        let handle = prepare(
            &serving,
            placement,
            [230 + index as u8; 32],
            b"close-drain-boundary-matrix",
        )
        .start();
        assert!(
            gate.await_arrival(),
            "mutation did not reach {checkpoint:?}"
        );
        drop(handle);

        let close = std::thread::spawn(move || serving.close());
        if let Some(closing_gate) = &closing_gate {
            assert!(closing_gate.await_arrival());
        }
        gate.release();
        if let Some(closing_gate) = closing_gate {
            closing_gate.release();
        }
        let shutdown = close
            .join()
            .expect("close thread must not panic")
            .mutations();
        assert_eq!(shutdown.started(), 1);
        assert_eq!(
            shutdown.completed() + shutdown.proven_no_effect() + shutdown.indeterminate(),
            1
        );
        if checkpoint == CertificationPhysicalMutationCheckpoint::BeforeEffectCutover {
            assert_eq!(shutdown.proven_no_effect(), 1);
            assert_eq!(shutdown.completed_unobserved(), 0);
        } else {
            assert_eq!(shutdown.completed(), 1);
            assert_eq!(shutdown.completed_unobserved(), 1);
        }
    }
}
