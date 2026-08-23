use crate::authoring::AspectFieldKey;
use crate::memory_workspace::{
    WorthQueryEntity, WorthQueryEntityNativeReplacement, WorthQueryEntityNativeReplacementValue,
};
use worth_foundational::facade::{AspectKey, CanonicalFieldPath};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct WorthQueryCollectionChangedNativeTarget {
    aspect: AspectKey,
    field_path: Option<CanonicalFieldPath>,
}

impl WorthQueryCollectionChangedNativeTarget {
    pub(crate) fn new(aspect: AspectKey, field_path: Option<CanonicalFieldPath>) -> Self {
        Self { aspect, field_path }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct WorthQueryCollectionMaintenanceTarget {
    aspect: AspectKey,
    field_path: CanonicalFieldPath,
    storage_paths: Vec<CanonicalFieldPath>,
}

impl WorthQueryCollectionMaintenanceTarget {
    fn from_query_field(field: &AspectFieldKey) -> Self {
        let aspect = field.native_aspect_key();
        let field_path = CanonicalFieldPath::new([field.native_field_key()])
            .expect("query fields always retain one admitted native field");
        Self::new(aspect, field_path)
    }

    fn from_collection_field(
        field: &worth_query_installation::facade::WorthQueryOperationCollectionField,
    ) -> Self {
        Self::new(field.aspect_key().clone(), field.field_path().clone())
    }

    pub(super) fn new(aspect: AspectKey, field_path: CanonicalFieldPath) -> Self {
        let storage_paths =
            crate::memory_workspace::normalized_native_storage_path(&aspect, &field_path)
                .into_iter()
                .collect();
        Self {
            aspect,
            field_path,
            storage_paths,
        }
    }

    pub(super) fn matches_change(&self, change: &WorthQueryCollectionChangedNativeTarget) -> bool {
        self.aspect == change.aspect
            && change
                .field_path
                .as_ref()
                .is_none_or(|changed| paths_overlap(&self.field_path, changed))
    }

    pub(super) fn matches_native_key(&self, key: &super::WorthQueryNativeAccessKey) -> bool {
        if &self.aspect != key.contract_key() {
            return false;
        }
        key.field_path().native_field_key().is_some_and(|field| {
            self.field_path.fields().len() == 1 && self.field_path.fields().first() == Some(field)
        })
    }

    pub(super) fn replacement(
        &self,
        fresh: &WorthQueryEntity,
    ) -> WorthQueryEntityNativeReplacement {
        let field = match self.field_path.fields() {
            [field] => Some(field.clone()),
            _ => None,
        };
        let value =
            crate::memory_workspace::aspect_relative_scalar(fresh, &self.aspect, &self.field_path)
                .or_else(|| fresh.aspect_value(&self.aspect))
                .cloned()
                .map(WorthQueryEntityNativeReplacementValue::Scalar)
                .unwrap_or(WorthQueryEntityNativeReplacementValue::Absent);
        if field.is_some() {
            WorthQueryEntityNativeReplacement::new(
                self.aspect.clone(),
                field,
                self.storage_paths.clone(),
                value,
            )
        } else {
            WorthQueryEntityNativeReplacement::canonical_paths_only(
                self.aspect.clone(),
                self.storage_paths.clone(),
                value,
            )
        }
    }
}

pub(super) fn maintenance_targets(
    request: &crate::declarative_live::DeclarativeLiveQueryRequest,
    grouping_fields: &[worth_query_installation::facade::WorthQueryOperationCollectionField],
) -> Vec<WorthQueryCollectionMaintenanceTarget> {
    let mut targets = request
        .query_projection()
        .iter()
        .map(|field| field.source_field_key())
        .chain(
            request
                .predicate_filters()
                .iter()
                .map(|filter| filter.source_field_key()),
        )
        .chain(
            request
                .ordering()
                .iter()
                .map(|ordering| ordering.source_field_key()),
        )
        .map(WorthQueryCollectionMaintenanceTarget::from_query_field)
        .chain(
            grouping_fields
                .iter()
                .map(WorthQueryCollectionMaintenanceTarget::from_collection_field),
        )
        .collect::<Vec<_>>();
    targets.sort();
    targets.dedup();
    targets
}

fn paths_overlap(left: &CanonicalFieldPath, right: &CanonicalFieldPath) -> bool {
    let shared = left.fields().len().min(right.fields().len());
    left.fields()[..shared] == right.fields()[..shared]
}

#[cfg(test)]
mod tests {
    use super::*;
    use worth_foundational::FieldKey;

    #[test]
    fn namespaced_aspect_identity_is_not_reconstructed_from_storage_segments() {
        let target = WorthQueryCollectionMaintenanceTarget::new(
            AspectKey::new("Portfolio.Facts").unwrap(),
            CanonicalFieldPath::new([FieldKey::new("rank").unwrap()]).unwrap(),
        );
        assert!(
            target.matches_change(&WorthQueryCollectionChangedNativeTarget::new(
                AspectKey::new("Portfolio.Facts").unwrap(),
                Some(CanonicalFieldPath::new([FieldKey::new("rank").unwrap()]).unwrap()),
            ))
        );
        assert!(
            !target.matches_change(&WorthQueryCollectionChangedNativeTarget::new(
                AspectKey::new("Portfolio").unwrap(),
                Some(CanonicalFieldPath::new([FieldKey::new("rank").unwrap()]).unwrap()),
            ))
        );
        assert_eq!(
            target.storage_paths,
            [CanonicalFieldPath::new([
                FieldKey::new("Portfolio").unwrap(),
                FieldKey::new("Facts").unwrap(),
                FieldKey::new("rank").unwrap(),
            ])
            .unwrap()]
        );
    }
}
