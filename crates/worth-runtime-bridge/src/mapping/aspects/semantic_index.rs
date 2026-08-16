use std::collections::{BTreeMap, BTreeSet};

use worth_foundational::facade::AspectKey;

use crate::mapping::{AspectKeySelector, MappingSelector, TruthPatchTargetSelector};

use super::FrozenAspectRegistration;

/// Private candidate-selection key for installed Bridge mappings.
///
/// This index is non-authoritative: every returned registration is still
/// validated by correspondence admission. It only prevents unrelated mapping
/// families from entering that validation frontier.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct BridgeAspectSemanticIndexKey {
    entity: MappingSelector,
    aspect: AspectKeySelector,
    target: TruthPatchTargetSelector,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct BridgeAspectSemanticCandidateIndex {
    buckets: BTreeMap<BridgeAspectSemanticIndexKey, Vec<usize>>,
}

impl BridgeAspectSemanticCandidateIndex {
    pub(super) fn build(registrations: &[FrozenAspectRegistration]) -> Self {
        let mut buckets = BTreeMap::<BridgeAspectSemanticIndexKey, Vec<usize>>::new();
        for (index, registration) in registrations.iter().enumerate() {
            let scope = registration.truth_scope();
            buckets
                .entry(BridgeAspectSemanticIndexKey {
                    entity: scope.entity_selector().clone(),
                    aspect: scope.aspect_selector().clone(),
                    target: scope.target_selector().clone(),
                })
                .or_default()
                .push(index);
        }
        Self { buckets }
    }

    pub(super) fn candidates(
        &self,
        entity: Option<&str>,
        aspect: &AspectKey,
        targets: &[TruthPatchTargetSelector],
    ) -> Vec<usize> {
        let mut entities = vec![MappingSelector::Any];
        if let Some(entity) = entity {
            entities.push(MappingSelector::exact(entity));
        }
        let aspects = [
            AspectKeySelector::Any,
            AspectKeySelector::exact(aspect.clone()),
        ];
        let mut target_selectors = vec![TruthPatchTargetSelector::Any];
        target_selectors.extend_from_slice(targets);
        target_selectors.sort();
        target_selectors.dedup();

        let mut candidates = BTreeSet::new();
        for entity in entities {
            for aspect in &aspects {
                for target in &target_selectors {
                    let key = BridgeAspectSemanticIndexKey {
                        entity: entity.clone(),
                        aspect: aspect.clone(),
                        target: target.clone(),
                    };
                    if let Some(indices) = self.buckets.get(&key) {
                        candidates.extend(indices.iter().copied());
                    }
                }
            }
        }
        candidates.into_iter().collect()
    }
}
