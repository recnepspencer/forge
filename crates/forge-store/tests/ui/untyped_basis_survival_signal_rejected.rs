fn require_basis_survival(_: forge_store::BasisSurvivalVerdict) {}

fn main() {
    require_basis_survival(Some(String::from("basis-a")));
}
