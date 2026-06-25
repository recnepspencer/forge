use crate::runtime::{
    WorthUiCompositionChildSizing, WorthUiCompositionParticipation,
    WorthUiLiveViewParticipationReceipt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLayoutParticipationPosture {
    Participating,
    AbsentRetainsState,
    Inert,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLayoutAllocatedChildSizing {
    Hug,
    Fill(u32),
}

impl WorthUiLayoutParticipationPosture {
    pub(super) fn from_composition_participation(
        participation: WorthUiCompositionParticipation,
    ) -> Self {
        match participation {
            WorthUiCompositionParticipation::Present => Self::Participating,
            WorthUiCompositionParticipation::AbsentRetainsState => Self::AbsentRetainsState,
            WorthUiCompositionParticipation::Inert => Self::Inert,
        }
    }

    pub(super) fn from_live_view_participation(
        participation: Option<&WorthUiLiveViewParticipationReceipt>,
        fallback: WorthUiCompositionParticipation,
    ) -> Self {
        participation.map_or_else(
            || Self::from_composition_participation(fallback),
            |receipt| {
                if receipt.participates_in_layout() {
                    Self::Participating
                } else if receipt.retained_state().token() == "retained" {
                    Self::AbsentRetainsState
                } else {
                    Self::Inert
                }
            },
        )
    }

    pub fn token(self) -> &'static str {
        match self {
            Self::Participating => "participating",
            Self::AbsentRetainsState => "absent_retains_state",
            Self::Inert => "inert",
        }
    }

    pub fn participates_in_layout(self) -> bool {
        matches!(self, Self::Participating)
    }
}

impl WorthUiLayoutAllocatedChildSizing {
    pub(super) fn from_composition_sizing(sizing: WorthUiCompositionChildSizing) -> Self {
        match sizing {
            WorthUiCompositionChildSizing::Auto | WorthUiCompositionChildSizing::Hug => Self::Hug,
            WorthUiCompositionChildSizing::Fill(weight) => Self::Fill(weight),
        }
    }

    pub fn token(self) -> String {
        match self {
            Self::Hug => "hug".to_owned(),
            Self::Fill(weight) => format!("fill({weight})"),
        }
    }

    pub fn fill_weight(self) -> Option<u32> {
        match self {
            Self::Hug => None,
            Self::Fill(weight) => Some(weight),
        }
    }
}
