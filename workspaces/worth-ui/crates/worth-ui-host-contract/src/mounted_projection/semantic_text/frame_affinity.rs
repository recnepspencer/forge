use std::sync::Arc;

impl super::UiMountedSemanticTextMechanic {
    #[doc(hidden)]
    pub fn retained_text_for_runtime_mounting(&self) -> Arc<str> {
        Arc::clone(&self.text)
    }

    #[doc(hidden)]
    pub fn retained_foregrounds_for_runtime_mounting(
        &self,
    ) -> Arc<[super::UiMountedTextForegroundSpan]> {
        Arc::clone(&self.foregrounds)
    }
}
