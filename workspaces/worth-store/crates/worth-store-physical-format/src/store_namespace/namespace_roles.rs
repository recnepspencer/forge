macro_rules! define_namespace_roles {
    ($($role:ident => [$($component:literal),+] : $entry_type:ident),+ $(,)?) => {
        /// Stable relative roles owned by the Store namespace format.
        ///
        /// These are semantic roles, not caller-provided paths. The filesystem
        /// owner resolves them beneath its admitted root in C.4 Phase 4.
        #[repr(usize)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum StoreNamespaceRelativeRole {
            $($role),+
        }

        impl StoreNamespaceRelativeRole {
            pub const ALL: [Self; define_namespace_roles!(@count $($role),+)] = [
                $(Self::$role),+
            ];

            pub const fn components(self) -> &'static [&'static str] {
                match self {
                    $(Self::$role => &[$($component),+]),+
                }
            }

            pub const fn expected_entry_type(self) -> super::NamespaceEntryType {
                match self {
                    $(Self::$role => super::NamespaceEntryType::$entry_type),+
                }
            }

            pub(super) const fn index(self) -> usize {
                self as usize
            }
        }
    };
    (@count $($role:ident),+) => {
        <[()]>::len(&[$(define_namespace_roles!(@unit $role)),+])
    };
    (@unit $role:ident) => { () };
}

define_namespace_roles!(
    NamespaceDirectory => ["namespace"]: Directory,
    IdentityRecord => ["namespace", "identity"]: RegularFile,
    MutationLock => ["namespace", "mutation.lock"]: RegularFile,
    FamiliesDirectory => ["families"]: Directory,
    StagingDirectory => ["staging"]: Directory,
);
