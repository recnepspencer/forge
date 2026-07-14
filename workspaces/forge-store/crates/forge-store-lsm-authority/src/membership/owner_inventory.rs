use super::LsmMembershipOwnerCaseDeclaration;

pub fn lsm_membership_owner_case_inventory(
) -> impl Iterator<Item = LsmMembershipOwnerCaseDeclaration> {
    super::runtime::owner_cases()
}
