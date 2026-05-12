use forge_query::facade::{ForgeQueryComputedBuilder, ForgeQueryLiveViewBuilder};

fn main() {
    let _ = ForgeQueryLiveViewBuilder::new("tasks.private");
    let _ = ForgeQueryComputedBuilder::new("tasks.private");
}
