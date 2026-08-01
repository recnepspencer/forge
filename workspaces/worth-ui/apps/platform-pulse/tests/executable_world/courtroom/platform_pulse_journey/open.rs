use super::*;

pub(in crate::courtroom) struct OpenPlatformPulseJourney {
    recovered: PulseExecutableWorld<Published<FinalRecovered>>,
    journey_started: Instant,
    first_publication: Duration,
    native_captures: u32,
    window_lookups: u32,
}

pub(in crate::courtroom) fn complete_open(
    deltas: PlatformPulseJourneyDeltas,
) -> OpenPlatformPulseJourney {
    let journey_started = Instant::now();
    let initial = launch_initial(deltas.canonical);
    let first_publication = initial.launch_to_first_publication();
    let mut native_captures = initial.evidence().capture_count();
    let window_lookups = initial.evidence().client_area().window_lookup_count();
    let input_reached = reach_native_input(initial, Instant::now() + TRANSITION_DEADLINE);
    native_captures += input_reached.evidence().capture_count();
    let first_current = publish_first_current(input_reached);
    native_captures += first_current.query_evidence().pixels().capture_count();
    let (visualized, visual_captures) = publish_visual_identity(first_current);
    native_captures += visual_captures;
    let second_current = publish_second_current(visualized);
    native_captures += second_current.query_evidence().pixels().capture_count();
    let green = publish_green(second_current, deltas.green);
    native_captures += green.evidence().capture_count();
    let preserved = preserve_green(green, deltas.malformed);
    native_captures += preserved.evidence().capture_count();
    let recovered = recover_blue(preserved, deltas.recovery);
    native_captures += recovered.evidence().capture_count();
    let stopped = stop_on_revision_schema(recovered, deltas.revision_schema);
    native_captures += stopped.evidence().replacement().capture_count();
    let recovered = recover_status_schema(stopped, deltas.status_schema_recovery);
    native_captures += recovered.evidence().replacement().capture_count();
    OpenPlatformPulseJourney {
        recovered,
        journey_started,
        first_publication,
        native_captures,
        window_lookups,
    }
}

impl OpenPlatformPulseJourney {
    pub(in crate::courtroom) fn into_recovered(
        self,
    ) -> PulseExecutableWorld<Published<FinalRecovered>> {
        self.recovered
    }

    pub(super) fn close(self) -> CompletedPlatformPulseJourney {
        let source_actions = self.recovered.source_action_count();
        let closed = close_recovered(self.recovered);
        let cost = PlatformPulseJourneyCost::from_completed(
            JourneyCostInputs {
                first_publication: self.first_publication,
                full_journey: self.journey_started.elapsed(),
                source_actions,
                native_captures: self.native_captures,
                window_lookups: self.window_lookups,
            },
            closed.evidence(),
        );
        CompletedPlatformPulseJourney { closed, cost }
    }
}
