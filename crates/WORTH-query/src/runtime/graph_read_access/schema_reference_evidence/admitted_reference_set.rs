use super::admitted_reference_rows::{
    WorthQueryAdmittedGraphReadOrderingField, WorthQueryAdmittedGraphReadPredicateField,
    WorthQueryAdmittedGraphReadProjectionField, WorthQueryAdmittedGraphReadRelation,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAdmittedQuerySchemaReferences {
    read_graph_digest: String,
    schema_basis_digest: String,
    root: String,
    relations: Vec<WorthQueryAdmittedGraphReadRelation>,
    projections: Vec<WorthQueryAdmittedGraphReadProjectionField>,
    predicates: Vec<WorthQueryAdmittedGraphReadPredicateField>,
    orderings: Vec<WorthQueryAdmittedGraphReadOrderingField>,
}

impl WorthQueryAdmittedQuerySchemaReferences {
    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn schema_basis_digest(&self) -> &str {
        &self.schema_basis_digest
    }

    pub fn root(&self) -> &str {
        &self.root
    }

    pub fn relations(&self) -> &[WorthQueryAdmittedGraphReadRelation] {
        &self.relations
    }

    pub fn projections(&self) -> &[WorthQueryAdmittedGraphReadProjectionField] {
        &self.projections
    }

    pub fn predicates(&self) -> &[WorthQueryAdmittedGraphReadPredicateField] {
        &self.predicates
    }

    pub fn orderings(&self) -> &[WorthQueryAdmittedGraphReadOrderingField] {
        &self.orderings
    }

    pub(crate) fn digest_parts(&self) -> Vec<String> {
        let mut parts = vec![
            format!("read_graph:{}", self.read_graph_digest),
            format!("schema_basis:{}", self.schema_basis_digest),
            format!("root:{}", self.root),
        ];
        parts.extend(self.relations.iter().map(|row| row.digest_part()));
        parts.extend(self.projections.iter().map(|row| row.digest_part()));
        parts.extend(self.predicates.iter().map(|row| row.digest_part()));
        parts.extend(self.orderings.iter().map(|row| row.digest_part()));
        parts
    }

    pub(crate) fn new(
        read_graph_digest: impl Into<String>,
        schema_basis_digest: impl Into<String>,
        root: impl Into<String>,
        relations: Vec<WorthQueryAdmittedGraphReadRelation>,
        projections: Vec<WorthQueryAdmittedGraphReadProjectionField>,
        predicates: Vec<WorthQueryAdmittedGraphReadPredicateField>,
        orderings: Vec<WorthQueryAdmittedGraphReadOrderingField>,
    ) -> Self {
        Self {
            read_graph_digest: read_graph_digest.into(),
            schema_basis_digest: schema_basis_digest.into(),
            root: root.into(),
            relations,
            projections,
            predicates,
            orderings,
        }
    }
}
