use super::SchedulePerturbationSeed;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum ScheduleDecision {
    WorkerStartOrder(WorkerStartOrder),
    EquivalentContenderIdentity(EquivalentContenderIdentity),
    GateReleaseOrder(GateReleaseOrder),
    IndependentReadyWorkSelection(IndependentReadyWorkSelection),
    DurabilityCheckpointOrder(DurabilityCheckpointOrder),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum WorkerStartOrder {
    FirstThenSecond,
    SecondThenFirst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum EquivalentContenderIdentity {
    FirstOwner,
    SecondOwner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum GateReleaseOrder {
    OwnerThenWaiter,
    WaiterThenOwner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum IndependentReadyWorkSelection {
    FirstWorkerThenSecond,
    SecondWorkerThenFirst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum DurabilityCheckpointOrder {
    CheckpointBeforeTarget,
    TargetSealedBeforeCheckpoint,
}

impl DurabilityCheckpointOrder {
    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn encoded(
        self,
    ) -> &'static str {
        match self {
            Self::CheckpointBeforeTarget => "durability-checkpoint-order=checkpoint-before-target",
            Self::TargetSealedBeforeCheckpoint => {
                "durability-checkpoint-order=target-sealed-before-checkpoint"
            }
        }
    }
}

impl ScheduleDecision {
    pub(in crate::courtroom_campaign::bounded_residency_siege) const VOCABULARY: [&'static str; 5] = [
        "worker-start-order",
        "equivalent-contender-identity",
        "gate-release-order",
        "independent-ready-work-selection",
        "durability-checkpoint-order",
    ];

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn family(
        self,
    ) -> &'static str {
        match self {
            Self::WorkerStartOrder(_) => Self::VOCABULARY[0],
            Self::EquivalentContenderIdentity(_) => Self::VOCABULARY[1],
            Self::GateReleaseOrder(_) => Self::VOCABULARY[2],
            Self::IndependentReadyWorkSelection(_) => Self::VOCABULARY[3],
            Self::DurabilityCheckpointOrder(_) => Self::VOCABULARY[4],
        }
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn choice(
        self,
    ) -> &'static str {
        match self {
            Self::WorkerStartOrder(WorkerStartOrder::FirstThenSecond) => "first-then-second",
            Self::WorkerStartOrder(WorkerStartOrder::SecondThenFirst) => "second-then-first",
            Self::EquivalentContenderIdentity(EquivalentContenderIdentity::FirstOwner) => {
                "first-owner"
            }
            Self::EquivalentContenderIdentity(EquivalentContenderIdentity::SecondOwner) => {
                "second-owner"
            }
            Self::GateReleaseOrder(GateReleaseOrder::OwnerThenWaiter) => "owner-then-waiter",
            Self::GateReleaseOrder(GateReleaseOrder::WaiterThenOwner) => "waiter-then-owner",
            Self::IndependentReadyWorkSelection(
                IndependentReadyWorkSelection::FirstWorkerThenSecond,
            ) => "first-worker-then-second",
            Self::IndependentReadyWorkSelection(
                IndependentReadyWorkSelection::SecondWorkerThenFirst,
            ) => "second-worker-then-first",
            Self::DurabilityCheckpointOrder(DurabilityCheckpointOrder::CheckpointBeforeTarget) => {
                "checkpoint-before-target"
            }
            Self::DurabilityCheckpointOrder(
                DurabilityCheckpointOrder::TargetSealedBeforeCheckpoint,
            ) => "target-sealed-before-checkpoint",
        }
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) fn encoded(&self) -> String {
        format!("{}={}", self.family(), self.choice())
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) fn canonical_trace_decision(
        self,
    ) -> worth_store_physical_certification::SchedulePerturbationDecision {
        worth_store_physical_certification::SchedulePerturbationDecision::new(
            self.family(),
            self.choice(),
        )
        .expect("the Courtroom C decision vocabulary is nonempty")
    }
}

pub(in crate::courtroom_campaign::bounded_residency_siege) fn derive(
    seed: SchedulePerturbationSeed,
) -> [ScheduleDecision; 5] {
    let decision_bits = seed.value();
    [
        ScheduleDecision::WorkerStartOrder(if decision_bits & 1 == 0 {
            WorkerStartOrder::FirstThenSecond
        } else {
            WorkerStartOrder::SecondThenFirst
        }),
        ScheduleDecision::EquivalentContenderIdentity(if decision_bits & 2 == 0 {
            EquivalentContenderIdentity::FirstOwner
        } else {
            EquivalentContenderIdentity::SecondOwner
        }),
        ScheduleDecision::GateReleaseOrder(if decision_bits & 4 == 0 {
            GateReleaseOrder::OwnerThenWaiter
        } else {
            GateReleaseOrder::WaiterThenOwner
        }),
        ScheduleDecision::IndependentReadyWorkSelection(if decision_bits & 8 == 0 {
            IndependentReadyWorkSelection::FirstWorkerThenSecond
        } else {
            IndependentReadyWorkSelection::SecondWorkerThenFirst
        }),
        ScheduleDecision::DurabilityCheckpointOrder(if decision_bits & 16 == 0 {
            DurabilityCheckpointOrder::CheckpointBeforeTarget
        } else {
            DurabilityCheckpointOrder::TargetSealedBeforeCheckpoint
        }),
    ]
}

pub(in crate::courtroom_campaign::bounded_residency_siege) fn parse_trace(
    encoded: &str,
) -> Result<[ScheduleDecision; 4], String> {
    let decisions = encoded
        .split(';')
        .map(parse_decision)
        .collect::<Result<Vec<_>, _>>()?;
    decisions
        .try_into()
        .map_err(|_| "executed schedule trace must contain exactly four decisions".to_owned())
}

fn parse_decision(encoded: &str) -> Result<ScheduleDecision, String> {
    match encoded {
        "worker-start-order=first-then-second" => Ok(ScheduleDecision::WorkerStartOrder(
            WorkerStartOrder::FirstThenSecond,
        )),
        "worker-start-order=second-then-first" => Ok(ScheduleDecision::WorkerStartOrder(
            WorkerStartOrder::SecondThenFirst,
        )),
        "equivalent-contender-identity=first-owner" => Ok(
            ScheduleDecision::EquivalentContenderIdentity(EquivalentContenderIdentity::FirstOwner),
        ),
        "equivalent-contender-identity=second-owner" => Ok(
            ScheduleDecision::EquivalentContenderIdentity(EquivalentContenderIdentity::SecondOwner),
        ),
        "gate-release-order=owner-then-waiter" => Ok(ScheduleDecision::GateReleaseOrder(
            GateReleaseOrder::OwnerThenWaiter,
        )),
        "gate-release-order=waiter-then-owner" => Ok(ScheduleDecision::GateReleaseOrder(
            GateReleaseOrder::WaiterThenOwner,
        )),
        "independent-ready-work-selection=first-worker-then-second" => {
            Ok(ScheduleDecision::IndependentReadyWorkSelection(
                IndependentReadyWorkSelection::FirstWorkerThenSecond,
            ))
        }
        "independent-ready-work-selection=second-worker-then-first" => {
            Ok(ScheduleDecision::IndependentReadyWorkSelection(
                IndependentReadyWorkSelection::SecondWorkerThenFirst,
            ))
        }
        _ => Err(format!("unknown executed schedule decision `{encoded}`")),
    }
}
