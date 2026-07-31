use worth_query_declaration::facade::application_capability::ApplicationCapabilityContextRef;

use super::{contract, contract_with_context, Axis, Schema};
use crate::application_capability::{
    canonical_basis::prepare_capability_basis,
    WorthQueryApplicationCapabilityInstallationDenialKind,
};

#[test]
fn capability_canonical_bytes_are_bounded_before_identity_derivation() {
    struct LongContext;
    let long_name = "x".repeat(70 * 1_024);
    let context = ApplicationCapabilityContextRef::<Schema, LongContext>::from_schema_identifier(
        Box::leak(long_name.into_boxed_str()),
    );
    let contract = contract_with_context(context);
    let package = worth_foundational::facade::CanonicalDigestId::new([1; 32]);
    let schema = worth_foundational::facade::CanonicalDigestId::new([2; 32]);
    let denial = prepare_capability_basis(&package, &schema, &contract).unwrap_err();
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationCapabilityInstallationDenialKind::CanonicalByteLimitExceeded
    );
}

#[test]
fn capability_canonical_entries_are_bounded_before_identity_derivation() {
    let contract = contract(Some(Axis::OversizedComposition));
    let package = worth_foundational::facade::CanonicalDigestId::new([1; 32]);
    let schema = worth_foundational::facade::CanonicalDigestId::new([2; 32]);
    let denial = prepare_capability_basis(&package, &schema, &contract).unwrap_err();
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationCapabilityInstallationDenialKind::CanonicalEntryLimitExceeded
    );
}
