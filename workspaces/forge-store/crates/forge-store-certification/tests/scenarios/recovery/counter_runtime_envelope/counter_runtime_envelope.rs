use forge_store_buffer_pool::{
    AllocationAdmission, AllocationRequest, AllocationScope, BufferPoolEvidenceSourceDenial,
    BufferPoolExecutedEvidenceSource,
};
use forge_store_io_scheduler::{IoQueueExecutionDenial, IoQueueExecutionRecorder};

#[path = "../../../support/recovery/counter_strength/support.rs"]
mod support;

use support::{counter_receipt, lower_s5_plan, observed_trace};

#[test]
fn denied_allocation_execution_cannot_mint_buffer_pool_counter_evidence() {
    let plan = lower_s5_plan();
    let mut admission =
        AllocationAdmission::from_declaration(plan.resource_envelope().allocation());
    let grant = admission
        .admit(AllocationRequest::copied_payload(AllocationScope::Foreground, 64).unwrap())
        .unwrap();
    admission.record_allocation(grant).unwrap();
    let too_large = admission.remaining(AllocationScope::Foreground) + 1;

    admission
        .admit(AllocationRequest::copied_payload(AllocationScope::Foreground, too_large).unwrap())
        .unwrap_err();

    let denial = BufferPoolExecutedEvidenceSource::from_allocation_execution(&admission)
        .expect_err("denied allocation execution must not become evidence");

    assert_eq!(
        denial,
        BufferPoolEvidenceSourceDenial::ExecutionContainedDeniedAllocation
    );
}

#[test]
fn denied_io_queue_depth_cannot_mint_io_counter_evidence() {
    let plan = lower_s5_plan();
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
    let plan = lower_s5_plan();
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
    let plan = lower_s5_plan();
    let receipt = counter_receipt(&plan, observed_trace(&plan));

    assert_eq!(
        receipt.rows().len(),
        plan.counter_contracts().iter().count()
    );
}
