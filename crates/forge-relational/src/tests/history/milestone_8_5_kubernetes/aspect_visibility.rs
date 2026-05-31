use super::*;

pub(super) fn replicas_canonical_bytes(
    record: &crate::storage::data::EntityReadRecord,
) -> Option<Vec<u8>> {
    let locator = AspectFieldLocator::new(
        LocatorAuthority::Planned,
        AspectKey::new("replicas").expect("valid replicas aspect"),
        CanonicalFieldPath::single(FieldKey::new("replicas").expect("valid replicas field")),
    );
    crate::visibility::materialization::read_records::entity_query_locus_comparison_key(
        record, &locator,
    )
    .map(|key| key.canonical_value_bytes().to_vec())
}
