#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiRealtimeFramePriority {
    HudOverlay,
    CriticalOverlay,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiHighFrequencyFramePolicyDenialReason {
    ZeroFrameBudgetMillis,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiHighFrequencyFramePolicyDenial {
    reason: WorthUiHighFrequencyFramePolicyDenialReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiHighFrequencyFramePolicy {
    frame_budget_millis: u16,
    priority: WorthUiRealtimeFramePriority,
}

impl WorthUiHighFrequencyFramePolicy {
    pub fn frame_budgeted(
        frame_budget_millis: u16,
        priority: WorthUiRealtimeFramePriority,
    ) -> Result<Self, WorthUiHighFrequencyFramePolicyDenial> {
        if frame_budget_millis == 0 {
            return Err(WorthUiHighFrequencyFramePolicyDenial {
                reason: WorthUiHighFrequencyFramePolicyDenialReason::ZeroFrameBudgetMillis,
            });
        }
        Ok(Self {
            frame_budget_millis,
            priority,
        })
    }

    pub fn frame_budget_millis(self) -> u16 {
        self.frame_budget_millis
    }

    pub fn priority(self) -> WorthUiRealtimeFramePriority {
        self.priority
    }

    pub(crate) fn canonical_digest(self) -> u64 {
        fold(
            fold(0x7265_616c_7469_6d65, u64::from(self.frame_budget_millis)),
            self.priority.canonical_tag(),
        )
    }
}

impl WorthUiHighFrequencyFramePolicyDenial {
    pub fn reason(self) -> WorthUiHighFrequencyFramePolicyDenialReason {
        self.reason
    }
}

impl WorthUiRealtimeFramePriority {
    pub(crate) fn canonical_tag(self) -> u64 {
        match self {
            Self::HudOverlay => 1,
            Self::CriticalOverlay => 2,
        }
    }
}

fn fold(mut digest: u64, value: u64) -> u64 {
    digest ^= value;
    digest.wrapping_mul(0x100000001b3)
}
