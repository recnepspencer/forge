use std::time::{Duration, Instant};

use worth_store::physical_runtime::PhysicalWorkCounterStage;

pub(super) fn await_read_signal_cleanup(
    serving: &worth_store::physical_runtime::ServingPhysicalRuntime,
) {
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        let observation = serving.physical_signal_observation().unwrap();
        if observation.active_locality_count() == 0 && observation.active_in_flight_count() == 0 {
            return;
        }
        if Instant::now() >= deadline {
            let records = serving.physical_work_observer().causal().records();
            let outcomes = records
                .iter()
                .map(|record| (record.backend_operation(), record.derived_completion()))
                .collect::<Vec<_>>();
            panic!(
                "canonical read work retained Signal state after termination: localities={}, in_flight={}, declared={}, terminal={}, settled={}, outcomes={outcomes:?}",
                observation.active_locality_count(),
                observation.active_in_flight_count(),
                serving
                    .physical_work_counters()
                    .total(PhysicalWorkCounterStage::Declared),
                serving
                    .physical_work_counters()
                    .total(PhysicalWorkCounterStage::Terminal),
                records.len(),
            );
        }
        std::thread::yield_now();
    }
}
