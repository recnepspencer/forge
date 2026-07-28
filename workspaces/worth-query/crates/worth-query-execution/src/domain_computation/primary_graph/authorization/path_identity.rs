use sha2::{Digest, Sha256};
use worth_query_installation::facade::{
    ApplicationAuthorizationPath, ApplicationAuthorizationPathEffect,
    ApplicationAuthorizationTraversalDirection,
};

pub(super) fn authorization_path_identity(path: &ApplicationAuthorizationPath) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash_text(&mut hash, "worth-query.authorization-path.v1");
    hash.update([match path.effect() {
        ApplicationAuthorizationPathEffect::Allow => 1,
        ApplicationAuthorizationPathEffect::Deny => 2,
    }]);
    hash_text(&mut hash, path.principal_entity());
    hash_text(&mut hash, path.scope_entity());
    hash.update(path.traversals().len().to_le_bytes());
    for traversal in path.traversals() {
        hash_text(&mut hash, traversal.relation());
        hash_text(&mut hash, traversal.from());
        hash_text(&mut hash, traversal.to());
        hash.update([match traversal.direction() {
            ApplicationAuthorizationTraversalDirection::Forward => 1,
            ApplicationAuthorizationTraversalDirection::Reverse => 2,
        }]);
    }
    hash.update(path.predicates().len().to_le_bytes());
    for predicate in path.predicates() {
        hash.update(predicate.traversal_ordinal().to_le_bytes());
        hash_text(&mut hash, predicate.entity());
        hash_text(&mut hash, predicate.aspect());
        hash_text(&mut hash, predicate.field());
        hash_text(
            &mut hash,
            worth_foundational::facade::prepare_aspect_value_identity_basis(predicate.value())
                .as_str(),
        );
    }
    hash.finalize().into()
}

fn hash_text(hash: &mut Sha256, value: &str) {
    hash.update(value.len().to_le_bytes());
    hash.update(value.as_bytes());
}
