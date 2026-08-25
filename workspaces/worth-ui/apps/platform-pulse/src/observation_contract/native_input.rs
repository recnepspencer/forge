use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseNativeInputIngressPosture {
    Retained,
    Stopped,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseNativeInputReached {
    event_count: u64,
    pointer_button_events: u64,
    keyboard_events: u64,
    text_events: u64,
    ime_preedit_events: u64,
    ime_commit_events: u64,
    ime_cancel_events: u64,
    posture: PlatformPulseNativeInputIngressPosture,
}

impl PlatformPulseNativeInputReached {
    pub fn event_count(self) -> u64 {
        self.event_count
    }

    pub fn pointer_button_events(self) -> u64 {
        self.pointer_button_events
    }

    pub fn keyboard_events(self) -> u64 {
        self.keyboard_events
    }

    pub fn text_events(self) -> u64 {
        self.text_events
    }

    pub fn ime_preedit_events(self) -> u64 {
        self.ime_preedit_events
    }

    pub fn ime_commit_events(self) -> u64 {
        self.ime_commit_events
    }

    pub fn ime_cancel_events(self) -> u64 {
        self.ime_cancel_events
    }

    pub fn posture(self) -> PlatformPulseNativeInputIngressPosture {
        self.posture
    }

    pub fn from_counts(
        event_count: u64,
        pointer_button_events: u64,
        keyboard_events: u64,
        text_events: u64,
        ime_preedit_events: u64,
        ime_commit_events: u64,
        ime_cancel_events: u64,
        posture: PlatformPulseNativeInputIngressPosture,
    ) -> Self {
        Self {
            event_count,
            pointer_button_events,
            keyboard_events,
            text_events,
            ime_preedit_events,
            ime_commit_events,
            ime_cancel_events,
            posture,
        }
    }
}
