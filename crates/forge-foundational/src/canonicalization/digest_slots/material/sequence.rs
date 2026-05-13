use super::super::evidence::{CanonicalDigestBasisBundle, CanonicalDigestBasisSequence};
use super::domain_tokens::domain_material_token;
use super::token_writer::{append_token, append_u32};
use super::value::append_value_material;
use crate::canonicalization::{CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus};

pub(super) fn append_bundle_material(material: &mut String, bundle: &CanonicalDigestBasisBundle) {
    append_token(material, "bundle-version", bundle.version().as_str());
    for sequence in bundle.sequences() {
        append_sequence_material(material, sequence);
    }
}

pub(super) fn append_sequence_material(
    material: &mut String,
    sequence: &CanonicalDigestBasisSequence,
) {
    append_token(material, "sequence-version", sequence.version().as_str());
    append_token(
        material,
        "sequence-domain",
        domain_material_token(sequence.domain()),
    );
    append_u32(material, "cost.entry-count", sequence.cost().entry_count());
    append_u32(
        material,
        "cost.ordering-comparisons",
        sequence.cost().ordering_comparisons(),
    );
    append_u32(
        material,
        "cost.nested-sequence-count",
        sequence.cost().nested_sequence_count(),
    );
    append_u32(
        material,
        "cost.compatibility-lowering-count",
        sequence.cost().compatibility_lowering_count(),
    );
    for entry in sequence.entries() {
        append_entry_material(material, entry);
    }
}

fn append_entry_material(material: &mut String, entry: &CanonicalBasisEntry) {
    append_token(
        material,
        "entry.domain",
        domain_material_token(entry.domain()),
    );
    append_locus_material(material, entry.locus());
    append_token(material, "entry.kind", entry_kind_token(entry.kind()));
    append_value_material(material, entry.value());
}

fn append_locus_material(material: &mut String, locus: &CanonicalBasisLocus) {
    match locus {
        CanonicalBasisLocus::Root => append_token(material, "locus", "root"),
        CanonicalBasisLocus::EntryOrdinal(ordinal) => {
            append_u32(material, "locus.ordinal", *ordinal);
        }
        CanonicalBasisLocus::Aspect(aspect) => {
            append_token(material, "locus.aspect", aspect.as_str());
        }
        CanonicalBasisLocus::AspectField { aspect, path } => {
            append_token(material, "locus.aspect-field.aspect", aspect.as_str());
            append_u32(
                material,
                "locus.aspect-field.len",
                path.fields().len() as u32,
            );
            for field in path.fields() {
                append_token(material, "locus.aspect-field.field", field.as_str());
            }
        }
        CanonicalBasisLocus::Named(value) => {
            append_token(material, "locus.named.kind", "named");
            super::value::append_interned_string(material, "locus.named", value);
        }
    }
}

fn entry_kind_token(kind: CanonicalBasisEntryKind) -> &'static str {
    match kind {
        CanonicalBasisEntryKind::Header => "header",
        CanonicalBasisEntryKind::Shape => "shape",
        CanonicalBasisEntryKind::Value => "value",
        CanonicalBasisEntryKind::Field => "field",
        CanonicalBasisEntryKind::Mask => "mask",
        CanonicalBasisEntryKind::StateAspect => "state-aspect",
        CanonicalBasisEntryKind::PatchOperation => "patch-operation",
        CanonicalBasisEntryKind::Identity => "identity",
        CanonicalBasisEntryKind::Locator => "locator",
        CanonicalBasisEntryKind::Profile => "profile",
        CanonicalBasisEntryKind::CompatibilityOrigin => "compatibility-origin",
        CanonicalBasisEntryKind::Cost => "cost",
        CanonicalBasisEntryKind::Future(value) => value,
    }
}
