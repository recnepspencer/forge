use worth_query::facade::foundation::{AdmittedIdentityEvolutionQuery, IdentityEvolutionComparisonBasisFamily};

fn main() {
    let _: fn(&mut AdmittedIdentityEvolutionQuery, IdentityEvolutionComparisonBasisFamily) =
        AdmittedIdentityEvolutionQuery::override_comparison_basis_family;
}
