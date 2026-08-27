use std::time::{Duration, Instant};

use crate::external_observation::PlatformPulseLifecycleStreamFailure;
use crate::external_observation::{NativeInputProbeKind, NativeKeyboardCommand};
use crate::native_platform::NativePlatformContract;

use super::{
    incidental_visual, unexpected, NativeBoundExecutableWorld, PlatformPulsePortalJourneyFailure,
    WatchedPulseObservationFailure,
};

pub(super) fn activate(
    world: &mut NativeBoundExecutableWorld,
    point: crate::external_observation::NativeClientPixelPoint,
) -> Result<(), PlatformPulsePortalJourneyFailure> {
    let delivery = world
        .platform
        .deliver_pointer_activation(&world.native_client, point)
        .map_err(PlatformPulsePortalJourneyFailure::Native)?;
    require_delivery(delivery, NativeInputProbeKind::Pointer)
}

pub(super) fn escape(
    world: &mut NativeBoundExecutableWorld,
) -> Result<(), PlatformPulsePortalJourneyFailure> {
    keyboard(world, NativeKeyboardCommand::Escape)
}

pub(super) fn require_intent_quiet_after_occupancy_click(
    world: &mut NativeBoundExecutableWorld,
) -> Result<(), PlatformPulsePortalJourneyFailure> {
    let deadline = Instant::now() + Duration::from_millis(500);
    loop {
        match world.lifecycle.next(deadline) {
            Ok(envelope) if incidental_visual(envelope.outcome()) => {}
            Ok(envelope) => return Err(unexpected(envelope.outcome())),
            Err(PlatformPulseLifecycleStreamFailure::Deadline) => return Ok(()),
            Err(failure) => {
                return Err(PlatformPulsePortalJourneyFailure::Observation(
                    WatchedPulseObservationFailure::Lifecycle(failure),
                ))
            }
        }
    }
}

fn keyboard(
    world: &mut NativeBoundExecutableWorld,
    command: NativeKeyboardCommand,
) -> Result<(), PlatformPulsePortalJourneyFailure> {
    let delivery = world
        .platform
        .deliver_keyboard_command(&world.native_client, command)
        .map_err(PlatformPulsePortalJourneyFailure::Native)?;
    require_delivery(delivery, NativeInputProbeKind::Keyboard)
}

fn require_delivery(
    delivery: crate::external_observation::NativeInputDeliveryObservation,
    expected: NativeInputProbeKind,
) -> Result<(), PlatformPulsePortalJourneyFailure> {
    if delivery.kind() == expected && delivery.delivered_event_count() == 2 {
        Ok(())
    } else {
        Err(PlatformPulsePortalJourneyFailure::InputDelivery(
            "native command did not remain one exact two-event delivery",
        ))
    }
}
