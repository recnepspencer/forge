pub(super) const TOPOLOGY_QUERY_ADOPTION_RESIDUE_SURFACE: &str =
    "crates/worth-topo/src/query_adoption/residue.rs";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topology_residue_surface_is_explicitly_owned() {
        assert_eq!(
            TOPOLOGY_QUERY_ADOPTION_RESIDUE_SURFACE,
            "crates/worth-topo/src/query_adoption/residue.rs"
        );
    }
}
