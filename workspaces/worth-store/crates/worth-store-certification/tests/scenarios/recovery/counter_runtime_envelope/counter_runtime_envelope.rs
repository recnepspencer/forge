use worth_store_io_scheduler::{IoQueueExecutionDenial, IoQueueExecutionRecorder};

use worth_store_test_support::harness::recovery::counter_evidence as support;

use support::{counter_receipt, lower_physical_isolation_plan, observed_trace};

#[test]
fn denied_io_queue_depth_cannot_mint_io_counter_evidence() {
    let plan = lower_physical_isolation_plan();
    let envelope = plan.resource_envelope().io_queue();
    let mut recorder = IoQueueExecutionRecorder::from_envelope(envelope);
    recorder
        .observe_queue_depth(envelope.max_queue_depth())
        .unwrap();

    let denial = recorder
        .observe_queue_depth(envelope.max_queue_depth() + 1)
        .unwrap_err();

    assert_eq!(
        denial,
        IoQueueExecutionDenial::QueueDepthExceeded {
            maximum: envelope.max_queue_depth(),
            actual: envelope.max_queue_depth() + 1,
        }
    );
    assert_eq!(recorder.executed_evidence().unwrap_err(), denial);
}

#[test]
fn denied_io_interference_cannot_mint_io_counter_evidence() {
    let plan = lower_physical_isolation_plan();
    let envelope = plan.resource_envelope().io_queue();
    let mut recorder = IoQueueExecutionRecorder::from_envelope(envelope);
    for _ in 0..envelope.max_interference_events() {
        recorder.record_interference_event().unwrap();
    }

    let denial = recorder.record_interference_event().unwrap_err();

    assert_eq!(
        denial,
        IoQueueExecutionDenial::InterferenceEventsExceeded {
            maximum: envelope.max_interference_events(),
            actual: envelope.max_interference_events() + 1,
        }
    );
    assert_eq!(recorder.executed_evidence().unwrap_err(), denial);
}

#[test]
fn in_envelope_runtime_sources_still_admit_counter_receipts() {
    let plan = lower_physical_isolation_plan();
    let receipt = counter_receipt(&plan, observed_trace(&plan));

    assert_eq!(
        receipt.rows().len(),
        plan.counter_contracts().iter().count()
    );
}
