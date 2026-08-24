use worth_foundational::facade::{prepare_canonical_basis_sequence, CanonicalizationRuleVersion};

use super::canonical_basis::{ApplicationSchemaCanonicalBasis, APPLICATION_SCHEMA_DOMAIN};
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
    let mut canonical = ApplicationSchemaCanonicalBasis::with_member_capacity(members.len());
    canonical.text("header.owner", header.owner);
    canonical.text("header.name", header.name);
    canonical.u32("header.major", header.major);
    canonical.u32("header.minor", header.minor);
    canonical.usize("member-count", members.len());
    for (index, member) in members.iter().enumerate() {
        append_member(&mut canonical, index, member);
    }
    let version =
        CanonicalizationRuleVersion::new(RULE_VERSION).expect("the schema identity rule is valid");
    let basis = prepare_canonical_basis_sequence(
        version,
        APPLICATION_SCHEMA_DOMAIN,
        canonical.into_entries(),
    )
    .into_result()
    .expect("schema identity loci are unique and typed");
    ApplicationSchemaIdentity::from_canonical_basis(basis)
}

mod member;
use member::append_member;

#[cfg(test)]
#[path = "canonical_identity_tests.rs"]
mod tests;
