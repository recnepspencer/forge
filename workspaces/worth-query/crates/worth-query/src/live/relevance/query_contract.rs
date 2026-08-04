use super::super::promotion::LiveQueryFamily;
use super::bridge_change::BridgeFieldDelta;
use crate::collection::CollectionPlanBundle;
use crate::validation::ValidatedQueryBundle;
use worth_foundational::facade::{AspectKey, FieldKey};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct QueryFieldKey {
    aspect_key: AspectKey,
    field_key: FieldKey,
}

impl QueryFieldKey {
    pub fn native_aspect_key(&self) -> &AspectKey {
        &self.aspect_key
    }

    pub fn native_field_key(&self) -> &FieldKey {
        &self.field_key
    }

    pub(in crate::live) fn new(aspect: impl Into<String>, field: impl Into<String>) -> Self {
        let aspect = aspect.into();
        let field = field.into();
        Self {
            aspect_key: AspectKey::new(aspect).expect("query field aspect must be foundational"),
            field_key: FieldKey::new(field).expect("query field name must be foundational"),
        }
    }

    pub(in crate::live) fn from_native_keys(aspect_key: AspectKey, field_key: FieldKey) -> Self {
        Self {
            aspect_key,
            field_key,
        }
    }

    pub(in crate::live) fn matches(&self, delta: &BridgeFieldDelta) -> bool {
        self == delta.field_key()
    }

    pub(in crate::live) fn terminal_digest_part(&self) -> String {
        format!("{}:{}", self.aspect_key.as_str(), self.field_key.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryRelevanceContract {
    family: LiveQueryFamily,
    projected_fields: Vec<QueryFieldKey>,
    ordering_fields: Vec<QueryFieldKey>,
    traversal_relations: Vec<String>,
}

impl QueryRelevanceContract {
    pub fn family(&self) -> &LiveQueryFamily {
        &self.family
    }

    pub fn projected_fields(&self) -> &[QueryFieldKey] {
        &self.projected_fields
    }

    pub fn ordering_fields(&self) -> &[QueryFieldKey] {
        &self.ordering_fields
    }

    pub fn traversal_relations(&self) -> &[String] {
        &self.traversal_relations
    }

    pub(in crate::live) fn for_detail(bundle: &ValidatedQueryBundle) -> Self {
        Self {
            family: LiveQueryFamily::Detail,
            projected_fields: bundle
                .query()
                .projection()
                .iter()
                .map(|entry| {
                    QueryFieldKey::from_native_keys(
                        entry.native_aspect_key().clone(),
                        entry.native_field_key().clone(),
                    )
                })
                .collect(),
            ordering_fields: Vec::new(),
            traversal_relations: Vec::new(),
        }
    }

    pub(in crate::live) fn for_ordered_collection(
        bundle: &ValidatedQueryBundle,
        _collection: &CollectionPlanBundle,
    ) -> Self {
        Self {
            family: LiveQueryFamily::OrderedCollection,
            projected_fields: bundle
                .query()
                .projection()
                .iter()
                .map(|entry| {
                    QueryFieldKey::from_native_keys(
                        entry.native_aspect_key().clone(),
                        entry.native_field_key().clone(),
                    )
                })
                .collect(),
            ordering_fields: bundle
                .query()
                .ordering()
                .entries()
                .iter()
                .map(|entry| {
                    QueryFieldKey::from_native_keys(
                        entry.native_aspect_key().clone(),
                        entry.native_field_key().clone(),
                    )
                })
                .collect(),
            traversal_relations: Vec::new(),
        }
    }

    pub(in crate::live) fn for_bounded_materialization(
        bundle: &ValidatedQueryBundle,
        collection: &CollectionPlanBundle,
    ) -> Self {
        Self {
            family: LiveQueryFamily::BoundedMaterialization,
            projected_fields: bundle
                .query()
                .projection()
                .iter()
                .map(|entry| {
                    QueryFieldKey::from_native_keys(
                        entry.native_aspect_key().clone(),
                        entry.native_field_key().clone(),
                    )
                })
                .collect(),
            ordering_fields: bundle
                .query()
                .ordering()
                .entries()
                .iter()
                .map(|entry| {
                    QueryFieldKey::from_native_keys(
                        entry.native_aspect_key().clone(),
                        entry.native_field_key().clone(),
                    )
                })
                .collect(),
            traversal_relations: collection
                .traversal_bound()
                .edge_classes()
                .iter()
                .map(|entry| entry.as_str().to_string())
                .collect(),
        }
    }
}
