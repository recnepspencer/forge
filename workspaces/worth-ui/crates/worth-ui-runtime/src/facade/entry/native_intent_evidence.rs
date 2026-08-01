use super::WorthUiNativeApplicationShell;

impl WorthUiNativeApplicationShell {
    pub fn lookup_intent_causal_trace(
        &self,
        reference: worth_ui_inspection::UiIntentEvidenceReference,
    ) -> worth_ui_inspection::UiIntentEvidenceLookup {
        self.session.lookup_intent_causal_trace(reference)
    }
}
