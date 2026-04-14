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
        Ok(Self {
            relation: RelationName::new(relation)?,
            depth,
        })
    }

    pub fn relation(&self) -> &str {
        self.relation.as_str()
    }

    pub fn relation_name(&self) -> &RelationName {
        &self.relation
    }

    pub fn depth(&self) -> u8 {
        self.depth
    }
}
