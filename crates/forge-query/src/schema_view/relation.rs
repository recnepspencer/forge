use crate::authoring::{AuthoringError, RelationName};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaRelationView {
    relation: RelationName,
    max_depth: u8,
}

impl SchemaRelationView {
    pub fn new(relation: impl Into<String>, max_depth: u8) -> Self {
        Self::try_from_relation_name(
            RelationName::new(relation)
                .expect("schema relation name must be non-empty at construction"),
            max_depth,
        )
        .expect("schema relation max depth must be non-zero at construction")
    }

    pub(crate) fn try_from_relation_name(
        relation: RelationName,
        max_depth: u8,
    ) -> Result<Self, AuthoringError> {
        if max_depth == 0 {
            return Err(AuthoringError::UnsupportedTraversalDepth { depth: 0 });
        }
        Ok(Self {
            relation,
            max_depth,
        })
    }

    pub fn relation(&self) -> &str {
        self.relation.as_str()
    }

    pub fn relation_name(&self) -> &RelationName {
        &self.relation
    }

    pub fn max_depth(&self) -> u8 {
        self.max_depth
    }
}

#[cfg(test)]
mod tests {
    use super::SchemaRelationView;
    use crate::authoring::{AuthoringError, RelationName};

    #[test]
    fn schema_relation_view_can_reuse_validated_relation_names() {
        let relation = RelationName::new("worth.half_edge_next").expect("valid relation");
        let view = SchemaRelationView::try_from_relation_name(relation.clone(), 4)
            .expect("validated relation names with non-zero depth should lower directly");

        assert_eq!(view.relation_name(), &relation);
        assert_eq!(view.relation(), "worth.half_edge_next");
        assert_eq!(view.max_depth(), 4);
    }

    #[test]
    fn schema_relation_view_rejects_zero_max_depth() {
        let relation = RelationName::new("worth.half_edge_next").expect("valid relation");
        let error = SchemaRelationView::try_from_relation_name(relation, 0)
            .expect_err("zero-depth schema relations must fail at construction");

        assert!(matches!(
            error,
            AuthoringError::UnsupportedTraversalDepth { depth: 0 }
        ));
    }
}
