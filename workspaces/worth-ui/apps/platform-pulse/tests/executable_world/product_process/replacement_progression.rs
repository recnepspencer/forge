use std::time::Instant;

use worth_ui_platform_pulse::observation_contract::PlatformPulseLifecycleObservationEnvelope;

use crate::adjudication::{
    adjudicate_replacement, CausalReplacementObservationSet, ExecutableReplacementEvidence,
    ExecutableReplacementFailure, ExpectedNativeColor, ReplacementExpectation,
};
use crate::failure_teardown::{
    teardown_native_bound_world, PulseExecutableWorldFailure, PulseExecutableWorldFailureReport,
};
use crate::source_delta::{
    AppliedPulseSourceDelta, CanonicalBlueRecoverySourceDelta, GreenPulseSourceDelta,
};

use super::watched_native_observation::{observe_watched_native, WatchedNativeObservation};
use super::{
    await_watched_observation, AwaitingRecovery, AwaitingReplacement, GreenSuccessor, InitialBlue,
    NativeBoundExecutableWorld, PreservedPredecessorEvidence, Published, PulseExecutableWorld,
    RecoveredBlue, WatchedPulseTransition,
};

struct WatchedReplacementObservation {
    envelope: PlatformPulseLifecycleObservationEnvelope,
    native: WatchedNativeObservation,
}

impl PulseExecutableWorld<AwaitingReplacement> {
    pub(crate) fn await_green_successor(
        self,
        deadline: Instant,
    ) -> Result<PulseExecutableWorld<Published<GreenSuccessor>>, PulseExecutableWorldFailureReport>
    {
        let AwaitingReplacement {
            mut world,
            initial,
            action,
        } = self.state;
        let observed = match observe_replacement(
            &mut world,
            WatchedPulseTransition::GreenReplacement,
            deadline,
        ) {
            Ok(observed) => observed,
            Err(primary) => return Err(teardown(world, primary)),
        };
        let evidence = match green_evidence(action, &initial, observed) {
            Ok(evidence) => evidence,
            Err(failure) => {
                return Err(teardown(
                    world,
                    PulseExecutableWorldFailure::Replacement(failure),
                ))
            }
        };
        Ok(PulseExecutableWorld {
            state: Published {
                world,
                stage: GreenSuccessor { initial, evidence },
            },
        })
    }
}

impl PulseExecutableWorld<AwaitingRecovery> {
    pub(crate) fn await_recovered_blue(
        self,
        deadline: Instant,
    ) -> Result<PulseExecutableWorld<Published<RecoveredBlue>>, PulseExecutableWorldFailureReport>
    {
        let AwaitingRecovery {
            mut world,
            preserved,
            action,
        } = self.state;
        let observed = match observe_replacement(
            &mut world,
            WatchedPulseTransition::CanonicalBlueRecovery,
            deadline,
        ) {
            Ok(observed) => observed,
            Err(primary) => return Err(teardown(world, primary)),
        };
        let evidence = match recovery_evidence(action, &preserved, observed) {
            Ok(evidence) => evidence,
            Err(failure) => {
                return Err(teardown(
                    world,
                    PulseExecutableWorldFailure::Replacement(failure),
                ))
            }
        };
        Ok(PulseExecutableWorld {
            state: Published {
                world,
                stage: RecoveredBlue {
                    preserved,
                    evidence,
                },
            },
        })
    }
}

fn observe_replacement(
    world: &mut NativeBoundExecutableWorld,
    transition: WatchedPulseTransition,
    deadline: Instant,
) -> Result<WatchedReplacementObservation, PulseExecutableWorldFailure> {
    let envelope = await_watched_observation(
        &mut world.process,
        &mut world.lifecycle,
        transition,
        deadline,
    )
    .map_err(PulseExecutableWorldFailure::WatchedObservation)?;
    let native = observe_watched_native(world)?;
    Ok(WatchedReplacementObservation { envelope, native })
}

fn green_evidence(
    action: AppliedPulseSourceDelta<GreenPulseSourceDelta>,
    initial: &InitialBlue,
    observed: WatchedReplacementObservation,
) -> Result<ExecutableReplacementEvidence<GreenPulseSourceDelta>, ExecutableReplacementFailure> {
    let causal = CausalReplacementObservationSet::new(
        action,
        initial.evidence.published_identity(),
        observed.envelope,
        ReplacementExpectation::green_successor(),
    );
    adjudicate_replacement(causal.join_native(
        observed.native.client,
        observed.native.liveness,
        observed.native.pixels,
        ExpectedNativeColor::Green,
    ))
}

fn recovery_evidence(
    action: AppliedPulseSourceDelta<CanonicalBlueRecoverySourceDelta>,
    preserved: &PreservedPredecessorEvidence,
    observed: WatchedReplacementObservation,
) -> Result<
    ExecutableReplacementEvidence<CanonicalBlueRecoverySourceDelta>,
    ExecutableReplacementFailure,
> {
    let canonical_digest = preserved
        .green
        .initial
        .evidence
        .first_frame()
        .source()
        .final_package_digest();
    let causal = CausalReplacementObservationSet::new(
        action,
        preserved.evidence.identity().clone(),
        observed.envelope,
        ReplacementExpectation::canonical_recovery(canonical_digest),
    );
    adjudicate_replacement(causal.join_native(
        observed.native.client,
        observed.native.liveness,
        observed.native.pixels,
        ExpectedNativeColor::Blue,
    ))
}

fn teardown(
    world: NativeBoundExecutableWorld,
    primary: PulseExecutableWorldFailure,
) -> PulseExecutableWorldFailureReport {
    teardown_native_bound_world(primary, world.into_failure_resources())
}
