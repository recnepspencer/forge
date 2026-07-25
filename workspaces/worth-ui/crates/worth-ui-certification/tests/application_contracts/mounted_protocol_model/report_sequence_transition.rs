use worth_ui::facade::observation_report::{
    UiHostObservationPayload, UiHostObservationSequence, UiHostObservationSequenceRange,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ModelCoalescingKey {
    PointerMotion {
        pointer: u64,
        capture_epoch: u64,
        pressed_buttons: u64,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AuthoredMechanicalReport {
    sequence: u64,
    payload: UiHostObservationPayload,
    coalescing_key: Option<ModelCoalescingKey>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ExpectedMechanicalState {
    pub(crate) terminal_payload: UiHostObservationPayload,
    pub(crate) retained_reports: usize,
    pub(crate) replaced: Option<UiHostObservationSequenceRange>,
}

#[derive(Clone)]
struct RetainedModelReport {
    payload: UiHostObservationPayload,
    coalescing_key: Option<ModelCoalescingKey>,
    sequence: u64,
    replaced: Option<(u64, u64)>,
}

impl AuthoredMechanicalReport {
    pub(crate) fn pointer_motion(sequence: u64, x_subpixels: i64) -> Self {
        let pointer = 7;
        let capture_epoch = 3;
        let pressed_buttons = 0;
        Self {
            sequence,
            payload: UiHostObservationPayload::PointerMotion {
                pointer,
                capture_epoch,
                pressed_buttons,
                x_subpixels,
                y_subpixels: i64::try_from(sequence).unwrap(),
            },
            coalescing_key: Some(ModelCoalescingKey::PointerMotion {
                pointer,
                capture_epoch,
                pressed_buttons,
            }),
        }
    }

    pub(crate) fn sequence(&self) -> u64 {
        self.sequence
    }

    pub(crate) fn payload(&self) -> &UiHostObservationPayload {
        &self.payload
    }
}

pub(crate) fn model_terminal_state(
    reports: &[AuthoredMechanicalReport],
) -> ExpectedMechanicalState {
    let mut retained = Vec::<RetainedModelReport>::new();
    for report in reports {
        let previous_matches = retained.last().is_some_and(|previous| {
            report.coalescing_key.is_some() && previous.coalescing_key == report.coalescing_key
        });
        if previous_matches {
            let previous = retained.pop().expect("matching predecessor exists");
            let first = previous
                .replaced
                .map(|range| range.0)
                .unwrap_or(previous.sequence);
            retained.push(RetainedModelReport {
                payload: report.payload.clone(),
                coalescing_key: report.coalescing_key,
                sequence: report.sequence,
                replaced: Some((first, previous.sequence)),
            });
        } else {
            retained.push(RetainedModelReport {
                payload: report.payload.clone(),
                coalescing_key: report.coalescing_key,
                sequence: report.sequence,
                replaced: None,
            });
        }
    }
    let terminal = retained.last().cloned().expect("trace is non-empty");
    ExpectedMechanicalState {
        terminal_payload: terminal.payload,
        retained_reports: retained.len(),
        replaced: terminal.replaced.map(|(first, last)| {
            UiHostObservationSequenceRange::new(
                UiHostObservationSequence::new(first),
                UiHostObservationSequence::new(last),
            )
        }),
    }
}
