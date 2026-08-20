use super::declarations::{UiNativePhysicalSignalDeclarations, UiNativePhysicalSignalOperation};
use super::routing::UiNativePhysicalSignalWork;
use super::worker_graph::UiNativePhysicalSignalGraph;
use super::worker_graph::UiNativePhysicalSignalPerformed;
use worth_signal::facade::adapters::RuntimeTelemetry;
use worth_signal::facade::{
    RawCompletionEnvelope, ResourceAttemptId, ResourceCancellationReason, ResourceRequestHandle,
    TemporalWakeId,
};

#[derive(Clone, Copy)]
pub(super) struct SignalRequest {
    pub(super) work: UiNativePhysicalSignalWork,
    pub(super) handle: ResourceRequestHandle,
    pub(super) attempt: ResourceAttemptId,
    pub(super) operation: UiNativePhysicalSignalOperation,
    pub(super) operation_slot: usize,
    pub(super) lineage: super::identity::UiNativePhysicalSignalSlotLineage,
    pub(super) retry_wake: Option<TemporalWakeId>,
    pub(super) poll_wake: Option<TemporalWakeId>,
}

pub(crate) struct UiNativePhysicalSignalWorker {
    pub(super) graph: UiNativePhysicalSignalGraph,
    pub(super) requests: Vec<SignalRequest>,
}

pub(crate) enum UiNativePhysicalWorkerSettlement {
    Current,
    Superseded,
}

impl UiNativePhysicalSignalWorker {
    pub(crate) fn new(declarations: UiNativePhysicalSignalDeclarations) -> Self {
        Self {
            graph: UiNativePhysicalSignalGraph::build(declarations),
            requests: Vec::new(),
        }
    }

    pub(crate) fn admit(
        &mut self,
        operation: UiNativePhysicalSignalOperation,
        work: UiNativePhysicalSignalWork,
    ) -> Result<(ResourceRequestHandle, UiNativePhysicalSignalPerformed), ()> {
        let lineage = work.slot_lineage();
        let operation_slot = self
            .requests
            .iter()
            .find(|request| request.operation == operation && request.lineage == lineage)
            .map(|request| request.operation_slot)
            .or_else(|| {
                (0..super::declarations::PHYSICAL_SIGNAL_ROUTE_CAPACITY).find(|slot| {
                    self.requests.iter().all(|request| {
                        request.operation != operation || request.operation_slot != *slot
                    })
                })
            })
            .ok_or(())?;
        let performed = self.graph.perform_transition(operation, work)?;
        let capability = self
            .graph
            .topology
            .operations
            .get(operation.index())
            .and_then(|slots| slots.get(operation_slot))
            .ok_or(())?;
        let report = self
            .graph
            .runtime
            .admit_async_node_request(capability.request_intent())
            .map_err(|_| ())?;
        let Some(admitted) = report.resource_admission() else {
            return Err(());
        };
        let admitted_request = admitted.admitted_request();
        let handle = admitted_request.handle();
        self.graph.record_current(operation, work, handle)?;
        self.requests.push(SignalRequest {
            work,
            handle,
            attempt: admitted_request.attempt(),
            operation,
            operation_slot,
            lineage,
            retry_wake: None,
            poll_wake: None,
        });
        Ok((handle, performed))
    }

    pub(crate) fn contains(&self, work: UiNativePhysicalSignalWork) -> bool {
        self.graph.contains_work(work)
            && self.requests.iter().any(|request| {
                request.work == work && self.graph.contains_current(work, request.handle)
            })
    }

    pub(crate) fn replace_work(
        &mut self,
        handle: ResourceRequestHandle,
        predecessor: UiNativePhysicalSignalWork,
        successor: UiNativePhysicalSignalWork,
    ) -> Option<UiNativePhysicalSignalPerformed> {
        if self
            .requests
            .iter()
            .any(|request| request.work == successor)
        {
            return None;
        }
        let Some(index) = self
            .requests
            .iter()
            .position(|request| request.handle == handle && request.work == predecessor)
        else {
            return None;
        };
        let performed = self
            .graph
            .perform_transition(self.requests[index].operation, successor)
            .ok()?;
        if !self.graph.replace_current(handle, predecessor, successor) {
            return None;
        }
        self.requests[index].work = successor;
        Some(performed)
    }

