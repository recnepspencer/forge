use forge_query::facade::runtime::ForgeQueryAdmittedGraphReadOrderingField;

fn main() {
    let field: ForgeQueryAdmittedGraphReadOrderingField = unreachable!();
    let _ = field.aspect();
    let _ = field.field();
}
