use worth_ui_host_contract::{
    UiHostObservationBatch, UiHostObservationBatchInput, UiHostObservationLoss,
    UiHostObservationPayload, UiHostObservationReport, UiHostObservationSequenceRange,
    UiHostObservationTimeBasis, WorthUiHostCapability, UI_HOST_OBSERVATION_BATCH_BYTE_LIMIT,
    UI_HOST_OBSERVATION_BATCH_REPORT_LIMIT,
};

use super::outcome::{
    UiEguiInputTranslatorFamily, UiEguiRawInputIngressStop, UiEguiRawInputIngressStopReason,
    UiEguiUnsupportedEventFamily,
};
use super::presentation_basis::UiEguiPresentedInputBasis;
use super::state::UiEguiInputTranslationState;
use super::UiEguiRawInputReachability;

#[derive(Clone, Copy)]
pub(crate) struct UiEguiInstalledInputTranslators {
    pointer: Option<super::pointer::UiEguiPointerTranslator>,
    keyboard: Option<super::keyboard::UiEguiKeyboardTranslator>,
    text: Option<super::text_ime::UiEguiTextTranslator>,
    ime: Option<super::text_ime::UiEguiImeTranslator>,
}

pub(super) struct UiEguiTranslatedInput {
    pub(super) reachability: UiEguiRawInputReachability,
    pub(super) state: UiEguiInputTranslationState,
    pub(super) batch: Option<UiHostObservationBatch>,
}

impl UiEguiInstalledInputTranslators {
    pub(crate) fn capabilities(self) -> impl Iterator<Item = WorthUiHostCapability> {
        [
            self.pointer.map(|translator| translator.capability()),
            self.keyboard.map(|translator| translator.capability()),
            self.text.map(|translator| translator.capability()),
            self.ime.map(|translator| translator.capability()),
        ]
        .into_iter()
        .flatten()
    }

    #[cfg(test)]
    pub(crate) fn without(mut self, family: UiEguiInputTranslatorFamily) -> Self {
        match family {
            UiEguiInputTranslatorFamily::Pointer => self.pointer = None,
            UiEguiInputTranslatorFamily::Keyboard => self.keyboard = None,
            UiEguiInputTranslatorFamily::Text => self.text = None,
            UiEguiInputTranslatorFamily::Ime => self.ime = None,
        }
        self
    }