    pub(crate) fn reconcile(
        &mut self,
        handle: ResourceRequestHandle,
        work: UiNativePhysicalSignalWork,
        status: super::routing::UiNativePhysicalSignalExternalStatus,
    ) -> Result<UiNativePhysicalWorkerSettlement, ()> {
        let Some(request) = self
            .requests
            .iter()
            .find(|request| request.work == work && request.handle == handle)
            .copied()
        else {
            return Err(());
        };
        if request.work != work
            || request.handle != handle
            || !self.graph.contains_current(work, handle)
        {
            return Err(());
        }
        self.graph
            .perform_transition(request.operation, request.work)?;
        let settlement = match status {
            super::routing::UiNativePhysicalSignalExternalStatus::Pending => {
                UiNativePhysicalWorkerSettlement::Current
            }
            super::routing::UiNativePhysicalSignalExternalStatus::Completed => {
                self.complete(request)?
            }
            super::routing::UiNativePhysicalSignalExternalStatus::RejectedBeforeEffects
            | super::routing::UiNativePhysicalSignalExternalStatus::RejectedAfterRasterization => {
                self.cancel(request)?;
                UiNativePhysicalWorkerSettlement::Current
            }
            super::routing::UiNativePhysicalSignalExternalStatus::EffectsIndeterminate => {
                UiNativePhysicalWorkerSettlement::Current
            }
        };
        if matches!(settlement, UiNativePhysicalWorkerSettlement::Current)
            && status != super::routing::UiNativePhysicalSignalExternalStatus::Pending
            && status != super::routing::UiNativePhysicalSignalExternalStatus::EffectsIndeterminate
        {
            let _ = self.graph.remove_current(work, handle);
            if let Some(index) = self
                .requests
                .iter()
                .position(|candidate| candidate.work == work && candidate.handle == handle)
            {
                self.requests.remove(index);
            }
        }
        Ok(settlement)
    }

    pub(crate) fn active_requests(&self) -> usize {
        self.requests.len()
    }

    pub(crate) fn active_operation_count(
        &self,
        operation: UiNativePhysicalSignalOperation,
    ) -> usize {
        self.requests
            .iter()
            .filter(|request| request.operation == operation)
            .count()
    }

    pub(crate) fn request_uses_operation(
        &self,
        handle: ResourceRequestHandle,
        work: UiNativePhysicalSignalWork,
        operation: UiNativePhysicalSignalOperation,
    ) -> bool {
        self.requests.iter().any(|request| {
            request.handle == handle
                && request.work == work
                && request.operation == operation
                && self.graph.contains_current(work, handle)
        })
    }

    pub(crate) fn telemetry(&self) -> RuntimeTelemetry {
        *self.graph.runtime.telemetry()
    }

    pub(crate) const fn performed_transitions(&self) -> u64 {
        self.graph.performed_transitions()
    }

    pub(crate) const fn performed_nodes(&self) -> u64 {
        self.graph.performed_nodes()
    }

    pub(crate) const fn last_performed(&self) -> Option<UiNativePhysicalSignalPerformed> {
        self.graph.last_performed()
    }

    fn complete(&mut self, request: SignalRequest) -> Result<UiNativePhysicalWorkerSettlement, ()> {
        let envelope = RawCompletionEnvelope::new(
            request.handle.request_id(),
            request.handle.generation(),
            request.handle.branch_epoch(),
            request.attempt,
            self.payload_contract(request),
            0,
        );
        let report = self.graph.runtime.admit_resource_completion(envelope);
        let admitted = match report.admitted_completion() {
            Some(admitted) => admitted,
            None => {
                let denied = report.denied_completion().ok_or(())?;
                if denied.class() != worth_signal::facade::core::CompletionDenialClass::Superseded {
                    return Err(());
                }
                self.retire_superseded(request)?;
                return Ok(UiNativePhysicalWorkerSettlement::Superseded);
            }
        };
        let staged = self
            .graph
            .runtime
            .stage_admitted_resource_completion(admitted)
            .map_err(|_| ())?;
        self.graph
            .runtime
            .commit_staged_resource_completion(staged.staged_effect())
            .map_err(|_| ())?;
        Ok(UiNativePhysicalWorkerSettlement::Current)
    }

