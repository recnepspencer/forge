use forge_query::facade::{ForgeQueryMutationDelta, ForgeQueryMutationKind};

fn main() {
    let _delta = ForgeQueryMutationDelta::new(
        "TopologyEntity",
        "entity:1:1:1".to_string(),
        ForgeQueryMutationKind::Updated,
        vec![],
    );
}
