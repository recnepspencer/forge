use std::num::NonZeroU32;

use sha2::{Digest, Sha256};

use super::{
    artifact_oracle::{
        effect_artifacts, require_exact_publication_delta, require_exact_short_write_residue,
        validate_mutation_coordination,
    },
    offline_protocol::OfflineArtifactObservation,
};

#[test]
fn exact_process_bound_mutation_coordination_is_accepted() {
    let process = NonZeroU32::new(41).unwrap();
    let lock = mutation_lock(process);
    assert!(validate_mutation_coordination(&[lock], process, "fixture").is_ok());
}

#[test]
fn foreign_process_or_inexact_mutation_coordination_is_rejected() {
    let process = NonZeroU32::new(41).unwrap();
    let foreign = NonZeroU32::new(42).unwrap();
    let lock = mutation_lock(process);
    assert!(
        validate_mutation_coordination(std::slice::from_ref(&lock), foreign, "fixture").is_err()
    );

    let damaged = OfflineArtifactObservation::for_test(
        lock.path(),
        lock.byte_length(),
        [17; 32],
        lock.prefix(),
        false,
    );
    assert!(validate_mutation_coordination(&[damaged], process, "fixture").is_err());
}

#[test]
fn effect_comparison_excludes_only_known_coordination_and_recovery() {
    let process = NonZeroU32::new(41).unwrap();
    let lock = mutation_lock(process);
    let recovery = artifact("families/physical-work/work.pending", b"recovery", true);
    let effect = artifact("families/records/extent.data", b"media", false);
    let artifacts = [lock, recovery, effect.clone()];

    assert_eq!(effect_artifacts(&artifacts), [&effect]);
}

#[test]
fn short_write_requires_one_exact_residue_and_no_collateral_media() {
    let prior = artifact("families/records/existing.data", b"prior", false);
    let residue = artifact("families/records/residue.data", b"x", false);
    assert!(require_exact_short_write_residue(&[&prior], &[&prior, &residue]).is_ok());

    let extra = artifact("families/records/extra.data", b"y", false);
    assert!(require_exact_short_write_residue(&[&prior], &[&prior, &residue, &extra]).is_err());

    let modified = artifact("families/records/existing.data", b"changed", false);
    assert!(require_exact_short_write_residue(&[&prior], &[&modified, &residue]).is_err());
    assert!(require_exact_short_write_residue(&[&prior], &[&residue]).is_err());
}

#[test]
fn publication_requires_the_exact_one_append_artifact_delta() {
    let catalog = artifact("families/records/bootstrap.catalog", b"old", false);
    let immutable = artifact("families/records/extents/extent-old.data", b"prior", false);
    let baseline = [&catalog, &immutable];
    let replacement = artifact("families/records/bootstrap.catalog", b"new", false);
    let mut observed = vec![replacement, immutable.clone()];
    observed.extend(publication_additions(3));
    let observed_refs = observed.iter().collect::<Vec<_>>();
    assert!(require_exact_publication_delta(&baseline, &observed_refs, 3).is_ok());

    let mut collateral = observed.clone();
    collateral[1] = artifact(
        "families/records/extents/extent-old.data",
        b"damaged",
        false,
    );
    let collateral_refs = collateral.iter().collect::<Vec<_>>();
    assert!(require_exact_publication_delta(&baseline, &collateral_refs, 3).is_err());

    let mut extra = observed.clone();
    extra.push(artifact("families/records/foreign.data", b"x", false));
    let extra_refs = extra.iter().collect::<Vec<_>>();
    assert!(require_exact_publication_delta(&baseline, &extra_refs, 3).is_err());

    observed.pop();
    let missing_refs = observed.iter().collect::<Vec<_>>();
    assert!(require_exact_publication_delta(&baseline, &missing_refs, 3).is_err());
}

fn mutation_lock(process: NonZeroU32) -> OfflineArtifactObservation {
    let bytes = format!(
        "version=1\nprocess={:010}\nruntime={}\nattempt={}\n",
        process.get(),
        "11".repeat(16),
        "22".repeat(16),
    );
    artifact("namespace/mutation.lock", bytes.as_bytes(), false)
}

fn artifact(path: &str, bytes: &[u8], recovery: bool) -> OfflineArtifactObservation {
    OfflineArtifactObservation::for_test(
        path,
        bytes.len() as u64,
        Sha256::digest(bytes).into(),
        bytes,
        recovery,
    )
}

fn publication_additions(generation: u64) -> Vec<OfflineArtifactObservation> {
    [
        "families/records/extents/extent-0000000000000002-0000000000000001.data".to_owned(),
        "families/records/extent-manifests/extent-0000000000000002-0000000000000001.manifest"
            .to_owned(),
        format!("families/records/free-space/free-space-{generation:016x}.manifest"),
        format!(
            "families/records/free-space/free-space-{generation:016x}-block-0000000000000003.manifest"
        ),
        format!("families/records/roots/root-{generation:016x}.manifest"),
        format!(
            "families/records/roots/root-{generation:016x}-block-0000000000000002.manifest"
        ),
    ]
    .into_iter()
    .map(|path| artifact(&path, path.as_bytes(), false))
    .collect()
}
