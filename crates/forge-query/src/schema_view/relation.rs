use crate::authoring::RelationName;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaRelationView {
    relation: RelationName,
    max_depth: u8,
}

impl SchemaRelationView {
    pub fn new(relation: impl Into<String>, max_depth: u8) -> Self {
        Self {
            relation: RelationName::new(relation)
                .expect("schema relation name must be non-empty at construction"),
            max_depth,
        }
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
