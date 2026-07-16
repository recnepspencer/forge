use std::path::Path;

use sha2::{Digest, Sha256};
use worth_store_physical_backend::NonCurrentStagingArtifact;

use super::authority_affecting::{
    AuthorityAffectingRepairLoweringDenial, AuthorityAffectingStagedRepairPlan,
};
use crate::OperationalOperationId;

pub(super) fn staging_artifacts(
    plan: &AuthorityAffectingStagedRepairPlan,
) -> Result<Vec<NonCurrentStagingArtifact>, AuthorityAffectingRepairLoweringDenial> {
    let materialized = plan.backup.custody().structural().materialized();
    let mut artifacts = Vec::new();
    artifacts
        .try_reserve_exact(materialized.manifest().artifacts().len().saturating_add(1))
        .map_err(|_| AuthorityAffectingRepairLoweringDenial::AllocationFailed)?;
    let manifest_bytes = std::fs::metadata(materialized.root().join("backup.manifest"))
        .map_err(
            |_| AuthorityAffectingRepairLoweringDenial::SourceArtifactUnavailable {
                output_name: "backup.manifest".into(),
            },
        )?
        .len();
    artifacts.push(
        NonCurrentStagingArtifact::admit(
            "backup.manifest",
            manifest_bytes,
            materialized.manifest_digest(),
        )
        .ok_or_else(
            || AuthorityAffectingRepairLoweringDenial::InvalidSourceArtifact {
                output_name: "backup.manifest".into(),
            },
        )?,
    );
    for row in materialized.manifest().artifacts() {
        artifacts.push(
            NonCurrentStagingArtifact::admit(row.output_name(), row.bytes(), row.content_digest())
                .ok_or_else(
                    || AuthorityAffectingRepairLoweringDenial::InvalidSourceArtifact {
                        output_name: row.output_name().to_owned(),
                    },
                )?,
        );
    }
    Ok(artifacts)
}

pub(super) fn operation_identity(operation: &OperationalOperationId) -> [u8; 32] {
    identity(
        b"worth-store-authority-repair-operation-v1",
        Sha256::digest(operation.as_str()).into(),
    )
}

pub(super) fn path_identity(path: &Path) -> [u8; 32] {
    identity(
        b"worth-store-authority-repair-target-v1",
        Sha256::digest(path.as_os_str().to_string_lossy().as_bytes()).into(),
    )
}

fn identity(domain: &[u8], value: [u8; 32]) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(domain);
    digest.update(value);
    digest.finalize().into()
}
