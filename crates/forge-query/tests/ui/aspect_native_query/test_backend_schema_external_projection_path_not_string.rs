use forge_query::ForgeQueryTestBackendSchema;

fn main() {
    let schema = ForgeQueryTestBackendSchema::single_collection("Task")
        .aspect("title.value", "title.value")
        .unwrap();

    for (_, external_projection_path) in schema.aspects() {
        let _: &str = external_projection_path;
    }
}
