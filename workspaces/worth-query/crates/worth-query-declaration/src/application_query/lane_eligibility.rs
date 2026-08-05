#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryLaneEligibility {
    one_shot: bool,
    historical: bool,
    live: bool,
    preview: bool,
}

impl ApplicationQueryLaneEligibility {
    pub const fn one_shot() -> Self {
        Self {
            one_shot: true,
            historical: false,
            live: false,
            preview: false,
        }
    }

    pub const fn with_historical(mut self) -> Self {
        self.historical = true;
        self
    }

    pub const fn with_live(mut self) -> Self {
        self.live = true;
        self
    }

    pub const fn with_preview(mut self) -> Self {
        self.preview = true;
        self
    }

    pub const fn one_shot_enabled(self) -> bool {
        self.one_shot
    }

    pub const fn historical_enabled(self) -> bool {
        self.historical
    }

    pub const fn live_enabled(self) -> bool {
        self.live
    }

    pub const fn preview_enabled(self) -> bool {
        self.preview
    }
}
