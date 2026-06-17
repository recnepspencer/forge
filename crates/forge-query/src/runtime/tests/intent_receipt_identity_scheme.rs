use crate::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceIdentityComparisonError,
    ForgeQueryEvidenceIdentityScheme, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

#[test]
fn intent_receipt_identity_scopes_fail_cross_scheme_typed() {
    for scope in [
        ForgeQueryEvidenceScope::AuthoritativeIntentReceipt,
        ForgeQueryEvidenceScope::EffectIntentReceipt,
        ForgeQueryEvidenceScope::PreviewIntentReceipt,
        ForgeQueryEvidenceScope::PreviewIntentReceiptInspectionBasis,
        ForgeQueryEvidenceScope::PreviewIntentReceiptInspection,
    ] {
        let v1 = probe_identity(scope, ForgeQueryEvidenceIdentityScheme::V1);
        let v1_same = probe_identity(scope, ForgeQueryEvidenceIdentityScheme::V1);
        let v2 = probe_identity(scope, ForgeQueryEvidenceIdentityScheme::V2);

        assert_eq!(v1.eq_same_scheme(&v1_same), Ok(true));
        assert_eq!(
            v1.eq_same_scheme(&v2),
            Err(ForgeQueryEvidenceIdentityComparisonError::SchemeMismatch {
                left: ForgeQueryEvidenceIdentityScheme::V1,
                right: ForgeQueryEvidenceIdentityScheme::V2,
            }),
            "{scope:?} must reject cross-scheme comparison as a typed mismatch"
        );
    }
}

fn probe_identity(
    scope: ForgeQueryEvidenceScope,
    scheme: ForgeQueryEvidenceIdentityScheme,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose_with_scheme(scope, scheme)
        .field_shape(ForgeQueryEvidenceTag::new("probe_shape"), "intent-receipt")
        .field_value(
            ForgeQueryEvidenceTag::new("probe_identity"),
            "identity|with:delimiter",
        )
        .seal()
}
