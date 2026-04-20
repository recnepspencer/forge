use crate::failure::{StoreError, StoreErrorKind};
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::core::BulkSourceMember;

pub const BULK_FAMILY_VERSION: u32 = 1;

pub(super) fn ensure_program_identity(program_id: &str, source_identity: &str) -> Result<(), StoreError> {
    if program_id.trim().is_empty() {
        return Err(StoreError::new(
            StoreErrorKind::BulkPlanDeterminismViolation,
            "bulk program id must be non-empty",
        ));
    }
    if source_identity.trim().is_empty() {
        return Err(StoreError::new(
            StoreErrorKind::BulkSourceIdentityUnavailable,
            "bulk source or transform identity must be non-empty",
        ));
    }
    Ok(())
}

pub(super) fn canonicalize_members(
    mut members: Vec<BulkSourceMember>,
) -> Result<Vec<BulkSourceMember>, StoreError> {
    if members.is_empty() {
        return Err(StoreError::new(
            StoreErrorKind::BulkPlanDeterminismViolation,
            "bulk planning requires at least one source or target member",
        ));
    }
    members.sort_by(|left, right| left.member_id.cmp(&right.member_id));
    let mut previous_member: Option<&str> = None;
    for member in &members {
        if member.member_id.trim().is_empty() {
            return Err(StoreError::new(
                StoreErrorKind::BulkPlanDeterminismViolation,
                "bulk members must declare non-empty identities",
            ));
        }
        if member.width_units == 0 {
            return Err(StoreError::new(
                StoreErrorKind::BulkPlanDeterminismViolation,
                format!(
                    "bulk member `{}` must declare positive width units",
                    member.member_id
                ),
            ));
        }
        if previous_member == Some(member.member_id()) {
            return Err(StoreError::new(
                StoreErrorKind::BulkPlanDeterminismViolation,
                format!(
                    "bulk member `{}` appears more than once in canonical ordering",
                    member.member_id
                ),
            ));
        }
        previous_member = Some(member.member_id());
    }
    Ok(members)
}

pub(super) fn stable_digest(value: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(value.as_bytes());
    format!("{:x}", digest.finalize())
}

pub(super) fn serialization_error(label: &'static str) -> impl FnOnce(serde_json::Error) -> StoreError {
    move |error| {
        StoreError::new(
            StoreErrorKind::Serialization,
            format!("failed to serialize {label}: {error}"),
        )
    }
}

pub(super) fn stable_json_digest<T: Serialize>(
    label: &'static str,
    input: &T,
) -> Result<String, StoreError> {
    let digest_basis = serde_json::to_string(input).map_err(serialization_error(label))?;
    Ok(stable_digest(&digest_basis))
}
