mod field;
mod relation;

use std::collections::BTreeMap;

use crate::authoring::{AspectName, FieldName, RelationName};
use crate::identity::SchemaBasisDigest;

pub use field::{SchemaFieldKind, SchemaFieldView};
pub use relation::SchemaRelationView;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySchemaView {
    basis: SchemaBasisDigest,
    fields: BTreeMap<AspectName, AspectSchemaView>,
    relations: BTreeMap<RelationName, SchemaRelationView>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AspectSchemaView {
    fields: BTreeMap<FieldName, SchemaFieldView>,
}

impl QuerySchemaView {
    pub fn new(
        basis_marker: impl Into<String>,
        fields: impl IntoIterator<Item = SchemaFieldView>,
        relations: impl IntoIterator<Item = SchemaRelationView>,
    ) -> Self {
        let mut fields_by_aspect: BTreeMap<AspectName, AspectSchemaView> = BTreeMap::new();
        for field in fields {
            fields_by_aspect
                .entry(field.aspect_name().clone())
                .or_insert_with(|| AspectSchemaView {
                    fields: BTreeMap::new(),
                })
                .fields
                .insert(field.field_name().clone(), field);
        }
        let relations: BTreeMap<RelationName, SchemaRelationView> = relations
            .into_iter()
            .map(|relation| (relation.relation_name().clone(), relation))
            .collect();

        let mut digest_parts = vec![format!("basis:{}", basis_marker.into())];
        for aspect_view in fields_by_aspect.values() {
            digest_parts.extend(aspect_view.fields.values().map(|field| {
                format!(
                    "field:{}:{}:{:?}:{}:{}:{}:{}:{}:{}",
                    field.aspect_name().as_str(),
                    field.field_name().as_str(),
                    field.kind(),
                    field.is_queryable(),
                    field.is_orderable(),
                    field.is_text_predicate_queryable(),
                    field.is_membership_predicate_queryable(),
                    field.is_presence_predicate_queryable(),
                    field.is_workflow_predicate_queryable()
                )
            }));
        }
        digest_parts.extend(relations.values().map(|relation| {
            format!(
                "relation:{}:{}",
                relation.terminal_relation_projection_for_boundary(),
                relation.max_depth()
            )
        }));

        Self {
            basis: SchemaBasisDigest::from_parts(&digest_parts),
            fields: fields_by_aspect,
            relations,
        }
    }

    pub fn basis(&self) -> &SchemaBasisDigest {
        &self.basis
    }

    pub fn field(&self, aspect: &AspectName, field: &FieldName) -> Option<&SchemaFieldView> {
        self.fields.get(aspect)?.fields.get(field)
    }

    pub fn has_aspect(&self, aspect: &AspectName) -> bool {
        self.fields.contains_key(aspect)
    }

    pub fn relation(&self, relation: &RelationName) -> Option<&SchemaRelationView> {
        self.relations.get(relation)
    }
}
