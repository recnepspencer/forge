use forge_query::facade::ForgeQueryMutationBatchBuilder;
use forge_relational::facade::identity::RelationId;

fn main() {
    let relation_id = RelationId::new(1, 1, 1);
    let _builder = ForgeQueryMutationBatchBuilder::new().retarget_existing_verified(
        todo!(),
        |_| todo!(),
        |update| {
            update.continuity_rebind_existing_target(
                format!("{relation_id:?}"),
                format!("{relation_id:?}:successor"),
            )
        },
    );
}
