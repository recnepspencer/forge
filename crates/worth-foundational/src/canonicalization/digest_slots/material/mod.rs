mod algorithm;
mod domain_tokens;
mod sequence;
mod sha256_hash;
mod token_writer;
mod value;
mod writer;

pub(crate) use sequence::basis_sequence_material;
pub(crate) use value::{struct_value_material, value_material};

pub(super) use domain_tokens::domain_material_token;
pub(super) use sha256_hash::sha256_digest;

use algorithm::append_algorithm_material;
use sequence::{append_bundle_material, append_sequence_material};
use token_writer::append_token;

use super::algorithm::CanonicalDigestAlgorithmMetadata;
use super::evidence::CanonicalDigestInputEvidence;
use writer::{
    CanonicalEncodedMaterial, CanonicalMaterialByteLimitExceeded, CanonicalMaterialResult,
    CanonicalMaterialWriter,
};

pub(super) fn canonical_digest_material(
    algorithm: &CanonicalDigestAlgorithmMetadata,
    evidence: &CanonicalDigestInputEvidence,
    maximum_encoded_bytes: usize,
) -> Result<CanonicalEncodedMaterial, CanonicalMaterialByteLimitExceeded> {
    let mut material = CanonicalMaterialWriter::bounded(maximum_encoded_bytes);
    append_algorithm_material(&mut material, algorithm)?;
    append_input_evidence_material(&mut material, evidence)?;
    Ok(material.finish())
}

fn append_input_evidence_material(
    material: &mut CanonicalMaterialWriter,
    evidence: &CanonicalDigestInputEvidence,
) -> CanonicalMaterialResult {
    match evidence {
        CanonicalDigestInputEvidence::SingleSequence(sequence) => {
            append_token(material, "input", "single")?;
            append_sequence_material(material, sequence)?;
        }
        CanonicalDigestInputEvidence::DomainBundle(bundle) => {
            append_token(material, "input", "domain-bundle")?;
            append_bundle_material(material, bundle)?;
        }
        CanonicalDigestInputEvidence::ExportBundle(bundle) => {
            append_token(material, "input", "export-bundle")?;
            append_bundle_material(material, bundle)?;
        }
    }
    Ok(())
}
