#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaRelationView {
    relation: String,
    max_depth: u8,
}

impl SchemaRelationView {
    pub fn new(relation: impl Into<String>, max_depth: u8) -> Self {
        Self {
            relation: relation.into(),
            max_depth,
        }
    }

    pub fn relation(&self) -> &str {
        &self.relation
    }

    pub fn max_depth(&self) -> u8 {
        self.max_depth
    }
}
