//! Owner-identity digests for installing aftermath into portable domain definitions.
//!
//! Portable domain operations author aftermath before package admission. The
//! owner digest is derived from the domain identity declaration through
//! Foundational canonical basis — never from hardcoded byte arrays.

use worth_foundational::facade::{
    canonicalization, prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue, CanonicalDigestAlgorithmId,
    CanonicalDigestId, CanonicalDigestWorkBudget, CanonicalIntegerWidth,
    CanonicalizationRuleVersion,
};

use super::denial::{
    WorthQueryAftermathInstallationDenial, WorthQueryAftermathInstallationDenialKind,
};

const DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("worth-query.aftermath-owner-identity");
const RULE_VERSION: &str = "worth-query-aftermath-owner-identity-v1";
const BUDGET: CanonicalDigestWorkBudget = match CanonicalDigestWorkBudget::new(16, 4 * 1_024) {
    Some(budget) => budget,
    None => panic!("fixed aftermath owner-identity budget is valid"),
};

/// Derive the owner digest bound into an aftermath install for one domain identity.
pub fn aftermath_owner_identity_digest(
    namespace: &str,
    domain_name: &str,
    major: u32,
    minor: u32,
) -> Result<CanonicalDigestId, WorthQueryAftermathInstallationDenial> {
    if namespace.trim().is_empty() || domain_name.trim().is_empty() {
        return Err(WorthQueryAftermathInstallationDenial::new(
            WorthQueryAftermathInstallationDenialKind::CanonicalDigestSlotRejected,
            "empty-aftermath-owner-identity",
        ));
    }
    let version = CanonicalizationRuleVersion::new(RULE_VERSION)
        .expect("the aftermath owner-identity rule is valid");
    let entries = vec![
        entry(
            "family",
            CanonicalBasisValue::ExactText("aftermath-owner".into()),
        ),
        entry(
            "namespace",
            CanonicalBasisValue::ExactText(namespace.to_owned().into()),
        ),
        entry(
            "domain",
            CanonicalBasisValue::ExactText(domain_name.to_owned().into()),
        ),
        entry(
            "major",
            CanonicalBasisValue::UnsignedInteger {
                width: CanonicalIntegerWidth::Bits32,
                value: major.into(),
            },
        ),
        entry(
            "minor",
            CanonicalBasisValue::UnsignedInteger {
                width: CanonicalIntegerWidth::Bits32,
                value: minor.into(),
            },
        ),
    ];
    let basis = prepare_canonical_basis_sequence(version, DOMAIN, entries)
        .into_result()
        .expect("aftermath owner-identity loci are unique and typed");
    let ready = canonicalization()
        .digest()
        .for_sequence_with_budget(basis, CanonicalDigestAlgorithmId::sha256(), BUDGET)
        .into_result()
        .map_err(|_| {
            WorthQueryAftermathInstallationDenial::new(
                WorthQueryAftermathInstallationDenialKind::CanonicalDigestSlotRejected,
                "aftermath-owner-identity",
            )
        })?;
    let derived = canonicalization().digest().derive(ready);
    Ok(CanonicalDigestId::new(*derived.value().bytes()))
}

fn entry(locus: &str, value: CanonicalBasisValue) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        DOMAIN,
        CanonicalBasisLocus::Named(locus.to_owned().into()),
        CanonicalBasisEntryKind::Identity,
        value,
    )
}
