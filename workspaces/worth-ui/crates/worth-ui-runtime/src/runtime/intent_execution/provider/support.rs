#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiIntentExecutionBindingSupport {
    Supported,
    Unsupported,
}

impl UiIntentExecutionBindingSupport {
    pub(crate) const fn digest_tag(self) -> &'static [u8] {
        match self {
            Self::Supported => b"supported",
            Self::Unsupported => b"unsupported",
        }
    }
}
