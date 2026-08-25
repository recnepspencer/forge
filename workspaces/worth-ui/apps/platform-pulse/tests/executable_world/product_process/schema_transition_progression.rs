use std::time::Instant;

use crate::adjudication::{
    adjudicate_replacement, adjudicate_schema_transition, require_replacement_lifecycle,
    CausalReplacementObservationSet, ExpectedNativeColor, ExpectedSchemaTransition,
    ReplacementExpectation,
};
use crate::failure_teardown::{
    teardown_native_bound_world, PulseExecutableWorldFailure, PulseExecutableWorldFailureReport,
};

use super::watched_native_observation::{
    observe_watched_native, observe_watched_native_until_pixels,
};
use super::{
    await_watched_observation, AwaitingSchemaStop, AwaitingStatusRecovery, FinalRecovered,
    NativeBoundExecutableWorld, Published, PulseExecutableWorld, SchemaStopped,
    WatchedPulseTransition,
};

impl PulseExecutableWorld<AwaitingSchemaStop> {
    pub(crate) fn await_schema_stopped(
        self,
        deadline: Instant,
    ) -> Result<PulseExecutableWorld<Published<SchemaStopped>>, PulseExecutableWorldFailureReport>
    {
        let AwaitingSchemaStop {
            mut world,
            recovered,
            action,
        } = self.state;
        let result = (|| {
            let envelope = await_watched_observation(
                &mut world.process,
                &mut world.lifecycle,
                WatchedPulseTransition::RevisionSchemaStopped,
                deadline,
            )
            .map_err(PulseExecutableWorldFailure::WatchedObservation)?;
            require_replacement_lifecycle(&envelope)
                .map_err(PulseExecutableWorldFailure::Replacement)?;
            let native = observe_watched_native(&mut world)?;
            let causal = CausalReplacementObservationSet::new(
                action,
                recovered.evidence.identity().clone(),
                envelope,
                ReplacementExpectation::revision_schema(),
            );
            let replacement = adjudicate_replacement(causal.join_native(
                native.client,
                native.liveness,
                native.pixels,
                ExpectedNativeColor::Blue,
            ))
            .map_err(PulseExecutableWorldFailure::Replacement)?;
            adjudicate_schema_transition(
                replacement,
                recovered.evidence.pixels(),
                None,
                recovered
                    .preserved
                    .green
                    .initial
                    .prior
                    .evidence
                    .projection(),
                ExpectedSchemaTransition::Stopped,
            )
            .map_err(PulseExecutableWorldFailure::SchemaTransition)
        })();
        let evidence = match result {
            Ok(evidence) => evidence,
            Err(primary) => return Err(teardown(world, primary)),
        };
        Ok(PulseExecutableWorld {
            state: Published {
                world,
                stage: SchemaStopped {
                    recovered,
                    evidence,
                },
            },
        })
    }
}

impl PulseExecutableWorld<AwaitingStatusRecovery> {
    pub(crate) fn await_status_recovered(
        self,
        deadline: Instant,
    ) -> Result<PulseExecutableWorld<Published<FinalRecovered>>, PulseExecutableWorldFailureReport>
    {
        let AwaitingStatusRecovery {
            mut world,
            stopped,
            action,
        } = self.state;
        let result = (|| {
            let envelope = await_watched_observation(
                &mut world.process,
                &mut world.lifecycle,
                WatchedPulseTransition::StatusSchemaRecovered,
                deadline,
            )
            .map_err(PulseExecutableWorldFailure::WatchedObservation)?;
            require_replacement_lifecycle(&envelope)
                .map_err(PulseExecutableWorldFailure::Replacement)?;
            let native = observe_watched_native_until_pixels(
                &mut world,
                stopped.recovered.evidence.pixels(),
                deadline,
                "canonical current recovery",
            )?;
            let causal = CausalReplacementObservationSet::new(
                action,
                stopped.evidence.replacement().identity().clone(),
                envelope,
                ReplacementExpectation::status_schema_recovery(
                    stopped
                        .recovered
                        .preserved
                        .green
                        .initial
                        .canonical_source_digest(),
                ),
            );
            let replacement = adjudicate_replacement(causal.join_native(
                native.client,
                native.liveness,
                native.pixels,
                ExpectedNativeColor::Blue,
            ))
            .map_err(PulseExecutableWorldFailure::Replacement)?;
            adjudicate_schema_transition(
                replacement,
                stopped.evidence.replacement().pixels(),
                Some(stopped.recovered.evidence.pixels()),
                stopped.evidence.query_basis(),
                ExpectedSchemaTransition::Recovered,
            )
            .map_err(PulseExecutableWorldFailure::SchemaTransition)
        })();
        let evidence = match result {
            Ok(evidence) => evidence,
            Err(primary) => return Err(teardown(world, primary)),
        };
        Ok(PulseExecutableWorld {
            state: Published {
                world,
                stage: FinalRecovered { stopped, evidence },
            },
        })
    }
}

fn teardown(
    world: NativeBoundExecutableWorld,
    primary: PulseExecutableWorldFailure,
) -> PulseExecutableWorldFailureReport {
    teardown_native_bound_world(primary, world.into_failure_resources())
}
