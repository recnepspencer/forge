use worth_ui::facade::app::WorthUiNativeApplicationShell;
use worth_ui::facade::inspection::{UiPixelsRequired, UiVisualSnapshotReceipt};

use crate::lifecycle_observation_publication::PlatformPulseObservationPublisher;

use super::super::{
    PlatformPulseRetainedSnapshot, PlatformPulseVisualExecutionDenial,
    PlatformPulseVisualIdentityState,
};

pub(super) fn retain_refreshed_snapshot(
    successor: UiVisualSnapshotReceipt<UiPixelsRequired>,
    publisher: &PlatformPulseObservationPublisher,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    publisher
        .refreshed_visual_snapshot(&successor)
        .map_err(PlatformPulseVisualExecutionDenial::Observation)?;
    Ok(PlatformPulseVisualIdentityState::ComparisonReady(
        PlatformPulseRetainedSnapshot {
            snapshot: successor,
            overlay_clear: None,
        },
    ))
}

pub(super) fn retire_refresh_predecessor(
    retained: PlatformPulseRetainedSnapshot,
    successor: &UiVisualSnapshotReceipt<UiPixelsRequired>,
    shell: &mut WorthUiNativeApplicationShell,
    publisher: &PlatformPulseObservationPublisher,
) -> Result<(), PlatformPulseVisualExecutionDenial> {
    match retained.snapshot.relation() {
        Ok(_) => retire_snapshot(retained, shell, publisher),
        Err(worth_ui::facade::inspection::UiVisualSnapshotRelationDenial::ExpiredFrame)
        | Err(worth_ui::facade::inspection::UiVisualSnapshotRelationDenial::UnknownFrame) => {
            let predecessor_frame = retained.snapshot.affinity().frame();
            let successor_frame = successor.affinity().frame();
            if predecessor_frame == successor_frame {
                return Err(PlatformPulseVisualExecutionDenial::SnapshotStillCurrent);
            }
            let snapshot = retained.snapshot.identity();
            let disposal = shell.dispose_visual_snapshot(retained.snapshot);
            publisher
                .visual_snapshot_retired_after_current_successor(
                    snapshot,
                    predecessor_frame,
                    successor_frame,
                    disposal,
                )
                .map_err(PlatformPulseVisualExecutionDenial::Observation)
        }
        Err(denial) => Err(PlatformPulseVisualExecutionDenial::SnapshotRelation(denial)),
    }
}

pub(in crate::visual_identity_execution) fn retire_snapshot(
    retained: PlatformPulseRetainedSnapshot,
    shell: &mut WorthUiNativeApplicationShell,
    publisher: &PlatformPulseObservationPublisher,
) -> Result<(), PlatformPulseVisualExecutionDenial> {
    let relation = retained
        .snapshot
        .relation()
        .map_err(PlatformPulseVisualExecutionDenial::SnapshotRelation)?;
    match relation {
        worth_ui::facade::inspection::UiVisualSnapshotRelation::Current => {
            return Err(PlatformPulseVisualExecutionDenial::SnapshotStillCurrent)
        }
        worth_ui::facade::inspection::UiVisualSnapshotRelation::RetainedPredecessor
        | worth_ui::facade::inspection::UiVisualSnapshotRelation::Historical => {}
    };
    let snapshot = retained.snapshot.identity();
    let disposal = shell.dispose_visual_snapshot(retained.snapshot);
    publisher
        .visual_snapshot_retired(snapshot, relation, disposal)
        .map_err(PlatformPulseVisualExecutionDenial::Observation)
}

pub(super) fn next_presentation_ticks(
    tick: &mut u64,
) -> Result<(u64, u64), PlatformPulseVisualExecutionDenial> {
    let current = tick
        .checked_add(1)
        .ok_or(PlatformPulseVisualExecutionDenial::TickExhausted)?;
    let deadline = current
        .checked_add(1)
        .ok_or(PlatformPulseVisualExecutionDenial::TickExhausted)?;
    *tick = current;
    Ok((deadline, current))
}
