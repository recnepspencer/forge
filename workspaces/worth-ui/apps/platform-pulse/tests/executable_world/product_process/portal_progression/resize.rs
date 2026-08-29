use std::time::Instant;

use worth_ui_platform_pulse::observation_contract::PlatformPulseLifecycleObservation;

use crate::adjudication::{
    adjudicate_authored_portal_pixels, adjudicate_open_portal_pixels,
    adjudicate_resized_wrapping_text, PlatformPulseAuthoredPortalPixelEvidence,
};
use crate::native_platform::NativePlatformContract;

use super::{
    capture, next, NativeBoundExecutableWorld, PlatformPulsePortalJourneyFailure,
    WatchedPulseTransition, PIXEL_POLL_SLICE, TRANSITION_DEADLINE,
};

const DEFAULT_EXTENT: [u32; 2] = [960, 600];
const RESIZED_EXTENT: [u32; 2] = [1_120, 700];

pub(super) fn exercise(
    world: &mut NativeBoundExecutableWorld,
    default_baseline: &crate::external_observation::NativeClientPixelCapture,
) -> Result<PlatformPulseAuthoredPortalPixelEvidence, PlatformPulsePortalJourneyFailure> {
    let observed = world
        .platform
        .observe_bound_client_area(&world.native_client)
        .map_err(PlatformPulsePortalJourneyFailure::Native)?;
    let resized_physical = project_extent(RESIZED_EXTENT, observed.dpi());
    let deadline = Instant::now() + TRANSITION_DEADLINE;
    world
        .platform
        .resize_bound_client_area(&mut world.native_client, resized_physical, deadline)
        .map_err(PlatformPulsePortalJourneyFailure::Native)?;
    await_presented_extent(world, resized_physical)?;
    let resized = await_authored(world, RESIZED_EXTENT, resized_physical, deadline)?;

    let restored_physical = project_extent(DEFAULT_EXTENT, observed.dpi());
    let deadline = Instant::now() + TRANSITION_DEADLINE;
    world
        .platform
        .resize_bound_client_area(&mut world.native_client, restored_physical, deadline)
        .map_err(PlatformPulsePortalJourneyFailure::Native)?;
    await_presented_extent(world, restored_physical)?;
    await_default_open(world, default_baseline, restored_physical, deadline)?;
    Ok(resized)
}

fn await_presented_extent(
    world: &mut NativeBoundExecutableWorld,
    expected: [u32; 2],
) -> Result<(), PlatformPulsePortalJourneyFailure> {
    loop {
        let envelope = next(world, WatchedPulseTransition::VisualSnapshot)?;
        match envelope.outcome() {
            PlatformPulseLifecycleObservation::VisualSnapshotCaptured(snapshot)
                if snapshot.coordinates().client_physical_dimensions() == expected =>
            {
                return Ok(())
            }
            PlatformPulseLifecycleObservation::VisualSnapshotCaptured(_)
            | PlatformPulseLifecycleObservation::VisualPointTrace(_)
            | PlatformPulseLifecycleObservation::VisualOverlayPublished(_)
            | PlatformPulseLifecycleObservation::VisualOverlayCleared(_)
            | PlatformPulseLifecycleObservation::VisualSnapshotRetired(_)
            | PlatformPulseLifecycleObservation::VisualComparison(_) => {}
            outcome => return Err(super::unexpected(outcome)),
        }
    }
}

fn await_authored(
    world: &mut NativeBoundExecutableWorld,
    logical_extent: [u32; 2],
    physical_extent: [u32; 2],
    deadline: Instant,
) -> Result<PlatformPulseAuthoredPortalPixelEvidence, PlatformPulsePortalJourneyFailure> {
    loop {
        let current = capture(world)?;
        if [current.width(), current.height()] == physical_extent {
            if let Ok(evidence) = adjudicate_authored_portal_pixels(&current, logical_extent) {
                if adjudicate_resized_wrapping_text(&current).is_ok() {
                    return Ok(evidence);
                }
            }
        }
        if Instant::now() >= deadline {
            let evidence = adjudicate_authored_portal_pixels(&current, logical_extent)
                .map_err(PlatformPulsePortalJourneyFailure::Pixels)?;
            adjudicate_resized_wrapping_text(&current)
                .map_err(PlatformPulsePortalJourneyFailure::TextClipping)?;
            return Ok(evidence);
        }
        std::thread::sleep(PIXEL_POLL_SLICE);
    }
}

fn await_default_open(
    world: &mut NativeBoundExecutableWorld,
    baseline: &crate::external_observation::NativeClientPixelCapture,
    physical_extent: [u32; 2],
    deadline: Instant,
) -> Result<(), PlatformPulsePortalJourneyFailure> {
    loop {
        let current = capture(world)?;
        if [current.width(), current.height()] == physical_extent
            && adjudicate_open_portal_pixels(baseline, &current).is_ok()
        {
            return Ok(());
        }
        if Instant::now() >= deadline {
            return adjudicate_open_portal_pixels(baseline, &current)
                .map(|_| ())
                .map_err(PlatformPulsePortalJourneyFailure::Pixels);
        }
        std::thread::sleep(PIXEL_POLL_SLICE);
    }
}

fn project_extent(logical: [u32; 2], dpi: u32) -> [u32; 2] {
    logical.map(|value| ((u64::from(value) * u64::from(dpi) + 48) / 96) as u32)
}
