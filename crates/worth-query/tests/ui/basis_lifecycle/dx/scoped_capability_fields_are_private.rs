use worth_query::facade::foundation::{basis_lifecycle, ScopedObservationBasis};

fn main() {
    let scoped = basis_lifecycle().current_head().observe().unwrap();
    let ScopedObservationBasis { family, .. } = scoped;
    let _ = family;
}
