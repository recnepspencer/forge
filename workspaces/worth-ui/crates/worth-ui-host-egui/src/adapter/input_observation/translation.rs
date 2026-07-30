use worth_ui_host_contract::{
    UiHostObservationBatch, UiHostObservationBatchInput, UiHostObservationLoss,
    UiHostObservationPayload, UiHostObservationReport, UiHostObservationSequenceRange,
    UiHostObservationTimeBasis, WorthUiHostCapability, UI_HOST_OBSERVATION_BATCH_BYTE_LIMIT,
    UI_HOST_OBSERVATION_BATCH_REPORT_LIMIT,
};

use super::outcome::{
    UiEguiRawInputIngressStop, UiEguiRawInputIngressStopReason, UiEguiUnsupportedEventFamily,
};
use super::presentation_basis::UiEguiPresentedInputBasis;
use super::state::UiEguiInputTranslationState;
use super::UiEguiRawInputReachability;

#[derive(Clone, Copy, Default)]
pub(crate) struct UiEguiInstalledInputTranslators;

pub(super) struct UiEguiTranslatedInput {
    pub(super) reachability: UiEguiRawInputReachability,
    pub(super) state: UiEguiInputTranslationState,
    pub(super) batch: Option<UiHostObservationBatch>,
}

impl UiEguiInstalledInputTranslators {
    pub(crate) const fn capabilities(self) -> [WorthUiHostCapability; 4] {
        [
            WorthUiHostCapability::PointerInput,
            WorthUiHostCapability::KeyboardInput,
            WorthUiHostCapability::TextInput,
            WorthUiHostCapability::Ime,
        ]
    }

    pub(super) fn translate(
        self,
        raw_input: &egui::RawInput,
        basis: UiEguiPresentedInputBasis,
        mut state: UiEguiInputTranslationState,
    ) -> Result<UiEguiTranslatedInput, UiEguiRawInputIngressStop> {
        let mut reachability = UiEguiRawInputReachability::for_event_count(raw_input.events.len());
        let mut reports = Vec::with_capacity(
            raw_input
                .events
                .len()
                .min(UI_HOST_OBSERVATION_BATCH_REPORT_LIMIT),
        );
        let mut retained_bytes = 0usize;
        let mut stop = None;
        for (index, event) in raw_input.events.iter().enumerate() {
            reachability.observe(event);
            if stop.is_some() {
                continue;
            }
            if event_text_bytes(event) > UI_HOST_OBSERVATION_BATCH_BYTE_LIMIT {
                stop = Some(UiEguiRawInputIngressStopReason::ByteLimitExceeded);
                continue;
            }
            match translate_event(event, &mut state) {
                Ok(Some(payload)) => {
                    let encoded_len = 24usize
                        .checked_add(payload.encoded_len())
                        .unwrap_or(usize::MAX);
                    if reports.len() == UI_HOST_OBSERVATION_BATCH_REPORT_LIMIT {
                        stop = Some(UiEguiRawInputIngressStopReason::ReportLimitExceeded);
                        continue;
                    }
                    let Some(next_bytes) = retained_bytes.checked_add(encoded_len) else {
                        stop = Some(UiEguiRawInputIngressStopReason::ByteLimitExceeded);
                        continue;
                    };
                    if next_bytes > UI_HOST_OBSERVATION_BATCH_BYTE_LIMIT {
                        stop = Some(UiEguiRawInputIngressStopReason::ByteLimitExceeded);
                        continue;
                    }
                    let Some(sequence) = state.take_sequence() else {
                        stop = Some(UiEguiRawInputIngressStopReason::SequenceExhausted);
                        continue;
                    };
                    reports.push(UiHostObservationReport::new(
                        sequence,
                        UiHostObservationTimeBasis::HostMonotonicTick(sequence.value()),
                        payload,
                    ));
                    retained_bytes = next_bytes;
                }
                Ok(None) => {}
                Err(reason) => stop = Some(reason.with_index(index)),
            }
        }
        if let Some(reason) = stop {
            return Err(UiEguiRawInputIngressStop::new(reachability, reason));
        }
        let batch = build_batch(basis, reports)
            .map_err(|reason| UiEguiRawInputIngressStop::new(reachability, reason))?;
        Ok(UiEguiTranslatedInput {
            reachability,
            state,
            batch,
        })
    }
}

fn build_batch(
    basis: UiEguiPresentedInputBasis,
    reports: Vec<UiHostObservationReport>,
) -> Result<Option<UiHostObservationBatch>, UiEguiRawInputIngressStopReason> {
    let Some(first) = reports.first().map(UiHostObservationReport::sequence) else {
        return Ok(None);
    };
    let last = reports
        .last()
        .expect("non-empty reports have a last sequence")
        .sequence();
    UiHostObservationBatch::new(UiHostObservationBatchInput {
        protocol: basis.protocol(),
        host_session: basis.host_session(),
        presentation: basis.presentation(),
        sequences: UiHostObservationSequenceRange::new(first, last),
        loss: UiHostObservationLoss::Complete,
        reports,
    })
    .map(Some)
    .map_err(UiEguiRawInputIngressStopReason::BatchConstruction)
}

enum UiEguiEventTranslationDenial {
    Unsupported(UiEguiUnsupportedEventFamily),
    Coordinate(super::UiEguiCoordinateConversionDenial),
    ImePreedit(worth_ui_host_contract::UiHostImePreeditConstructionDenial),
    TextRevisionExhausted,
    PointerCaptureEpochExhausted,
}

