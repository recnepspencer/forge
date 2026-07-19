use super::WorthQueryAspectTouch;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum WorthQueryDeniedAspectTouch {
    Admitted(WorthQueryAspectTouch),
}

impl WorthQueryDeniedAspectTouch {
    pub(super) fn admitted_touch(&self) -> Option<&WorthQueryAspectTouch> {
        match self {
            Self::Admitted(touch) => Some(touch),
        }
    }

    pub(super) fn admitted_touch_digest_part(&self) -> String {
        match self {
            Self::Admitted(touch) => touch.admitted_touch_digest_part(),
        }
    }
}
