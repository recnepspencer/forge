use worth_foundational::facade::{
    AspectContractRevision, AspectIdentity, CanonicalBasisLocus, CanonicalBasisValue,
    CanonicalIntegerWidth, InternedString,
};

use super::{ApplicationSchemaDeclarationBuilder, ApplicationSchemaMember};

crate::worth_query_application_schema! {
    schema StableAspectSchema {
        owner: "worth.test",
        version: (1, 0),
        members: |schema| {
            schema
                .entity(Account::reference())
                .aspect(Account::reference(), AccountFacts::reference())
                .field(Account::reference(), Balance::reference())
        }
    }
}

crate::worth_query_entity!(Account in StableAspectSchema);
crate::worth_query_aspect!(
    AccountFacts in StableAspectSchema, Account;
    identity = AspectIdentity(0x9161_1f01),
    revision = AspectContractRevision(2),
);
crate::worth_query_field!(
    Balance in StableAspectSchema, Account, AccountFacts: u64, read_only, equality
);

#[test]
fn authored_identity_and_revision_survive_reference_and_erasure() {
    let reference = AccountFacts::reference();
    assert_eq!(reference.identity(), AspectIdentity(0x9161_1f01));
    assert_eq!(reference.revision(), AspectContractRevision(2));

    let declaration = StableAspectSchema::declaration().unwrap();
    let aspect = declaration
        .erased()
        .members()
        .iter()
        .find(|member| matches!(member, ApplicationSchemaMember::Aspect { .. }))
        .expect("the authored aspect is retained");
    assert!(matches!(
        aspect,
        ApplicationSchemaMember::Aspect {
            identity: AspectIdentity(0x9161_1f01),
            revision: AspectContractRevision(2),
            ..
        }
    ));
}

#[test]
fn canonical_schema_v11_encodes_identity_then_revision_as_u64() {
    let declaration = StableAspectSchema::declaration().unwrap();
    let sequence = declaration.identity().canonical_basis().payload();
    assert_eq!(
        sequence.version().as_str(),
        "worth-query-application-schema-v11"
    );
    let entries = sequence.entries();
    let identity = canonical_entry(entries, ".identity");
    let revision = canonical_entry(entries, ".revision");
    assert!(identity.0 < revision.0);
    assert_eq!(
        identity.1,
        &CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: 0x9161_1f01,
        }
    );
    assert_eq!(
        revision.1,
        &CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: 2,
        }
    );
}

#[test]
fn aspect_member_order_is_canonical() {
    let first = ApplicationSchemaDeclarationBuilder::<StableAspectSchema>::for_schema()
        .entity(Account::reference())
        .aspect(Account::reference(), AccountFacts::reference())
        .field(Account::reference(), Balance::reference())
        .build()
        .unwrap();
    let reordered = ApplicationSchemaDeclarationBuilder::<StableAspectSchema>::for_schema()
        .field(Account::reference(), Balance::reference())
        .aspect(Account::reference(), AccountFacts::reference())
        .entity(Account::reference())
        .build()
        .unwrap();
    assert_eq!(first.identity(), reordered.identity());
}

fn canonical_entry<'a>(
    entries: &'a [worth_foundational::facade::CanonicalBasisEntry],
    suffix: &str,
) -> (usize, &'a CanonicalBasisValue) {
    entries
        .iter()
        .enumerate()
        .find_map(|(index, entry)| match entry.locus() {
            CanonicalBasisLocus::Named(InternedString::Raw(name)) if name.ends_with(suffix) => {
                Some((index, entry.value()))
            }
            _ => None,
        })
        .expect("the aspect canonical component is retained")
}
