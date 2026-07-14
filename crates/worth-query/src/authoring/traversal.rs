use super::{AuthoringError, RelationName};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TraversalSelector {
    relation: RelationName,
    depth: u8,
}

impl TraversalSelector {
    pub fn bounded(relation: impl Into<String>, depth: u8) -> Result<Self, AuthoringError> {
        if depth == 0 {
            return Err(AuthoringError::UnsupportedTraversalDepth { depth });
        }
        Self::bounded_relation_name(RelationName::new(relation)?, depth)
    }

    pub fn bounded_relation_name(
        relation: RelationName,
        depth: u8,
    ) -> Result<Self, AuthoringError> {
        if depth == 0 {
            return Err(AuthoringError::UnsupportedTraversalDepth { depth });
        }
        Ok(Self { relation, depth })
    }

    pub(crate) fn terminal_relation_projection_for_boundary(&self) -> &str {
        self.relation.as_str()
    }

    pub fn relation_name(&self) -> &RelationName {
        &self.relation
    }

    pub fn depth(&self) -> u8 {
        self.depth
    }
}

#[cfg(test)]
mod tests {
    use super::{RelationName, TraversalSelector};
    use crate::authoring::AuthoringError;

    #[test]
    fn bounded_relation_name_preserves_validated_relation() {
        let relation = RelationName::new("worth.half_edge_next").expect("valid relation");
        let selector = TraversalSelector::bounded_relation_name(relation.clone(), 3)
            .expect("validated relation names should lower directly");

        assert_eq!(selector.relation_name(), &relation);
        assert_eq!(selector.depth(), 3);
    }

    #[test]
    fn bounded_relation_name_rejects_zero_depth() {
        let relation = RelationName::new("worth.half_edge_next").expect("valid relation");
        let error = TraversalSelector::bounded_relation_name(relation, 0)
            .expect_err("zero-depth traversal must fail");

        assert!(matches!(
            error,
            AuthoringError::UnsupportedTraversalDepth { depth: 0 }
        ));
    }
}
