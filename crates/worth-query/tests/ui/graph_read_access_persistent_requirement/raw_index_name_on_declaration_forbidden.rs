use worth_query::facade::runtime::WorthQueryPersistentGraphIndexRequirementDeclaration;

fn main() {
    fn attach_raw_index_name(declaration: &WorthQueryPersistentGraphIndexRequirementDeclaration) {
        let _ = declaration.with_index_name("caller-provided-index-name");
    }
}
