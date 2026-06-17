use forge_query::facade::{
    admit_subscription_continuation_evidence, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag, SubscriptionContinuationClass,
};

fn evidence(label: &str) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "subscription_continuation_ui_identity_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("label"), label)
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
