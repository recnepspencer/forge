use super::super::*;

#[test]
fn canonical_roles_have_one_complete_unambiguous_grammar() {
    let expected = [
        (
            StoreNamespaceRelativeRole::NamespaceDirectory,
            &["namespace"][..],
            NamespaceEntryType::Directory,
        ),
        (
            StoreNamespaceRelativeRole::IdentityRecord,
            &["namespace", "identity"][..],
            NamespaceEntryType::RegularFile,
        ),
        (
            StoreNamespaceRelativeRole::MutationLock,
            &["namespace", "mutation.lock"][..],
            NamespaceEntryType::RegularFile,
        ),
        (
            StoreNamespaceRelativeRole::FamiliesDirectory,
            &["families"][..],
            NamespaceEntryType::Directory,
        ),
        (
            StoreNamespaceRelativeRole::StagingDirectory,
            &["staging"][..],
            NamespaceEntryType::Directory,
        ),
    ];

    assert_eq!(StoreNamespaceRelativeRole::ALL.len(), expected.len());
    for ((role, components, entry_type), actual_role) in
        expected.into_iter().zip(StoreNamespaceRelativeRole::ALL)
    {
        assert_eq!(actual_role, role);
        assert_eq!(actual_role.components(), components);
        assert_eq!(actual_role.expected_entry_type(), entry_type);
    }
}