    fn retire_superseded(&mut self, request: SignalRequest) -> Result<(), ()> {
        if let Some(wake) = request.poll_wake {
            self.graph
                .runtime
                .retire_temporal_wake(
                    wake,
                    worth_signal::facade::TemporalWakeRetirementReason::Superseded,
                )
                .map_err(|_| ())?;
        }
        if !self.graph.remove_current(request.work, request.handle) {
            return Err(());
        }
        let index = self
            .requests
            .iter()
            .position(|candidate| {
                candidate.work == request.work && candidate.handle == request.handle
            })
            .ok_or(())?;
        self.requests.remove(index);
        Ok(())
    }

    fn cancel(&mut self, request: SignalRequest) -> Result<bool, ()> {
        self.cancel_handle(request.handle)
    }

    pub(crate) fn cancel_handle(&mut self, handle: ResourceRequestHandle) -> Result<bool, ()> {
        let request = self
            .requests
            .iter()
            .find(|request| request.handle == handle)
            .copied()
            .ok_or(())?;
        self.graph
            .perform_transition(request.operation, request.work)?;
        let poll_wake = request.poll_wake;
        let report = self
            .graph
            .runtime
            .cancel_resource_request(handle, ResourceCancellationReason::HostRequested)
            .map_err(|_| ())?;
        let cancelled = report.cancelled_request().is_some();
        if cancelled {
            if !self.graph.remove_current(request.work, handle) {
                return Err(());
            }
            if let Some(wake) = poll_wake {
                self.graph
                    .runtime
                    .retire_temporal_wake(
                        wake,
                        worth_signal::facade::TemporalWakeRetirementReason::Cancelled,
                    )
                    .map_err(|_| ())?;
            }
            if let Some(index) = self
                .requests
                .iter()
                .position(|request| request.handle == handle)
            {
                self.requests.remove(index);
            }
        }
        Ok(cancelled)
    }

    pub(crate) fn schedule_pending_poll(
        &mut self,
        handle: ResourceRequestHandle,
    ) -> Result<bool, ()> {
        let index = self
            .requests
            .iter()
            .position(|request| request.handle == handle)
            .ok_or(())?;
        if self.requests[index].poll_wake.is_some() {
            return Err(());
        }
        let operation = self.requests[index].operation;
        let operation_slot = self.requests[index].operation_slot;
        let due = self.graph.context.clock_revision.checked_add(1).ok_or(())?;
        let scheduled = self
            .graph
            .runtime
            .schedule_owned_temporal_wake(
                worth_signal::facade::TemporalWakeOwner::ResourceNode(
                    self.graph.topology.operations[operation.index()][operation_slot].node(),
                ),
                worth_signal::facade::TemporalCondition::after(1).map_err(|_| ())?,
                worth_signal::facade::ClockTick::new(due),
            )
            .map_err(|_| ())?;
        self.requests[index].poll_wake = Some(scheduled.id());
        Ok(true)
    }

    pub(crate) fn retire_timed_out_handle(&mut self, handle: ResourceRequestHandle) -> bool {
        let Some(index) = self
            .requests
            .iter()
            .position(|request| request.handle == handle)
        else {
            return false;
        };
        if self.requests[index].poll_wake.is_some() || self.requests[index].retry_wake.is_some() {
            return false;
        }
        if !self.graph.remove_current(self.requests[index].work, handle) {
            return false;
        }
        self.requests.remove(index);
        true
    }

    fn payload_contract(
        &self,
        request: SignalRequest,
    ) -> worth_signal::facade::ResourcePayloadContractDigest {
        self.graph.topology.operations[request.operation.index()][request.operation_slot]
            .payload_contract_digest()
            .clone()
    }
}
