use forge_query::facade::runtime::ForgeQueryAdmittedQuerySchemaReferences;

fn main() {
    let _ = ForgeQueryAdmittedQuerySchemaReferences {
        read_graph_digest: "read".to_string(),
        schema_basis_digest: "basis".to_string(),
        root: "user".to_string(),
        relations: Vec::new(),
        projections: Vec::new(),
        predicates: Vec::new(),
        orderings: Vec::new(),
    };
}
