use worth_query::facade::foundation::IdentityEvolutionResultBundle;

fn main() {
    let _: fn(&IdentityEvolutionResultBundle) -> &str = IdentityEvolutionResultBundle::best_match;
}
