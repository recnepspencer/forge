use super::super::read_repository_document;

const RESERVATION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                           durability/wal/group_reservation/mod.rs";
const MEMBER_PLANNING: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                               durability/wal/group_reservation/member_planning.rs";
const GROUP_PORT: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                          durability/wal/port/group.rs";
const RUNTIME_OWNER: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                             durability/wal/runtime_owner.rs";
const REOPEN: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                      durability/wal/inventory/reopen.rs";
const JOURNEY: &str = "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/\
                       durability_admission/wal_group_continuation.rs";

#[test]
fn wal_segment_lifecycle_preserves_whole_group_rotation_and_bounded_reopen() {
    inspect(&sources()).unwrap();
}

#[test]
fn wal_segment_lifecycle_gate_rejects_rotation_continuation_and_reopen_mutants() {
    let source = sources();

    let mut per_member_create = source.clone();
    per_member_create.member_planning = per_member_create.member_planning.replace(
        "PhysicalWalFrameWriteDisposition::AppendExistingSegment\n        };",
        "PhysicalWalFrameWriteDisposition::CreateSegment\n        };",
    );
    assert!(inspect(&per_member_create).is_err());

    let mut rotated_lsn_rebase = source.clone();
    rotated_lsn_rebase.reservation = rotated_lsn_rebase.reservation.replacen(
        "next_frontier,\n            group_lsn_start,",
        "next_frontier,\n            next_frontier.last_lsn_end().unwrap(),",
        1,
    );
    assert!(inspect(&rotated_lsn_rebase).is_err());

    let mut suffix_replanning = source.clone();
    suffix_replanning.group_port = suffix_replanning.group_port.replace(
        "self.append_reserved_group(basis, appended, members)",
        "self.reserve_and_append_group(basis, appended, members)",
    );
    assert!(inspect(&suffix_replanning).is_err());

    let mut premature_release = source.clone();
    premature_release.group_port = premature_release.group_port.replace(
        "PhysicalWalGroupAppendRemainder::Reserved(pending)",
        "{ self.owner.finish_group(); PhysicalWalGroupAppendRemainder::Reserved(pending) }",
    );
    assert!(inspect(&premature_release).is_err());

    let mut unbounded_inventory = source.clone();
    unbounded_inventory.reopen = unbounded_inventory
        .reopen
        .replace("list_file_names_bounded", "list_file_names");
    assert!(inspect(&unbounded_inventory).is_err());

    let mut oversized_read = source.clone();
    oversized_read.reopen = oversized_read.reopen.replace(
        "if byte_count > byte_limit {",
        "if false && byte_count > byte_limit {",
    );
    assert!(inspect(&oversized_read).is_err());

    let mut unsealed_reopen = source;
    unsealed_reopen.reopen = unsealed_reopen.reopen.replace(
        "let requires_inspection = cutoff.lsn().is_none();",
        "let requires_inspection = false;",
    );
    assert!(inspect(&unsealed_reopen).is_err());
}

#[derive(Clone)]
struct WalSegmentLifecycleSources {
    reservation: String,
    member_planning: String,
    group_port: String,
    runtime_owner: String,
    reopen: String,
    journey: String,
}

fn sources() -> WalSegmentLifecycleSources {
    WalSegmentLifecycleSources {
        reservation: read_repository_document(RESERVATION).expect("read WAL group reservation"),
        member_planning: read_repository_document(MEMBER_PLANNING)
            .expect("read WAL member planning"),
        group_port: read_repository_document(GROUP_PORT).expect("read WAL group port"),
        runtime_owner: read_repository_document(RUNTIME_OWNER).expect("read WAL runtime owner"),
        reopen: read_repository_document(REOPEN).expect("read WAL inventory reopen"),
        journey: read_repository_document(JOURNEY).expect("read WAL continuation journey"),
    }
}

fn inspect(source: &WalSegmentLifecycleSources) -> Result<(), &'static str> {
    inspect_whole_group_reservation(&source.reservation, &source.member_planning)?;
    inspect_exact_continuation(&source.group_port, &source.runtime_owner)?;
    inspect_bounded_reopen(&source.reopen)?;
    inspect_journey(&source.journey)?;
    Ok(())
}

