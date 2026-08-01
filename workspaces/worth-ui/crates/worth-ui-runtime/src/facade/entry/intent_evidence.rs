use super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    pub fn lookup_intent_causal_trace(
        &self,
        reference: worth_ui_inspection::UiIntentEvidenceReference,
    ) -> worth_ui_inspection::UiIntentEvidenceLookup {
        self.intent_evidence.lookup_trace(reference)
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn latest_intent_evidence_reference_for_certification(
        &self,
    ) -> Option<worth_ui_inspection::UiIntentEvidenceReference> {
        self.intent_evidence.latest_reference()
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn lookup_intent_evidence_for_certification(
        &self,
        reference: worth_ui_inspection::UiIntentEvidenceReference,
    ) -> Option<worth_ui_inspection::UiIntentInteractionEvidence> {
        self.intent_evidence.lookup(reference)
    }
}
