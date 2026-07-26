use std::path::Path;

use super::workspace_source::read;
use crate::workspace_root;

const FRAME_ADMISSION: &str =
    "crates/worth-store-buffer-pool/src/physical_residency/pool/frame_admission/mod.rs";
const BOUNDED_COMPLETION: &str = "crates/worth-store-buffer-pool/src/physical_residency/pool/bounded_frame_admission/completion.rs";
const BOUNDED_ADMISSION: &str =
    "crates/worth-store-buffer-pool/src/physical_residency/pool/bounded_frame_admission/mod.rs";
const BOUNDED_LOADER: &str = "crates/worth-store/src/physical_runtime/record_serving/residency/frame_loading/bounded_loader.rs";

#[test]
fn loading_access_attaches_a_waiter_while_only_absence_reserves_an_owner() {
    let source =
        read(&workspace_root().join(FRAME_ADMISSION)).expect("read physical frame admission");
    inspect_fault_ownership(Path::new(FRAME_ADMISSION), &source)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn fault_ownership_gate_kills_the_duplicate_source_load_mutant_locally() {
    let mutant = r#"
FrameAccessPosture::Loading(identity) => {
    return self.reserve_loading(&mut state, scope, key);
}
FrameAccessPosture::LoadFailed(terminal) => return Err(terminal),
FrameAccessPosture::Absent => {
    return self.reserve_loading(&mut state, scope, key);
}
"#;
    let denial = inspect_fault_ownership(Path::new("controlled_mutant.rs"), mutant)
        .expect_err("duplicate fault-owner mutant must be denied");
    assert!(denial.contains("loading access must attach one waiter"));
}

#[test]
fn bounded_access_decides_ownership_before_length_discovery() {
    let source = read(&workspace_root().join(BOUNDED_LOADER)).expect("read bounded frame loader");
    inspect_bounded_decision_before_source(Path::new(BOUNDED_LOADER), &source)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn fault_ownership_gate_kills_the_bounded_pre_source_mutant_locally() {
    let mutant = r#"
let length = source.file_length(artifact)?;
let access = self.pool.access_bounded_frame(allocation, key)?;
match access {
    PhysicalBoundedFrameAccess::Hit(lease) => lease,
    PhysicalBoundedFrameAccess::Coalesced(waiter) => waiter.wait()?,
    PhysicalBoundedFrameAccess::Fault(owner) => owner.load(length),
}
"#;
    let denial = inspect_bounded_decision_before_source(Path::new("controlled_mutant.rs"), mutant)
        .expect_err("bounded pre-source mutant must be denied");
    assert!(denial.contains("must decide ownership before length discovery"));
}

#[test]
fn bounded_waiters_share_only_an_identical_request_limit() {
    let source =
        read(&workspace_root().join(BOUNDED_ADMISSION)).expect("read bounded frame admission");
    inspect_bounded_limit_identity(Path::new(BOUNDED_ADMISSION), &source)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn fault_ownership_gate_kills_the_cross_limit_coalescence_mutant_locally() {
    let mutant = r#"
BoundedAccessPosture::Loading {
    identity,
    admitted_limit,
} => {
    return self.attach_bounded_waiter(&mut state, scope, key, identity);
}
BoundedAccessPosture::LoadFailed(terminal) => return Err(terminal),
"#;
    let denial = inspect_bounded_limit_identity(Path::new("controlled_mutant.rs"), mutant)
        .expect_err("cross-limit coalescence mutant must be denied");
    assert!(denial.contains("identical request limits"));
}

#[test]
fn rejected_bounded_completion_wakes_coalesced_participants() {
    let source = read(&workspace_root().join(BOUNDED_COMPLETION)).expect("read bounded completion");
    inspect_bounded_rejection_wakeup(Path::new(BOUNDED_COMPLETION), &source)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn fault_ownership_gate_kills_the_missing_terminal_wakeup_mutant_locally() {
    let mutant = r#"
fn reject_bounded_completion(&self, state: &mut PoolState) -> Result<(), Error> {
    let terminal = Self::fail_bounded_loading_state(state);
    Err(terminal)
}
"#;
    let denial = inspect_bounded_rejection_wakeup(Path::new("controlled_mutant.rs"), mutant)
        .expect_err("missing terminal-wakeup mutant must be denied");
    assert!(denial.contains("rejected bounded completion must wake"));
}

fn inspect_fault_ownership(path: &Path, source: &str) -> Result<(), String> {
    let loading = branch(
        source,
        "FrameAccessPosture::Loading",
        "FrameAccessPosture::LoadFailed",
    )
    .ok_or_else(|| format!("loading access branch missing in {}", path.display()))?;
    if !loading.contains("attach_loading_waiter") || loading.contains("reserve_loading") {
        return Err(format!(
            "physical residency boundary: loading access must attach one waiter in {}",
            path.display()
        ));
    }
    let absent = source
        .find("FrameAccessPosture::Absent")
        .map(|start| &source[start..])
        .ok_or_else(|| format!("absent access branch missing in {}", path.display()))?;
    if !absent.contains("reserve_loading") {
        return Err(format!(
            "physical residency boundary: absent access must reserve the sole fault owner in {}",
            path.display()
        ));
    }
    Ok(())
}

fn inspect_bounded_decision_before_source(path: &Path, source: &str) -> Result<(), String> {
    let bounded = bounded_loader_body(source);
    let access = bounded
        .find(".access_bounded_frame")
        .ok_or_else(|| format!("bounded pool access missing in {}", path.display()))?;
    let discoveries = bounded
        .match_indices("source.file_length")
        .collect::<Vec<_>>();
    if discoveries.len() != 1 {
        return Err(format!(
            "physical residency boundary: bounded fault must own exactly one length discovery in {}",
            path.display()
        ));
    }
    let discovery = discoveries[0].0;
    let fault = bounded
        .find("PhysicalBoundedFrameAccess::Fault")
        .ok_or_else(|| format!("bounded fault arm missing in {}", path.display()))?;
    if access >= discovery || fault >= discovery {
        return Err(format!(
            "physical residency boundary: bounded access must decide ownership before length discovery in {}",
            path.display()
        ));
    }
    Ok(())
}

fn inspect_bounded_rejection_wakeup(path: &Path, source: &str) -> Result<(), String> {
    let rejection = function_body(source, "fn reject_bounded_completion")
        .ok_or_else(|| format!("bounded completion rejection missing in {}", path.display()))?;
    if !rejection.contains("fail_bounded_loading_state")
        || !rejection.contains("changed.notify_all")
    {
        return Err(format!(
            "physical residency boundary: rejected bounded completion must wake coalesced participants in {}",
            path.display()
        ));
    }
    Ok(())
}

fn inspect_bounded_limit_identity(path: &Path, source: &str) -> Result<(), String> {
    let loading = branch(
        source,
        "BoundedAccessPosture::Loading",
        "BoundedAccessPosture::LoadFailed",
    )
    .ok_or_else(|| format!("bounded loading branch missing in {}", path.display()))?;
    let limit_check = loading.find("key.limit() != admitted_limit");
    let conflict = loading.find("BoundedLoadLimitConflict");
    let attach = loading.find("attach_bounded_waiter");
    if !matches!((limit_check, conflict, attach), (Some(limit), Some(conflict), Some(attach))
        if limit < conflict && conflict < attach)
    {
        return Err(format!(
            "physical residency boundary: bounded coalescence requires identical request limits in {}",
            path.display()
        ));
    }
    Ok(())
}

fn bounded_loader_body(source: &str) -> &str {
    let Some(start) = source.find("fn load_bounded(") else {
        return source;
    };
    let body = &source[start..];
    match body.find("\n    fn file_length(") {
        Some(end) => &body[..end],
        None => body,
    }
}

fn function_body<'source>(source: &'source str, signature: &str) -> Option<&'source str> {
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

fn branch<'source>(source: &'source str, start: &str, end: &str) -> Option<&'source str> {
    let start = source.find(start)?;
    let tail = &source[start..];
    let end = tail.find(end)?;
    Some(&tail[..end])
}
