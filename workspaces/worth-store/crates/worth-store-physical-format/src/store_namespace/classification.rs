use super::{
    StableStoreIdentity, StagedNamespaceName, StoreNamespaceIdentityDecodeError,
    StoreNamespaceIdentityRecord, StoreNamespaceRelativeRole,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceEntryType {
    Directory,
    RegularFile,
    LinkLike,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceContention {
    Available,
    Contended,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamespaceEntryObservation {
    CanonicalRole {
        role: StoreNamespaceRelativeRole,
        entry_type: NamespaceEntryType,
    },
    PublishedIdentity {
        entry_type: NamespaceEntryType,
        bytes: Vec<u8>,
    },
    StagedIdentity {
        name: StagedNamespaceName,
        entry_type: NamespaceEntryType,
    },
    Unknown {
        relative_name: String,
        entry_type: NamespaceEntryType,
    },
}

impl NamespaceEntryObservation {
    pub fn canonical(role: StoreNamespaceRelativeRole, entry_type: NamespaceEntryType) -> Self {
        Self::CanonicalRole { role, entry_type }
    }

    pub fn published_identity(bytes: Vec<u8>) -> Self {
        Self::PublishedIdentity {
            entry_type: NamespaceEntryType::RegularFile,
            bytes,
        }
    }

    pub fn staged_identity(name: StagedNamespaceName) -> Self {
        Self::StagedIdentity {
            name,
            entry_type: NamespaceEntryType::RegularFile,
        }
    }

    pub fn unknown(relative_name: impl Into<String>, entry_type: NamespaceEntryType) -> Self {
        Self::Unknown {
            relative_name: relative_name.into(),
            entry_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamespaceRootObservation {
    Absent,
    ExistingDirectory {
        entries: Vec<NamespaceEntryObservation>,
        contention: NamespaceContention,
    },
    ExistingNonDirectory,
}

impl NamespaceRootObservation {
    pub fn directory(entries: Vec<NamespaceEntryObservation>) -> Self {
        Self::ExistingDirectory {
            entries,
            contention: NamespaceContention::Available,
        }
    }

    pub fn contended_directory(entries: Vec<NamespaceEntryObservation>) -> Self {
        Self::ExistingDirectory {
            entries,
            contention: NamespaceContention::Contended,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreNamespaceClassification {
    AbsentEligible,
    EmptyEligible,
    IncompleteScaffold {
        staged_identity_count: usize,
    },
    Initialized {
        identity: StableStoreIdentity,
        staged_residue_count: usize,
    },
    ContendedCompatible {
        identity: StableStoreIdentity,
        staged_residue_count: usize,
    },
    UnsupportedVersion(StoreNamespaceIdentityDecodeError),
    Damaged(NamespaceDamage),
    Ambiguous(NamespaceAmbiguity),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamespaceDamage {
    RootIsNotDirectory,
    WrongEntryType {
        role: StoreNamespaceRelativeRole,
        observed: NamespaceEntryType,
    },
    WrongStagedEntryType {
        observed: NamespaceEntryType,
    },
    StagedIdentityWithoutStagingDirectory,
    MalformedIdentity(StoreNamespaceIdentityDecodeError),
    PublishedIdentityWithoutCompleteScaffold,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamespaceAmbiguity {
    UnknownEntry {
        relative_name: String,
        entry_type: NamespaceEntryType,
    },
    DuplicateCanonicalRole(StoreNamespaceRelativeRole),
    MultiplePublishedIdentities,
}

pub fn classify_store_namespace(
    observation: &NamespaceRootObservation,
) -> StoreNamespaceClassification {
    match observation {
        NamespaceRootObservation::Absent => StoreNamespaceClassification::AbsentEligible,
        NamespaceRootObservation::ExistingNonDirectory => {
            StoreNamespaceClassification::Damaged(NamespaceDamage::RootIsNotDirectory)
        }
        NamespaceRootObservation::ExistingDirectory {
            entries,
            contention: _,
        } if entries.is_empty() => StoreNamespaceClassification::EmptyEligible,
        NamespaceRootObservation::ExistingDirectory {
            entries,
            contention,
        } => match ObservedNamespaceInventory::from_entries(entries) {
            Ok(inventory) => inventory.classify(*contention),
            Err(classification) => classification,
        },
    }
}

struct ObservedNamespaceInventory<'a> {
    seen: [bool; StoreNamespaceRelativeRole::ALL.len()],
    published_identity: Option<&'a [u8]>,
    staged_identity_count: usize,
}

impl<'a> ObservedNamespaceInventory<'a> {
    fn from_entries(
        entries: &'a [NamespaceEntryObservation],
    ) -> Result<Self, StoreNamespaceClassification> {
        let mut inventory = Self {
            seen: [false; StoreNamespaceRelativeRole::ALL.len()],
            published_identity: None,
            staged_identity_count: 0,
        };
        for entry in entries {
            inventory.observe(entry)?;
        }
        Ok(inventory)
    }

    fn observe(
        &mut self,
        entry: &'a NamespaceEntryObservation,
    ) -> Result<(), StoreNamespaceClassification> {
        match entry {
            NamespaceEntryObservation::CanonicalRole { role, entry_type } => {
                self.observe_canonical_role(*role, *entry_type)
            }
            NamespaceEntryObservation::PublishedIdentity { entry_type, bytes } => {
                self.observe_published_identity(*entry_type, bytes)
            }
            NamespaceEntryObservation::StagedIdentity { entry_type, .. } => {
                if *entry_type != NamespaceEntryType::RegularFile {
                    return Err(damaged(NamespaceDamage::WrongStagedEntryType {
                        observed: *entry_type,
                    }));
                }
                self.staged_identity_count += 1;
                Ok(())
            }
            NamespaceEntryObservation::Unknown {
                relative_name,
                entry_type,
            } => Err(ambiguous(NamespaceAmbiguity::UnknownEntry {
                relative_name: relative_name.clone(),
                entry_type: *entry_type,
            })),
        }
    }

    fn observe_canonical_role(
        &mut self,
        role: StoreNamespaceRelativeRole,
        entry_type: NamespaceEntryType,
    ) -> Result<(), StoreNamespaceClassification> {
        if role == StoreNamespaceRelativeRole::IdentityRecord {
            return Err(damaged(NamespaceDamage::MalformedIdentity(
                StoreNamespaceIdentityDecodeError::IncorrectLength,
            )));
        }
        if entry_type != role.expected_entry_type() {
            return Err(damaged(NamespaceDamage::WrongEntryType {
                role,
                observed: entry_type,
            }));
        }
        let index = role.index();
        if self.seen[index] {
            return Err(ambiguous(NamespaceAmbiguity::DuplicateCanonicalRole(role)));
        }
        self.seen[index] = true;
        Ok(())
    }

    fn observe_published_identity(
        &mut self,
        entry_type: NamespaceEntryType,
        bytes: &'a [u8],
    ) -> Result<(), StoreNamespaceClassification> {
        if entry_type != NamespaceEntryType::RegularFile {
            return Err(damaged(NamespaceDamage::WrongEntryType {
                role: StoreNamespaceRelativeRole::IdentityRecord,
                observed: entry_type,
            }));
        }
        let index = StoreNamespaceRelativeRole::IdentityRecord.index();
        if self.seen[index] || self.published_identity.is_some() {
            return Err(ambiguous(NamespaceAmbiguity::MultiplePublishedIdentities));
        }
        self.seen[index] = true;
        self.published_identity = Some(bytes);
        Ok(())
    }

    fn classify(self, contention: NamespaceContention) -> StoreNamespaceClassification {
        let Some(bytes) = self.published_identity else {
            return self.classify_incomplete();
        };
        if !all_canonical_roles_seen(&self.seen) {
            return damaged(NamespaceDamage::PublishedIdentityWithoutCompleteScaffold);
        }
        let record = match decode_published_identity(bytes) {
            Ok(record) => record,
            Err(classification) => return classification,
        };
        let identity = StableStoreIdentity::from_published_record(record.proposed_identity());
        match contention {
            NamespaceContention::Available => StoreNamespaceClassification::Initialized {
                identity,
                staged_residue_count: self.staged_identity_count,
            },
            NamespaceContention::Contended => StoreNamespaceClassification::ContendedCompatible {
                identity,
                staged_residue_count: self.staged_identity_count,
            },
        }
    }

    fn classify_incomplete(self) -> StoreNamespaceClassification {
        let has_staging = self.seen[StoreNamespaceRelativeRole::StagingDirectory.index()];
        if self.staged_identity_count != 0 && !has_staging {
            return damaged(NamespaceDamage::StagedIdentityWithoutStagingDirectory);
        }
        StoreNamespaceClassification::IncompleteScaffold {
            staged_identity_count: self.staged_identity_count,
        }
    }
}

fn decode_published_identity(
    bytes: &[u8],
) -> Result<StoreNamespaceIdentityRecord, StoreNamespaceClassification> {
    match StoreNamespaceIdentityRecord::decode(bytes) {
        Ok(record) => Ok(record),
        Err(error @ StoreNamespaceIdentityDecodeError::UnsupportedEncodingVersion(_))
        | Err(error @ StoreNamespaceIdentityDecodeError::UnsupportedNamespaceVersion(_)) => {
            Err(StoreNamespaceClassification::UnsupportedVersion(error))
        }
        Err(error) => Err(damaged(NamespaceDamage::MalformedIdentity(error))),
    }
}

fn damaged(damage: NamespaceDamage) -> StoreNamespaceClassification {
    StoreNamespaceClassification::Damaged(damage)
}

fn ambiguous(ambiguity: NamespaceAmbiguity) -> StoreNamespaceClassification {
    StoreNamespaceClassification::Ambiguous(ambiguity)
}

fn all_canonical_roles_seen(seen: &[bool; StoreNamespaceRelativeRole::ALL.len()]) -> bool {
    seen.iter().all(|present| *present)
}
