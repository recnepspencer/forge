use std::path::Path;

use super::workspace_source::read;
use crate::workspace_root;

const CANDIDATE_ADMISSION: &str =
    "crates/worth-store-buffer-pool/src/physical_residency/pool/candidate_admission.rs";

#[test]
fn candidate_admission_preserves_exact_state_and_artifact_alias_meaning() {
    let source =
        read(&workspace_root().join(CANDIDATE_ADMISSION)).expect("read candidate admission");
    inspect_candidate_identity(Path::new(CANDIDATE_ADMISSION), &source)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn candidate_identity_gate_kills_collapsed_residency_mutant() {
    let mutant = r#"
fn admit_candidate_set(state: State, keys: Keys) {
    if state.frames.contains_key(key) {
        return Err(FrameAlreadyResident);
    }
    self.validate_candidate_capacity(state, scope, keys);
}
fn reserve_next_candidate(state: State, key: Key) {
    if state.frames.contains_artifact_alias(key.artifact()) {
        return Err(FrameAlreadyResident);
    }
    self.reserve_frame_space(state, scope, bytes);
}
"#;
    let denial = inspect_candidate_identity(Path::new("controlled_mutant.rs"), mutant)
        .expect_err("collapsed candidate identity mutant must be denied");
    assert!(denial.contains("shared identity classifier"));
}

fn inspect_candidate_identity(path: &Path, source: &str) -> Result<(), String> {
    inspect_classifier_use(path, source)?;
    let classifier =
        function_body(source, "fn validate_candidate_identity_available").ok_or_else(|| {
            format!(
                "candidate boundary: shared identity classifier missing in {}",
                path.display()
            )
        })?;
    let preserves_terminal = classifier.contains("FrameState::LoadFailed")
        && classifier.contains("PhysicalResidencyDenial::FrameLoadTerminated");
    let names_live_identity = classifier.contains("FrameState::Loading")
        && classifier.contains("FrameState::CandidateReserved")
        && classifier.contains("PhysicalResidencyDenial::FrameIdentityOccupied");
    let proves_residency = classifier.contains("FrameState::Resident")
        && classifier.contains("PhysicalResidencyDenial::FrameAlreadyResident");
    let names_alias = classifier.contains("contains_artifact_alias")
        && classifier.contains("PhysicalResidencyDenial::ArtifactIdentityOccupied");
    if !(preserves_terminal && names_live_identity && proves_residency && names_alias) {
        return Err(format!(
            "candidate boundary: identity classifier collapsed lifecycle or alias meaning in {}",
            path.display()
        ));
    }
    Ok(())
}

fn inspect_classifier_use(path: &Path, source: &str) -> Result<(), String> {
    let batch = function_body(source, "fn admit_candidate_set")
        .ok_or_else(|| format!("candidate batch admission missing in {}", path.display()))?;
    let frame = function_body(source, "fn reserve_next_candidate")
        .ok_or_else(|| format!("candidate frame admission missing in {}", path.display()))?;
    let batch_classification = batch.find("Self::validate_candidate_identity_available");
    let batch_capacity = batch.find("self.validate_candidate_capacity");
    let frame_classification = frame.find("Self::validate_candidate_identity_available");
    let frame_capacity = frame.find("self.reserve_frame_space");
    if !matches!(
        (batch_classification, batch_capacity),
        (Some(classify), Some(capacity)) if classify < capacity
    ) || !matches!(
        (frame_classification, frame_capacity),
        (Some(classify), Some(capacity)) if classify < capacity
    ) {
        return Err(format!(
            "candidate boundary: batch and frame admission must use the shared identity classifier before capacity mutation in {}",
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
