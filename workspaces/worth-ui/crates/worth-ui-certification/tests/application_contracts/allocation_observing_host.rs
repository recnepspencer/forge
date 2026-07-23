use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use worth_ui_host_contract::{
    UiHostObservationValue, UiMeasurementRequest, WorthUiHostCapabilityReport, WorthUiHostContract,
    WorthUiHostOutputDisposition, WorthUiHostOutputEnvelope, WorthUiMeasurementHostAdapter,
    WorthUiOperationalHostAdapter,
};

pub(super) struct HostAllocationObservation {
    call_count: AtomicU64,
    last_allocation_count: AtomicU64,
}

pub(super) struct AllocationObservingHost<Host> {
    host: Host,
    observation: Arc<HostAllocationObservation>,
}

impl<Host> AllocationObservingHost<Host> {
    pub(super) fn new(host: Host) -> (Self, Arc<HostAllocationObservation>) {
        let observation = Arc::new(HostAllocationObservation {
            call_count: AtomicU64::new(0),
            last_allocation_count: AtomicU64::new(0),
        });
        (
            Self {
                host,
                observation: Arc::clone(&observation),
            },
            observation,
        )
    }
}

impl HostAllocationObservation {
    pub(super) fn call_count(&self) -> u64 {
        self.call_count.load(Ordering::Acquire)
    }

    pub(super) fn last_allocation_count(&self) -> u64 {
        self.last_allocation_count.load(Ordering::Relaxed)
    }
}

impl<Host> WorthUiMeasurementHostAdapter for AllocationObservingHost<Host>
where
    Host: WorthUiMeasurementHostAdapter,
{
    fn observe_measurement(&self, request: &UiMeasurementRequest) -> UiHostObservationValue {
        self.host.observe_measurement(request)
    }
}

impl<Host> WorthUiOperationalHostAdapter for AllocationObservingHost<Host>
where
    Host: WorthUiOperationalHostAdapter,
{
    fn operational_host_contract(&self) -> WorthUiHostContract {
        self.host.operational_host_contract()
    }

    fn operational_capability_report(&self) -> WorthUiHostCapabilityReport {
        self.host.operational_capability_report()
    }

    fn consume_output(&self, output: &WorthUiHostOutputEnvelope) -> WorthUiHostOutputDisposition {
        let mut disposition = WorthUiHostOutputDisposition::UnsupportedPayload;
        let allocations = allocation_counter::measure(|| {
            disposition = self.host.consume_output(output);
        });
        self.observation
            .last_allocation_count
            .store(allocations.count_total, Ordering::Relaxed);
        self.observation.call_count.fetch_add(1, Ordering::Release);
        disposition
    }
}
