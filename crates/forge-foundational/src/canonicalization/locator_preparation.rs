use forge_proof::TransitionOutcome;

use crate::aspects::{AspectKey, CanonicalFieldPath};
use crate::locators::{
    AspectContractLocator, AspectFieldLocator, AspectLocator, AspectMaskLocator,
    BoundaryArtifactField, BoundaryArtifactLocator, BoundaryMismatchLocator, BoundarySourceLocator,
    LocatorAuthority,
};

use super::{
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact, CanonicalBasisValue,
    CanonicalIntegerWidth, CanonicalizationRuleVersion,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalLocatorInput {
    BoundaryArtifact(BoundaryArtifactLocator),
    Aspect(AspectLocator),
    AspectField(AspectFieldLocator),
    AspectContract(AspectContractLocator),
    Source(BoundarySourceLocator),
    Mismatch(BoundaryMismatchLocator),
}

pub fn prepare_locator_for_canonical_basis(
    version: CanonicalizationRuleVersion,
    locator: CanonicalLocatorInput,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, super::CanonicalBasisConstructionDenial> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Locator,
        canonical_locator_entries(locator),
    )
}

pub fn locator_canonical_basis_entries(
    ready: &CanonicalBasisReadyArtifact,
) -> &[CanonicalBasisEntry] {
    ready.payload().entries()
}

fn canonical_locator_entries(locator: CanonicalLocatorInput) -> Vec<CanonicalBasisEntry> {
    match locator {
        CanonicalLocatorInput::BoundaryArtifact(locator) => {
            boundary_artifact_locator_entries("boundary_artifact", locator)
        }
        CanonicalLocatorInput::Aspect(locator) => aspect_locator_entries("aspect", &locator),
        CanonicalLocatorInput::AspectField(locator) => {
            aspect_field_locator_entries("aspect_field", &locator)
        }
        CanonicalLocatorInput::AspectContract(locator) => {
            vec![
                locator_text_entry("aspect_contract.kind", "aspect_contract"),
                aspect_key_entry("aspect_contract.aspect_key", locator.aspect_key()),
            ]
        }
        CanonicalLocatorInput::Source(locator) => match locator {
            BoundarySourceLocator::Aspect(locator) => {
                aspect_locator_entries("source.aspect", &locator)
            }
            BoundarySourceLocator::AspectField(locator) => {
                aspect_field_locator_entries("source.aspect_field", &locator)
            }
            BoundarySourceLocator::BoundaryArtifact(locator) => {
                boundary_artifact_locator_entries("source.boundary_artifact", locator)
            }
        },
        CanonicalLocatorInput::Mismatch(locator) => match locator {
            BoundaryMismatchLocator::Aspect(locator) => {
                aspect_locator_entries("mismatch.aspect", &locator)
            }
            BoundaryMismatchLocator::AspectField(locator) => {
                aspect_field_locator_entries("mismatch.aspect_field", &locator)
            }
            BoundaryMismatchLocator::BoundaryArtifact(locator) => {
                boundary_artifact_locator_entries("mismatch.boundary_artifact", locator)
            }
        },
    }
}

fn boundary_artifact_locator_entries(
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

fn aspect_locator_entries(
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

fn aspect_field_locator_entries(
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

fn aspect_key_entry(locus: impl Into<String>, key: &AspectKey) -> CanonicalBasisEntry {
    locator_text_entry(locus, key.as_str())
}

fn field_path_entry(locus: impl Into<String>, path: &CanonicalFieldPath) -> CanonicalBasisEntry {
    let value = path
        .fields()
        .iter()
        .map(|field| field.as_str())
        .collect::<Vec<_>>()
        .join(".");

    locator_text_entry(locus, value)
}

fn locator_text_entry(locus: impl Into<String>, value: impl Into<String>) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Locator,
        CanonicalBasisLocus::Named(locus.into().into()),
        CanonicalBasisEntryKind::Locator,
        CanonicalBasisValue::ExactText(value.into().into()),
    )
}

fn locator_integer_entry(locus: impl Into<String>, value: u128) -> CanonicalBasisEntry {
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

fn boundary_artifact_field_name(field: BoundaryArtifactField) -> &'static str {
    match field {
        BoundaryArtifactField::Payload => "payload",
        BoundaryArtifactField::Proofs => "proofs",
        BoundaryArtifactField::Basis => "basis",
    }
}

fn locator_authority_name(authority: LocatorAuthority) -> &'static str {
    match authority {
        LocatorAuthority::Authoritative => "authoritative",
        LocatorAuthority::Derived => "derived",
        LocatorAuthority::Projected => "projected",
        LocatorAuthority::SupportOnly => "support_only",
        LocatorAuthority::Planned => "planned",
        LocatorAuthority::ReceiptBearing => "receipt_bearing",
    }
}

fn concat_locus(prefix: &str, suffix: &str) -> String {
    format!("{prefix}.{suffix}")
}

pub fn projection_mask_locator_canonical_basis_entries(
    locator: &AspectMaskLocator<crate::aspects::ProjectionMask>,
) -> Vec<CanonicalBasisEntry> {
    mask_locator_entries(
        "projection_mask",
        locator.authority(),
        locator.aspect_key(),
        locator.paths(),
    )
}

pub fn mutation_mask_locator_canonical_basis_entries(
    locator: &AspectMaskLocator<crate::aspects::MutationMask>,
) -> Vec<CanonicalBasisEntry> {
    mask_locator_entries(
        "mutation_mask",
        locator.authority(),
        locator.aspect_key(),
        locator.paths(),
    )
}

pub fn diagnostic_mask_locator_canonical_basis_entries(
    locator: &AspectMaskLocator<crate::aspects::DiagnosticMask>,
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
