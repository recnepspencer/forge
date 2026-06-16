use forge_query::facade::ForgeQueryExistingRelationTarget;
use forge_relational::facade::identity::RelationId;

fn main() {
    let relation_id = RelationId::new(1, 1, 1);
    let _target = ForgeQueryExistingRelationTarget::new(
        format!("{relation_id:?}"),
        todo!(),
    )
    .expect("target");
}
