use worth_query::facade::runtime::WorthQueryAdmittedQuerySchemaReferences;

fn main() {
    let _ = WorthQueryAdmittedQuerySchemaReferences {
        read_graph_digest: "read".to_string(),
        schema_basis_digest: "basis".to_string(),
        root: "user".to_string(),
        relations: Vec::new(),
        projections: Vec::new(),
        predicates: Vec::new(),
        orderings: Vec::new(),
    };
}
