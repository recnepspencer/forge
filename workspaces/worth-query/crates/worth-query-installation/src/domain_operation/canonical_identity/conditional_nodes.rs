use sha2::Sha256;

use crate::canonical_hash_encoding::hash_text_field;
use crate::domain_operation::WorthQueryPortableConditionalNodeDeclaration;

pub(super) fn hash_conditional_nodes(
    hasher: &mut Sha256,
    nodes: &[WorthQueryPortableConditionalNodeDeclaration],
    tag: &'static str,
) {
    if nodes.is_empty() {
        hash_text_field(hasher, tag, "not-required");
        return;
    }
    for node in nodes {
        hash_text_field(hasher, tag, &node.canonical_token());
    }
}
