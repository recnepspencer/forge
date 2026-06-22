use super::ForgeQueryAspectTouch;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum ForgeQueryDeniedAspectTouch {
    Admitted(ForgeQueryAspectTouch),
}

impl ForgeQueryDeniedAspectTouch {
    pub(super) fn admitted_touch(&self) -> Option<&ForgeQueryAspectTouch> {
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
