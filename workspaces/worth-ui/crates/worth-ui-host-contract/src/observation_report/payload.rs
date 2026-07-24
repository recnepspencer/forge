use super::UiHostObservationFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiHostObservationPayload {
    Viewport {
        width_subpixels: i64,
        height_subpixels: i64,
    },
    DeviceScale {
        micros: u32,
    },
    PointerMotion {
        pointer: u64,
        capture_epoch: u64,
        pressed_buttons: u64,
        x_subpixels: i64,
        y_subpixels: i64,
    },
    PointerButton {
        pointer: u64,
        capture_epoch: u64,
        button: u16,
        pressed: bool,
    },
    Keyboard {
        physical_key: u32,
        pressed: bool,
        repeat: bool,
    },
    Focus {
        focused: bool,
    },
    ScrollDelta {
        x_subpixels: i64,
        y_subpixels: i64,
    },
    Clock {
        tick: u64,
    },
    Tick {
        tick: u64,
    },
    TextComposition {
        revision: u64,
        text: Box<str>,
    },
    ImeComposition {
        revision: u64,
        text: Box<str>,
        selection_start: u32,
        selection_end: u32,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostObservationCoalescingIdentity {
    Family(UiHostObservationFamily),
    PointerMotion {
        pointer: u64,
        capture_epoch: u64,
        pressed_buttons: u64,
    },
}

impl UiHostObservationPayload {
    pub const fn family(&self) -> UiHostObservationFamily {
        match self {
            Self::Viewport { .. } => UiHostObservationFamily::Viewport,
            Self::DeviceScale { .. } => UiHostObservationFamily::DeviceScale,
            Self::PointerMotion { .. } => UiHostObservationFamily::PointerMotion,
            Self::PointerButton { .. } => UiHostObservationFamily::PointerButton,
            Self::Keyboard { .. } => UiHostObservationFamily::Keyboard,
            Self::Focus { .. } => UiHostObservationFamily::Focus,
            Self::ScrollDelta { .. } => UiHostObservationFamily::ScrollDelta,
            Self::Clock { .. } => UiHostObservationFamily::Clock,
            Self::Tick { .. } => UiHostObservationFamily::Tick,
            Self::TextComposition { .. } => UiHostObservationFamily::TextComposition,
            Self::ImeComposition { .. } => UiHostObservationFamily::ImeComposition,
        }
    }

    pub fn encoded_len(&self) -> usize {
        match self {
            Self::Viewport { .. } => 16,
            Self::DeviceScale { .. } => 4,
            Self::PointerMotion { .. } => 40,
            Self::PointerButton { .. } => 19,
            Self::Keyboard { .. } => 6,
            Self::Focus { .. } => 1,
            Self::ScrollDelta { .. } => 16,
            Self::Clock { .. } | Self::Tick { .. } => 8,
            Self::TextComposition { text, .. } => 8 + text.len(),
            Self::ImeComposition { text, .. } => 16 + text.len(),
        }
    }

    pub const fn coalescing_identity(&self) -> Option<UiHostObservationCoalescingIdentity> {
        match self {
            Self::Viewport { .. }
            | Self::DeviceScale { .. }
            | Self::Clock { .. }
            | Self::Tick { .. } => Some(UiHostObservationCoalescingIdentity::Family(self.family())),
            Self::PointerMotion {
                pointer,
                capture_epoch,
                pressed_buttons,
                ..
            } => Some(UiHostObservationCoalescingIdentity::PointerMotion {
                pointer: *pointer,
                capture_epoch: *capture_epoch,
                pressed_buttons: *pressed_buttons,
            }),
            _ => None,
        }
    }

    pub(super) fn integrity_digest(&self) -> u64 {
        let mut digest = UiHostObservationPayloadDigest::for_family(self.family());
        match self {
            Self::Viewport {
                width_subpixels,
                height_subpixels,
            } => digest.fold_pair(*width_subpixels as u64, *height_subpixels as u64),
            Self::DeviceScale { micros } => digest.fold(u64::from(*micros)),
            Self::PointerMotion {
                pointer,
                capture_epoch,
                pressed_buttons,
                x_subpixels,
                y_subpixels,
            } => {
                digest.fold(*pointer);
                digest.fold(*capture_epoch);
                digest.fold(*pressed_buttons);
                digest.fold_pair(*x_subpixels as u64, *y_subpixels as u64);
            }
            Self::PointerButton {
                pointer,
                capture_epoch,
                button,
                pressed,
            } => digest.fold_pointer_button(*pointer, *capture_epoch, *button, *pressed),
            Self::Keyboard {
                physical_key,
                pressed,
                repeat,
            } => digest.fold_keyboard(*physical_key, *pressed, *repeat),
            Self::Focus { focused } => digest.fold(u64::from(*focused)),
            Self::ScrollDelta {
                x_subpixels,
                y_subpixels,
            } => digest.fold_pair(*x_subpixels as u64, *y_subpixels as u64),
            Self::Clock { tick } | Self::Tick { tick } => digest.fold(*tick),
            Self::TextComposition { revision, text } => {
                digest.fold_text(*revision, text);
            }
            Self::ImeComposition {
                revision,
                text,
                selection_start,
                selection_end,
            } => {
                digest.fold_ime(*revision, text, *selection_start, *selection_end);
            }
        }
        digest.finish()
    }
}

struct UiHostObservationPayloadDigest(u64);

impl UiHostObservationPayloadDigest {
    fn for_family(family: UiHostObservationFamily) -> Self {
        Self(family as u64 + 1)
    }

    fn fold(&mut self, value: u64) {
        self.0 = self.0.rotate_left(9) ^ value.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    }

    fn fold_pair(&mut self, first: u64, second: u64) {
        self.fold(first);
        self.fold(second);
    }

    fn fold_pointer_button(
        &mut self,
        pointer: u64,
        capture_epoch: u64,
        button: u16,
        pressed: bool,
    ) {
        self.fold(pointer);
        self.fold(capture_epoch);
        self.fold_pair(u64::from(button), u64::from(pressed));
    }

    fn fold_keyboard(&mut self, physical_key: u32, pressed: bool, repeat: bool) {
        self.fold(u64::from(physical_key));
        self.fold_pair(u64::from(pressed), u64::from(repeat));
    }

    fn fold_text(&mut self, revision: u64, text: &str) {
        self.fold(revision);
        for byte in text.bytes() {
            self.fold(u64::from(byte));
        }
    }

    fn fold_ime(&mut self, revision: u64, text: &str, selection_start: u32, selection_end: u32) {
        self.fold(revision);
        self.fold_pair(u64::from(selection_start), u64::from(selection_end));
        for byte in text.bytes() {
            self.fold(u64::from(byte));
        }
    }

    fn finish(self) -> u64 {
        self.0
    }
}
