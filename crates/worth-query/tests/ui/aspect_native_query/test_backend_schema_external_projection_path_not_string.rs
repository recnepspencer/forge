use worth_query::WorthQueryTestBackendSchema;

fn main() {
    let schema = WorthQueryTestBackendSchema::single_collection("Task")
        .aspect("title.value", "title.value")
        .unwrap();

    for (aspect_touch, native_field_path) in schema.aspects() {
        let _: &str = aspect_touch;
        let _: &str = native_field_path;
    }
}
