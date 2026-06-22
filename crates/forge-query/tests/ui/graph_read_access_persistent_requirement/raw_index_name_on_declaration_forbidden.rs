use forge_query::facade::runtime::ForgeQueryPersistentGraphIndexRequirementDeclaration;

fn main() {
    fn attach_raw_index_name(declaration: &ForgeQueryPersistentGraphIndexRequirementDeclaration) {
        let _ = declaration.with_index_name("caller-provided-index-name");
    }
}
