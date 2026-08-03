#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::bounded_residency) enum WorkerStartOrder {
    FirstThenSecond,
    SecondThenFirst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::bounded_residency) enum EquivalentContenderIdentity {
    FirstOwner,
    SecondOwner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::bounded_residency) enum GateReleaseOrder {
    OwnerThenWaiter,
    WaiterThenOwner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::bounded_residency) enum IndependentReadyWorkSelection {
    FirstWorkerThenSecond,
    SecondWorkerThenFirst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BoundedResidencySchedulePlan {
    worker_start_order: WorkerStartOrder,
    contender_identity: EquivalentContenderIdentity,
    gate_release_order: GateReleaseOrder,
    ready_work_selection: IndependentReadyWorkSelection,
}

impl BoundedResidencySchedulePlan {
    pub(crate) fn parse(encoded: &str) -> Result<Self, String> {
        let decisions = encoded.split(';').collect::<Vec<_>>();
        let [worker, contender, release, ready] = decisions.as_slice() else {
            return Err("bounded-residency schedule must contain exactly four decisions".into());
        };
        Ok(Self {
            worker_start_order: parse_worker(worker)?,
            contender_identity: parse_contender(contender)?,
            gate_release_order: parse_release(release)?,
            ready_work_selection: parse_ready(ready)?,
        })
    }

    pub(in crate::bounded_residency) const fn worker_start_order(self) -> WorkerStartOrder {
        self.worker_start_order
    }

    pub(in crate::bounded_residency) const fn contender_identity(
        self,
    ) -> EquivalentContenderIdentity {
        self.contender_identity
    }

    pub(in crate::bounded_residency) const fn gate_release_order(self) -> GateReleaseOrder {
        self.gate_release_order
    }

    pub(in crate::bounded_residency) const fn ready_work_selection(
        self,
    ) -> IndependentReadyWorkSelection {
        self.ready_work_selection
    }
}

pub(super) const fn worker_label(value: WorkerStartOrder) -> &'static str {
    match value {
        WorkerStartOrder::FirstThenSecond => "worker-start-order=first-then-second",
        WorkerStartOrder::SecondThenFirst => "worker-start-order=second-then-first",
    }
}

pub(super) const fn contender_label(value: EquivalentContenderIdentity) -> &'static str {
    match value {
        EquivalentContenderIdentity::FirstOwner => "equivalent-contender-identity=first-owner",
        EquivalentContenderIdentity::SecondOwner => "equivalent-contender-identity=second-owner",
    }
}

pub(super) const fn release_label(value: GateReleaseOrder) -> &'static str {
    match value {
        GateReleaseOrder::OwnerThenWaiter => "gate-release-order=owner-then-waiter",
        GateReleaseOrder::WaiterThenOwner => "gate-release-order=waiter-then-owner",
    }
}

pub(super) const fn ready_label(value: IndependentReadyWorkSelection) -> &'static str {
    match value {
        IndependentReadyWorkSelection::FirstWorkerThenSecond => {
            "independent-ready-work-selection=first-worker-then-second"
        }
        IndependentReadyWorkSelection::SecondWorkerThenFirst => {
            "independent-ready-work-selection=second-worker-then-first"
        }
    }
}

fn parse_worker(encoded: &str) -> Result<WorkerStartOrder, String> {
    match encoded {
        "worker-start-order=first-then-second" => Ok(WorkerStartOrder::FirstThenSecond),
        "worker-start-order=second-then-first" => Ok(WorkerStartOrder::SecondThenFirst),
        _ => Err(format!(
            "unknown worker-start schedule decision `{encoded}`"
        )),
    }
}

fn parse_contender(encoded: &str) -> Result<EquivalentContenderIdentity, String> {
    match encoded {
        "equivalent-contender-identity=first-owner" => Ok(EquivalentContenderIdentity::FirstOwner),
        "equivalent-contender-identity=second-owner" => {
            Ok(EquivalentContenderIdentity::SecondOwner)
        }
        _ => Err(format!("unknown contender schedule decision `{encoded}`")),
    }
}

fn parse_release(encoded: &str) -> Result<GateReleaseOrder, String> {
    match encoded {
        "gate-release-order=owner-then-waiter" => Ok(GateReleaseOrder::OwnerThenWaiter),
        "gate-release-order=waiter-then-owner" => Ok(GateReleaseOrder::WaiterThenOwner),
        _ => Err(format!("unknown release schedule decision `{encoded}`")),
    }
}

fn parse_ready(encoded: &str) -> Result<IndependentReadyWorkSelection, String> {
    match encoded {
        "independent-ready-work-selection=first-worker-then-second" => {
            Ok(IndependentReadyWorkSelection::FirstWorkerThenSecond)
        }
        "independent-ready-work-selection=second-worker-then-first" => {
            Ok(IndependentReadyWorkSelection::SecondWorkerThenFirst)
        }
        _ => Err(format!("unknown ready-work schedule decision `{encoded}`")),
    }
}

#[cfg(test)]
mod tests {
    use super::BoundedResidencySchedulePlan;

    #[test]
    fn schedule_plan_accepts_only_the_closed_vocabulary() {
        let encoded = "worker-start-order=second-then-first;\
                       equivalent-contender-identity=second-owner;\
                       gate-release-order=waiter-then-owner;\
                       independent-ready-work-selection=second-worker-then-first";
        let plan = BoundedResidencySchedulePlan::parse(encoded).unwrap();
        assert_eq!(
            plan.ready_work_selection(),
            super::IndependentReadyWorkSelection::SecondWorkerThenFirst
        );
        assert!(BoundedResidencySchedulePlan::parse(&format!("{encoded};foreign=choice")).is_err());
    }
}
