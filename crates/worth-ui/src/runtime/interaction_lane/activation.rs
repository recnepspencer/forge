#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiMountedInteractionGesture {
    PrimaryClick,
}

impl WorthUiMountedInteractionGesture {
    pub fn primary_click() -> Self {
        Self::PrimaryClick
    }

    pub fn token(self) -> &'static str {
        match self {
            Self::PrimaryClick => "primary_click",
        }
    }
}
