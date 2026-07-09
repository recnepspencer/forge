use worth_query::facade::runtime::WorthQueryGraphReadAccessRequirementSet;

fn main() {
    fn accepts_set(set: &WorthQueryGraphReadAccessRequirementSet) {
        let _ = set.requires_kind("directional_adjacency");
    }
}
