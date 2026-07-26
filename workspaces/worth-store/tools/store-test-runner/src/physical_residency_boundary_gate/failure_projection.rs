use std::path::Path;

use super::workspace_source::read;
use crate::workspace_root;

const STORE_FAILURE: &str =
    "crates/worth-store/src/physical_runtime/record_serving/residency/failure.rs";

#[test]
fn store_failure_projection_preserves_exact_causal_reasons() {
    let root = workspace_root();
    let path = root.join(STORE_FAILURE);
    let source = read(&path).expect("read Store residency failure projection");
    inspect_failure_projection((&path, &source)).unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn failure_projection_gate_kills_declaration_occupancy_collapse_mutant() {
    let mutant = r#"
pub enum PhysicalRecordResidencyFailureReason {
    FrameIdentityOccupied,
}
pub const fn reason(self) -> PhysicalRecordResidencyFailureReason {
    match self.denial {
        PhysicalResidencyDenial::CompleteArtifactRequiresOffsetZero => {
            PhysicalRecordResidencyFailureReason::FrameIdentityOccupied
        }
        PhysicalResidencyDenial::ArtifactIdentityOccupied => {
            PhysicalRecordResidencyFailureReason::FrameIdentityOccupied
        }
        PhysicalResidencyDenial::FrameIdentityOccupied => {
            PhysicalRecordResidencyFailureReason::FrameIdentityOccupied
        }
        PhysicalResidencyDenial::FrameLengthMismatch => {
            PhysicalRecordResidencyFailureReason::FrameIdentityOccupied
        }
        PhysicalResidencyDenial::FramePinned => {
            PhysicalRecordResidencyFailureReason::FrameIdentityOccupied
        }
        PhysicalResidencyDenial::WriteBackFrameAlreadyClaimed => {
            PhysicalRecordResidencyFailureReason::FrameIdentityOccupied
        }
        PhysicalResidencyDenial::WriteBackReceiptMismatch => {
            PhysicalRecordResidencyFailureReason::FrameIdentityOccupied
        }
    }
}
"#;
    let denial = inspect_failure_projection((Path::new("failure.rs"), mutant))
        .expect_err("collapsed causal-reason mutant must be denied");
    assert!(denial.contains("exact Store reason"));
}

fn inspect_failure_projection(source: (&Path, &str)) -> Result<(), String> {
    let body = required_body(source, "pub const fn reason")?;
    for (denial, reason) in [
        (
            "CompleteArtifactRequiresOffsetZero",
            "CompleteArtifactRequiresOffsetZero",
        ),
        ("ArtifactIdentityOccupied", "ArtifactIdentityOccupied"),
        ("FrameIdentityOccupied", "FrameIdentityOccupied"),
        ("IdentityAlreadyCurrent", "IdentityAlreadyCurrent"),
        ("FrameLengthMismatch", "FrameLengthMismatch"),
        ("FramePinned", "FramePinned"),
        ("FrameDirty", "FrameDirty"),
        (
            "WriteBackFrameAlreadyClaimed",
            "WritebackFrameAlreadyClaimed",
        ),
        ("WriteBackReceiptMismatch", "WritebackReceiptMismatch"),
    ] {
        require_exact_arm(source.0, body, denial, reason)?;
    }
    for field in [
        "active_limit,\n                requested_limit,",
        "declared,\n                    provided,",
    ] {
        if !body.contains(field) {
            return Err(format!(
                "failure projection: actionable conflict fields are not retained in {}",
                source.0.display()
            ));
        }
    }
    Ok(())
}

fn require_exact_arm(path: &Path, body: &str, denial: &str, reason: &str) -> Result<(), String> {
    let denial_needle = format!("PhysicalResidencyDenial::{denial}");
    let start = body.find(&denial_needle).ok_or_else(|| {
        format!(
            "failure projection: lower denial `{denial}` missing in {}",
            path.display()
        )
    })?;
    let tail = &body[start..];
    let end = tail[denial_needle.len()..]
        .find("PhysicalResidencyDenial::")
        .map_or(tail.len(), |offset| denial_needle.len() + offset);
    let arm = &tail[..end];
    let reason_needle = format!("PhysicalRecordResidencyFailureReason::{reason}");
    if !arm.contains(&reason_needle) {
        return Err(format!(
            "failure projection: `{denial}` lacks its exact Store reason in {}",
            path.display()
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
            "failure projection: `{signature}` missing in {}",
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
