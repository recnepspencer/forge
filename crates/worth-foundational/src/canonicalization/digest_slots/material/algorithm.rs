use super::super::algorithm::CanonicalDigestAlgorithmMetadata;
use super::domain_tokens::{input_domain_token, input_shape_token};
use super::token_writer::append_token;

pub(super) fn append_algorithm_material(
    material: &mut String,
    algorithm: &CanonicalDigestAlgorithmMetadata,
) {
    append_token(material, "algorithm", algorithm.id().as_str());
    append_token(material, "version", algorithm.rule_version().as_str());
    append_token(
        material,
        "shape",
        input_shape_token(algorithm.input_shape()),
    );
    append_token(
        material,
        "domain",
        &input_domain_token(algorithm.input_domain()),
    );
}
