use worth_query::facade::runtime::{WorthQueryLiveViewBuilder, WorthQueryUnrefinedLiveShape};

fn main() {
    let _ = WorthQueryLiveViewBuilder::surface("tasks")
        .from("Task")
        .select(["identity.id", "title.value"])
        .order_by("identity.id")
        .build();
    let _: Option<WorthQueryUnrefinedLiveShape> = None;
}
