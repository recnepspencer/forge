use sha2::Sha256;

use super::canonical_identity::hash_field;
use super::ApplicationOperationProgramTarget;

pub(super) fn hash_operation_target(hash: &mut Sha256, target: &ApplicationOperationProgramTarget) {
    match target {
        ApplicationOperationProgramTarget::Create { entity } => {
            hash_field(hash, "program-action", "create");
            hash_field(hash, "entity", entity);
        }
        ApplicationOperationProgramTarget::Delete { entity } => {
            hash_field(hash, "program-action", "delete");
            hash_field(hash, "entity", entity);
        }
        ApplicationOperationProgramTarget::Write {
            entity,
            aspect,
            field,
        } => {
            hash_field(hash, "program-action", "write");
            hash_field(hash, "entity", entity);
            hash_field(hash, "aspect", aspect);
            hash_field(hash, "field", field);
        }
        ApplicationOperationProgramTarget::Link { relation, from, to } => {
            hash_field(hash, "program-action", "link");
            hash_field(hash, "relation", relation);
            hash_field(hash, "from", from);
            hash_field(hash, "to", to);
        }
        ApplicationOperationProgramTarget::Unlink { relation, from, to } => {
            hash_field(hash, "program-action", "unlink");
            hash_field(hash, "relation", relation);
            hash_field(hash, "from", from);
            hash_field(hash, "to", to);
        }
        ApplicationOperationProgramTarget::Emit { effect } => {
            hash_field(hash, "program-action", "emit");
            hash_field(hash, "effect", effect);
        }
    }
}
