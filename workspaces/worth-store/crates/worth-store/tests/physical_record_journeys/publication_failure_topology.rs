use worth_store_physical_backend::MediaOperationRole;

#[path = "publication_failure_topology/evidence.rs"]
mod evidence;
#[path = "publication_failure_topology/result_codec.rs"]
mod result_codec;
#[path = "publication_failure_topology/scenario.rs"]
mod scenario;

#[test]
fn publication_cutover_never_invents_current_truth() {
    for case in failure_cases() {
        let observed = scenario::observe(case);
        evidence::adjudicate_and_emit(&observed);
    }
}

#[derive(Clone, Copy)]
struct FailureCase {
    pub(super) name: &'static str,
    pub(super) role: MediaOperationRole,
    pub(super) role_name: &'static str,
    pub(super) append_ordinal: u64,
    pub(super) payload_bytes: usize,
    pub(super) directive: &'static str,
    pub(super) expected_generation: u64,
    pub(super) expected_records: usize,
    pub(super) expected_residue: bool,
}

fn failure_cases() -> [FailureCase; 6] {
    [
        failure(
            "short-data-write",
            MediaOperationRole::PositionedWrite,
            "positioned-write",
            1,
            9,
            "prefix",
        ),
        failure(
            "extent-truncation",
            MediaOperationRole::PositionedWrite,
            "positioned-write",
            1,
            20_000,
            "prefix",
        ),
        failure(
            "data-sync",
            MediaOperationRole::SynchronizeFileState,
            "file-sync",
            1,
            9,
            "before",
        ),
        failure(
            "manifest-write",
            MediaOperationRole::PositionedWrite,
            "positioned-write",
            2,
            9,
            "before",
        ),
        failure(
            "post-manifest-pre-catalog",
            MediaOperationRole::PositionedWrite,
            "positioned-write",
            7,
            9,
            "before",
        ),
        FailureCase {
            name: "post-catalog-pre-directory-sync",
            role: MediaOperationRole::AtomicReplace,
            role_name: "atomic-replace",
            append_ordinal: 1,
            payload_bytes: 9,
            directive: "after",
            expected_generation: 3,
            expected_records: 2,
            expected_residue: false,
        },
    ]
}

const fn failure(
    name: &'static str,
    role: MediaOperationRole,
    role_name: &'static str,
    append_ordinal: u64,
    payload_bytes: usize,
    directive: &'static str,
) -> FailureCase {
    FailureCase {
        name,
        role,
        role_name,
        append_ordinal,
        payload_bytes,
        directive,
        expected_generation: 2,
        expected_records: 1,
        expected_residue: true,
    }
}
