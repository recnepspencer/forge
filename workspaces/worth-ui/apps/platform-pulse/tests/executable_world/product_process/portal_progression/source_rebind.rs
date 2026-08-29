use std::time::Instant;

use worth_ui_platform_pulse::observation_contract::PlatformPulseLifecycleObservation;

use crate::adjudication::{
    adjudicate_focus_fallback_portal_pixels, PlatformPulsePortalFocusFallbackPixelEvidence,
};
use crate::installation::CanonicalPlatformPulse;
use crate::source_delta::{PortalFocusFallbackSourceDelta, PulseSourceDeltaIdentity};

use super::{
    capture, incidental_visual, next, unexpected, NativeBoundExecutableWorld,
    PlatformPulsePortalJourneyFailure, WatchedPulseTransition, PIXEL_POLL_SLICE,
    TRANSITION_DEADLINE,
};

pub(super) struct PlatformPulsePortalRebindEvidence {
    replacement_sequence: u64,
    pixels: PlatformPulsePortalFocusFallbackPixelEvidence,
}

pub(super) fn exercise(
    world: &mut NativeBoundExecutableWorld,
    before: &crate::external_observation::NativeClientPixelCapture,
) -> Result<PlatformPulsePortalRebindEvidence, PlatformPulsePortalJourneyFailure> {
    let delta =
        PortalFocusFallbackSourceDelta::from_checked_in(CanonicalPlatformPulse::checked_in())
            .map_err(PlatformPulsePortalJourneyFailure::SourceDefinition)?;
    let action = delta
        .apply(&world.installation)
        .map_err(PlatformPulsePortalJourneyFailure::SourceAction)?;
    if action.identity() != PulseSourceDeltaIdentity::PortalFocusFallback
        || action.action_count() != 1
        || action.written_bytes() == 0
        || action.content_fingerprint() == 0
        || action.entry_source() != world.installation.entry_source()
    {
        return Err(PlatformPulsePortalJourneyFailure::RuntimeServiceEvidence(
            "portal focus fallback was not one exact external source action",
        ));
    }
    let replacement_sequence = await_replacement(world)?;
    let pixels = await_fallback_pixels(world, before)?;
    Ok(PlatformPulsePortalRebindEvidence {
        replacement_sequence,
        pixels,
    })
}

fn await_replacement(
    world: &mut NativeBoundExecutableWorld,
) -> Result<u64, PlatformPulsePortalJourneyFailure> {
    loop {
        let envelope = next(world, WatchedPulseTransition::GreenReplacement)?;
        match envelope.outcome() {
            PlatformPulseLifecycleObservation::RebindPublished(replacement)
                if replacement.actual_native_effect_count() > 0
                    && replacement.schema_transition().is_none() =>
            {
                return Ok(envelope.sequence().value())
            }
            outcome if incidental_visual(outcome) => {}
            outcome => return Err(unexpected(outcome)),
        }
    }
}

fn await_fallback_pixels(
    world: &mut NativeBoundExecutableWorld,
    before: &crate::external_observation::NativeClientPixelCapture,
) -> Result<PlatformPulsePortalFocusFallbackPixelEvidence, PlatformPulsePortalJourneyFailure> {
    let deadline = Instant::now() + TRANSITION_DEADLINE;
    loop {
        let after = capture(world)?;
        if let Ok(evidence) = adjudicate_focus_fallback_portal_pixels(before, &after) {
            return Ok(evidence);
        }
        if Instant::now() >= deadline {
            return adjudicate_focus_fallback_portal_pixels(before, &after)
                .map_err(PlatformPulsePortalJourneyFailure::Pixels);
        }
        std::thread::sleep(PIXEL_POLL_SLICE);
    }
}

impl PlatformPulsePortalRebindEvidence {
    pub(crate) const fn replacement_sequence(&self) -> u64 {
        self.replacement_sequence
    }

    pub(crate) const fn pixels(&self) -> PlatformPulsePortalFocusFallbackPixelEvidence {
        self.pixels
    }
}
