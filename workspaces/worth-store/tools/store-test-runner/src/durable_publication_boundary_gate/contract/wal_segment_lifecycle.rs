use super::super::read_repository_document;

mod continuation;
mod reopen;
mod reservation;

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

#[test]
fn wal_segment_lifecycle_resolves_through_real_rust_syntax() {
    inspect(&sources()).unwrap_or_else(|denial| {
        panic!("MUTANT_PREDICATE:wal-segment-lifecycle-source-counterfeit-accepted: {denial}")
    });
}

#[test]
fn wal_segment_lifecycle_rejects_wrong_owners_without_pinning_local_phrasing() {
    let source = sources();

    let mut per_member_create = source.clone();
    per_member_create.member_planning = per_member_create.member_planning.replace(
        "PhysicalWalFrameWriteDisposition::AppendExistingSegment\n        };",
        "PhysicalWalFrameWriteDisposition::CreateSegment\n        };",
    );
    assert!(inspect(&per_member_create).is_err());

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

    let mut unbounded_inventory = source;
    unbounded_inventory.reopen = unbounded_inventory
        .reopen
        .replace("list_file_names_bounded", "list_file_names");
    assert!(inspect(&unbounded_inventory).is_err());
}

#[test]
fn comments_and_literals_cannot_counterfeit_wal_lifecycle_steps() {
    let source = sources();
    let mut comment_counterfeit = source.clone();
    comment_counterfeit.reopen = comment_counterfeit.reopen.replace(
        ".list_file_names_bounded(&directory, inventory_limit)",
        ".list_file_names(&directory) // .list_file_names_bounded(&directory, inventory_limit)",
    );
    assert!(inspect(&comment_counterfeit).is_err());

    let mut delimiter_literal = source;
    delimiter_literal.reopen = delimiter_literal.reopen.replace(
        "let directory = wal_directory();",
        "let _delimiter_literal = \"{ a comment-like } brace cannot move a function boundary\";\n    let directory = wal_directory();",
    );
    inspect(&delimiter_literal).expect("string delimiters do not alter Rust syntax ownership");
}

#[test]
fn dead_shape_plus_macro_hidden_behavior_cannot_counterfeit_reopen() {
    let mut source = sources();
    source.reopen = source.reopen.replace(
        "    let names = tree\n        .list_file_names_bounded(&directory, inventory_limit)\n        .map_err(map_listing_failure)?;",
        "    let _counterfeit = || {\n        tree.list_file_names_bounded(&directory, inventory_limit)\n    };\n    let names = worth_dbg!(tree.list_file_names(&directory));",
    );
    assert!(
        inspect(&source).is_err(),
        "MUTANT_PREDICATE:wal-reopen-dead-shape-macro-behavior-accepted"
    );
}

#[test]
fn delegated_reopen_verification_is_the_current_semantic_owner() {
    let mut source = sources();
    source.reopen = source.reopen.replace(
        "interrupted_active_tail::inspect(",
        "inspect_verified_wal_segment(",
    );
    assert!(
        inspect(&source).is_err(),
        "MUTANT_PREDICATE:wal-segment-lifecycle-stale-reopen-owner-accepted"
    );
}

#[derive(Clone)]
struct WalSegmentLifecycleSources {
    reservation: String,
    member_planning: String,
    group_port: String,
    runtime_owner: String,
    reopen: String,
}

fn sources() -> WalSegmentLifecycleSources {
    WalSegmentLifecycleSources {
        reservation: read(RESERVATION),
        member_planning: read(MEMBER_PLANNING),
        group_port: read(GROUP_PORT),
        runtime_owner: read(RUNTIME_OWNER),
        reopen: read(REOPEN),
    }
}

fn inspect(source: &WalSegmentLifecycleSources) -> Result<(), String> {
    reservation::inspect(&source.reservation, &source.member_planning)?;
    continuation::inspect(&source.group_port, &source.runtime_owner)?;
    reopen::inspect(&source.reopen)?;
    super::wal_reopen_origin::inspect(&source.reopen)
}

fn read(path: &str) -> String {
    read_repository_document(path).unwrap_or_else(|error| panic!("{error}"))
}
