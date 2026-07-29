use std::time::Instant;

use worth_ui_platform_pulse::observation_contract::PlatformPulseLifecycleObservationEnvelope;

use crate::adjudication::{
    adjudicate_replacement, adjudicate_successor_visual_snapshot, adjudicate_visual_comparison,
    adjudicate_visual_retirement, CausalReplacementObservationSet, ExecutableReplacementEvidence,
    ExecutableReplacementFailure, ExecutableVisualComparisonEvidence,
    ExecutableVisualIdentityFailure, ExecutableVisualRetirementEvidence, ExpectedNativeColor,
    ReplacementExpectation,
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
    NativeBoundExecutableWorld, OverlayCleared, PreservedPredecessorEvidence, Published,
    PulseExecutableWorld, RecoveredBlue, WatchedPulseTransition,
};

struct WatchedReplacementObservation {
    envelope: PlatformPulseLifecycleObservationEnvelope,
    successor_snapshot: Option<PlatformPulseLifecycleObservationEnvelope>,
    comparison: Option<PlatformPulseLifecycleObservationEnvelope>,
    retirement: Option<PlatformPulseLifecycleObservationEnvelope>,
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
        let (evidence, comparison, retirement) = match green_evidence(action, &initial, observed) {
            Ok(evidence) => evidence,
            Err(primary) => return Err(teardown(world, primary)),
        };
        Ok(PulseExecutableWorld {
            state: Published {
                world,
                stage: GreenSuccessor {
                    initial,
                    evidence,
                    comparison,
                    retirement,
                },
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
    let (successor_snapshot, comparison, retirement) = match transition {
        WatchedPulseTransition::GreenReplacement => (
            Some(
                await_watched_observation(
                    &mut world.process,
                    &mut world.lifecycle,
                    WatchedPulseTransition::VisualSuccessorSnapshot,
                    deadline,
                )
                .map_err(PulseExecutableWorldFailure::WatchedObservation)?,
            ),
            Some(
                await_watched_observation(
                    &mut world.process,
                    &mut world.lifecycle,
                    WatchedPulseTransition::VisualComparison,
                    deadline,
                )
                .map_err(PulseExecutableWorldFailure::WatchedObservation)?,
            ),
            Some(
                await_watched_observation(
                    &mut world.process,
                    &mut world.lifecycle,
                    WatchedPulseTransition::VisualSnapshotRetired,
                    deadline,
                )
                .map_err(PulseExecutableWorldFailure::WatchedObservation)?,
            ),
        ),
        _ => (None, None, None),
    };
    let native = observe_watched_native(world)?;
    Ok(WatchedReplacementObservation {
        envelope,
        successor_snapshot,
        comparison,
        retirement,
        native,
    })
}

fn green_evidence(
    action: AppliedPulseSourceDelta<GreenPulseSourceDelta>,
    initial: &OverlayCleared<InitialBlue>,
    observed: WatchedReplacementObservation,
) -> Result<
    (
        ExecutableReplacementEvidence<GreenPulseSourceDelta>,
        ExecutableVisualComparisonEvidence,
        ExecutableVisualRetirementEvidence,
    ),
    PulseExecutableWorldFailure,
> {
    let WatchedReplacementObservation {
        envelope,
        successor_snapshot,
        comparison,
        retirement,
        native,
    } = observed;
    let causal = CausalReplacementObservationSet::new(
        action,
        initial.initial().evidence.published_identity(),
        envelope,
        ReplacementExpectation::green_successor(),
    );
    let evidence = adjudicate_replacement(causal.join_native(
        native.client,
        native.liveness,
        native.pixels,
        ExpectedNativeColor::Green,
    ))
    .map_err(PulseExecutableWorldFailure::Replacement)?;
    let successor_snapshot =
        successor_snapshot.ok_or(PulseExecutableWorldFailure::VisualIdentity(
            ExecutableVisualIdentityFailure::WrongEvent("successor visual snapshot"),
        ))?;
    let successor_snapshot = adjudicate_successor_visual_snapshot(
        successor_snapshot,
        evidence.replacement().successor_frame().diagnostic_value(),
    )
    .map_err(PulseExecutableWorldFailure::VisualIdentity)?;
    let comparison = comparison.ok_or(PulseExecutableWorldFailure::VisualIdentity(
        ExecutableVisualIdentityFailure::WrongEvent("visual comparison"),
    ))?;
    let comparison =
        adjudicate_visual_comparison(comparison, initial.snapshot_evidence(), &successor_snapshot)
            .map_err(PulseExecutableWorldFailure::VisualIdentity)?;
    let retirement = retirement.ok_or(PulseExecutableWorldFailure::VisualIdentity(
        ExecutableVisualIdentityFailure::WrongEvent("visual snapshot retired"),
    ))?;
    let retirement = adjudicate_visual_retirement(
        retirement,
        initial.snapshot_evidence(),
        evidence.replacement().successor_frame().diagnostic_value(),
    )
    .map_err(PulseExecutableWorldFailure::VisualIdentity)?;
    Ok((evidence, comparison, retirement))
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
        .initial()
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
