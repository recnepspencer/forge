pub trait WorthUiIntentEvidenceCertificationExt {
    fn latest_intent_evidence_reference_for_certification(
        &self,
    ) -> Option<worth_ui_inspection::UiIntentEvidenceReference>;

    fn lookup_intent_evidence_for_certification(
        &self,
        reference: worth_ui_inspection::UiIntentEvidenceReference,
    ) -> Option<worth_ui_inspection::UiIntentInteractionEvidence>;
}

impl WorthUiIntentEvidenceCertificationExt for crate::facade::WorthUiActiveApplicationSession {
    fn latest_intent_evidence_reference_for_certification(
        &self,
    ) -> Option<worth_ui_inspection::UiIntentEvidenceReference> {
        self.latest_intent_evidence_reference_for_certification()
    }

    fn lookup_intent_evidence_for_certification(
        &self,
        reference: worth_ui_inspection::UiIntentEvidenceReference,
    ) -> Option<worth_ui_inspection::UiIntentInteractionEvidence> {
        self.lookup_intent_evidence_for_certification(reference)
    }
}
