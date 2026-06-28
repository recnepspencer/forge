use forge_store_recovery_physics::{
    CheckpointIntervalContract, RecoveryBoundednessEvidence, RecoveryPhysicsCertificationBundle,
    RecoveryPhysicsCloseoutCollector, RecoveryPhysicsCloseoutDenial,
    RecoveryPhysicsCloseoutEvidence, WalCheckpointLsnRecoveryPhysicsSuite, WalTailReplayBudget,
};

use super::crash_evidence::{crash_scheduler_evidence, required_crash_seams};
use super::executed_recovery::{
    executed_recovery_fixture, executed_recovery_receipt_with_cursor_lsn, CloseoutFixture,
};
use super::shortcut_evidence::all_shortcut_evidence;

pub fn certify_complete_closeout() -> RecoveryPhysicsCertificationBundle {
    WalCheckpointLsnRecoveryPhysicsSuite::from_required_s4_lanes()
        .certify(complete_closeout_evidence())
        .unwrap()
}

pub fn complete_closeout_evidence() -> RecoveryPhysicsCloseoutEvidence {
    let fixture = executed_recovery_fixture();
    complete_closeout_collector(&fixture).finish().unwrap()
}

pub fn evidence_with_missing_crash_seam() -> RecoveryPhysicsCloseoutEvidence {
    let fixture = executed_recovery_fixture();
    let mut collector = base_closeout_collector(&fixture);
    for seam in required_crash_seams()
        .into_iter()
        .filter(|seam| *seam != forge_store_recovery_physics::S4RecoveryCrashSeam::RenameDurability)
    {
        collector = collector
            .record_crash_recovery(crash_scheduler_evidence(seam).unwrap())
            .unwrap();
    }
    for evidence in all_shortcut_evidence() {
        collector = collector.record_synthetic_shortcut_denial(evidence);
    }
    collector.finish().unwrap()
}

pub fn evidence_with_missing_shortcut_rejection_denial() -> RecoveryPhysicsCloseoutDenial {
    let fixture = executed_recovery_fixture();
    let mut collector = base_closeout_collector(&fixture);
    for seam in required_crash_seams() {
        collector = collector
            .record_crash_recovery(crash_scheduler_evidence(seam).unwrap())
            .unwrap();
    }
    collector.finish().unwrap_err()
}

pub fn mixed_authority_closeout_denial() -> RecoveryPhysicsCloseoutDenial {
    let fixture = executed_recovery_fixture();
    let alien = executed_recovery_receipt_with_cursor_lsn(20);
    RecoveryPhysicsCloseoutCollector::from_executed_recovery(
        fixture.receipt.clone(),
        fixture.source_trace,
        fixture.foundational_evidence,
        RecoveryBoundednessEvidence::from_admitted_budget(
            CheckpointIntervalContract::max_tail_frames(4),
            WalTailReplayBudget::max_frames(4)
                .with_max_scanned_segments(2)
                .with_max_page_redos(4),
            &alien,
        )
        .unwrap(),
    )
    .unwrap_err()
}

pub fn unbounded_closeout_denial() -> RecoveryPhysicsCloseoutDenial {
    let fixture = executed_recovery_fixture();
    RecoveryBoundednessEvidence::from_admitted_budget(
        CheckpointIntervalContract::max_tail_frames(0),
        WalTailReplayBudget::max_frames(0),
        &fixture.receipt,
    )
    .unwrap_err()
}

fn complete_closeout_collector(fixture: &CloseoutFixture) -> RecoveryPhysicsCloseoutCollector {
    let mut collector = base_closeout_collector(fixture);
    for seam in required_crash_seams() {
        collector = collector
            .record_crash_recovery(crash_scheduler_evidence(seam).unwrap())
            .unwrap();
    }
    for evidence in all_shortcut_evidence() {
        collector = collector.record_synthetic_shortcut_denial(evidence);
    }
    collector
}

fn base_closeout_collector(fixture: &CloseoutFixture) -> RecoveryPhysicsCloseoutCollector {
    let boundedness = RecoveryBoundednessEvidence::from_admitted_budget(
        CheckpointIntervalContract::max_tail_frames(4),
        WalTailReplayBudget::max_frames(4)
            .with_max_scanned_segments(2)
            .with_max_page_redos(4),
        &fixture.receipt,
    )
    .unwrap();
    RecoveryPhysicsCloseoutCollector::from_executed_recovery(
        fixture.receipt.clone(),
        fixture.source_trace.clone(),
        fixture.foundational_evidence.clone(),
        boundedness,
    )
    .unwrap()
}