impl UiEguiEventTranslationDenial {
    fn with_index(self, index: usize) -> UiEguiRawInputIngressStopReason {
        match self {
            Self::Unsupported(family) => {
                UiEguiRawInputIngressStopReason::UnsupportedEvent { index, family }
            }
            Self::Coordinate(denial) => {
                UiEguiRawInputIngressStopReason::Coordinate { index, denial }
            }
            Self::ImePreedit(denial) => {
                UiEguiRawInputIngressStopReason::ImePreedit { index, denial }
            }
            Self::TextRevisionExhausted => UiEguiRawInputIngressStopReason::TextRevisionExhausted,
            Self::PointerCaptureEpochExhausted => {
                UiEguiRawInputIngressStopReason::PointerCaptureEpochExhausted
            }
        }
    }
}

fn translate_event(
    event: &egui::Event,
    state: &mut UiEguiInputTranslationState,
) -> Result<Option<UiHostObservationPayload>, UiEguiEventTranslationDenial> {
    match event {
        egui::Event::Copy => unsupported(UiEguiUnsupportedEventFamily::Copy),
        egui::Event::Cut => unsupported(UiEguiUnsupportedEventFamily::Cut),
        egui::Event::Paste(text) | egui::Event::Text(text) => state
            .text_ime
            .text(text)
            .map(Some)
            .map_err(map_text_ime_denial),
        egui::Event::Key {
            key,
            physical_key,
            pressed,
            repeat,
            modifiers,
        } => Ok(Some(super::keyboard::translate(
            *key,
            *physical_key,
            *pressed,
            *repeat,
            *modifiers,
        ))),
        egui::Event::PointerMoved(position) => state
            .pointer
            .motion(*position)
            .map(Some)
            .map_err(UiEguiEventTranslationDenial::Coordinate),
        egui::Event::MouseMoved(_) | egui::Event::Touch { .. } => Ok(None),
        egui::Event::PointerButton {
            pos,
            button,
            pressed,
            ..
        } => state
            .pointer
            .button(*pos, *button, *pressed)
            .map(Some)
            .map_err(UiEguiEventTranslationDenial::Coordinate),
        egui::Event::PointerGone => {
            state
                .pointer
                .end_capture()
                .map_err(|()| UiEguiEventTranslationDenial::PointerCaptureEpochExhausted)?;
            Ok(None)
        }
        egui::Event::Zoom(_) => unsupported(UiEguiUnsupportedEventFamily::Zoom),
        egui::Event::Rotate(_) => unsupported(UiEguiUnsupportedEventFamily::Rotate),
        egui::Event::Ime(event) => translate_ime(event, &mut state.text_ime).map(Some),
        egui::Event::MouseWheel { unit, delta, .. } => match unit {
            egui::MouseWheelUnit::Point => {
                super::pointer::UiEguiPointerTranslationState::scroll(*delta)
                    .map(Some)
                    .map_err(UiEguiEventTranslationDenial::Coordinate)
            }
            egui::MouseWheelUnit::Line => unsupported(UiEguiUnsupportedEventFamily::LineScroll),
            egui::MouseWheelUnit::Page => unsupported(UiEguiUnsupportedEventFamily::PageScroll),
        },
        egui::Event::WindowFocused(focused) => {
            if !focused {
                state
                    .pointer
                    .end_capture()
                    .map_err(|()| UiEguiEventTranslationDenial::PointerCaptureEpochExhausted)?;
            }
            Ok(Some(UiHostObservationPayload::Focus { focused: *focused }))
        }
        egui::Event::AccessKitActionRequest(_) => {
            unsupported(UiEguiUnsupportedEventFamily::AccessKitAction)
        }
        egui::Event::Screenshot { .. } => unsupported(UiEguiUnsupportedEventFamily::Screenshot),
    }
}

#[allow(deprecated)]
fn translate_ime(
    event: &egui::ImeEvent,
    state: &mut super::text_ime::UiEguiTextImeTranslationState,
) -> Result<UiHostObservationPayload, UiEguiEventTranslationDenial> {
    match event {
        egui::ImeEvent::Preedit {
            text,
            active_range_chars,
        } => state
            .preedit(text, active_range_chars.clone())
            .map_err(map_text_ime_denial),
        egui::ImeEvent::Commit(text) => state.commit(text).map_err(map_text_ime_denial),
        egui::ImeEvent::Enabled | egui::ImeEvent::Disabled => Err(
            UiEguiEventTranslationDenial::Unsupported(UiEguiUnsupportedEventFamily::ImeLifecycle),
        ),
    }
}

fn map_text_ime_denial(
    denial: super::text_ime::UiEguiTextImeTranslationDenial,
) -> UiEguiEventTranslationDenial {
    match denial {
        super::text_ime::UiEguiTextImeTranslationDenial::RevisionExhausted => {
            UiEguiEventTranslationDenial::TextRevisionExhausted
        }
        super::text_ime::UiEguiTextImeTranslationDenial::Preedit(denial) => {
            UiEguiEventTranslationDenial::ImePreedit(denial)
        }
    }
}

fn unsupported(
    family: UiEguiUnsupportedEventFamily,
) -> Result<Option<UiHostObservationPayload>, UiEguiEventTranslationDenial> {
    Err(UiEguiEventTranslationDenial::Unsupported(family))
}

fn event_text_bytes(event: &egui::Event) -> usize {
    match event {
        egui::Event::Paste(text)
        | egui::Event::Text(text)
        | egui::Event::Ime(egui::ImeEvent::Commit(text))
        | egui::Event::Ime(egui::ImeEvent::Preedit { text, .. }) => text.len(),
        _ => 0,
    }
}
