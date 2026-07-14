use crate::locators::{
    AspectContractLocator, AspectFieldLocator, AspectLocator, AspectValueLocator,
};

use super::common_entries::{
    aspect_key_entry, concat_locus, field_path_entry, locator_authority_name, locator_text_entry,
};
use crate::canonicalization::CanonicalBasisEntry;

pub(super) fn aspect_locator_entries(
    prefix: &'static str,
    locator: &AspectLocator,
) -> Vec<CanonicalBasisEntry> {
    vec![
        locator_text_entry(concat_locus(prefix, "kind"), "aspect"),
        locator_text_entry(
            concat_locus(prefix, "authority"),
            locator_authority_name(locator.authority()),
        ),
        aspect_key_entry(concat_locus(prefix, "aspect_key"), locator.aspect_key()),
    ]
}

pub(super) fn aspect_field_locator_entries(
    prefix: &'static str,
    locator: &AspectFieldLocator,
) -> Vec<CanonicalBasisEntry> {
    let mut entries = aspect_locator_entries(prefix, locator.aspect());
    entries.push(field_path_entry(
        concat_locus(prefix, "field_path"),
        locator.field_path(),
    ));
    entries
}

pub(super) fn aspect_contract_locator_entries(
    prefix: &'static str,
    locator: &AspectContractLocator,
) -> Vec<CanonicalBasisEntry> {
    vec![
        locator_text_entry(concat_locus(prefix, "kind"), "aspect_contract"),
        aspect_key_entry(concat_locus(prefix, "aspect_key"), locator.aspect_key()),
    ]
}

pub(super) fn value_locator_entries(locator: AspectValueLocator) -> Vec<CanonicalBasisEntry> {
    match locator {
        AspectValueLocator::WholeAspect(locator) => {
            let mut entries = aspect_locator_entries("value.whole_aspect", &locator);
            entries[0] = locator_text_entry("value.kind", "whole_aspect");
            entries
        }
        AspectValueLocator::StructField(locator) => {
            let mut entries = aspect_field_locator_entries("value.struct_field", &locator);
            entries[0] = locator_text_entry("value.kind", "struct_field");
            entries
        }
    }
}