fn inspect_whole_group_reservation(
    reservation: &str,
    member_planning: &str,
) -> Result<(), &'static str> {
    let reserve = compact(
        function_body(reservation, "fn reserve_group(")
            .ok_or("whole-group reservation owner is absent")?,
    );
    if reserve.matches("plan_group(").count() != 2
        || reserve.matches("state.in_flight=true").count() != 2
    {
        return Err("group reservation no longer plans exactly one current and one rotated world");
    }
    for required in [
        "letgroup_lsn_start=state.frontier.last_lsn_end()",
        "admitted,state.frontier,group_lsn_start",
        "letadmitted=current.release_after_no_effect()",
        "ifstate.segment_count>=inventory_limit",
        "admitted,next_frontier,group_lsn_start",
        "PhysicalWalFrameWriteDisposition::CreateSegment",
    ] {
        if !reserve.contains(required) {
            return Err("rotated group placement lost its pre-effect whole-group authority");
        }
    }

    let planning = compact(
        function_body(member_planning, "fn plan_group(")
            .ok_or("whole-group member planning is absent")?,
    );
    for required in [
        "whileletSome((prepared,group))=pending.next()",
        "iffirst{first_disposition}else{PhysicalWalFrameWriteDisposition::AppendExistingSegment}",
        "frontier=member.mutation().resulting_frontier()",
        "Ok(ReservedPhysicalWalGroupMembers(nonempty(planned)))",
    ] {
        if !planning.contains(required) {
            return Err("member planning can no longer prove one contiguous reserved group");
        }
    }
    Ok(())
}

fn inspect_exact_continuation(group_port: &str, runtime_owner: &str) -> Result<(), &'static str> {
    let continuation = compact(
        function_body(group_port, "fn continue_prepared_group(")
            .ok_or("group continuation entry is absent")?,
    );
    if !continuation.contains(
        "PhysicalWalGroupAppendRemainder::Reserved(members)=>{self.append_reserved_group(basis,appended,members)}",
    ) {
        return Err("reserved suffix continuation can re-enter reservation or replanning");
    }

    let append = compact(
        function_body(group_port, "fn append_reserved_group(")
            .ok_or("reserved group append owner is absent")?,
    );
    for required in [
        "ifappended.is_empty(){self.owner.release_group_before_effect()",
        "}else{PhysicalWalGroupAppendRemainder::Reserved(pending)}",
        "self.owner.finish_group();matchSealedPhysicalDurabilityGroupMembers::seal",
    ] {
        if !append.contains(required) {
            return Err(
                "partial continuation lost exact suffix ownership or terminal release order",
            );
        }
    }
    let owner = compact(runtime_owner);
    if owner.matches(".in_flight=false").count() != 3
        || !owner.contains("pub(super)fncomplete_member(")
    {
        return Err(
            "WAL runtime ownership no longer separates member completion from group release",
        );
    }
    Ok(())
}

fn inspect_bounded_reopen(source: &str) -> Result<(), &'static str> {
    let reopen = compact(
        function_body(source, "fn reopen_wal_inventory(")
            .ok_or("bounded WAL reopen owner is absent")?,
    );
    if !contains_in_order(
        &reopen,
        &[
            "list_file_names_bounded(&directory,inventory_limit)",
            "WalSegmentArtifactIdentity::parse(name)",
            "letbyte_count=tree.file_length(&artifact)",
            "ifbyte_count==0",
            "ifbyte_count>byte_limit",
            "try_reserve_exact(allocation)",
            "tree.read_exact_at(&artifact,0,&mutbytes)",
            "inspect_verified_wal_segment(identity,&bytes)",
            "WalTopologyScan::from_segment_scan(scans)",
            "PhysicalWalSegmentInventory::from_reopened(inspections)",
            "require_checkpoint_cutoff_within_retained_wal(cutoff,&segment_inventory,active_lsn_end)",
            "letrequires_inspection=cutoff.lsn().is_none()",
            "requires_inspection,",
        ],
    ) {
        return Err("WAL reopen no longer rejects hostile inventory before bounded allocation");
    }
    Ok(())
}

fn inspect_journey(source: &str) -> Result<(), &'static str> {
    for required in [
        "PhysicalWalGroupAppendOutcome::PartiallyAppended",
        "PhysicalWalReservationDenial::AppendInFlight",
        "assert_exact_rotated_group(&rotated)",
        "second.artifact_range().offset()",
        "first.lsn_range().end_exclusive()",
    ] {
        if !source.contains(required) {
            return Err("adversarial rotated-group continuation journey lost causal evidence");
        }
    }
    Ok(())
}

fn function_body<'a>(source: &'a str, signature: &str) -> Option<&'a str> {
    let start = source.find(signature)?;
    let open = source[start..].find('{')? + start;
    let mut depth = 0_u32;
    for (offset, character) in source[open..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&source[open + 1..open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

fn compact(source: &str) -> String {
    source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}

fn contains_in_order(source: &str, required: &[&str]) -> bool {
    let mut offset = 0;
    required.iter().all(|needle| {
        let Some(found) = source[offset..].find(needle) else {
            return false;
        };
        offset += found + needle.len();
        true
    })
}
