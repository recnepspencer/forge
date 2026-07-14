use worth_query::facade::foundation::basis_lifecycle;

fn main() {
    let _ = basis_lifecycle().current_head().execute();
}
