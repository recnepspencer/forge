#[path = "canonical_certification/support.rs"]
mod support;

use bank_domain::{
    estate::{EstateCapabilityPurpose, RestrictedBankField},
    queries,
};
use support::{certification_fixture, ESTATE};
use worth_foundational::{
    admit_canonical_sequence_digest_derivation, compare_canonical_basis, derive_canonical_digest,
    prepare_canonical_basis_sequence, prepare_canonical_comparison, AspectValue,
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisReadyArtifact, CanonicalBasisValue, CanonicalComparisonOutcome,
    CanonicalDigestAlgorithmId, CanonicalEquivalenceBasis,
    CanonicalSingleSequenceDigestAlgorithmSlot, CanonicalizationRuleVersion, InternedString,
};
use worth_query_host::facade::domain::TypedApplicationValue;

const RULE_VERSION: &str = "bank.estate.governed-disclosure-field.v1";

#[test]
fn capability_disclosure_and_publication_meaning_compare_canonically() {
    let fixture = certification_fixture();
    let request = queries::estate_legal_compliance(ESTATE);
    let capability_meaning = request.capability_request();
    assert_eq!(
        capability_meaning.purpose(),
        EstateCapabilityPurpose::LegalCompliance
    );
    let capability_value = capability_meaning
        .field()
        .expect("the product request carries its exact field")
        .into_foundational_value();

    let definition = queries::estate_legal_compliance_definition();
    let disclosure_rules = definition.disclosure().rules();
    assert!(!disclosure_rules.is_empty());
    let disclosure_value = disclosure_rules[0].disclosure_value().clone();
    assert!(disclosure_rules
        .iter()
        .all(|rule| rule.disclosure_value() == &disclosure_value));

    let result = fixture
        .runtime
        .query(bank_server::queries::estate_legal_compliance(ESTATE))
        .as_principal(&fixture.principal)
        .controls(fixture.controls)
        .execute()
        .expect("the external consumer should execute the public product query");
    let publication = result.receipt().disclosure();
    assert!(!publication.decisions().is_empty());
    let publication_value = publication.decisions()[0].required_disclosure().clone();
    assert!(publication
        .decisions()
        .iter()
        .all(|decision| decision.required_disclosure() == &publication_value));

    assert_equivalent(&capability_value, &disclosure_value);
    assert_equivalent(&capability_value, &publication_value);
    assert_mismatched(
        &capability_value,
        &RestrictedBankField::AuditTrail.into_foundational_value(),
    );

    let version = canonical_version();
    let admitted = admit_canonical_sequence_digest_derivation(
        ready(&capability_value),
        CanonicalSingleSequenceDigestAlgorithmSlot::single_sequence(
            CanonicalDigestAlgorithmId::sha256(),
            CanonicalBasisDomain::Value,
            version,
        ),
    )
    .into_result()
    .expect("the matching single-sequence slot should admit the exact basis");
    let digest = derive_canonical_digest(admitted);
    assert_ne!(digest.value().bytes(), &[0; 32]);
}

fn assert_equivalent(left: &AspectValue, right: &AspectValue) {
    assert!(matches!(
        compare(left, right),
        CanonicalComparisonOutcome::Equivalent(_)
    ));
}

fn assert_mismatched(left: &AspectValue, right: &AspectValue) {
    assert!(matches!(
        compare(left, right),
        CanonicalComparisonOutcome::Mismatched(_)
    ));
}

fn compare(left: &AspectValue, right: &AspectValue) -> CanonicalComparisonOutcome {
    let comparison = prepare_canonical_comparison(
        CanonicalEquivalenceBasis::ExactCanonicalBasis,
        ready(left),
        ready(right),
    )
    .into_result()
    .expect("the same-domain comparison should prepare");
    compare_canonical_basis(&comparison)
}

fn ready(value: &AspectValue) -> CanonicalBasisReadyArtifact {
    let AspectValue::String(value) = value else {
        panic!("the governed field must retain its typed string representation")
    };
    prepare_canonical_basis_sequence(
        canonical_version(),
        CanonicalBasisDomain::Value,
        [CanonicalBasisEntry::new(
            CanonicalBasisDomain::Value,
            CanonicalBasisLocus::Named("governed-disclosure-field".into()),
            CanonicalBasisEntryKind::Value,
            CanonicalBasisValue::ExactText(clone_text(value)),
        )],
    )
    .into_result()
    .expect("the typed field basis should prepare")
}

fn clone_text(value: &InternedString) -> InternedString {
    value.clone()
}

fn canonical_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new(RULE_VERSION).expect("the rule version is static and valid")
}
