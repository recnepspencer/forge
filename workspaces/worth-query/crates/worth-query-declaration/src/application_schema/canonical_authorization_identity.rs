use sha2::Sha256;

use super::authorization_policy::ApplicationAuthorizationPath;
use super::canonical_identity::hash_field;

pub(super) fn hash_authorization_path(hash: &mut Sha256, path: &ApplicationAuthorizationPath) {
    hash_field(hash, "path-effect", &format!("{:?}", path.effect()));
    hash_field(hash, "path-principal", path.principal_entity());
    hash_field(hash, "path-scope", path.scope_entity());
    for traversal in path.traversals() {
        hash_field(hash, "path-relation", traversal.relation());
        hash_field(hash, "path-from", traversal.from());
        hash_field(hash, "path-to", traversal.to());
        hash_field(
            hash,
            "path-direction",
            &format!("{:?}", traversal.direction()),
        );
    }
    for predicate in path.predicates() {
        hash_field(
            hash,
            "predicate-traversal-ordinal",
            &predicate.traversal_ordinal().to_string(),
        );
        hash_field(hash, "predicate-entity", predicate.entity());
        hash_field(hash, "predicate-aspect", predicate.aspect());
        hash_field(hash, "predicate-field", predicate.field());
        hash_field(
            hash,
            "predicate-value",
            worth_foundational::facade::prepare_aspect_value_identity_basis(predicate.value())
                .as_str(),
        );
    }
}
