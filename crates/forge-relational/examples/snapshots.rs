mod support;

use forge_relational::facade::runtime::RelationalRuntimeApi;

fn main() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(support::demo_schema_registry())
        .build();

    let (_created, entity_id) = support::create_entity(&mut runtime, "first");
    let snapshot = runtime.snapshots().snapshot();
    let _updated = support::update_entity(&mut runtime, entity_id, "first-updated");

    let read_path = runtime
        .read_truth()
        .inspect_snapshot_read_path(&snapshot)
        .expect("snapshot read path");
    let snap_read = runtime
        .read_truth()
        .read_snapshot(&snapshot)
        .expect("snapshot read");

    let authoritative_aspect_state = snap_read
        .get_entity(entity_id)
        .expect("snapshot entity")
        .authoritative_aspect_state
        .clone();
    println!("snapshot diagnostics entries={}", read_path.entries.len());
    println!("snapshot preserved authoritative aspect state={authoritative_aspect_state:?}");

    assert!(runtime.snapshots().release_snapshot(&snapshot));
}
