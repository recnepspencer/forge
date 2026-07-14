use crate::aspects::{AspectKey, CanonicalFieldPath, DiagnosticMask, MutationMask, ProjectionMask};
use crate::canonicalization::CanonicalBasisEntry;
use crate::locators::{AspectMaskLocator, LocatorAuthority};

use super::common_entries::{
    aspect_key_entry, concat_locus, field_path_entry, locator_authority_name, locator_text_entry,
};

pub fn projection_mask_locator_canonical_basis_entries(
    locator: &AspectMaskLocator<ProjectionMask>,
) -> Vec<CanonicalBasisEntry> {
    mask_locator_entries(
        "projection_mask",
        locator.authority(),
        locator.aspect_key(),
        locator.paths(),
    )
}

pub fn mutation_mask_locator_canonical_basis_entries(
    locator: &AspectMaskLocator<MutationMask>,
) -> Vec<CanonicalBasisEntry> {
    mask_locator_entries(
        "mutation_mask",
        locator.authority(),
        locator.aspect_key(),
        locator.paths(),
    )
}

pub fn diagnostic_mask_locator_canonical_basis_entries(
    locator: &AspectMaskLocator<DiagnosticMask>,
) -> Vec<CanonicalBasisEntry> {
    mask_locator_entries(
        "diagnostic_mask",
        locator.authority(),
        locator.aspect_key(),
        locator.paths(),
    )
}

fn mask_locator_entries(
    prefix: &'static str,
    authority: LocatorAuthority,
    aspect_key: &AspectKey,
    paths: &[CanonicalFieldPath],
) -> Vec<CanonicalBasisEntry> {
    let mut entries = vec![
        locator_text_entry(concat_locus(prefix, "kind"), prefix),
        locator_text_entry(
            concat_locus(prefix, "authority"),
            locator_authority_name(authority),
        ),
        aspect_key_entry(concat_locus(prefix, "aspect_key"), aspect_key),
    ];

    entries.extend(
        paths
            .iter()
            .enumerate()
            .map(|(index, path)| field_path_entry(format!("{prefix}.path.{index}"), path)),
    );
    entries
}
