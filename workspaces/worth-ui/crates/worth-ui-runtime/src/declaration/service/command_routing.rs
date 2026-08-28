#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiDeclaredCommandRoutingContract {
    TypedInvocation,
}

impl UiDeclaredCommandRoutingContract {
    pub(crate) const fn family(self) -> crate::capability::UiRuntimeServiceFamily {
        crate::capability::UiRuntimeServiceFamily::CommandRouting
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiCommandRoutingPolicy {
    maximum_strokes: u8,
    maximum_prefix_wait_millis: u16,
    suppress_repeats: bool,
    suppress_during_ime: bool,
    suppress_during_text_input: bool,
}

impl UiCommandRoutingPolicy {
    pub const fn desktop() -> Self {
        Self {
            maximum_strokes: 2,
            maximum_prefix_wait_millis: 1_000,
            suppress_repeats: true,
            suppress_during_ime: true,
            suppress_during_text_input: true,
        }
    }

    pub const fn with_repeat_suppression(mut self, enabled: bool) -> Self {
        self.suppress_repeats = enabled;
        self
    }

    pub const fn with_text_input_suppression(mut self, enabled: bool) -> Self {
        self.suppress_during_text_input = enabled;
        self
    }

    pub const fn with_maximum_prefix_wait_millis(mut self, millis: u16) -> Self {
        self.maximum_prefix_wait_millis = millis;
        self
    }

    pub const fn maximum_strokes(self) -> u8 {
        self.maximum_strokes
    }

    pub const fn maximum_prefix_wait_millis(self) -> u16 {
        self.maximum_prefix_wait_millis
    }

    pub const fn suppresses_repeats(self) -> bool {
        self.suppress_repeats
    }

    pub const fn suppresses_during_ime(self) -> bool {
        self.suppress_during_ime
    }

    pub const fn suppresses_during_text_input(self) -> bool {
        self.suppress_during_text_input
    }

    pub(crate) const fn digest_basis(self) -> u64 {
        self.maximum_strokes as u64
            | (self.maximum_prefix_wait_millis as u64) << 16
            | (self.suppress_repeats as u64) << 8
            | (self.suppress_during_ime as u64) << 9
            | (self.suppress_during_text_input as u64) << 10
    }
}
