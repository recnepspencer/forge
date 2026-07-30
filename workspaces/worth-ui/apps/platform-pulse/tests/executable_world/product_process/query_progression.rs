use std::time::Instant;

use crate::adjudication::{
    adjudicate_query_current, adjudicate_visual_retirement, adjudicate_visual_snapshot,
};
use crate::failure_teardown::{
    teardown_native_bound_world, PulseExecutableWorldFailure, PulseExecutableWorldFailureReport,
};
use crate::source_delta::{QueryStatusV1, QueryStatusV2};

use super::watched_native_observation::observe_watched_native;
use super::{
    await_watched_observation, AwaitingQueryCurrent, ComparisonBasisRefreshed, FirstCurrent,
    InitialBlue, OverlayCleared, Published, PulseExecutableWorld, QueryCurrent, SecondCurrent,
    SecondQueryCurrent, WatchedPulseTransition,
};

impl PulseExecutableWorld<Published<InitialBlue>> {
    pub(crate) fn publish_first_query_value(
        self,
        value: QueryStatusV1,
    ) -> Result<
        PulseExecutableWorld<AwaitingQueryCurrent<InitialBlue, QueryStatusV1>>,
        PulseExecutableWorldFailureReport,
    > {
        apply_query(self, value, QueryStatusV1::apply)
    }
}

impl PulseExecutableWorld<Published<OverlayCleared<FirstCurrent>>> {
    pub(crate) fn publish_second_query_value(
        self,
        value: QueryStatusV2,
    ) -> Result<
        PulseExecutableWorld<AwaitingQueryCurrent<OverlayCleared<FirstCurrent>, QueryStatusV2>>,
        PulseExecutableWorldFailureReport,
    > {
        apply_query(self, value, QueryStatusV2::apply)
    }
}

impl PulseExecutableWorld<AwaitingQueryCurrent<InitialBlue, QueryStatusV1>> {
    pub(crate) fn await_first_query_value(
        self,
        deadline: Instant,
    ) -> Result<PulseExecutableWorld<Published<FirstCurrent>>, PulseExecutableWorldFailureReport>
    {
        await_query(self, QueryStatusV1::VALUE, 2, deadline, |prior| {
            prior.evidence.pixels().rgba()
        })
    }
}

impl PulseExecutableWorld<AwaitingQueryCurrent<OverlayCleared<FirstCurrent>, QueryStatusV2>> {
    pub(crate) fn await_second_query_value(
        self,
        deadline: Instant,
    ) -> Result<PulseExecutableWorld<Published<SecondCurrent>>, PulseExecutableWorldFailureReport>
    {
        let current = await_query(self, QueryStatusV2::VALUE, 5, deadline, |prior| {
            prior.overlay.trace.snapshot.prior.evidence.pixels().rgba()
        })?;
        await_comparison_basis_refresh(current, deadline)
    }
}

fn await_comparison_basis_refresh(
    current: PulseExecutableWorld<Published<SecondQueryCurrent>>,
    deadline: Instant,
) -> Result<PulseExecutableWorld<Published<SecondCurrent>>, PulseExecutableWorldFailureReport> {
    let Published {
        mut world,
        stage: current,
    } = current.state;
    let result = (|| {
        let retirement = await_watched_observation(
            &mut world.process,
            &mut world.lifecycle,
            WatchedPulseTransition::VisualSnapshotRetired,
            deadline,
        )
        .map_err(PulseExecutableWorldFailure::WatchedObservation)?;
        let refreshed = await_watched_observation(
            &mut world.process,
            &mut world.lifecycle,
            WatchedPulseTransition::VisualSnapshot,
            deadline,
        )
        .map_err(PulseExecutableWorldFailure::WatchedObservation)?;
        let frame = current.evidence.publication().frame().diagnostic_value();
        let expected_retirement = current.evidence.published_sequence().saturating_add(1);
        let retirement = adjudicate_visual_retirement(
            retirement,
            current.prior.snapshot_evidence(),
            frame,
            expected_retirement,
        )
        .map_err(PulseExecutableWorldFailure::VisualIdentity)?;
        let snapshot =
            adjudicate_visual_snapshot(refreshed, frame, retirement.sequence().saturating_add(1))
                .map_err(PulseExecutableWorldFailure::VisualIdentity)?;
        Ok((retirement, snapshot))
    })();
    match result {
        Ok((retirement, snapshot)) => Ok(PulseExecutableWorld {
            state: Published {
                world,
                stage: ComparisonBasisRefreshed {
                    prior: current,
                    retirement,
                    snapshot,
                },
            },
        }),
        Err(primary) => Err(teardown_native_bound_world(
            primary,
            world.into_failure_resources(),
        )),
    }
}

fn apply_query<Stage, Kind>(
    world: PulseExecutableWorld<Published<Stage>>,
    value: Kind,
    apply: fn(
        Kind,
        &crate::installation::IsolatedPulseInstallation,
    ) -> Result<
        crate::source_delta::AppliedPulseSourceDelta<Kind>,
        crate::source_delta::PulseSourceActionFailure,
    >,
) -> Result<
    PulseExecutableWorld<AwaitingQueryCurrent<Stage, Kind>>,
    PulseExecutableWorldFailureReport,
> {
    let Published {
        world: native,
        stage,
    } = world.state;
    let action = match apply(value, &native.installation) {
        Ok(action) => action,
        Err(primary) => {
            return Err(teardown_native_bound_world(
                PulseExecutableWorldFailure::SourceAction(primary),
                native.into_failure_resources(),
            ))
        }
    };
    Ok(PulseExecutableWorld {
        state: AwaitingQueryCurrent {
            world: native,
            prior: stage,
            action,
        },
    })
}

fn await_query<Stage, Kind>(
    world: PulseExecutableWorld<AwaitingQueryCurrent<Stage, Kind>>,
    expected: &str,
    owner_order: u64,
    deadline: Instant,
    predecessor: impl FnOnce(&Stage) -> &[u8],
) -> Result<
    PulseExecutableWorld<Published<QueryCurrent<Stage, Kind>>>,
    PulseExecutableWorldFailureReport,
> {
    let AwaitingQueryCurrent {
        mut world,
        prior,
        action,
    } = world.state;
    let result = (|| {
        let issued = await_watched_observation(
            &mut world.process,
            &mut world.lifecycle,
            WatchedPulseTransition::QueryProjectionIssued,
            deadline,
        )
        .map_err(PulseExecutableWorldFailure::WatchedObservation)?;
        let published = await_watched_observation(
            &mut world.process,
            &mut world.lifecycle,
            WatchedPulseTransition::QueryProjectionPublished,
            deadline,
        )
        .map_err(PulseExecutableWorldFailure::WatchedObservation)?;
        let native = observe_watched_native(&mut world)?;
        adjudicate_query_current(
            issued,
            published,
            expected,
            owner_order,
            native.client,
            native.pixels,
            predecessor(&prior),
        )
        .map_err(PulseExecutableWorldFailure::QueryCurrent)
    })();
    match result {
        Ok(evidence) => Ok(PulseExecutableWorld {
            state: Published {
                world,
                stage: QueryCurrent {
                    prior,
                    action,
                    evidence,
                },
            },
        }),
        Err(primary) => Err(teardown_native_bound_world(
            primary,
            world.into_failure_resources(),
        )),
    }
}
