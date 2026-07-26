use std::path::Path;

use super::workspace_source::read;
use crate::workspace_root;

const POOL_API: &str = "crates/worth-store-buffer-pool/src/physical_residency/pool/public_api.rs";
const CANDIDATE_ADMISSION: &str =
    "crates/worth-store-buffer-pool/src/physical_residency/pool/candidate_admission.rs";
const CANDIDATE_AUTHORITY: &str =
    "crates/worth-store-buffer-pool/src/physical_residency/lease/candidate.rs";
const STORE_PUBLISHER: &str =
    "crates/worth-store/src/physical_runtime/record_serving/residency/candidate_frame_publishers.rs";

#[test]
fn candidate_projection_and_metadata_retain_exact_named_allocation_use() {
    let root = workspace_root();
    let pool_api = read(&root.join(POOL_API)).expect("read pool candidate API");
    let admission =
        read(&root.join(CANDIDATE_ADMISSION)).expect("read candidate allocation admission");
    let authority =
        read(&root.join(CANDIDATE_AUTHORITY)).expect("read candidate allocation authority");
    let publisher = read(&root.join(STORE_PUBLISHER)).expect("read Store candidate publisher");
    inspect_candidate_allocation(
        (&root.join(POOL_API), &pool_api),
        (&root.join(CANDIDATE_ADMISSION), &admission),
        (&root.join(CANDIDATE_AUTHORITY), &authority),
        (&root.join(STORE_PUBLISHER), &publisher),
    )
    .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn candidate_allocation_gate_kills_scope_laundering_mutant() {
    let pool_api = r#"
fn begin_candidate_batch(allocation: &OperationAllocationGrant, count: NonZeroUsize) {
    self.validate_candidate_projection_start();
    let allocation_use = allocation.reserve_use(self, candidate_bytes(count));
    Admission { allocation_use }
}
"#;
    let admission = r#"
fn admit_candidate_batch(admission: Admission, keys: &[Key]) {
    if admission.candidate_count.get() != keys.len() { return Err(Mismatch); }
    self.validate_candidate_set(keys);
}
fn finish_candidate_batch(&self) {
    state.active_candidate_publications =
        state.active_candidate_publications.checked_sub(1).expect("active publication");
}
"#;
    let authority = r#"
struct PhysicalCandidateBatchReservation<'grant> {
    allocation_use: OperationAllocationUse<'grant>,
}
pub fn reserve_next(&mut self, allocation: &OperationAllocationGrant, candidate: Key) {
    let scope = allocation.scope_for(&self.owner);
    self.owner.reserve_next_candidate(scope, candidate);
}
"#;
    let publisher = honest_publisher();
    let denial = inspect_candidate_allocation(
        (Path::new("pool_api.rs"), pool_api),
        (Path::new("admission.rs"), admission),
        (Path::new("authority.rs"), authority),
        (Path::new("publisher.rs"), publisher),
    )
    .expect_err("second-grant scope-laundering mutant must be denied");
    assert!(denial.contains("exact grant use"));
}

#[test]
fn candidate_allocation_gate_kills_store_preallocation_mutant() {
    let publisher = r#"
fn begin(allocation: &OperationAllocationGrant, candidate: &CandidateFrameSet) {
    self.pool.validate_operation_allocation(allocation);
    self.counters.submissions.fetch_add(1, Ordering::AcqRel);
    let mut keys = Vec::new();
    let reservations = self.pool.reserve_candidate_frames(allocation, &keys);
}
"#;
    let denial = inspect_candidate_allocation(
        (Path::new("pool_api.rs"), honest_pool_api()),
        (Path::new("admission.rs"), honest_admission()),
        (Path::new("authority.rs"), honest_authority()),
        (Path::new("publisher.rs"), publisher),
    )
    .expect_err("Store preallocation mutant must be denied");
    assert!(denial.contains("before Store counters and key projection"));
}

#[test]
fn candidate_allocation_gate_kills_missing_lifecycle_linearization_mutant() {
    let pool_api = r#"
fn begin_candidate_batch(allocation: &OperationAllocationGrant, count: NonZeroUsize) {
    let allocation_use = allocation.reserve_use(self, candidate_bytes(count));
    Admission { allocation_use }
}
"#;
    let denial = inspect_candidate_allocation(
        (Path::new("pool_api.rs"), pool_api),
        (Path::new("admission.rs"), honest_admission()),
        (Path::new("authority.rs"), honest_authority()),
        (Path::new("publisher.rs"), honest_publisher()),
    )
    .expect_err("missing candidate lifecycle linearization must be denied");
    assert!(denial.contains("lifecycle"));
}

#[test]
fn candidate_allocation_gate_kills_saturating_cleanup_mutant() {
    let admission = r#"
fn admit_candidate_batch(admission: Admission, keys: &[Key]) {
    if admission.candidate_count.get() != keys.len() { return Err(Mismatch); }
    self.validate_candidate_set(keys);
}
fn finish_candidate_batch(&self) {
    state.active_candidate_publications =
        state.active_candidate_publications.saturating_sub(1);
}
"#;
    let denial = inspect_candidate_allocation(
        (Path::new("pool_api.rs"), honest_pool_api()),
        (Path::new("admission.rs"), admission),
        (Path::new("authority.rs"), honest_authority()),
        (Path::new("publisher.rs"), honest_publisher()),
    )
    .expect_err("saturating candidate-cleanup mutant must be denied");
    assert!(denial.contains("exact checked release"));
}

