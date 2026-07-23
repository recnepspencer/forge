use std::collections::BTreeMap;

use worth_foundational::facade::{AspectKey, CanonicalFieldPath};

use super::super::{WorthQueryNativeAccessKey, WorthQueryNativeFactLane};

type KeyCoordinate = (WorthQueryNativeFactLane, usize);

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct WorthQueryNativeTouchCoordinate {
    pub(crate) aspect_key: AspectKey,
    pub(crate) field_path: Option<CanonicalFieldPath>,
}

pub(crate) struct WorthQueryAffectedNativeKeyIndex {
    by_aspect: BTreeMap<AspectKey, Vec<WorthQueryNativeAccessKey>>,
    whole_aspect: BTreeMap<AspectKey, Vec<WorthQueryNativeAccessKey>>,
    by_field: BTreeMap<
        AspectKey,
        crate::canonical_field_path_overlap_index::WorthQueryCanonicalPathOverlapIndex<
            KeyCoordinate,
        >,
    >,
    keys: BTreeMap<KeyCoordinate, WorthQueryNativeAccessKey>,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct WorthQueryNativeKeyNarrowingCounters {
    pub(crate) index_lookups: usize,
    pub(crate) targeted_key_visits: usize,
    pub(crate) path_index_probes: usize,
    pub(crate) overlap_deduplications: usize,
}

impl WorthQueryAffectedNativeKeyIndex {
    pub(crate) fn compile(
        display: &[WorthQueryNativeAccessKey],
        derived: &[WorthQueryNativeAccessKey],
    ) -> Self {
        let mut index = Self {
            by_aspect: BTreeMap::new(),
            whole_aspect: BTreeMap::new(),
            by_field: BTreeMap::new(),
            keys: BTreeMap::new(),
        };
        for key in display.iter().chain(derived) {
            let coordinate = (key.lane(), key.lane_slot());
            index.keys.insert(coordinate, key.clone());
            index
                .by_aspect
                .entry(key.contract_key().clone())
                .or_default()
                .push(key.clone());
            match canonical_path(key) {
                Some(path) => index
                    .by_field
                    .entry(key.contract_key().clone())
                    .or_default()
                    .insert(&path, coordinate),
                None => index
                    .whole_aspect
                    .entry(key.contract_key().clone())
                    .or_default()
                    .push(key.clone()),
            }
        }
        index
    }

    pub(crate) fn affected_keys(
        &self,
        touches: &[WorthQueryNativeTouchCoordinate],
    ) -> (
        Vec<WorthQueryNativeAccessKey>,
        WorthQueryNativeKeyNarrowingCounters,
    ) {
        let mut counters = WorthQueryNativeKeyNarrowingCounters::default();
        let mut selected = BTreeMap::<KeyCoordinate, WorthQueryNativeAccessKey>::new();
        for touch in touches {
            if let Some(path) = &touch.field_path {
                insert_indexed(
                    self.whole_aspect.get(&touch.aspect_key),
                    &mut selected,
                    &mut counters,
                );
                counters.index_lookups += 1;
                if let Some(index) = self.by_field.get(&touch.aspect_key) {
                    let (coordinates, path_work) = index.overlapping(path);
                    counters.path_index_probes += path_work.node_probes;
                    counters.index_lookups += path_work.node_probes;
                    counters.overlap_deduplications += path_work.overlap_deduplications;
                    for coordinate in coordinates {
                        counters.index_lookups += 1;
                        let key = self
                            .keys
                            .get(&coordinate)
                            .expect("native path index coordinate retains its access key");
                        counters.targeted_key_visits += 1;
                        if selected.insert(coordinate, key.clone()).is_some() {
                            counters.overlap_deduplications += 1;
                        }
                    }
                }
            } else {
                insert_indexed(
                    self.by_aspect.get(&touch.aspect_key),
                    &mut selected,
                    &mut counters,
                );
            }
        }
        (selected.into_values().collect(), counters)
    }
}

fn insert_indexed(
    keys: Option<&Vec<WorthQueryNativeAccessKey>>,
    selected: &mut BTreeMap<KeyCoordinate, WorthQueryNativeAccessKey>,
    counters: &mut WorthQueryNativeKeyNarrowingCounters,
) {
    counters.index_lookups += 1;
    if let Some(keys) = keys {
        for key in keys {
            counters.targeted_key_visits += 1;
            selected.insert((key.lane(), key.lane_slot()), key.clone());
        }
    }
}

fn canonical_path(key: &WorthQueryNativeAccessKey) -> Option<CanonicalFieldPath> {
    key.field_path()
        .canonical_field_path()
        .cloned()
        .or_else(|| {
            key.field_path()
                .native_field_key()
                .cloned()
                .map(CanonicalFieldPath::single)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use worth_foundational::facade::{
        AbsenceLaw, AspectContract, AspectContractRevision, AspectIdentity, AspectValuePosture,
        FieldKey, ScalarAspectType,
    };

    #[test]
    fn descendant_touch_selects_parent_native_key_through_path_index() {
        let contract = AspectContract::scalar(
            AspectKey::new("profile").unwrap(),
            AspectIdentity(7),
            AspectContractRevision(1),
            ScalarAspectType::String,
        );
        let key = WorthQueryNativeAccessKey::mint(
            1,
            crate::domain_installation::WorthQueryDomainInstallationGeneration::initial(),
            2,
            3,
            &contract,
            crate::projection_consumption::ProjectionFactFieldPath::from_canonical_field_path(
                path(&["address"]),
            ),
            AspectValuePosture::Scalar(ScalarAspectType::String),
            AbsenceLaw::Required,
            WorthQueryNativeFactLane::Display,
            0,
            1,
        );
        let index = WorthQueryAffectedNativeKeyIndex::compile(std::slice::from_ref(&key), &[]);
        let (selected, work) = index.affected_keys(&[WorthQueryNativeTouchCoordinate {
            aspect_key: contract.key().clone(),
            field_path: Some(path(&["address", "city"])),
        }]);

        assert_eq!(selected, [key]);
        assert_eq!(work.path_index_probes, 2);
        assert_eq!(work.targeted_key_visits, 1);
    }

    fn path(fields: &[&str]) -> CanonicalFieldPath {
        CanonicalFieldPath::new(fields.iter().map(|field| FieldKey::new(*field).unwrap())).unwrap()
    }
}
