pub(super) const KERNEL_QUERY_ADOPTION_RESIDUE_SURFACE: &str =
    "crates/worth-kernel/src/query_adoption/residue.rs";
pub(super) const KERNEL_PUBLIC_FACADE_CERTIFICATION_RESIDUE_SURFACE: &str =
    KERNEL_QUERY_ADOPTION_RESIDUE_SURFACE;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_residue_surface_is_explicitly_owned() {
        assert_eq!(
            KERNEL_QUERY_ADOPTION_RESIDUE_SURFACE,
            "crates/worth-kernel/src/query_adoption/residue.rs"
        );
        assert_eq!(
            KERNEL_PUBLIC_FACADE_CERTIFICATION_RESIDUE_SURFACE,
            KERNEL_QUERY_ADOPTION_RESIDUE_SURFACE
        );
    }
}
