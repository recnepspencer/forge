use crate::failure_teardown::{
    teardown_native_bound_world, PulseExecutableWorldFailure, PulseExecutableWorldFailureReport,
};
use crate::source_delta::{
    CanonicalBlueRecoverySourceDelta, GreenPulseSourceDelta, MalformedPulseSourceDelta,
};

use super::{
    AwaitingPreservation, AwaitingRecovery, AwaitingReplacement, GreenSuccessor, InitialBlue,
    PreservedPredecessor, PreservedPredecessorEvidence, Published, PulseExecutableWorld,
};

impl PulseExecutableWorld<Published<InitialBlue>> {
    pub(crate) fn apply_green(
        self,
        delta: GreenPulseSourceDelta,
    ) -> Result<PulseExecutableWorld<AwaitingReplacement>, PulseExecutableWorldFailureReport> {
        let Published { world, stage } = self.state;
        let action = match delta.apply(&world.installation) {
            Ok(action) => action,
            Err(failure) => {
                return Err(teardown_native_bound_world(
                    PulseExecutableWorldFailure::SourceAction(failure),
                    world.into_failure_resources(),
                ))
            }
        };
        Ok(PulseExecutableWorld {
            state: AwaitingReplacement {
                world,
                initial: stage,
                action,
            },
        })
    }
}

impl PulseExecutableWorld<Published<GreenSuccessor>> {
    pub(crate) fn apply_malformed(
        self,
        delta: MalformedPulseSourceDelta,
    ) -> Result<PulseExecutableWorld<AwaitingPreservation>, PulseExecutableWorldFailureReport> {
        let Published { world, stage } = self.state;
        let action = match delta.apply(&world.installation) {
            Ok(action) => action,
            Err(failure) => {
                return Err(teardown_native_bound_world(
                    PulseExecutableWorldFailure::SourceAction(failure),
                    world.into_failure_resources(),
                ))
            }
        };
        Ok(PulseExecutableWorld {
            state: AwaitingPreservation {
                world,
                green: stage,
                action,
            },
        })
    }
}

impl PulseExecutableWorld<PreservedPredecessor> {
    pub(crate) fn restore_canonical(
        self,
        delta: CanonicalBlueRecoverySourceDelta,
    ) -> Result<PulseExecutableWorld<AwaitingRecovery>, PulseExecutableWorldFailureReport> {
        let PreservedPredecessor {
            world,
            green,
            evidence,
        } = self.state;
        let action = match delta.apply(&world.installation) {
            Ok(action) => action,
            Err(failure) => {
                return Err(teardown_native_bound_world(
                    PulseExecutableWorldFailure::SourceAction(failure),
                    world.into_failure_resources(),
                ))
            }
        };
        Ok(PulseExecutableWorld {
            state: AwaitingRecovery {
                world,
                preserved: PreservedPredecessorEvidence { green, evidence },
                action,
            },
        })
    }
}
