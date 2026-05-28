use crate::domain_capabilities::ForgeQuerySupportContributionAuthoring;

use super::support::{
    admitted_handle, counting_geometry_canonicalization_count,
    reset_counting_geometry_canonicalization_count, CountingGeometryInput,
};

#[test]
fn grouped_contributions_reuse_member_progression_after_group_admission() {
    let handle = admitted_handle("main");
    reset_counting_geometry_canonicalization_count();

    let result = handle.grouped_contributions_checked(
        handle
            .geometry_helpers()
            .local_neighborhood_for_active_face_selection(CountingGeometryInput::new("face-a"))
            .with_member(CountingGeometryInput::new("face-b"))
            .with_shared_support_contribution(
                ForgeQuerySupportContributionAuthoring::declaration_support(
                    "support.shared",
                    "shared grouped support",
                ),
            ),
    );

    assert!(result.is_ok(), "grouped contributions should admit");
    assert_eq!(
        counting_geometry_canonicalization_count(),
        2,
        "grouped contributions should canonicalize each member once during grouped admission, not rebuild member declaration work during contribution lowering",
    );
}
