mod algorithm;
mod domain_tokens;
mod sequence;
mod stable_fixture_hash;
mod token_writer;
mod value;

pub(crate) use value::{append_struct_value_material, append_value_material};

pub(super) use domain_tokens::domain_material_token;
pub(super) use stable_fixture_hash::stable_fixture_digest;

use algorithm::append_algorithm_material;
use sequence::{append_bundle_material, append_sequence_material};
use token_writer::append_token;

use super::evidence::{CanonicalDigestDerivationInput, CanonicalDigestInputEvidence};

pub(super) fn canonical_digest_material(input: &CanonicalDigestDerivationInput) -> String {
    let mut material = String::new();
    append_algorithm_material(&mut material, input.algorithm());
    append_input_evidence_material(&mut material, input.evidence());
    material
}

fn append_input_evidence_material(material: &mut String, evidence: &CanonicalDigestInputEvidence) {
    match evidence {
        CanonicalDigestInputEvidence::SingleSequence(sequence) => {
            append_token(material, "input", "single");
            append_sequence_material(material, sequence);
        }
        CanonicalDigestInputEvidence::DomainBundle(bundle) => {
            append_token(material, "input", "domain-bundle");
            append_bundle_material(material, bundle);
        }
        CanonicalDigestInputEvidence::ExportBundle(bundle) => {
            append_token(material, "input", "export-bundle");
            append_bundle_material(material, bundle);
        }
    }
}
