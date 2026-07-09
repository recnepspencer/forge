use worth_query::facade::{WorthQueryComputedBuilder, WorthQueryLiveViewBuilder};

fn main() {
    let _ = WorthQueryLiveViewBuilder::new("tasks.private");
    let _ = WorthQueryComputedBuilder::new("tasks.private");
}
