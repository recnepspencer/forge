use crate::{IoQueueExecutionDenial, IoQueueExecutionRecorder, IoQueueResourceEnvelope};

#[test]
fn queue_depth_denial_blocks_executed_evidence() {
    let envelope = IoQueueResourceEnvelope::bounded(2, 1).unwrap();
    let mut recorder = IoQueueExecutionRecorder::from_envelope(envelope);
    recorder.observe_queue_depth(2).unwrap();

    let denial = recorder.observe_queue_depth(3).unwrap_err();

    assert_eq!(
        denial,
        IoQueueExecutionDenial::QueueDepthExceeded {
            maximum: 2,
            actual: 3
        }
    );
    assert_eq!(recorder.executed_evidence().unwrap_err(), denial);
}

#[test]
fn interference_denial_blocks_executed_evidence() {
    let envelope = IoQueueResourceEnvelope::bounded(2, 1).unwrap();
    let mut recorder = IoQueueExecutionRecorder::from_envelope(envelope);
    recorder.record_interference_event().unwrap();

    let denial = recorder.record_interference_event().unwrap_err();

    assert_eq!(
        denial,
        IoQueueExecutionDenial::InterferenceEventsExceeded {
            maximum: 1,
            actual: 2
        }
    );
    assert_eq!(recorder.executed_evidence().unwrap_err(), denial);
}
