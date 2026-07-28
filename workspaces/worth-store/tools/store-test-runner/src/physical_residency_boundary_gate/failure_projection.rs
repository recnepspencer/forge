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

#[test]
fn failure_projection_gate_reads_forwarding_semantically_across_layouts() {
    let compact = r#"
pub const fn reason(self) -> PhysicalRecordResidencyFailureReason {
    match self.denial {
        PhysicalResidencyDenial::BoundedLoadLimitConflict { active_limit, requested_limit } =>
            PhysicalRecordResidencyFailureReason::BoundedLoadLimitConflict {
                active_limit, requested_limit,
            },
        PhysicalResidencyDenial::CandidateCardinalityMismatch {
            declared,
            provided,
        } => PhysicalRecordResidencyFailureReason::CandidateCardinalityMismatch {
            declared,
            provided,
        },
    }
}
"#;
    require_forwarded_fields(
        Path::new("compact.rs"),
        required_body((Path::new("compact.rs"), compact), "pub const fn reason").unwrap(),
        "BoundedLoadLimitConflict",
        &["active_limit", "requested_limit"],
    )
    .unwrap();

    let dropped = compact.replace(
        "active_limit, requested_limit,",
        "active_limit, requested_limit: 1,",
    );
    let body = required_body((Path::new("dropped.rs"), &dropped), "pub const fn reason").unwrap();
    let denial = require_forwarded_fields(
        Path::new("dropped.rs"),
        body,
        "BoundedLoadLimitConflict",
        &["active_limit", "requested_limit"],
    )
    .expect_err("a fixed replacement must not count as forwarded conflict evidence");
    assert!(denial.contains("does not forward `requested_limit`"));
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
        (
            "CandidateCleanAuthorityMismatch",
            "CandidateCleanAuthorityMismatch",
        ),
        (
            "WritebackCleanAuthorityMismatch",
            "WritebackCleanAuthorityMismatch",
        ),
    ] {
        require_exact_arm(source.0, body, denial, reason)?;
    }
    require_forwarded_fields(
        source.0,
        body,
        "BoundedLoadLimitConflict",
        &["active_limit", "requested_limit"],
    )?;
    require_forwarded_fields(
        source.0,
        body,
        "CandidateCardinalityMismatch",
        &["declared", "provided"],
    )?;
    Ok(())
}

fn require_exact_arm(path: &Path, body: &str, denial: &str, reason: &str) -> Result<(), String> {
    let arm = exact_arm(path, body, denial)?;
    let reason_needle = format!("PhysicalRecordResidencyFailureReason::{reason}");
    if !arm.contains(&reason_needle) {
        return Err(format!(
            "failure projection: `{denial}` lacks its exact Store reason in {}",
            path.display()
        ));
    }
    Ok(())
}

fn require_forwarded_fields(
    path: &Path,
    body: &str,
    denial: &str,
    fields: &[&str],
) -> Result<(), String> {
    let arm = exact_arm(path, body, denial)?;
    let (_, projection) = arm.split_once("=>").ok_or_else(|| {
        format!(
            "failure projection: `{denial}` has no projection arm in {}",
            path.display()
        )
    })?;
    for field in fields {
        if !contains_shorthand_field(projection, field) {
            return Err(format!(
                "failure projection: `{denial}` does not forward `{field}` into its Store reason in {}",
                path.display()
            ));
        }
    }
    Ok(())
}

fn contains_shorthand_field(projection: &str, field: &str) -> bool {
    projection.match_indices(field).any(|(start, _)| {
        let before = projection[..start].chars().next_back();
        let after = projection[start + field.len()..].chars().next();
        let identifier_boundary = |value: Option<char>| {
            value.is_none_or(|character| !character.is_ascii_alphanumeric() && character != '_')
        };
        if !identifier_boundary(before) || !identifier_boundary(after) {
            return false;
        }
        !projection[start + field.len()..]
            .trim_start()
            .starts_with(':')
    })
}

fn exact_arm<'body>(path: &Path, body: &'body str, denial: &str) -> Result<&'body str, String> {
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
    Ok(&tail[..end])
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
