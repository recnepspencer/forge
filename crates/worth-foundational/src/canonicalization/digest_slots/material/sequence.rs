use super::super::evidence::{CanonicalDigestBasisBundle, CanonicalDigestBasisSequence};
use super::domain_tokens::domain_material_token;
use super::token_writer::{append_token, append_u32};
use super::value::append_value_material;
use crate::canonicalization::{CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus};
use crate::canonicalization::{
    CanonicalBasisSequence, CanonicalizationCost, CanonicalizationRuleVersion,
};

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
    append_sequence_parts(
        material,
        sequence.version(),
        sequence.domain(),
        sequence.cost(),
        sequence.entries(),
    );
}

pub(crate) fn basis_sequence_material(sequence: &CanonicalBasisSequence) -> String {
    let mut material = String::new();
    append_sequence_parts(
        &mut material,
        sequence.version(),
        sequence.domain(),
        sequence.cost(),
        sequence.entries(),
    );
    material
}

fn append_sequence_parts(
    material: &mut String,
    version: &CanonicalizationRuleVersion,
    domain: crate::canonicalization::CanonicalBasisDomain,
    cost: CanonicalizationCost,
    entries: &[CanonicalBasisEntry],
) {
    append_token(material, "sequence-version", version.as_str());
    append_token(material, "sequence-domain", domain_material_token(domain));
    append_u32(material, "cost.entry-count", cost.entry_count());
    append_u32(
        material,
        "cost.ordering-comparisons",
        cost.ordering_comparisons(),
    );
    append_u32(
        material,
        "cost.nested-sequence-count",
        cost.nested_sequence_count(),
    );
    append_u32(
        material,
        "cost.compatibility-lowering-count",
        cost.compatibility_lowering_count(),
    );
    for entry in entries {
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
        CanonicalBasisEntryKind::PerformanceClaim => "performance-claim",
        CanonicalBasisEntryKind::PerformanceLayout => "performance-layout",
        CanonicalBasisEntryKind::PerformanceCounter => "performance-counter",
        CanonicalBasisEntryKind::PerformanceSupport => "performance-support",
        CanonicalBasisEntryKind::BoundaryArtifact => "boundary-artifact",
        CanonicalBasisEntryKind::BoundaryAttachment => "boundary-attachment",
        CanonicalBasisEntryKind::TransitionArtifact => "transition-artifact",
        CanonicalBasisEntryKind::TransitionLocator => "transition-locator",
        CanonicalBasisEntryKind::DiagnosticBundle => "diagnostic-bundle",
        CanonicalBasisEntryKind::DiagnosticRow => "diagnostic-row",
        CanonicalBasisEntryKind::DiagnosticGap => "diagnostic-gap",
        CanonicalBasisEntryKind::CompatibilityOrigin => "compatibility-origin",
        CanonicalBasisEntryKind::Cost => "cost",
        CanonicalBasisEntryKind::Future(value) => value,
    }
}
