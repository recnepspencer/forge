use std::time::{Duration, Instant};

use worth_ui_platform_pulse::observation_contract::{
    PlatformPulseIntentAttemptObservationReference, PlatformPulseIntentPostureObservation,
    PlatformPulseLifecycleObservation, PlatformPulseSemanticFocusPublished,
};

use crate::adjudication::{
    adjudicate_closed_portal_pixels, adjudicate_open_portal_pixels,
    adjudicate_portal_control_point, portal_action_points, portal_occupancy_point,
    IntentControlPointFailure, PlatformPulseAuthoredPortalPixelEvidence,
    PlatformPulsePortalPixelEvidence, PlatformPulsePortalPixelFailure,
};
use crate::external_observation::PlatformPulseLifecycleStreamFailure;
use crate::failure_teardown::{
    teardown_native_bound_world, PulseExecutableWorldFailure, PulseExecutableWorldFailureReport,
};
use crate::native_platform::{NativePlatformContract, NativePlatformFailure};

use super::{
    await_watched_observation, FinalRecovered, NativeBoundExecutableWorld, Published,
    PulseExecutableWorld, WatchedPulseObservationFailure, WatchedPulseTransition,
};

mod focus_observation;
mod input;
mod resize;
use focus_observation::{
    await_portal_dismissed, await_semantic_focus, require_open_focus, require_restoration,
};
use input::{activate, escape, require_intent_quiet_after_occupancy_click};

