#![deny(deprecated)]

use forge_query::facade::{ForgeQueryCollection, ForgeQueryRuntime};

fn main() {
    let _ = ForgeQueryRuntime::builder()
        .in_memory_collections([ForgeQueryCollection::new("Task", [])]);
}
