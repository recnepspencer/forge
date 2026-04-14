use super::AuthoringError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TraversalSelector {
    relation: String,
    depth: u8,
}

impl TraversalSelector {
    pub fn bounded(relation: impl Into<String>, depth: u8) -> Result<Self, AuthoringError> {
        let relation = relation.into();
        if relation.trim().is_empty() {
            return Err(AuthoringError::EmptyTraversalRelation);
        }
        if depth == 0 {
            return Err(AuthoringError::UnsupportedTraversalDepth { depth });
        }
        Ok(Self { relation, depth })
    }

    pub fn relation(&self) -> &str {
        &self.relation
    }

    pub fn depth(&self) -> u8 {
        self.depth
    }
}
