use std::collections::{BTreeMap, BTreeSet};

use crate::projection_consumption::{
    ConsumedFieldValueFact, ConsumedNativeValue, ConsumedProjectionFactSet, ProjectionFactFieldPath,
};
use worth_foundational::facade::{AspectKey, CanonicalFieldPath};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct WorthQueryProjectionChangeTarget {
    aspect: AspectKey,
    field_path: Option<CanonicalFieldPath>,
}

impl WorthQueryProjectionChangeTarget {
    pub(crate) fn new(aspect: AspectKey, field_path: Option<CanonicalFieldPath>) -> Self {
        Self { aspect, field_path }
    }

    fn matches(&self, fact: &ProjectionFactFieldPath) -> bool {
        if fact.native_aspect_key() != Some(&self.aspect) {
            return false;
        }
        self.field_path.as_ref().is_none_or(|changed| {
            fact.native_field_key().is_some_and(|field| {
                let fact_path = CanonicalFieldPath::new([field.clone()])
                    .expect("native projection fields retain one field");
                canonical_paths_overlap(changed, &fact_path)
            })
        })
    }
}

fn canonical_paths_overlap(left: &CanonicalFieldPath, right: &CanonicalFieldPath) -> bool {
    let shared = left.fields().len().min(right.fields().len());
    left.fields()[..shared] == right.fields()[..shared]
}

/// Runtime-owned comparison state for the result that granular maintenance
/// has actually published. It narrows work; it does not carry projection
/// authority.
pub(crate) struct WorthQueryProjectionMaintenanceState {
    by_field: BTreeMap<(String, ProjectionFactFieldPath), ConsumedNativeValue>,
}

pub(crate) struct WorthQueryProjectionMaintenancePreview {
    fields: Vec<ConsumedFieldValueFact>,
    prior_field_comparisons: usize,
    pending: WorthQueryPendingProjectionMaintenanceState,
}

pub(crate) struct WorthQueryPendingProjectionMaintenanceState {
    replace_all: bool,
    replaced_sources: BTreeSet<String>,
    fresh: BTreeMap<(String, ProjectionFactFieldPath), ConsumedNativeValue>,
}

impl WorthQueryProjectionMaintenanceState {
    pub(crate) fn from_initial(facts: &ConsumedProjectionFactSet) -> Self {
        Self {
            by_field: fact_values(facts),
        }
    }

    pub(crate) fn preview(
        &self,
        facts: &ConsumedProjectionFactSet,
        affected_sources: BTreeSet<String>,
        select_all_fields: bool,
        broad_projection_change: bool,
        changed_targets: &[WorthQueryProjectionChangeTarget],
    ) -> WorthQueryProjectionMaintenancePreview {
        let fresh = fact_values(facts);
        let mut prior_field_comparisons = 0;
        let fields = facts
            .display_fields()
            .iter()
            .chain(facts.derived_fields())
            .filter(|fact| {
                select_all_fields
                    || broad_projection_change
                    || changed_targets
                        .iter()
                        .any(|target| target.matches(fact.field_path()))
            })
            .filter_map(|fact| {
                let key = (
                    fact.source_row_identity().to_owned(),
                    fact.field_path().clone(),
                );
                let unchanged = self.by_field.get(&key).is_some_and(|prior| {
                    prior_field_comparisons += 1;
                    prior.view() == fact.native_value()
                });
                (!unchanged).then(|| fact.clone())
            })
            .collect();
        WorthQueryProjectionMaintenancePreview {
            fields,
            prior_field_comparisons,
            pending: WorthQueryPendingProjectionMaintenanceState {
                replace_all: affected_sources.is_empty(),
                replaced_sources: affected_sources,
                fresh,
            },
        }
    }

    pub(crate) fn apply(&mut self, pending: WorthQueryPendingProjectionMaintenanceState) {
        if pending.replace_all {
            self.by_field.clear();
        } else {
            self.by_field
                .retain(|(source, _), _| !pending.replaced_sources.contains(source));
        }
        self.by_field.extend(pending.fresh);
    }
}

impl WorthQueryProjectionMaintenancePreview {
    pub(crate) fn into_parts(
        self,
    ) -> (
        Vec<ConsumedFieldValueFact>,
        usize,
        WorthQueryPendingProjectionMaintenanceState,
    ) {
        (self.fields, self.prior_field_comparisons, self.pending)
    }
}

fn fact_values(
    facts: &ConsumedProjectionFactSet,
) -> BTreeMap<(String, ProjectionFactFieldPath), ConsumedNativeValue> {
    facts
        .display_fields()
        .iter()
        .chain(facts.derived_fields())
        .map(|fact| {
            (
                (
                    fact.source_row_identity().to_owned(),
                    fact.field_path().clone(),
                ),
                fact.cloned_native_value(),
            )
        })
        .collect()
}

#[cfg(test)]
mod change_target_tests {
    use super::*;
    use worth_foundational::facade::FieldKey;

    #[test]
    fn whole_aspect_projection_target_is_aspect_local_and_collision_safe() {
        let target =
            WorthQueryProjectionChangeTarget::new(AspectKey::new("Portfolio.Facts").unwrap(), None);
        let same_aspect = ProjectionFactFieldPath::from_native_keys(
            AspectKey::new("Portfolio.Facts").unwrap(),
            FieldKey::new("rank").unwrap(),
        );
        let segmented_collision = ProjectionFactFieldPath::from_native_keys(
            AspectKey::new("Portfolio").unwrap(),
            FieldKey::new("Facts").unwrap(),
        );
        let unrelated = ProjectionFactFieldPath::from_native_keys(
            AspectKey::new("Risk").unwrap(),
            FieldKey::new("rank").unwrap(),
        );
        assert!(target.matches(&same_aspect));
        assert!(!target.matches(&segmented_collision));
        assert!(!target.matches(&unrelated));
    }
}
