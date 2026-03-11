use crate::transactions::data::{CommitConflict, TransactionIntent};

use super::{
    bulk_create_entities, bulk_create_relations, create_entity, create_relation, delete_entity,
    delete_relation, replace_entity, update_entity,
};
use crate::authority::mutation::{MutationEffect, MutationWorkspace};

pub(crate) fn dispatch_intent(
    intent: &TransactionIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationEffect, CommitConflict> {
    match intent {
        TransactionIntent::CreateEntity(_) => create_entity::apply(intent, workspace),
        TransactionIntent::BulkCreateEntities { .. } => {
            bulk_create_entities::apply(intent, workspace)
        }
        TransactionIntent::UpdateEntity { .. } => update_entity::apply(intent, workspace),
        TransactionIntent::ReplaceEntity { .. } => replace_entity::apply(intent, workspace),
        TransactionIntent::DeleteEntity { .. } => delete_entity::apply(intent, workspace),
        TransactionIntent::CreateRelation(_) => create_relation::apply(intent, workspace),
        TransactionIntent::BulkCreateRelations { .. } => {
            bulk_create_relations::apply(intent, workspace)
        }
        TransactionIntent::DeleteRelation { .. } => delete_relation::apply(intent, workspace),
    }
}
