use std::collections::{BTreeMap, BTreeSet};

use super::artifact_binding::BoundArtifact;
use super::targets::{
    AuthorityLane, ObserverProcessRole, Recipe, RecoveryProcessRole, TargetSpec, WriterProcessRole,
};

const AUTHORITY_FEATURES: [&str; 2] = ["certification-test-authority", "recovery-runtime-owner"];

pub(crate) fn verify(
    recipe: &Recipe,
    writer: &BoundArtifact<WriterProcessRole>,
    observer: &BoundArtifact<ObserverProcessRole>,
    recovery: &BoundArtifact<RecoveryProcessRole>,
) -> Result<(), String> {
    verify_target(&recipe.targets.writer, writer)?;
    verify_target(&recipe.targets.observer, observer)?;
    verify_target(&recipe.targets.recovery, recovery)
}

fn verify_target<R>(target: &TargetSpec<R>, artifact: &BoundArtifact<R>) -> Result<(), String> {
    let observed = compiler_feature_map(artifact);
    let target_features = observed
        .get(&(target.package.to_owned(), target.binary.to_owned()))
        .ok_or_else(|| format!("Cargo omitted target feature set for {}", target.binary))?;
    let expected_target = expected_target_features(target.lane);
    if target_features != &expected_target {
        return Err(format!(
            "{} target feature set drifted: actual={target_features:?} expected={expected_target:?}",
            target.binary
        ));
    }
    let projection = authority_projection(&observed);
    let expected_projection = expected_projection(target.lane);
    if projection != expected_projection {
        return Err(format!(
            "{} authority feature projection drifted: actual={projection:?} expected={expected_projection:?}",
            target.binary
        ));
    }
    Ok(())
}

fn compiler_feature_map<R>(
    artifact: &BoundArtifact<R>,
) -> BTreeMap<(String, String), BTreeSet<String>> {
    artifact
        .compiler_artifacts()
        .iter()
        .map(|record| {
            (
                (record.package().to_owned(), record.target().to_owned()),
                record.features().iter().cloned().collect(),
            )
        })
        .collect()
}

fn expected_target_features(lane: AuthorityLane) -> BTreeSet<String> {
    match lane {
        AuthorityLane::Ordinary => BTreeSet::new(),
        AuthorityLane::Recovery | AuthorityLane::CourtroomWriter => {
            BTreeSet::from(["certification-test-authority".to_owned()])
        }
    }
}

fn authority_projection(
    observed: &BTreeMap<(String, String), BTreeSet<String>>,
) -> BTreeMap<String, BTreeSet<String>> {
    let mut projection = BTreeMap::new();
    for ((package, _), features) in observed {
        let authority = features
            .iter()
            .filter(|feature| AUTHORITY_FEATURES.contains(&feature.as_str()))
            .cloned()
            .collect::<BTreeSet<_>>();
        if !authority.is_empty() {
            projection
                .entry(package.clone())
                .or_insert_with(BTreeSet::new)
                .extend(authority);
        }
    }
    projection
}

fn expected_projection(lane: AuthorityLane) -> BTreeMap<String, BTreeSet<String>> {
    match lane {
        AuthorityLane::Ordinary => BTreeMap::new(),
        AuthorityLane::CourtroomWriter => [
            (
                "worth-store".to_owned(),
                BTreeSet::from(["certification-test-authority".to_owned()]),
            ),
            (
                "worth-store-physical-backend".to_owned(),
                BTreeSet::from(["certification-test-authority".to_owned()]),
            ),
            (
                "worth-store-buffer-pool".to_owned(),
                BTreeSet::from(["certification-test-authority".to_owned()]),
            ),
            (
                "worth-store-io-scheduler".to_owned(),
                BTreeSet::from(["certification-test-authority".to_owned()]),
            ),
            (
                "worth-store-security".to_owned(),
                BTreeSet::from(["certification-test-authority".to_owned()]),
            ),
        ]
        .into_iter()
        .collect(),
        AuthorityLane::Recovery => [
            (
                "worth-store".to_owned(),
                BTreeSet::from([
                    "certification-test-authority".to_owned(),
                    "recovery-runtime-owner".to_owned(),
                ]),
            ),
            (
                "worth-store-physical-backend".to_owned(),
                BTreeSet::from([
                    "certification-test-authority".to_owned(),
                    "recovery-runtime-owner".to_owned(),
                ]),
            ),
            (
                "worth-store-buffer-pool".to_owned(),
                BTreeSet::from(["certification-test-authority".to_owned()]),
            ),
            (
                "worth-store-io-scheduler".to_owned(),
                BTreeSet::from(["certification-test-authority".to_owned()]),
            ),
            (
                "worth-store-security".to_owned(),
                BTreeSet::from(["certification-test-authority".to_owned()]),
            ),
            (
                "worth-store-recovery-runtime".to_owned(),
                BTreeSet::from(["certification-test-authority".to_owned()]),
            ),
        ]
        .into_iter()
        .collect(),
    }
}
