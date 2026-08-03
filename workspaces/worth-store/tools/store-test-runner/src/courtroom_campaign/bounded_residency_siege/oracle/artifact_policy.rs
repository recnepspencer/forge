use std::collections::BTreeSet;

use super::super::offline_protocol::OfflineObservation;

#[cfg(test)]
#[path = "artifact_policy/tests.rs"]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DurableArtifactKind {
    NamespaceIdentity,
    MutationOwnerDiagnostic,
    BootstrapCatalog,
    RootManifest,
    RootRoutingBlock,
    Segment,
    SegmentManifest,
    SegmentMembershipBlock,
    Extent,
    ExtentManifest,
    FreeSpaceManifest,
    FreeSpaceMembershipBlock,
    WalSegment,
    Checkpoint,
    CheckpointCandidate,
    CatalogCandidate,
    PhysicalWorkRecoveryObligation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DurabilityArtifactManifestStage {
    CleanBaseline,
    PostBoundary,
}

impl DurableArtifactKind {
    const fn requires_post_boundary(self) -> bool {
        matches!(
            self,
            Self::CheckpointCandidate
                | Self::CatalogCandidate
                | Self::PhysicalWorkRecoveryObligation
        )
    }
}

pub(super) fn verify_artifact_manifest(offline: &OfflineObservation) -> Result<u64, String> {
    if offline.recovery_obligations() != 0 {
        return Err("Courtroom C offline manifest retained recovery work".to_owned());
    }
    verify_paths(offline.artifacts().iter().map(|artifact| artifact.path()))?;
    artifact_bytes(offline)
}

pub(super) fn verify_durability_artifact_manifest(
    offline: &OfflineObservation,
    stage: DurabilityArtifactManifestStage,
    expected_recovery_obligations: u64,
) -> Result<u64, String> {
    verify_recovery_obligation_count(
        offline.recovery_obligations(),
        expected_recovery_obligations,
        stage,
    )?;
    verify_paths_at_stage(
        offline.artifacts().iter().map(|artifact| artifact.path()),
        stage == DurabilityArtifactManifestStage::PostBoundary,
    )?;
    artifact_bytes(offline)
}

fn verify_recovery_obligation_count(
    observed: u64,
    expected: u64,
    stage: DurabilityArtifactManifestStage,
) -> Result<(), String> {
    if observed == expected {
        Ok(())
    } else {
        Err(format!(
            "C.7 artifact manifest carried {observed} recovery obligations; expected {expected} at {stage:?}"
        ))
    }
}

fn artifact_bytes(offline: &OfflineObservation) -> Result<u64, String> {
    offline
        .artifacts()
        .iter()
        .try_fold(0_u64, |total, artifact| {
            total
                .checked_add(artifact.byte_length())
                .ok_or_else(|| "Courtroom C artifact byte total overflowed".to_owned())
        })
}

fn verify_paths<'path>(paths: impl IntoIterator<Item = &'path str>) -> Result<(), String> {
    verify_paths_at_stage(paths, false)
}

fn verify_paths_at_stage<'path>(
    paths: impl IntoIterator<Item = &'path str>,
    allow_staged: bool,
) -> Result<(), String> {
    let mut observed = BTreeSet::new();
    for path in paths {
        let Some(kind) = classify(path) else {
            return Err(format!(
                "Courtroom C offline manifest retained forbidden artifact `{path}`"
            ));
        };
        if kind.requires_post_boundary() && !allow_staged {
            return Err(format!(
                "Courtroom C clean manifest retained staged artifact `{path}`"
            ));
        }
        if !observed.insert(path) {
            return Err(format!(
                "Courtroom C offline manifest duplicated artifact `{path}`"
            ));
        }
    }
    if observed.is_empty() {
        return Err("Courtroom C offline manifest omitted durable artifacts".to_owned());
    }
    Ok(())
}

