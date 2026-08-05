use worth_query_host::facade::domain::WorthQueryInstalledApplicationSchema;

struct Schema;

fn assert_clone<T: Clone>() {}

fn main() {
    assert_clone::<WorthQueryInstalledApplicationSchema<Schema>>();
}
