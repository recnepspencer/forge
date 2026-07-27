use std::path::Path;

use super::workspace_source::read;
use crate::workspace_root;

const GRANT: &str = "crates/worth-store-buffer-pool/src/physical_residency/operation_allocation/foreground_write.rs";
const POOL_API: &str = "crates/worth-store-buffer-pool/src/physical_residency/pool/public_api.rs";
const FRAME_LEASE: &str = "crates/worth-store-buffer-pool/src/physical_residency/lease/frame.rs";

#[test]
fn foreground_write_grant_is_pool_issued_and_required_by_mutation() {
    let root = workspace_root();
    let grant = read(&root.join(GRANT)).expect("read foreground-write grant");
    let pool_api = read(&root.join(POOL_API)).expect("read pool API");
    let frame_lease = read(&root.join(FRAME_LEASE)).expect("read frame lease");
    inspect_foreground_write_authority(
        (&root.join(GRANT), &grant),
        (&root.join(POOL_API), &pool_api),
        (&root.join(FRAME_LEASE), &frame_lease),
    )
    .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn foreground_write_gate_kills_scope_erased_dirty_replacement() {
    let frame_lease = r#"
pub fn begin_dirty_replacement<'grant>(
    self,
    allocation: &'grant OperationAllocationGrant,
) -> Result<Reservation<'grant>, Denial> {
    let allocation_use = allocation.reserve_use(&self.owner, self.bytes());
    Reservation::new(self, allocation_use)
}
"#;
    let denial = inspect_foreground_write_authority(
        (Path::new("grant.rs"), honest_grant()),
        (Path::new("pool_api.rs"), honest_pool_api()),
        (Path::new("frame.rs"), frame_lease),
    )
    .expect_err("scope-erased dirty replacement must be denied");
    assert!(denial.contains("dirty replacement"));
}

#[test]
fn foreground_write_gate_kills_forgeable_grant_representation() {
    let grant = honest_grant().replace(
        "operation: OperationAllocationGrant",
        "pub operation: OperationAllocationGrant",
    );
    let denial = inspect_foreground_write_authority(
        (Path::new("grant.rs"), &grant),
        (Path::new("pool_api.rs"), honest_pool_api()),
        (Path::new("frame.rs"), honest_frame_lease()),
    )
    .expect_err("public foreground-write authority field must be denied");
    assert!(denial.contains("private generic grant"));
}

fn inspect_foreground_write_authority(
    grant: (&Path, &str),
    pool_api: (&Path, &str),
    frame_lease: (&Path, &str),
) -> Result<(), String> {
    inspect_grant_shape(grant)?;
    inspect_pool_issuance(grant)?;
    inspect_candidate_signatures(pool_api)?;
    inspect_dirty_replacement_signature(frame_lease)
}

fn inspect_grant_shape(grant: (&Path, &str)) -> Result<(), String> {
    let body = required_body(grant, "pub struct ForegroundWriteAllocationGrant")?;
    if !body.contains("operation: OperationAllocationGrant")
        || body.contains("pub operation:")
        || grant
            .1
            .contains("impl Clone for ForegroundWriteAllocationGrant")
        || grant.1.contains("fn new(")
    {
        return Err(format!(
            "foreground-write authority: grant must wrap one private generic grant without a reusable constructor in {}",
            grant.0.display()
        ));
    }
    Ok(())
}

fn inspect_pool_issuance(grant: (&Path, &str)) -> Result<(), String> {
    let body = required_body(grant, "pub fn begin_foreground_write_operation")?;
    let compact: String = body.split_whitespace().collect();
    if !compact
        .contains("self.begin_operation(PhysicalOperationAllocationScope::ForegroundWrite,bytes)")
        || !compact.contains("ForegroundWriteAllocationGrant{operation}")
    {
        return Err(format!(
            "foreground-write authority: colocated pool issuance must bind the exact scope and private field in {}",
            grant.0.display()
        ));
    }
    Ok(())
}

fn inspect_candidate_signatures(pool_api: (&Path, &str)) -> Result<(), String> {
    for signature in [
        "pub fn materialize_dirty_candidate",
        "pub fn reserve_candidate_frames",
        "pub fn begin_candidate_batch",
    ] {
        let declaration = required_declaration(pool_api, signature)?;
        if !declaration.contains("ForegroundWriteAllocationGrant") {
            return Err(format!(
                "foreground-write authority: `{signature}` must require the typed grant in {}",
                pool_api.0.display()
            ));
        }
    }
    Ok(())
}

fn inspect_dirty_replacement_signature(frame_lease: (&Path, &str)) -> Result<(), String> {
    let declaration = required_declaration(frame_lease, "pub fn begin_dirty_replacement")?;
    if !declaration.contains("ForegroundWriteAllocationGrant") {
        return Err(format!(
            "foreground-write authority: dirty replacement must require the typed grant in {}",
            frame_lease.0.display()
        ));
    }
    Ok(())
}

fn required_body<'source>(
    source: (&Path, &'source str),
    signature: &str,
) -> Result<&'source str, String> {
    let start = source
        .1
        .find(signature)
        .ok_or_else(|| missing(source.0, signature))?;
    let tail = &source.1[start..];
    let body_start = tail.find('{').ok_or_else(|| missing(source.0, signature))?;
    let mut depth = 0_u32;
    for (offset, byte) in tail[body_start..].bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(&tail[body_start..=body_start + offset]);
                }
            }
            _ => {}
        }
    }
    Err(missing(source.0, signature))
}

fn required_declaration<'source>(
    source: (&Path, &'source str),
    signature: &str,
) -> Result<&'source str, String> {
    let start = source
        .1
        .find(signature)
        .ok_or_else(|| missing(source.0, signature))?;
    let tail = &source.1[start..];
    let body = tail.find('{').ok_or_else(|| missing(source.0, signature))?;
    Ok(&tail[..body])
}

fn missing(path: &Path, signature: &str) -> String {
    format!(
        "foreground-write authority: `{signature}` missing or malformed in {}",
        path.display()
    )
}

fn honest_grant() -> &'static str {
    r#"
pub struct ForegroundWriteAllocationGrant {
    operation: OperationAllocationGrant,
}
impl PhysicalResidencyPool {
    pub fn begin_foreground_write_operation(bytes: NonZeroU64) -> Result<ForegroundWriteAllocationGrant, Denial> {
        let operation = self.begin_operation(PhysicalOperationAllocationScope::ForegroundWrite, bytes)?;
        Ok(ForegroundWriteAllocationGrant { operation })
    }
}
"#
}

fn honest_pool_api() -> &'static str {
    r#"
pub fn materialize_dirty_candidate(allocation: &ForegroundWriteAllocationGrant) {}
pub fn reserve_candidate_frames(allocation: &ForegroundWriteAllocationGrant) {}
pub fn begin_candidate_batch(allocation: &ForegroundWriteAllocationGrant) {}
"#
}

fn honest_frame_lease() -> &'static str {
    r#"
pub fn begin_dirty_replacement<'grant>(
    self,
    allocation: &'grant ForegroundWriteAllocationGrant,
) -> Result<Reservation<'grant>, Denial> {
    let allocation_use = allocation.reserve_use(&self.owner, self.bytes());
    Reservation::new(self, allocation_use)
}
"#
}
