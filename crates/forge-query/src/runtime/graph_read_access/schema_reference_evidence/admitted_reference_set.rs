use super::admitted_reference_rows::{
    ForgeQueryAdmittedGraphReadOrderingField, ForgeQueryAdmittedGraphReadPredicateField,
    ForgeQueryAdmittedGraphReadProjectionField, ForgeQueryAdmittedGraphReadRelation,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedQuerySchemaReferences {
    read_graph_digest: String,
    schema_basis_digest: String,
    root: String,
    relations: Vec<ForgeQueryAdmittedGraphReadRelation>,
    projections: Vec<ForgeQueryAdmittedGraphReadProjectionField>,
    predicates: Vec<ForgeQueryAdmittedGraphReadPredicateField>,
    orderings: Vec<ForgeQueryAdmittedGraphReadOrderingField>,
}

impl ForgeQueryAdmittedQuerySchemaReferences {
    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn schema_basis_digest(&self) -> &str {
        &self.schema_basis_digest
    }

    pub fn root(&self) -> &str {
        &self.root
    }

    pub fn relations(&self) -> &[ForgeQueryAdmittedGraphReadRelation] {
        &self.relations
    }

    pub fn projections(&self) -> &[ForgeQueryAdmittedGraphReadProjectionField] {
        &self.projections
    }

    pub fn predicates(&self) -> &[ForgeQueryAdmittedGraphReadPredicateField] {
        &self.predicates
    }

    pub fn orderings(&self) -> &[ForgeQueryAdmittedGraphReadOrderingField] {
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
        relations: Vec<ForgeQueryAdmittedGraphReadRelation>,
        projections: Vec<ForgeQueryAdmittedGraphReadProjectionField>,
        predicates: Vec<ForgeQueryAdmittedGraphReadPredicateField>,
        orderings: Vec<ForgeQueryAdmittedGraphReadOrderingField>,
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
