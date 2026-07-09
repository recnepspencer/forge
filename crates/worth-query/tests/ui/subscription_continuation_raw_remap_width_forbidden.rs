use worth_query::facade::{
    admit_subscription_continuation_evidence, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag, SubscriptionContinuationClass,
};

fn evidence(label: &str) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "subscription_continuation_ui_identity_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("label"), label)
        .seal()
}

fn main() {
    let _evidence = admit_subscription_continuation_evidence(
        todo!(),
        SubscriptionContinuationClass::IdentityRemap,
        evidence("employee:old"),
        evidence("employee:new"),
        evidence("basis:current"),
        evidence("identity-evolution-authority"),
        1,
    )
    .unwrap();
}