fn inspect_candidate_allocation(
    pool_api: (&Path, &str),
    admission: (&Path, &str),
    authority: (&Path, &str),
    publisher: (&Path, &str),
) -> Result<(), String> {
    inspect_preallocation_use(pool_api)?;
    inspect_declaration_and_cleanup(admission)?;
    inspect_reservation_authority(authority)?;
    inspect_store_projection_order(publisher)
}

fn inspect_preallocation_use(pool_api: (&Path, &str)) -> Result<(), String> {
    let begin = required_body(pool_api, "fn begin_candidate_batch")?;
    let lifecycle = begin.find("validate_candidate_projection_start");
    let allocation_use = begin.find("allocation.reserve_use");
    if !matches!(
        (lifecycle, allocation_use),
        (Some(check), Some(reserve)) if check < reserve
    ) {
        return Err(format!(
            "candidate allocation: lifecycle must linearize before the exact grant use in {}",
            pool_api.0.display()
        ));
    }
    Ok(())
}

fn inspect_declaration_and_cleanup(admission: (&Path, &str)) -> Result<(), String> {
    let consume = required_body(admission, "fn admit_candidate_batch")?;
    let cardinality = consume.find("candidate_count.get() != keys.len()");
    let metadata = consume.find("self.validate_candidate_set");
    if !matches!((cardinality, metadata), (Some(check), Some(allocate)) if check < allocate) {
        return Err(format!(
            "candidate allocation: declared cardinality must be checked before metadata allocation in {}",
            admission.0.display()
        ));
    }
    let finish = required_body(admission, "fn finish_candidate_batch")?;
    if !finish.contains("checked_sub") || finish.contains("saturating_sub") {
        return Err(format!(
            "candidate allocation: active publication cleanup must use exact checked release in {}",
            admission.0.display()
        ));
    }
    Ok(())
}

fn inspect_reservation_authority(authority: (&Path, &str)) -> Result<(), String> {
    let reservation = required_body(authority, "struct PhysicalCandidateBatchReservation")?;
    let progression = required_body(authority, "pub fn reserve_next")?;
    if !reservation.contains("OperationAllocationUse")
        || !progression.contains("self.allocation_use.scope()")
        || progression.contains("OperationAllocationGrant")
    {
        return Err(format!(
            "candidate allocation: reservation and progression must retain one exact grant use in {}",
            authority.0.display()
        ));
    }
    Ok(())
}

fn inspect_store_projection_order(publisher: (&Path, &str)) -> Result<(), String> {
    let publish = required_body(publisher, "fn begin")?;
    let preallocation = publish.find("begin_candidate_batch");
    let counters = publish.find("self.counters.submissions");
    let keys = publish.find("let mut keys");
    if !matches!(
        (preallocation, counters, keys),
        (Some(preallocate), Some(count), Some(project)) if preallocate < count && preallocate < project
    ) {
        return Err(format!(
            "candidate allocation: named use must be admitted before Store counters and key projection in {}",
            publisher.0.display()
        ));
    }
    Ok(())
}

fn required_body<'source>(
    source: (&Path, &'source str),
    signature: &str,
) -> Result<&'source str, String> {
    delimited_body(source.1, signature).ok_or_else(|| {
        format!(
            "candidate allocation: `{signature}` missing in {}",
            source.0.display()
        )
    })
}

fn delimited_body<'source>(source: &'source str, signature: &str) -> Option<&'source str> {
    let start = source.find(signature)?;
    let tail = &source[start..];
    let body_start = tail.find('{')?;
    let mut depth = 0_u32;
    for (offset, byte) in tail[body_start..].bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&tail[body_start..=body_start + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

fn honest_pool_api() -> &'static str {
    r#"
fn begin_candidate_batch(allocation: &OperationAllocationGrant, count: NonZeroUsize) {
    self.validate_candidate_projection_start();
    let allocation_use = allocation.reserve_use(self, candidate_bytes(count));
    Admission { allocation_use }
}
"#
}

fn honest_admission() -> &'static str {
    r#"
fn admit_candidate_batch(admission: Admission, keys: &[Key]) {
    if admission.candidate_count.get() != keys.len() { return Err(Mismatch); }
    self.validate_candidate_set(keys);
}
fn finish_candidate_batch(&self) {
    state.active_candidate_publications =
        state.active_candidate_publications.checked_sub(1).expect("active publication");
}
"#
}

fn honest_authority() -> &'static str {
    r#"
struct PhysicalCandidateBatchReservation<'grant> {
    allocation_use: OperationAllocationUse<'grant>,
}
pub fn reserve_next(&mut self, candidate: Key) {
    let scope = self.allocation_use.scope();
    self.owner.reserve_next_candidate(scope, candidate);
}
"#
}

fn honest_publisher() -> &'static str {
    r#"
fn begin(allocation: &OperationAllocationGrant, candidate: &CandidateFrameSet) {
    let admission = self.pool.begin_candidate_batch(allocation, candidate.count());
    self.counters.submissions.fetch_add(1, Ordering::AcqRel);
    let mut keys = Vec::new();
    admission.reserve(&keys);
}
"#
}
