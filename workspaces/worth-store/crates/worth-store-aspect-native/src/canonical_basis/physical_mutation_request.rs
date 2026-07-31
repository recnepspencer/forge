use worth_foundational::canonicalization_api::lower_lane::basis::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisValue, CanonicalIntegerWidth,
};
use worth_foundational::{CanonicalDigestId, InternedString};

const DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("store.physical.mutation.request-fingerprint.v1");
const FIELD: CanonicalBasisEntryKind =
    CanonicalBasisEntryKind::Future("store-physical-mutation-request-field");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorePhysicalMutationRequestCanonicalSource {
    store: [u8; 16],
    durability_policy: [u8; 32],
    scope_family: u8,
    scope_identity: [u8; 32],
    payload: [u8; 32],
    durability_request: u8,
    operation_family: u8,
    security_bases: Box<[[u8; 32]]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorePhysicalMutationRequestCanonicalFields {
    pub store: [u8; 16],
    pub durability_policy: [u8; 32],
    pub scope_family: u8,
    pub scope_identity: [u8; 32],
    pub payload: [u8; 32],
    pub durability_request: u8,
    pub operation_family: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorePhysicalMutationRequestBasisDenial {
    MissingSecurityBasis,
    DuplicateSecurityBasis,
}

impl StorePhysicalMutationRequestCanonicalSource {
    pub fn new(
        fields: StorePhysicalMutationRequestCanonicalFields,
        security_bases: impl IntoIterator<Item = [u8; 32]>,
    ) -> Result<Self, StorePhysicalMutationRequestBasisDenial> {
        let mut security_bases = security_bases.into_iter().collect::<Vec<_>>();
        if security_bases.is_empty() {
            return Err(StorePhysicalMutationRequestBasisDenial::MissingSecurityBasis);
        }
        security_bases.sort_unstable();
        if security_bases.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(StorePhysicalMutationRequestBasisDenial::DuplicateSecurityBasis);
        }
        Ok(Self {
            store: fields.store,
            durability_policy: fields.durability_policy,
            scope_family: fields.scope_family,
            scope_identity: fields.scope_identity,
            payload: fields.payload,
            durability_request: fields.durability_request,
            operation_family: fields.operation_family,
            security_bases: security_bases.into_boxed_slice(),
        })
    }

    pub fn into_canonical_entries(self) -> Vec<CanonicalBasisEntry> {
        let mut entries = Vec::with_capacity(8 + self.security_bases.len());
        entries.push(entry(
            "000.store",
            CanonicalBasisValue::UuidBytes(self.store),
        ));
        entries.push(digest_entry(
            "001.durability-policy",
            self.durability_policy,
        ));
        entries.push(u8_entry("002.scope-family", self.scope_family));
        entries.push(digest_entry("003.scope-identity", self.scope_identity));
        entries.push(digest_entry("004.payload", self.payload));
        entries.push(u8_entry("005.durability-request", self.durability_request));
        entries.push(u8_entry("006.operation-family", self.operation_family));
        entries.push(u32_entry(
            "007.security-basis-count",
            self.security_bases.len() as u32,
        ));
        entries.extend(self.security_bases.into_vec().into_iter().enumerate().map(
            |(ordinal, basis)| digest_entry(format!("008.security-basis.{ordinal:04}"), basis),
        ));
        entries
    }
}

fn digest_entry(locus: impl Into<InternedString>, digest: [u8; 32]) -> CanonicalBasisEntry {
    entry(
        locus,
        CanonicalBasisValue::BytesDigest(CanonicalDigestId::new(digest)),
    )
}

fn u8_entry(locus: impl Into<InternedString>, value: u8) -> CanonicalBasisEntry {
    entry(
        locus,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits8,
            value: u128::from(value),
        },
    )
}

fn u32_entry(locus: impl Into<InternedString>, value: u32) -> CanonicalBasisEntry {
    entry(
        locus,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits32,
            value: u128::from(value),
        },
    )
}

fn entry(locus: impl Into<InternedString>, value: CanonicalBasisValue) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        DOMAIN,
        CanonicalBasisLocus::Named(locus.into()),
        FIELD,
        value,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIELDS: StorePhysicalMutationRequestCanonicalFields =
        StorePhysicalMutationRequestCanonicalFields {
            store: [1; 16],
            durability_policy: [2; 32],
            scope_family: 3,
            scope_identity: [4; 32],
            payload: [5; 32],
            durability_request: 6,
            operation_family: 7,
        };

    #[test]
    fn request_source_requires_unique_security_bases_and_freezes_field_order() {
        assert_eq!(
            StorePhysicalMutationRequestCanonicalSource::new(FIELDS, []),
            Err(StorePhysicalMutationRequestBasisDenial::MissingSecurityBasis)
        );
        assert_eq!(
            StorePhysicalMutationRequestCanonicalSource::new(FIELDS, [[8; 32], [8; 32]],),
            Err(StorePhysicalMutationRequestBasisDenial::DuplicateSecurityBasis)
        );
        let source =
            StorePhysicalMutationRequestCanonicalSource::new(FIELDS, [[9; 32], [8; 32]]).unwrap();
        let entries = source.into_canonical_entries();
        assert_eq!(entries.len(), 10);
        assert!(matches!(
            entries[8].value(),
            CanonicalBasisValue::BytesDigest(value) if value.bytes() == &[8; 32]
        ));
    }
}
