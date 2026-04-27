use forge_query::facade::{ForgeQueryCollection, ForgeQueryMemoryApp};

fn main() {
    let _ = ForgeQueryMemoryApp::new([ForgeQueryCollection::new("Task", [])]);
}