fn classify(path: &str) -> Option<DurableArtifactKind> {
    match path {
        "namespace/identity" => return Some(DurableArtifactKind::NamespaceIdentity),
        "namespace/mutation.lock" => {
            return Some(DurableArtifactKind::MutationOwnerDiagnostic);
        }
        "families/records/bootstrap.catalog" => {
            return Some(DurableArtifactKind::BootstrapCatalog);
        }
        "families/checkpoint.current" => return Some(DurableArtifactKind::Checkpoint),
        _ => {}
    }

    if checkpoint_candidate(path) {
        return Some(DurableArtifactKind::CheckpointCandidate);
    }
    if catalog_candidate(path) {
        return Some(DurableArtifactKind::CatalogCandidate);
    }
    if wal_segment(path) {
        return Some(DurableArtifactKind::WalSegment);
    }
    if physical_work_recovery_obligation(path) {
        return Some(DurableArtifactKind::PhysicalWorkRecoveryObligation);
    }

    let (directory, file) = path.rsplit_once('/')?;
    match directory {
        "families/records/roots" if one_hex(file, "root-", ".manifest") => {
            Some(DurableArtifactKind::RootManifest)
        }
        "families/records/roots" if two_hex(file, "root-", "-block-", ".manifest") => {
            Some(DurableArtifactKind::RootRoutingBlock)
        }
        "families/records/segments" if two_hex(file, "segment-", "-", ".pages") => {
            Some(DurableArtifactKind::Segment)
        }
        "families/records/segment-manifests" if two_hex(file, "segment-", "-", ".manifest") => {
            Some(DurableArtifactKind::SegmentManifest)
        }
        "families/records/segment-manifests"
            if two_hex(file, "segments-", "-block-", ".manifest") =>
        {
            Some(DurableArtifactKind::SegmentMembershipBlock)
        }
        "families/records/extents" if two_hex(file, "extent-", "-", ".data") => {
            Some(DurableArtifactKind::Extent)
        }
        "families/records/extent-manifests" if two_hex(file, "extent-", "-", ".manifest") => {
            Some(DurableArtifactKind::ExtentManifest)
        }
        "families/records/free-space" if one_hex(file, "free-space-", ".manifest") => {
            Some(DurableArtifactKind::FreeSpaceManifest)
        }
        "families/records/free-space" if two_hex(file, "free-space-", "-block-", ".manifest") => {
            Some(DurableArtifactKind::FreeSpaceMembershipBlock)
        }
        _ => None,
    }
}

fn physical_work_recovery_obligation(path: &str) -> bool {
    let Some(body) = path
        .strip_prefix("families/physical-work/effect-")
        .and_then(|body| body.strip_suffix(".pending"))
    else {
        return false;
    };
    let identities = body.split('-').collect::<Vec<_>>();
    identities.len() == 3 && identities.into_iter().all(is_lower_hex_16)
}

fn checkpoint_candidate(path: &str) -> bool {
    path.strip_prefix("staging/checkpoint-")
        .and_then(|body| body.strip_suffix(".candidate"))
        .is_some_and(is_lower_hex_16)
}

fn catalog_candidate(path: &str) -> bool {
    path.strip_prefix("staging/records/bootstrap-")
        .and_then(|body| body.strip_suffix(".candidate"))
        .is_some_and(is_lower_hex_16)
}

fn wal_segment(path: &str) -> bool {
    let Some(body) = path
        .strip_prefix("families/wal/segment-")
        .and_then(|body| body.strip_suffix(".wal"))
    else {
        return false;
    };
    let Some((segment, generation)) = body.split_once("-generation-") else {
        return false;
    };
    decimal_identity(segment) && decimal_identity(generation)
}

fn decimal_identity(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit())
}

fn one_hex(file: &str, prefix: &str, suffix: &str) -> bool {
    file.strip_prefix(prefix)
        .and_then(|body| body.strip_suffix(suffix))
        .is_some_and(is_lower_hex_16)
}

fn two_hex(file: &str, prefix: &str, separator: &str, suffix: &str) -> bool {
    let Some(body) = file
        .strip_prefix(prefix)
        .and_then(|body| body.strip_suffix(suffix))
    else {
        return false;
    };
    let Some((first, second)) = body.split_once(separator) else {
        return false;
    };
    is_lower_hex_16(first) && is_lower_hex_16(second)
}

fn is_lower_hex_16(value: &str) -> bool {
    value.len() == 16
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
