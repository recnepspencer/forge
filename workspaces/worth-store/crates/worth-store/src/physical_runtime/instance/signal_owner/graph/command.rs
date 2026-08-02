use super::super::route::PhysicalSignalRouteCommand;
use super::PhysicalSignalGraph;

impl PhysicalSignalGraph {
    pub(in crate::physical_runtime::instance::signal_owner) fn apply_command(
        &mut self,
        route_slot: usize,
        command: PhysicalSignalRouteCommand,
    ) -> bool {
        let Some(route) = self
            .bindings
            .binding_for_slot(route_slot)
            .map(|binding| binding.digest())
        else {
            return false;
        };
        match command {
            PhysicalSignalRouteCommand::Apply(delta, reply) => {
                let _ = reply.send(self.apply_delta(route_slot, route, &delta));
            }
            PhysicalSignalRouteCommand::Request(admitted, reply) => {
                let _ = reply.send(self.request(route, admitted));
            }
            PhysicalSignalRouteCommand::RevalidateReady(ready, reply) => {
                let _ = reply.send(self.revalidate_ready(route, ready));
            }
            PhysicalSignalRouteCommand::RevalidateBlocked(admitted, active, reply) => {
                let _ = reply.send(self.revalidate_signal(route, admitted, active));
            }
            PhysicalSignalRouteCommand::RecordSettlement(envelope, reply) => {
                let _ = reply.send(self.record_settlement(envelope));
            }
            PhysicalSignalRouteCommand::RecordSettlementBatch(envelopes, reply) => {
                let _ = reply.send(self.record_settlement_batch(envelopes));
            }
            PhysicalSignalRouteCommand::Cancel(handle, reply) => {
                let result = self
                    .runtime
                    .cancel_resource_request(
                        handle,
                        worth_signal::facade::ResourceCancellationReason::HostRequested,
                    )
                    .map_err(|_| ());
                if result
                    .as_ref()
                    .is_ok_and(|report| report.cancelled_request().is_some())
                {
                    self.release_signal(handle);
                }
                let _ = reply.send(result);
            }
            PhysicalSignalRouteCommand::ScheduleRetry(handle, reply) => {
                let _ = reply.send(
                    self.runtime
                        .schedule_resource_retry(
                            handle,
                            worth_signal::facade::ResourceRetryReason::HostRequested,
                        )
                        .map_err(|_| ()),
                );
            }
            PhysicalSignalRouteCommand::AdmitRetry(handle, wake, reply) => {
                let result = self
                    .runtime
                    .promote_temporal_wake_ready(wake)
                    .map_err(|_| ())
                    .and_then(|ready| {
                        self.runtime
                            .admit_scheduled_resource_retry(handle, ready)
                            .map_err(|_| ())
                    });
                let _ = reply.send(result);
            }
            PhysicalSignalRouteCommand::AdvanceClock(request, reply) => {
                let _ = reply.send(self.runtime.advance_clock(request).map_err(|_| ()));
            }
            PhysicalSignalRouteCommand::Timeout(handle, reply) => {
                let result = self
                    .runtime
                    .in_flight_resource_request(handle)
                    .and_then(|request| request.timeout_wake_id())
                    .ok_or(())
                    .and_then(|wake| {
                        self.runtime
                            .promote_temporal_wake_ready(wake)
                            .map_err(|_| ())
                    })
                    .and_then(|ready| {
                        self.runtime
                            .admit_resource_timeout(handle, ready)
                            .map_err(|_| ())
                    });
                if result
                    .as_ref()
                    .is_ok_and(|report| report.timed_out_request().is_some())
                {
                    self.release_signal(handle);
                }
                let _ = reply.send(result);
            }
            PhysicalSignalRouteCommand::Release(identity) => {
                self.release_identity(identity);
            }
            PhysicalSignalRouteCommand::Observation(reply) => {
                let _ = reply.send(self.observation());
            }
        }
        self.healthy
    }
}
