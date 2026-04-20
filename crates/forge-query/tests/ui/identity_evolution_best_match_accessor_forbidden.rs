use forge_query::facade::IdentityEvolutionResultBundle;

fn main() {
    let _: fn(&IdentityEvolutionResultBundle) -> &str = IdentityEvolutionResultBundle::best_match;
}
