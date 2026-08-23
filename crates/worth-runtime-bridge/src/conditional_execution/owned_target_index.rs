use std::collections::BTreeMap;

use super::{
    BridgeConditionalDenial, BridgeConditionalDenialKind, BridgeInstalledConditionalLowering,
};
use crate::correspondence::{
    BridgeSemanticDependencyCandidate, InstalledCorrespondenceTarget, ProvenCorrespondenceTargets,
};

#[derive(Default)]
pub(super) struct BridgeOwnedConditionalTargetIndex {
    by_dependency: BTreeMap<String, BridgeOwnedConditionalTargetBucket>,
}

struct BridgeOwnedConditionalTargetBucket {
    dependency: BridgeSemanticDependencyCandidate,
    targets: BTreeMap<InstalledCorrespondenceTarget, usize>,
}

impl BridgeOwnedConditionalTargetIndex {
    pub(super) fn register(&mut self, lowering: &BridgeInstalledConditionalLowering) {
        for correspondence in &lowering.correspondences {
            let dependency = correspondence.dependency();
            let key = dependency.canonical_registration_key();
            let bucket = self.by_dependency.entry(key).or_insert_with(|| {
                BridgeOwnedConditionalTargetBucket {
                    dependency: dependency.clone(),
                    targets: BTreeMap::new(),
                }
            });
            assert_eq!(
                bucket.dependency, *dependency,
                "canonical semantic dependency identity must retain one exact meaning",
            );
            for target in correspondence.targets.as_slice() {
                *bucket.targets.entry(target.clone()).or_default() += 1;
            }
        }
    }

    pub(super) fn unregister(&mut self, lowering: &BridgeInstalledConditionalLowering) {
        let mut empty = Vec::new();
        for correspondence in &lowering.correspondences {
            let dependency = correspondence.dependency();
            let key = dependency.canonical_registration_key();
            let bucket = self
                .by_dependency
                .get_mut(&key)
                .expect("installed owned lowering remains indexed until retirement");
            assert_eq!(bucket.dependency, *dependency);
            for target in correspondence.targets.as_slice() {
                let references = bucket
                    .targets
                    .get_mut(target)
                    .expect("installed owned target remains indexed until retirement");
                if *references == 1 {
                    bucket.targets.remove(target);
                } else {
                    *references -= 1;
                }
            }
            if bucket.targets.is_empty() {
                empty.push(key);
            }
        }
        for key in empty {
            self.by_dependency.remove(&key);
        }
    }

    pub(super) fn resolve(
        &self,
        dependency: &BridgeSemanticDependencyCandidate,
    ) -> Result<ProvenCorrespondenceTargets, BridgeConditionalDenial> {
        let key = dependency.canonical_registration_key();
        let bucket = self
            .by_dependency
            .get(&key)
            .filter(|bucket| bucket.dependency == *dependency)
            .ok_or_else(|| {
                BridgeConditionalDenial::new(
                    BridgeConditionalDenialKind::DeclarationCorrespondenceMismatch,
                    "owned semantic dependency has no current target registration",
                )
            })?;
        ProvenCorrespondenceTargets::admit(bucket.targets.keys().cloned().collect()).map_err(
            |kind| {
                BridgeConditionalDenial::new(
                    BridgeConditionalDenialKind::CorrespondenceAdmission,
                    format!("owned semantic target index was denied: {kind:?}"),
                )
            },
        )
    }
}
