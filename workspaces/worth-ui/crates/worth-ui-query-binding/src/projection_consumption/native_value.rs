use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiNativeTextValue {
    text: Arc<str>,
}

impl UiNativeTextValue {
    pub(crate) fn from_raw(text: String) -> Self {
        Self {
            text: Arc::from(text),
        }
    }

    pub fn as_str(&self) -> &str {
        self.text.as_ref()
    }

    pub fn byte_len(&self) -> usize {
        self.text.len()
    }
}