    pub(super) fn translate(
        self,
        raw_input: &egui::RawInput,
        basis: UiEguiPresentedInputBasis,
        recipient: Option<worth_ui_host_contract::UiHostInputRecipientBindingReceipt>,
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
            match self.translate_event(event, &mut state) {
                Ok(Some(payload)) => {
                    let requires_recipient = payload_requires_recipient(&payload);
                    if requires_recipient && recipient.is_none() {
                        stop = Some(
                            UiEguiRawInputIngressStopReason::MissingInputRecipientAffinity {
                                index,
                            },
                        );
                        continue;
                    }
                    let encoded_len = 24usize
                        .saturating_add(payload.encoded_len())
                        .saturating_add(usize::from(requires_recipient) * 96);
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
                    let report = UiHostObservationReport::new(
                        sequence,
                        UiHostObservationTimeBasis::HostMonotonicTick(sequence.value()),
                        payload,
                    );
                    reports.push(
                        if let Some(recipient) = recipient.filter(|_| requires_recipient) {
                            report.with_input_affinity(
                            worth_ui_host_contract::UiHostInputRecipientAffinityReceipt::
                                at_event_time(recipient, basis.presentation()),
                        )
                        } else {
                            report
                        },
                    );
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

    #[allow(deprecated)]
    fn translate_event(
        self,
        event: &egui::Event,
        state: &mut UiEguiInputTranslationState,
    ) -> Result<Option<UiHostObservationPayload>, UiEguiEventTranslationDenial> {
        match event {
            egui::Event::Copy => unsupported(UiEguiUnsupportedEventFamily::Copy),
            egui::Event::Cut => unsupported(UiEguiUnsupportedEventFamily::Cut),
            egui::Event::Paste(text) | egui::Event::Text(text) => self
                .text
                .ok_or(UiEguiEventTranslationDenial::TranslatorUnavailable(
                    UiEguiInputTranslatorFamily::Text,
                ))?
                .translate(&mut state.text_ime, text)
                .map(Some)
                .map_err(map_text_ime_denial),
            egui::Event::Key {
                key,
                physical_key,
                pressed,
                repeat,
                modifiers,
            } => Ok(Some(
                self.keyboard
                    .ok_or(UiEguiEventTranslationDenial::TranslatorUnavailable(
                        UiEguiInputTranslatorFamily::Keyboard,
                    ))?
                    .translate(*key, *physical_key, *pressed, *repeat, *modifiers),
            )),
            egui::Event::PointerMoved(position) => self
                .pointer
                .ok_or(UiEguiEventTranslationDenial::TranslatorUnavailable(
                    UiEguiInputTranslatorFamily::Pointer,
                ))?
                .motion(&state.pointer, *position)
                .map(Some)
                .map_err(UiEguiEventTranslationDenial::Coordinate),
            egui::Event::MouseMoved(_) | egui::Event::Touch { .. } => Ok(None),
            egui::Event::PointerButton {
                pos,
                button,
                pressed,
                ..
            } => self
                .pointer
                .ok_or(UiEguiEventTranslationDenial::TranslatorUnavailable(
                    UiEguiInputTranslatorFamily::Pointer,
                ))?
                .button(&mut state.pointer, *pos, *button, *pressed)
                .map(Some)
                .map_err(UiEguiEventTranslationDenial::Coordinate),
            egui::Event::PointerGone => {
                self.pointer
                    .ok_or(UiEguiEventTranslationDenial::TranslatorUnavailable(
                        UiEguiInputTranslatorFamily::Pointer,
                    ))?
                    .end_capture(&mut state.pointer)
                    .map_err(|()| UiEguiEventTranslationDenial::PointerCaptureEpochExhausted)?;
                Ok(None)
            }
            egui::Event::Zoom(_) => unsupported(UiEguiUnsupportedEventFamily::Zoom),
            egui::Event::Rotate(_) => unsupported(UiEguiUnsupportedEventFamily::Rotate),
            egui::Event::Ime(egui::ImeEvent::Enabled | egui::ImeEvent::Disabled) => Ok(None),
            egui::Event::Ime(event) => self
                .ime
                .ok_or(UiEguiEventTranslationDenial::TranslatorUnavailable(
                    UiEguiInputTranslatorFamily::Ime,
                ))
                .and_then(|translator| translate_ime(translator, event, &mut state.text_ime)),
            egui::Event::MouseWheel { unit, delta, .. } => match unit {
                egui::MouseWheelUnit::Point => self
                    .pointer
                    .ok_or(UiEguiEventTranslationDenial::TranslatorUnavailable(
                        UiEguiInputTranslatorFamily::Pointer,
                    ))?
                    .scroll(*delta)
                    .map(Some)
                    .map_err(UiEguiEventTranslationDenial::Coordinate),
                egui::MouseWheelUnit::Line => unsupported(UiEguiUnsupportedEventFamily::LineScroll),
                egui::MouseWheelUnit::Page => unsupported(UiEguiUnsupportedEventFamily::PageScroll),
            },
            egui::Event::WindowFocused(focused) => {
                if !focused {
                    if let Some(pointer) = self.pointer {
                        pointer.end_capture(&mut state.pointer).map_err(|()| {
                            UiEguiEventTranslationDenial::PointerCaptureEpochExhausted
                        })?;
                    }
                }
                Ok(Some(UiHostObservationPayload::Focus { focused: *focused }))
            }
            egui::Event::AccessKitActionRequest(_) => {
                unsupported(UiEguiUnsupportedEventFamily::AccessKitAction)
            }
            egui::Event::Screenshot { .. } => Ok(None),
        }
    }
}

fn payload_requires_recipient(payload: &UiHostObservationPayload) -> bool {
    matches!(
        payload,
        UiHostObservationPayload::Keyboard { .. }
            | UiHostObservationPayload::TextInput { .. }
            | UiHostObservationPayload::ImeComposition { .. }
    )
}

impl Default for UiEguiInstalledInputTranslators {
    fn default() -> Self {
        Self {
            pointer: Some(super::pointer::UiEguiPointerTranslator),
            keyboard: Some(super::keyboard::UiEguiKeyboardTranslator),
            text: Some(super::text_ime::UiEguiTextTranslator),
            ime: Some(super::text_ime::UiEguiImeTranslator),
        }
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
    TranslatorUnavailable(UiEguiInputTranslatorFamily),
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
            Self::TranslatorUnavailable(family) => {
                UiEguiRawInputIngressStopReason::TranslatorUnavailable { index, family }
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

#[allow(deprecated)]
fn translate_ime(
    translator: super::text_ime::UiEguiImeTranslator,
    event: &egui::ImeEvent,
    state: &mut super::text_ime::UiEguiTextImeTranslationState,
) -> Result<Option<UiHostObservationPayload>, UiEguiEventTranslationDenial> {
    match event {
        egui::ImeEvent::Preedit {
            text,
            active_range_chars,
        } => translator
            .preedit(state, text, active_range_chars.clone())
            .map(Some)
            .map_err(map_text_ime_denial),
        egui::ImeEvent::Commit(text) => translator
            .commit(state, text)
            .map(Some)
            .map_err(map_text_ime_denial),
        egui::ImeEvent::Enabled | egui::ImeEvent::Disabled => Ok(None),
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
