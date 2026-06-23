use forge_query::facade::runtime::ForgeQueryGraphReadAccessRequirementSet;

fn main() {
    fn accepts_set(set: &ForgeQueryGraphReadAccessRequirementSet) {
        let _ = set.requires_kind("directional_adjacency");
    }
}