const TRANSITION_DEADLINE: Duration = Duration::from_secs(5);
const PIXEL_POLL_SLICE: Duration = Duration::from_millis(10);
const LIFECYCLE_IDLE_SLICE: Duration = Duration::from_millis(100);
#[derive(Debug)]
pub(crate) enum PlatformPulsePortalJourneyFailure {
    Native(NativePlatformFailure),
    ControlPoint(IntentControlPointFailure),
    Pixels(PlatformPulsePortalPixelFailure),
    Observation(WatchedPulseObservationFailure),
    FocusEvidence(&'static str),
    UnexpectedObservation(String),
    InputDelivery(&'static str),
}

pub(crate) struct PlatformPulsePortalJourneyEvidence {
    intent_close_pixels: PlatformPulsePortalPixelEvidence,
    resized_open_pixels: PlatformPulseAuthoredPortalPixelEvidence,
    escape_close_pixels: PlatformPulsePortalPixelEvidence,
    first_open_focus: PlatformPulseSemanticFocusPublished,
    intent_close_focus: PlatformPulseSemanticFocusPublished,
    second_open_focus: PlatformPulseSemanticFocusPublished,
    escape_close_focus: PlatformPulseSemanticFocusPublished,
    escape_dismissed_frame: u64,
    expected_shutdown_sequence: u64,
}

pub(crate) struct CompletedPlatformPulsePortalJourney {
    recovered: PulseExecutableWorld<Published<FinalRecovered>>,
    evidence: PlatformPulsePortalJourneyEvidence,
}

impl PulseExecutableWorld<Published<FinalRecovered>> {
    pub(crate) fn complete_portal_journey(
        self,
    ) -> Result<CompletedPlatformPulsePortalJourney, PulseExecutableWorldFailureReport> {
        let Published { mut world, stage } = self.state;
        let result = complete(&mut world);
        match result {
            Ok(evidence) => Ok(CompletedPlatformPulsePortalJourney {
                recovered: PulseExecutableWorld {
                    state: Published { world, stage },
                },
                evidence,
            }),
            Err(failure) => Err(teardown_native_bound_world(
                PulseExecutableWorldFailure::PortalJourney(failure),
                world.into_failure_resources(),
            )),
        }
    }
}

fn complete(
    world: &mut NativeBoundExecutableWorld,
) -> Result<PlatformPulsePortalJourneyEvidence, PlatformPulsePortalJourneyFailure> {
    let baseline = capture(world)?;
    let target = adjudicate_portal_control_point(&baseline)
        .map_err(PlatformPulsePortalJourneyFailure::ControlPoint)?;

    activate(world, target.point())?;
    let first_open_focus = await_completed_portal_intent(world)?;
    require_open_focus(first_open_focus)?;
    let opened = await_open_pixels(world, &baseline)?;
    let resized_open_pixels = resize::exercise(world, &baseline)?;
    let resized_capture = capture(world)?;
    let occupancy = portal_occupancy_point(&resized_capture)
        .map_err(PlatformPulsePortalJourneyFailure::Pixels)?;
    let [primary, cancel] = portal_action_points(&resized_capture)
        .map_err(PlatformPulsePortalJourneyFailure::Pixels)?;
    activate(world, occupancy)?;
    require_intent_quiet_after_occupancy_click(world)?;
    await_open_pixels(world, &baseline)?;
    activate(world, primary)?;
    await_started_intent(world)?;
    await_open_pixels(world, &baseline)?;
    activate(world, cancel)?;
    let intent_close_focus = await_completed_portal_intent(world)?;
    require_restoration(first_open_focus, intent_close_focus)?;
    await_closed_pixels(world, &baseline)?;

    activate(world, target.point())?;
    let second_open_focus = await_completed_portal_intent(world)?;
    require_open_focus(second_open_focus)?;
    let reopened = await_open_pixels(world, &baseline)?;
    escape(world)?;
    let dismissed = await_portal_dismissed(world)?;
    let escape_close_focus = await_semantic_focus(world)?;
    require_restoration(second_open_focus, escape_close_focus)?;
    if dismissed.frame().diagnostic_value() != escape_close_focus.frame() {
        return Err(PlatformPulsePortalJourneyFailure::FocusEvidence(
            "Escape dismissal and Focus restoration did not share one publication frame",
        ));
    }
    await_closed_pixels(world, &baseline)?;
    let expected_shutdown_sequence = drain_until_idle(world)?;

    Ok(PlatformPulsePortalJourneyEvidence {
        intent_close_pixels: opened,
        resized_open_pixels,
        escape_close_pixels: reopened,
        first_open_focus,
        intent_close_focus,
        second_open_focus,
        escape_close_focus,
        escape_dismissed_frame: dismissed.frame().diagnostic_value(),
        expected_shutdown_sequence,
    })
}

fn capture(
    world: &mut NativeBoundExecutableWorld,
) -> Result<crate::external_observation::NativeClientPixelCapture, PlatformPulsePortalJourneyFailure>
{
    world
        .platform
        .capture_client_area(&world.native_client)
        .map_err(PlatformPulsePortalJourneyFailure::Native)
}

fn await_completed_portal_intent(
    world: &mut NativeBoundExecutableWorld,
) -> Result<PlatformPulseSemanticFocusPublished, PlatformPulsePortalJourneyFailure> {
    let admitted = await_admitted(world)?;
    if await_executor_or_completion(world, admitted)? {
        await_completed(world, admitted)?;
    }
    await_semantic_focus(world)
}

fn await_started_intent(
    world: &mut NativeBoundExecutableWorld,
) -> Result<(), PlatformPulsePortalJourneyFailure> {
    let admitted = await_admitted(world)?;
    if !await_executor_or_completion(world, admitted)? {
        return Err(PlatformPulsePortalJourneyFailure::FocusEvidence(
            "portal primary action completed without exercising its real executor",
        ));
    }
    Ok(())
}

fn await_admitted(
    world: &mut NativeBoundExecutableWorld,
) -> Result<PlatformPulseIntentAttemptObservationReference, PlatformPulsePortalJourneyFailure> {
    loop {
        let envelope = next(world, WatchedPulseTransition::IntentPosturePublished)?;
        match envelope.outcome() {
            PlatformPulseLifecycleObservation::IntentPosturePublished(posture) => {
                if let PlatformPulseIntentPostureObservation::Admitted { reference } =
                    posture.posture()
                {
                    return Ok(*reference);
                }
                return Err(unexpected(envelope.outcome()));
            }
            outcome if incidental_visual(outcome) => {}
            outcome => return Err(unexpected(outcome)),
        }
    }
}

fn await_executor_or_completion(
    world: &mut NativeBoundExecutableWorld,
    admitted: PlatformPulseIntentAttemptObservationReference,
) -> Result<bool, PlatformPulsePortalJourneyFailure> {
    loop {
        let envelope = next(world, WatchedPulseTransition::IntentExecutorStarted)?;
        match envelope.outcome() {
            PlatformPulseLifecycleObservation::IntentExecutorStarted(started)
                if started.reference() == admitted =>
            {
                return Ok(true)
            }
            PlatformPulseLifecycleObservation::IntentPosturePublished(posture)
                if matches!(
                    posture.posture(),
                    PlatformPulseIntentPostureObservation::Completed { reference }
                        if *reference == admitted
                ) =>
            {
                return Ok(false)
            }
            outcome if incidental_visual(outcome) => {}
            outcome => return Err(unexpected(outcome)),
        }
    }
}

fn await_completed(
    world: &mut NativeBoundExecutableWorld,
    admitted: PlatformPulseIntentAttemptObservationReference,
) -> Result<(), PlatformPulsePortalJourneyFailure> {
    loop {
        let envelope = next(world, WatchedPulseTransition::IntentPosturePublished)?;
        match envelope.outcome() {
            PlatformPulseLifecycleObservation::IntentPosturePublished(posture)
                if matches!(
                    posture.posture(),
                    PlatformPulseIntentPostureObservation::Completed { reference }
                        if *reference == admitted
                ) =>
            {
                return Ok(())
            }
            outcome if incidental_visual(outcome) => {}
            outcome => return Err(unexpected(outcome)),
        }
    }
}

fn next(
    world: &mut NativeBoundExecutableWorld,
    expected: WatchedPulseTransition,
) -> Result<
    worth_ui_platform_pulse::observation_contract::PlatformPulseLifecycleObservationEnvelope,
    PlatformPulsePortalJourneyFailure,
> {
    await_watched_observation(
        &mut world.process,
        &mut world.lifecycle,
        expected,
        Instant::now() + TRANSITION_DEADLINE,
    )
    .map_err(PlatformPulsePortalJourneyFailure::Observation)
}

fn incidental_visual(outcome: &PlatformPulseLifecycleObservation) -> bool {
    matches!(
        outcome,
        PlatformPulseLifecycleObservation::VisualSnapshotCaptured(_)
            | PlatformPulseLifecycleObservation::VisualSnapshotRetired(_)
    )
}

fn unexpected(outcome: &PlatformPulseLifecycleObservation) -> PlatformPulsePortalJourneyFailure {
    PlatformPulsePortalJourneyFailure::UnexpectedObservation(format!("{outcome:?}"))
}

fn await_open_pixels(
    world: &mut NativeBoundExecutableWorld,
    baseline: &crate::external_observation::NativeClientPixelCapture,
) -> Result<PlatformPulsePortalPixelEvidence, PlatformPulsePortalJourneyFailure> {
    let deadline = Instant::now() + TRANSITION_DEADLINE;
    loop {
        let current = match capture(world) {
            Ok(current) => current,
            Err(PlatformPulsePortalJourneyFailure::Native(
                NativePlatformFailure::ClientCapture(_),
            )) if Instant::now() < deadline => {
                std::thread::sleep(PIXEL_POLL_SLICE);
                continue;
            }
            Err(failure) => return Err(failure),
        };
        if let Ok(evidence) = adjudicate_open_portal_pixels(baseline, &current) {
            return Ok(evidence);
        }
        if Instant::now() >= deadline {
            return adjudicate_open_portal_pixels(baseline, &current)
                .map_err(PlatformPulsePortalJourneyFailure::Pixels);
        }
        std::thread::sleep(PIXEL_POLL_SLICE);
    }
}

fn await_closed_pixels(
    world: &mut NativeBoundExecutableWorld,
    baseline: &crate::external_observation::NativeClientPixelCapture,
) -> Result<(), PlatformPulsePortalJourneyFailure> {
    let deadline = Instant::now() + TRANSITION_DEADLINE;
    loop {
        let current = match capture(world) {
            Ok(current) => current,
            Err(PlatformPulsePortalJourneyFailure::Native(
                NativePlatformFailure::ClientCapture(_),
            )) if Instant::now() < deadline => {
                std::thread::sleep(PIXEL_POLL_SLICE);
                continue;
            }
            Err(failure) => return Err(failure),
        };
        if adjudicate_closed_portal_pixels(baseline, &current).is_ok() {
            return Ok(());
        }
        if Instant::now() >= deadline {
            return adjudicate_closed_portal_pixels(baseline, &current)
                .map_err(PlatformPulsePortalJourneyFailure::Pixels);
        }
        std::thread::sleep(PIXEL_POLL_SLICE);
    }
}

fn drain_until_idle(
    world: &mut NativeBoundExecutableWorld,
) -> Result<u64, PlatformPulsePortalJourneyFailure> {
    loop {
        match world.lifecycle.next(Instant::now() + LIFECYCLE_IDLE_SLICE) {
            Ok(envelope) if incidental_visual(envelope.outcome()) => {}
            Ok(envelope) => return Err(unexpected(envelope.outcome())),
            Err(PlatformPulseLifecycleStreamFailure::Deadline) => {
                return Ok(world.lifecycle.measurement().accepted_events() as u64 + 1)
            }
            Err(failure) => {
                return Err(PlatformPulsePortalJourneyFailure::Observation(
                    WatchedPulseObservationFailure::Lifecycle(failure),
                ))
            }
        }
    }
}

impl CompletedPlatformPulsePortalJourney {
    pub(crate) fn evidence(&self) -> &PlatformPulsePortalJourneyEvidence {
        &self.evidence
    }

    pub(crate) fn into_recovered(self) -> PulseExecutableWorld<Published<FinalRecovered>> {
        self.recovered
    }
}

impl PlatformPulsePortalJourneyEvidence {
    pub(crate) const fn intent_close_pixels(&self) -> PlatformPulsePortalPixelEvidence {
        self.intent_close_pixels
    }

    pub(crate) const fn escape_close_pixels(&self) -> PlatformPulsePortalPixelEvidence {
        self.escape_close_pixels
    }

    pub(crate) const fn resized_open_pixels(&self) -> PlatformPulseAuthoredPortalPixelEvidence {
        self.resized_open_pixels
    }

    pub(crate) const fn expected_shutdown_sequence(&self) -> u64 {
        self.expected_shutdown_sequence
    }

    pub(crate) const fn focus_publications(&self) -> [PlatformPulseSemanticFocusPublished; 4] {
        [
            self.first_open_focus,
            self.intent_close_focus,
            self.second_open_focus,
            self.escape_close_focus,
        ]
    }

    pub(crate) const fn escape_dismissed_frame(&self) -> u64 {
        self.escape_dismissed_frame
    }
}

impl std::fmt::Display for PlatformPulsePortalJourneyFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
