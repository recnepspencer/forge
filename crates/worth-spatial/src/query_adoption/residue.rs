pub(super) const SPATIAL_QUERY_ADOPTION_RESIDUE_SURFACE: &str =
    "crates/worth-spatial/src/query_adoption/residue.rs";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spatial_residue_surface_is_explicitly_owned() {
        assert_eq!(
            SPATIAL_QUERY_ADOPTION_RESIDUE_SURFACE,
            "crates/worth-spatial/src/query_adoption/residue.rs"
        );
    }
}
