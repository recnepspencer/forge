use serde_json::json;

use super::child_process::{run_courtroom_reopener, run_courtroom_writer};
use super::courtroom_evidence_support::{
    hex, identity_set_digest, inline_placement, locator_identities, offline_completion,
    placement_identities, reopener_completion, segment_files, segment_page_bytes,
    writer_completion,
};
use super::scenario_evidence::{ScenarioEvidence, ScenarioPredicate, ScenarioProcessEvidence};

#[test]
fn record_world_survives_fresh_processes() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let locators = parent.path().join("sealed-locators.csv");
    let oracle_path = parent.path().join("sealed-workload.csv");
    let oracle = super::courtroom_oracle::seal(&oracle_path);
    let (_, placement, _) = super::scenario_configuration::courtroom_configuration();

    let writer_stdout = run_courtroom_writer(&root, &locators, &oracle_path);
    let writer = ScenarioProcessEvidence::parse_child(&writer_stdout, "writer");
    let writer_completion = writer_completion(&writer_stdout);
    let oracle_payload_digest = super::courtroom_oracle::payload_digest(&locators, &oracle);
    let oracle_point_digest = super::courtroom_oracle::point_digest(&locators, &oracle);
    let oracle_scan_digest = super::courtroom_oracle::scan_digest(&locators, &oracle);
    let oracle_records = oracle.len();
    assert_eq!(
        super::courtroom_oracle::locator_count(&locators),
        oracle_records
    );

    let reopener_stdout = run_courtroom_reopener(&root, &locators);
    let reopener = ScenarioProcessEvidence::parse_child(&reopener_stdout, "reopener");
    let reopened = reopener_completion(&reopener_stdout);

    let observer_stdout = super::observer::run(&root);
    let observer =
        ScenarioProcessEvidence::offline_process(&observer_stdout, &super::observer::binary_path());
    let offline_fields = offline_completion(&observer_stdout);
    let format = super::scenario_configuration::courtroom_configuration().0;
    let walk = worth_store_offline_verifier::walk_current_durable_record_manifest(
        &root,
        format.declaration(),
    )
    .unwrap();
    let first_locator = std::fs::read_to_string(&locators)
        .unwrap()
        .lines()
        .next()
        .map(super::child_process::decode_locator)
        .unwrap();
    let first_identity = (
        first_locator.encode()[16..32].try_into().unwrap(),
        u64::from_le_bytes(first_locator.encode()[32..40].try_into().unwrap()),
    );
    let prior = worth_store_offline_verifier::walk_non_current_durable_record_manifest(
        &root,
        format.declaration(),
        2,
    )
    .unwrap();
    let current_residue_refused = matches!(
        worth_store_offline_verifier::walk_non_current_durable_record_manifest(
            &root,
            format.declaration(),
            walk.root_generation(),
        ),
        Err(worth_store_offline_verifier::OfflineDurableManifestDenial::CurrentRootRequestedAsResidue)
    );
    assert!(current_residue_refused);
    let prior_first = inline_placement(&prior, first_identity);
    let current_first = inline_placement(&walk, first_identity);

    assert_eq!(writer_completion.root_generation, reopened.root_generation);
    assert_eq!(writer_completion.root_generation, walk.root_generation());
    assert_eq!(reopened.records, oracle_records);
    assert_eq!(reopened.records, walk.placements().len());
    assert_eq!(reopened.deferred_records, 1);
    assert_eq!(reopened.point_digest, oracle_point_digest);
    assert_eq!(reopened.scan_digest, oracle_scan_digest);
    assert_eq!(offline_fields.records, oracle_records);
    assert_eq!(
        offline_fields.root_generation,
        writer_completion.root_generation
    );
    assert_eq!(offline_fields.payload_digest, oracle_payload_digest);
    assert_eq!(offline_fields.payload_digest, hex(&walk.payload_digest()));
    assert!(segment_files(&root) >= 3);
    assert!(walk.manifest_blocks() > 1);
    assert!(segment_page_bytes(&root) >= 64 * 65_536);
    let locator_identity_set = locator_identities(&locators);
    let placement_identity_set = placement_identities(&walk);
    assert_eq!(placement_identity_set, locator_identity_set);
    assert_ne!(writer_completion.positioned_writes, 0);
    assert!(writer_completion.file_barriers >= writer_completion.positioned_writes);
    assert!(writer_completion.directory_barriers >= writer_completion.file_barriers);
    assert_eq!(writer_completion.catalog_replacements, 12);
    let extent_placements = walk
        .placements()
        .iter()
        .filter(|placement| {
            matches!(
                placement,
                worth_store_offline_verifier::OfflineRecordPlacement::Extent { .. }
            )
        })
        .count();
    assert_eq!(extent_placements, 2);
    assert_eq!(prior_first.slot, current_first.slot);
    assert_eq!(prior_first.page, current_first.page);
    assert!(current_first.page_generation > prior_first.page_generation);

    let processes = [writer, reopener, observer];
    let predicates = [
        ScenarioPredicate::equality(
            "writer_reopener_generation",
            writer_completion.root_generation,
            reopened.root_generation,
        ),
        ScenarioPredicate::equality(
            "reopener_offline_generation",
            reopened.root_generation,
            walk.root_generation(),
        ),
        ScenarioPredicate::equality(
            "point_oracle_digest",
            oracle_point_digest.clone(),
            reopened.point_digest.clone(),
        ),
        ScenarioPredicate::equality(
            "scan_oracle_digest",
            oracle_scan_digest.clone(),
            reopened.scan_digest.clone(),
        ),
        ScenarioPredicate::equality(
            "offline_oracle_digest",
            oracle_payload_digest.clone(),
            offline_fields.payload_digest.clone(),
        ),
        ScenarioPredicate::equality(
            "record_count",
            oracle_records as u64,
            reopened.records as u64,
        ),
        ScenarioPredicate::equality(
            "deferred_extent_count",
            1_u64,
            reopened.deferred_records as u64,
        ),
        ScenarioPredicate::equality("extent_placement_count", 2_u64, extent_placements as u64),
        ScenarioPredicate::equality(
            "record_identity_set",
            identity_set_digest(&locator_identity_set),
            identity_set_digest(&placement_identity_set),
        ),
        ScenarioPredicate::equality(
            "writer_physical_writes_present",
            true,
            writer_completion.positioned_writes != 0,
        ),
        ScenarioPredicate::equality(
            "writer_file_barrier_per_write_minimum",
            true,
            writer_completion.file_barriers >= writer_completion.positioned_writes,
        ),
        ScenarioPredicate::equality(
            "writer_directory_barrier_per_file_minimum",
            true,
            writer_completion.directory_barriers >= writer_completion.file_barriers,
        ),
        ScenarioPredicate::equality(
            "writer_catalog_replacement_per_batch",
            12_u64,
            writer_completion.catalog_replacements,
        ),
        ScenarioPredicate::equality(
            "cow_slot_offset",
            prior_first.slot as u64,
            current_first.slot as u64,
        ),
        ScenarioPredicate::equality("cow_page_cell", prior_first.page, current_first.page),
        ScenarioPredicate::equality(
            "cow_generation_advanced",
            true,
            current_first.page_generation > prior_first.page_generation,
        ),
        ScenarioPredicate::equality("current_root_is_not_residue", true, current_residue_refused),
    ];
    super::scenario_evidence::emit(ScenarioEvidence {
        courtroom: "A",
        world: "mixed-record-world",
        root: &root,
        seed: 0,
        action_trace: &[
            "initialize-and-append",
            "writer-exit",
            "fresh-reopen",
            "offline-walk",
        ],
        authority_transitions: &[
            "absent-to-initialized",
            "candidate-to-published",
            "closed-to-readmitted",
        ],
        walk: &walk,
        placement,
        publication_identity: Some(writer_completion.publication_identity),
        processes: &processes,
        counters: json!({
            "segments": segment_files(&root),
            "manifest_blocks": walk.manifest_blocks(),
            "segment_page_bytes": segment_page_bytes(&root),
            "writer_positioned_writes": writer_completion.positioned_writes,
            "writer_file_barriers": writer_completion.file_barriers,
            "writer_catalog_replacements": writer_completion.catalog_replacements,
            "writer_directory_barriers": writer_completion.directory_barriers,
            "scan_manifest_blocks": reopened.scan_manifest_blocks,
            "scan_manifest_comparisons": reopened.scan_manifest_comparisons,
            "scan_payload_bytes": reopened.scan_payload_bytes,
        }),
        runtime_result: json!({
            "records": reopened.records,
            "deferred_records": reopened.deferred_records,
            "root_generation": reopened.root_generation,
            "point_digest": reopened.point_digest,
            "scan_digest": reopened.scan_digest,
        }),
        oracle_result: json!({
            "records": oracle_records,
            "payload_digest": oracle_payload_digest,
            "point_digest": oracle_point_digest,
            "scan_digest": oracle_scan_digest,
        }),
        mutant_posture: "independent-observer-process",
        predicates: &predicates,
    });
}
