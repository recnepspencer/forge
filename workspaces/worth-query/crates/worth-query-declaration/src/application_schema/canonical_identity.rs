use worth_foundational::facade::{prepare_canonical_basis_sequence, CanonicalizationRuleVersion};

use super::canonical_basis::{
    ApplicationSchemaCanonicalBasis, ApplicationSchemaCanonicalBasisBudgetDenial,
    ApplicationSchemaCanonicalBasisWork, APPLICATION_SCHEMA_DOMAIN,
};
use super::{ApplicationSchemaIdentity, ApplicationSchemaMember};

const RULE_VERSION: &str = "worth-query-application-schema-v11";

pub(super) struct ApplicationSchemaCanonicalHeader<'a> {
    pub owner: &'a str,
    pub name: &'a str,
    pub major: u32,
    pub minor: u32,
}

pub(super) fn canonical_identity(
    header: ApplicationSchemaCanonicalHeader<'_>,
    members: &[ApplicationSchemaMember],
) -> ApplicationSchemaIdentity {
    canonical_identity_with_limits(header, members, u64::MAX, u64::MAX)
        .expect("ordinary schema construction uses an unbounded source observation")
        .0
}

pub(super) fn canonical_identity_with_limits(
    header: ApplicationSchemaCanonicalHeader<'_>,
    members: &[ApplicationSchemaMember],
    maximum_source_bytes: u64,
    maximum_entries: u64,
) -> Result<
    (
        ApplicationSchemaIdentity,
        ApplicationSchemaCanonicalBasisWork,
    ),
    ApplicationSchemaCanonicalBasisBudgetDenial,
> {
    let mut canonical = ApplicationSchemaCanonicalBasis::with_member_capacity_and_limits(
        members.len(),
        maximum_source_bytes,
        maximum_entries,
    );
    canonical.text("header.owner", header.owner);
    canonical.text("header.name", header.name);
    canonical.u32("header.major", header.major);
    canonical.u32("header.minor", header.minor);
    canonical.usize("member-count", members.len());
    for (index, member) in members.iter().enumerate() {
        append_member(&mut canonical, index, member);
        if canonical.is_denied() {
            break;
        }
    }
    let version =
        CanonicalizationRuleVersion::new(RULE_VERSION).expect("the schema identity rule is valid");
    let (entries, work) = canonical.into_entries()?;
    let basis = prepare_canonical_basis_sequence(version, APPLICATION_SCHEMA_DOMAIN, entries)
        .into_result()
        .expect("schema identity loci are unique and typed");
    Ok((ApplicationSchemaIdentity::from_canonical_basis(basis), work))
}

mod member;
use member::append_member;

#[cfg(test)]
#[path = "canonical_identity_tests.rs"]
mod tests;
