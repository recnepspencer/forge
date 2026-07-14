use super::super::LsmExecutionOwnerCaseDeclaration;

pub(in crate::strategy::lsm) fn owner_cases(
) -> impl Iterator<Item = LsmExecutionOwnerCaseDeclaration> {
    super::preparation::owner_cases()
        .chain(super::physical_binding::owner_cases())
        .chain(super::membership_activation::owner_cases())
        .chain(super::publication::owner_cases())
}
