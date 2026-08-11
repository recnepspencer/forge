use super::*;

#[test]
fn persisted_input_roles_bind_real_producers_or_explicit_gaps() {
    let document = read_repository_document(PERSISTED_INPUTS).expect("read C.8 persisted inputs");
    let rows = parse_rows(&document).expect("parse C.8 persisted inputs");
    validate_rows(&rows).expect("validate C.8 persisted inputs");
    for row in rows
        .iter()
        .filter(|row| row.posture != "required-producer-gap")
    {
        source_defines_surface(&row.producer_source, &row.producer_type)
            .expect("bind persisted producer type");
        source_defines_surface(&row.admission_source, &row.admission_surface)
            .expect("bind persisted decoder or admission surface");
    }
}

#[test]
fn omitted_foreign_and_derived_proxy_mutants_are_rejected() {
    let document = read_repository_document(PERSISTED_INPUTS).expect("read C.8 persisted inputs");
    let rows = parse_rows(&document).expect("parse C.8 persisted inputs");
    assert!(validate_rows(&rows[1..]).is_err());

    let mut foreign = rows.clone();
    foreign[0].producer_type = "Store".into();
    assert!(validate_rows(&foreign).is_err());

    let mut derived = rows.clone();
    let gap = derived
        .iter_mut()
        .find(|row| row.role == "compaction-cutover-record")
        .unwrap();
    gap.producer_type = "CompactionCutoverRecoveryPosture".into();
    gap.admission_surface = "CompactionCutoverRecoveryPosture::admit_visible_product".into();
    gap.producer_source = "workspaces/worth-store/crates/worth-store-recovery-physics/src/source_precedence/compaction_visibility/artifact_residue.rs".into();
    gap.admission_source = gap.producer_source.clone();
    gap.posture = "decoded-from-persisted-bytes".into();
    gap.disposition = "preserve".into();
    assert!(validate_rows(&derived).is_err());

    let mut shallow = rows.clone();
    shallow
        .iter_mut()
        .find(|row| row.role == "operation-terminal-fate")
        .unwrap()
        .causal_sources = format!("{DURABILITY_ROOT}/mutation/idempotency/fate/persisted.rs");
    assert!(validate_rows(&shallow).is_err());

    let mut unframed = rows.clone();
    let attempt = unframed
        .iter_mut()
        .find(|row| row.role == "operation-attempt-binding-wal")
        .unwrap();
    attempt.causal_sources = attempt.causal_sources.replace(
        ";workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/wal/group_reservation/member_planning.rs",
        "",
    );
    assert!(validate_rows(&unframed).is_err());

    let mut unpublished = rows.clone();
    let checkpoint = unpublished
        .iter_mut()
        .find(|row| row.role == "checkpoint-binding-compaction")
        .unwrap();
    checkpoint.causal_sources = checkpoint.causal_sources.replace(
        ";workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/checkpoint/publication.rs",
        "",
    );
    assert!(validate_rows(&unpublished).is_err());
}
