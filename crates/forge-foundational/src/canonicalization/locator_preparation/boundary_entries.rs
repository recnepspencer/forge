use crate::locators::{BoundaryArtifactLocator, BoundaryMismatchLocator, BoundarySourceLocator};

use super::aspect_entries::{aspect_field_locator_entries, aspect_locator_entries};
use super::common_entries::{
    boundary_artifact_field_name, concat_locus, locator_integer_entry, locator_text_entry,
};
use crate::canonicalization::CanonicalBasisEntry;

pub(super) fn boundary_artifact_locator_entries(
    prefix: &'static str,
    locator: BoundaryArtifactLocator,
) -> Vec<CanonicalBasisEntry> {
    vec![
        locator_text_entry(concat_locus(prefix, "kind"), "boundary_artifact"),
        locator_integer_entry(
            concat_locus(prefix, "artifact_id"),
            u128::from(locator.artifact_id().get()),
        ),
        locator_text_entry(
            concat_locus(prefix, "field"),
            boundary_artifact_field_name(locator.field()),
        ),
    ]
}

pub(super) fn source_locator_entries(locator: BoundarySourceLocator) -> Vec<CanonicalBasisEntry> {
    match locator {
        BoundarySourceLocator::Aspect(locator) => aspect_locator_entries("source.aspect", &locator),
        BoundarySourceLocator::AspectField(locator) => {
            aspect_field_locator_entries("source.aspect_field", &locator)
        }
        BoundarySourceLocator::BoundaryArtifact(locator) => {
            boundary_artifact_locator_entries("source.boundary_artifact", locator)
        }
    }
}

pub(super) fn mismatch_locator_entries(
    locator: BoundaryMismatchLocator,
) -> Vec<CanonicalBasisEntry> {
    match locator {
        BoundaryMismatchLocator::Aspect(locator) => {
            aspect_locator_entries("mismatch.aspect", &locator)
        }
        BoundaryMismatchLocator::AspectField(locator) => {
            aspect_field_locator_entries("mismatch.aspect_field", &locator)
        }
        BoundaryMismatchLocator::BoundaryArtifact(locator) => {
            boundary_artifact_locator_entries("mismatch.boundary_artifact", locator)
        }
    }
}
