use super::super::LsmExecutionOwnerCaseDeclaration;

pub(in crate::strategy::lsm) fn owner_cases(
) -> impl Iterator<Item = LsmExecutionOwnerCaseDeclaration> {
    super::operation::owner_cases()
}
