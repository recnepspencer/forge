/// Typed placeholder for future command runtime intent binding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandRuntimeIntentBinding {
    intent_key: String,
}

impl CommandRuntimeIntentBinding {
    pub fn named(intent_key: impl Into<String>) -> Self {
        Self {
            intent_key: intent_key.into(),
        }
    }

    pub fn intent_key(&self) -> &str {
        &self.intent_key
    }

    pub(crate) fn digest_basis(&self) -> &str {
        &self.intent_key
    }
}
