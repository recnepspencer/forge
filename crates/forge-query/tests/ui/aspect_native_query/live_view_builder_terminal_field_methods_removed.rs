use forge_query::facade::{ForgeQueryLiveViewBuilder, ForgeQueryNativeRow};

fn main() {
    let _ = ForgeQueryLiveViewBuilder::surface("tasks")
        .from("Task")
        .select(["identity.id", "title.value"])
        .order_by("identity.id")
        .build();
    let _: Option<ForgeQueryNativeRow> = None;
}
