use crate::transactions::data::{
    CommitConflict, CreateIntent, EntityMutationIntent, MutationIntent, RelationMutationIntent,
};

use super::{
    bulk_create_entities, bulk_create_relations, create_entity, create_relation, delete_entity,
    delete_relation, replace_entity, update_entity, update_entity_fields,
};
use crate::authority::mutation::outcomes::MutationOutcome;
use crate::authority::mutation::MutationWorkspace;

pub(crate) fn dispatch_intent(
    intent: &MutationIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    match intent {
        MutationIntent::Create(CreateIntent::Entity(spec)) => create_entity::apply(spec, workspace),
        MutationIntent::Create(CreateIntent::BulkEntities(spec)) => {
            bulk_create_entities::apply(spec, workspace)
        }
        MutationIntent::Entity(EntityMutationIntent::Update(spec)) => {
            update_entity::apply(spec, workspace)
        }
        MutationIntent::Entity(EntityMutationIntent::UpdateFields(spec)) => {
            update_entity_fields::apply(spec, workspace)
        }
        MutationIntent::Entity(EntityMutationIntent::Replace(spec)) => {
            replace_entity::apply(spec, workspace)
        }
        MutationIntent::Entity(EntityMutationIntent::Delete(spec)) => {
            delete_entity::apply(spec, workspace)
        }
        MutationIntent::Create(CreateIntent::Relation(spec)) => {
            create_relation::apply(spec, workspace)
        }
        MutationIntent::Create(CreateIntent::BulkRelations(spec)) => {
            bulk_create_relations::apply(spec, workspace)
        }
        MutationIntent::Relation(RelationMutationIntent::Delete(spec)) => {
            delete_relation::apply(spec, workspace)
        }
    }
}
