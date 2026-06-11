#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyWorkloadDeclarationIdentity {
    name: String,
    query_declaration: String,
}

impl TopologyWorkloadDeclarationIdentity {
    pub(crate) fn new(name: String, query_declaration: String) -> Self {
        Self {
            name,
            query_declaration,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn query_declaration(&self) -> &str {
        &self.query_declaration
    }
}
