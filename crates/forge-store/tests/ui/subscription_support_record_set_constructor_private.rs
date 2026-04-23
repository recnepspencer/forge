use forge_store::{
    SubscriptionResumeClassification, SubscriptionSupportFamilyKind, SubscriptionSupportRole,
    SubscriptionSupportStoredRecordKey, SubscriptionSupportStoredRecordSet,
};

fn main() {
    let key = SubscriptionSupportStoredRecordKey {
        family_id: "basis-bound-continuation-support".into(),
        artifact_id: "subscription-support:basis-bound-continuation-support:abc".into(),
    };

    let _ = SubscriptionSupportStoredRecordSet {
        key,
        family_kind: SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        role: SubscriptionSupportRole::ExactContinuation,
        declaration_digest: "declaration".into(),
        artifact_digest: "artifact".into(),
        payload_digest: "payload".into(),
        basis_digest: "basis".into(),
        cursor_digest: "cursor".into(),
        checkpoint_digest: "checkpoint".into(),
        schema_digest: "schema".into(),
        compatibility_binding: "compatibility-binding".into(),
        compatibility_digest: "compatibility".into(),
        initial_classification: Some(SubscriptionResumeClassification::Exact),
        restart_shard: None,
    };
}
