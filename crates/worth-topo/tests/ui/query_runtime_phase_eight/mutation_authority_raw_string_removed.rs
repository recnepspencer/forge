use forge_query::facade::ForgeQueryExistingEntityTarget;

fn main() {
    let _target =
        ForgeQueryExistingEntityTarget::new("entity:1:1:1".to_string(), todo!()).expect("target");
}
