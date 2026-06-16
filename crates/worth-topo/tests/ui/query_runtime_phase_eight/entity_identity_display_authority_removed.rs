use forge_query::facade::ForgeQueryExistingEntityTarget;
use forge_relational::facade::identity::EntityId;

fn main() {
    let entity_id = EntityId::new(1, 1, 1);
    let _target = ForgeQueryExistingEntityTarget::new(format!("{entity_id:?}"), todo!())
        .expect("target");
}
