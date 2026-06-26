use forge_query::facade::{ForgeQueryMutationDelta, ForgeQueryMutationKind};

fn main() {
    let _delta = ForgeQueryMutationDelta::from_touched_aspects(
        "TopologyEntity",
        "entity:1:1:1".to_string(),
        ForgeQueryMutationKind::Updated,
        vec![],
    );
}
