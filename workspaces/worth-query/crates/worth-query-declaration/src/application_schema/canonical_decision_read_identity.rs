use sha2::Sha256;

use super::canonical_identity::hash_field;
use super::ApplicationOperationDecisionReadTarget;

pub(super) fn hash_decision_read_target(
    hash: &mut Sha256,
    target: &ApplicationOperationDecisionReadTarget,
) {
    match target {
        ApplicationOperationDecisionReadTarget::Entity { entity } => {
            hash_field(hash, "read-kind", "entity");
            hash_field(hash, "entity", entity);
        }
        ApplicationOperationDecisionReadTarget::Field {
            entity,
            aspect,
            field,
        } => {
            hash_field(hash, "read-kind", "field");
            hash_field(hash, "entity", entity);
            hash_field(hash, "aspect", aspect);
            hash_field(hash, "field", field);
        }
        ApplicationOperationDecisionReadTarget::Relation { relation, from, to } => {
            hash_field(hash, "read-kind", "relation");
            hash_field(hash, "relation", relation);
            hash_field(hash, "from", from);
            hash_field(hash, "to", to);
        }
    }
}
