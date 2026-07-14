pub enum UiEvidenceMaterializedDetail {
    Obligation(()),
    Generic(serde_json::Value),
}
