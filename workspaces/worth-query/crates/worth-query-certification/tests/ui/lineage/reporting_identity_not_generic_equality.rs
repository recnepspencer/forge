use worth_query::facade::domain::InstalledIdentityEvolutionOutcome;

fn requires_generic_equality<T: PartialEq>() {}

fn main() {
    requires_generic_equality::<InstalledIdentityEvolutionOutcome>();
}
