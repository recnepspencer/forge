use std::path::Path;

use super::workspace_source::read;
use crate::workspace_root;

const IDENTITY_TRANSITION: &str =
    "crates/worth-store-buffer-pool/src/physical_residency/pool/identity_transition.rs";

#[test]
fn clean_invalidation_requires_resident_lifecycle_before_mutation() {
    let source =
        read(&workspace_root().join(IDENTITY_TRANSITION)).expect("read identity transition");
    inspect_clean_invalidation(Path::new(IDENTITY_TRANSITION), &source)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn clean_invalidation_gate_kills_pins_and_dirty_only_mutant() {
    let mutant = r#"
fn invalidate_clean(&self, key: Key) {
    if entry.pins != 0 { return Err(FramePinned); }
    if entry.dirty { return Err(FrameDirty); }
    state.detach_evictable(key.coordinate);
    state.frames.remove(&key.coordinate);
}
"#;
    let denial = inspect_clean_invalidation(Path::new("controlled_mutant.rs"), mutant)
        .expect_err("pins-and-dirty-only invalidation mutant must be denied");
    assert!(denial.contains("resident lifecycle"));
}

#[test]
fn clean_identity_promotion_preflights_coverage_alias_and_target_lifecycle() {
    let source =
        read(&workspace_root().join(IDENTITY_TRANSITION)).expect("read identity transition");
    inspect_clean_identity_promotion(Path::new(IDENTITY_TRANSITION), &source)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn identity_transition_gate_kills_nonzero_complete_target_mutant() {
    let mutant = r#"
fn promote_clean_identity(&self, source: Key, target: Key) {
    Self::validate_promotion_source(state, source, target);
    Self::validate_promotion_target(state, target);
    Self::apply_clean_identity_promotion(state, source, target);
}
fn validate_promotion_source(state: State, source: Key, target: Key) {
    let source_is_complete = true;
}
fn validate_promotion_target(state: State, target: Key) {
    FrameState::LoadFailed(terminal) => FrameLoadTerminated(terminal),
    FrameState::Loading | FrameState::CandidateReserved => FrameIdentityOccupied,
    FrameState::Resident(_) => {}
}
fn apply_clean_identity_promotion(state: State, source: Key, target: Key) {
    state.detach_evictable(source.coordinate);
    state.frames.remove(&source.coordinate);
}
"#;
    let denial = inspect_clean_identity_promotion(Path::new("controlled_mutant.rs"), mutant)
        .expect_err("nonzero complete-target mutant must be denied");
    assert!(denial.contains("offset-zero preflight"));
}

#[test]
fn identity_transition_gate_kills_occupied_alias_mutant() {
    let mutant = r#"
fn promote_clean_identity(&self, source: Key, target: Key) {
    Self::validate_promotion_source(state, source, target);
    Self::validate_promotion_target(state, target);
    Self::apply_clean_identity_promotion(state, source, target);
}
fn validate_promotion_source(state: State, source: Key, target: Key) {
    let source_is_complete = true;
    if source_is_complete && target.coordinate.offset() != 0 {
        return Err(CompleteArtifactRequiresOffsetZero);
    }
}
fn validate_promotion_target(state: State, target: Key) {
    FrameState::LoadFailed(terminal) => FrameLoadTerminated(terminal),
    FrameState::Loading | FrameState::CandidateReserved => FrameIdentityOccupied,
    FrameState::Resident(_) => {}
}
fn apply_clean_identity_promotion(state: State, source: Key, target: Key) {
    state.detach_evictable(source.coordinate);
    state.frames.remove(&source.coordinate);
}
"#;
    let denial = inspect_clean_identity_promotion(Path::new("controlled_mutant.rs"), mutant)
        .expect_err("occupied target-alias mutant must be denied");
    assert!(denial.contains("artifact-alias preflight"));
}

#[test]
fn identity_transition_gate_kills_pins_and_dirty_only_target_mutant() {
    let mutant = r#"
fn promote_clean_identity(&self, source: Key, target: Key) {
    Self::validate_promotion_source(state, source, target);
    Self::validate_promotion_target(state, target);
    Self::apply_clean_identity_promotion(state, source, target);
}
fn validate_promotion_source(state: State, source: Key, target: Key) {
    CompleteArtifactRequiresOffsetZero;
    state.frames.contains_artifact_alias(target.artifact);
    ArtifactIdentityOccupied;
}
fn validate_promotion_target(state: State, target: Key) {
    if target_entry.pins != 0 { return Err(FramePinned); }
    if target_entry.dirty { return Err(FrameDirty); }
}
fn apply_clean_identity_promotion(state: State, source: Key, target: Key) {
    state.detach_evictable(source.coordinate);
}
"#;
    let denial = inspect_clean_identity_promotion(Path::new("controlled_mutant.rs"), mutant)
        .expect_err("pins-and-dirty-only target mutant must be denied");
    assert!(denial.contains("target lifecycle preflight"));
}

fn inspect_clean_identity_promotion(path: &Path, source: &str) -> Result<(), String> {
    let body = function_body(source, "fn promote_clean_identity").ok_or_else(|| {
        format!(
            "physical residency boundary: clean identity promotion missing in {}",
            path.display()
        )
    })?;
    let source_validation = body.find("Self::validate_promotion_source");
    let target_validation = body.find("Self::validate_promotion_target");
    let application = body.find("Self::apply_clean_identity_promotion");
    if !matches!(
        (source_validation, target_validation, application),
        (Some(source), Some(target), Some(apply)) if source < target && target < apply
    ) {
        return Err(format!(
            "physical residency boundary: identity validation must precede mutation in {}",
            path.display()
        ));
    }
    inspect_source_preflight(path, source)?;
    inspect_target_preflight(path, source)?;
    let apply_body = function_body(source, "fn apply_clean_identity_promotion")
        .ok_or_else(|| format!("identity mutation boundary missing in {}", path.display()))?;
    if !apply_body.contains("state.detach_evictable(source.coordinate)") {
        return Err(format!(
            "identity mutation boundary missing in {}",
            path.display()
        ));
    }
    Ok(())
}

fn inspect_clean_invalidation(path: &Path, source: &str) -> Result<(), String> {
    let body = function_body(source, "fn invalidate_clean").ok_or_else(|| {
        format!(
            "physical residency boundary: clean invalidation missing in {}",
            path.display()
        )
    })?;
    let validation = body.find("Self::validate_clean_invalidation");
    let mutation = body.find("state.detach_evictable");
    if !matches!((validation, mutation), (Some(validate), Some(mutate)) if validate < mutate) {
        return Err(format!(
            "physical residency boundary: clean invalidation must prove resident lifecycle before mutation in {}",
            path.display()
        ));
    }
    let predicate = function_body(source, "fn validate_clean_invalidation")
        .ok_or_else(|| format!("clean invalidation predicate missing in {}", path.display()))?;
    let preserves_terminal = predicate.contains("FrameState::LoadFailed")
        && predicate.contains("PhysicalResidencyDenial::FrameLoadTerminated");
    let protects_in_progress = predicate.contains("FrameState::Loading")
        && predicate.contains("FrameState::CandidateReserved")
        && predicate.contains("PhysicalResidencyDenial::FrameIdentityOccupied");
    let proves_resident = predicate.contains("FrameState::Resident");
    if !(preserves_terminal && protects_in_progress && proves_resident) {
        return Err(format!(
            "physical residency boundary: clean invalidation lost its resident lifecycle predicate in {}",
            path.display()
        ));
    }
    Ok(())
}

fn inspect_source_preflight(path: &Path, source: &str) -> Result<(), String> {
    let body = function_body(source, "fn validate_promotion_source")
        .ok_or_else(|| format!("identity source preflight missing in {}", path.display()))?;
    if !body.contains("CompleteArtifactRequiresOffsetZero") {
        return Err(format!(
            "physical residency boundary: complete-artifact promotion lost its offset-zero preflight in {}",
            path.display()
        ));
    }
    let alias_lookup = body.find("contains_artifact_alias");
    let alias_denial = body.find("ArtifactIdentityOccupied");
    if !matches!((alias_lookup, alias_denial), (Some(lookup), Some(denial)) if lookup < denial) {
        return Err(format!(
            "physical residency boundary: complete-artifact promotion lost its artifact-alias preflight in {}",
            path.display()
        ));
    }
    Ok(())
}

fn inspect_target_preflight(path: &Path, source: &str) -> Result<(), String> {
    let body = function_body(source, "fn validate_promotion_target")
        .ok_or_else(|| format!("identity target preflight missing in {}", path.display()))?;
    let proves_failed_terminal = body.contains("FrameState::LoadFailed")
        && body.contains("PhysicalResidencyDenial::FrameLoadTerminated");
    let proves_in_progress_authority = body.contains("FrameState::Loading")
        && body.contains("FrameState::CandidateReserved")
        && body.contains("PhysicalResidencyDenial::FrameIdentityOccupied");
    let proves_resident_replacement = body.contains("FrameState::Resident");
    if !(proves_failed_terminal && proves_in_progress_authority && proves_resident_replacement) {
        return Err(format!(
            "physical residency boundary: clean identity promotion lost its target lifecycle preflight in {}",
            path.display()
        ));
    }
    Ok(())
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
