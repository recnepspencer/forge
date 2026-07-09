use crate::aspects::{AspectKey, CanonicalFieldPath};
use crate::canonicalization::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisValue, CanonicalIntegerWidth,
};
use crate::locators::{BoundaryArtifactField, LocatorAuthority};

pub(super) fn aspect_key_entry(locus: impl Into<String>, key: &AspectKey) -> CanonicalBasisEntry {
    locator_text_entry(locus, key.as_str())
}

pub(super) fn field_path_entry(
    locus: impl Into<String>,
    path: &CanonicalFieldPath,
) -> CanonicalBasisEntry {
    let value = path
        .fields()
        .iter()
        .map(|field| field.as_str())
        .collect::<Vec<_>>()
        .join(".");

    locator_text_entry(locus, value)
}

pub(super) fn locator_text_entry(
    locus: impl Into<String>,
    value: impl Into<String>,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Locator,
        CanonicalBasisLocus::Named(locus.into().into()),
        CanonicalBasisEntryKind::Locator,
        CanonicalBasisValue::ExactText(value.into().into()),
    )
}

pub(super) fn locator_integer_entry(locus: impl Into<String>, value: u128) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Locator,
        CanonicalBasisLocus::Named(locus.into().into()),
        CanonicalBasisEntryKind::Locator,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value,
        },
    )
}

pub(super) fn boundary_artifact_field_name(field: BoundaryArtifactField) -> &'static str {
    match field {
        BoundaryArtifactField::Payload => "payload",
        BoundaryArtifactField::Proofs => "proofs",
        BoundaryArtifactField::Basis => "basis",
    }
}

pub(super) fn locator_authority_name(authority: LocatorAuthority) -> &'static str {
    match authority {
        LocatorAuthority::Authoritative => "authoritative",
        LocatorAuthority::Derived => "derived",
        LocatorAuthority::Projected => "projected",
        LocatorAuthority::SupportOnly => "support_only",
        LocatorAuthority::Planned => "planned",
        LocatorAuthority::ReceiptBearing => "receipt_bearing",
    }
}

pub(super) fn concat_locus(prefix: &str, suffix: &str) -> String {
    format!("{prefix}.{suffix}")
}
