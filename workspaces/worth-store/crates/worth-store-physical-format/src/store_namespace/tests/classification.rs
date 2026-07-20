use super::super::*;
use sha2::{Digest, Sha256};

fn encoded_identity(byte: u8) -> Vec<u8> {
    let proposed = ProposedStoreIdentity::from_nonzero_bytes([byte; 16]).expect("nonzero identity");
    StoreNamespaceIdentityRecord::new(StoreNamespaceVersion::CURRENT, proposed)
        .encode()
        .to_vec()
}

fn resign(mut bytes: Vec<u8>) -> Vec<u8> {
    let digest = Sha256::digest(&bytes[..40]);
    bytes[40..].copy_from_slice(&digest);
    bytes
}

fn canonical_entries(identity: Vec<u8>) -> Vec<NamespaceEntryObservation> {
    vec![
        NamespaceEntryObservation::canonical(
            StoreNamespaceRelativeRole::NamespaceDirectory,
            NamespaceEntryType::Directory,
        ),
        NamespaceEntryObservation::published_identity(identity),
        NamespaceEntryObservation::canonical(
            StoreNamespaceRelativeRole::MutationLock,
            NamespaceEntryType::RegularFile,
        ),
        NamespaceEntryObservation::canonical(
            StoreNamespaceRelativeRole::FamiliesDirectory,
            NamespaceEntryType::Directory,
        ),
        NamespaceEntryObservation::canonical(
            StoreNamespaceRelativeRole::StagingDirectory,
            NamespaceEntryType::Directory,
        ),
    ]
}

#[test]
fn namespace_classification_keeps_creation_initialized_and_blocked_states_distinct() {
    assert_eq!(
        classify_store_namespace(&NamespaceRootObservation::Absent),
        StoreNamespaceClassification::AbsentEligible
    );
    assert_eq!(
        classify_store_namespace(&NamespaceRootObservation::directory(vec![])),
        StoreNamespaceClassification::EmptyEligible
    );
    assert_eq!(
        classify_store_namespace(&NamespaceRootObservation::ExistingNonDirectory),
        StoreNamespaceClassification::Damaged(NamespaceDamage::RootIsNotDirectory)
    );
    assert_eq!(
        classify_store_namespace(&NamespaceRootObservation::directory(vec![
            NamespaceEntryObservation::canonical(
                StoreNamespaceRelativeRole::MutationLock,
                NamespaceEntryType::Directory,
            ),
        ])),
        StoreNamespaceClassification::Damaged(NamespaceDamage::WrongEntryType {
            role: StoreNamespaceRelativeRole::MutationLock,
            observed: NamespaceEntryType::Directory,
        })
    );

    let attempt = NamespaceInitializationAttempt::from_nonzero_bytes([4; 16]).unwrap();
    let incomplete = NamespaceRootObservation::directory(vec![
        NamespaceEntryObservation::canonical(
            StoreNamespaceRelativeRole::StagingDirectory,
            NamespaceEntryType::Directory,
        ),
        NamespaceEntryObservation::staged_identity(StagedNamespaceName::for_identity(attempt)),
    ]);
    assert_eq!(
        classify_store_namespace(&incomplete),
        StoreNamespaceClassification::IncompleteScaffold {
            staged_identity_count: 1
        }
    );

    let initialized = canonical_entries(encoded_identity(5));
    assert!(matches!(
        classify_store_namespace(&NamespaceRootObservation::directory(initialized.clone())),
        StoreNamespaceClassification::Initialized { .. }
    ));
    assert!(matches!(
        classify_store_namespace(&NamespaceRootObservation::contended_directory(initialized)),
        StoreNamespaceClassification::ContendedCompatible { .. }
    ));

    let mut unsupported = encoded_identity(5);
    unsupported[10..12].copy_from_slice(&9_u16.to_le_bytes());
    assert!(matches!(
        classify_store_namespace(&NamespaceRootObservation::directory(canonical_entries(
            resign(unsupported)
        ))),
        StoreNamespaceClassification::UnsupportedVersion(
            StoreNamespaceIdentityDecodeError::UnsupportedNamespaceVersion(9)
        )
    ));

    let mut malformed = encoded_identity(5);
    malformed[24] ^= 1;
    assert!(matches!(
        classify_store_namespace(&NamespaceRootObservation::directory(canonical_entries(
            malformed
        ))),
        StoreNamespaceClassification::Damaged(NamespaceDamage::MalformedIdentity(
            StoreNamespaceIdentityDecodeError::ChecksumMismatch
        ))
    ));

    let ambiguous = NamespaceRootObservation::directory(vec![NamespaceEntryObservation::unknown(
        "customer-data",
        NamespaceEntryType::Directory,
    )]);
    assert!(matches!(
        classify_store_namespace(&ambiguous),
        StoreNamespaceClassification::Ambiguous(NamespaceAmbiguity::UnknownEntry {
            relative_name,
            entry_type: NamespaceEntryType::Directory,
        }) if relative_name == "customer-data"
    ));
}

#[test]
fn residue_and_conflicts_never_change_published_identity_meaning() {
    let attempt = NamespaceInitializationAttempt::from_nonzero_bytes([8; 16]).unwrap();
    let mut initialized_with_residue = canonical_entries(encoded_identity(5));
    initialized_with_residue.push(NamespaceEntryObservation::staged_identity(
        StagedNamespaceName::for_identity(attempt),
    ));
    assert!(matches!(
        classify_store_namespace(&NamespaceRootObservation::directory(
            initialized_with_residue
        )),
        StoreNamespaceClassification::Initialized {
            identity,
            staged_residue_count: 1,
        } if identity.bytes() == [5; 16]
    ));

    let conflicting_identities = NamespaceRootObservation::directory(vec![
        NamespaceEntryObservation::published_identity(encoded_identity(1)),
        NamespaceEntryObservation::published_identity(encoded_identity(2)),
    ]);
    assert_eq!(
        classify_store_namespace(&conflicting_identities),
        StoreNamespaceClassification::Ambiguous(NamespaceAmbiguity::MultiplePublishedIdentities)
    );
}

#[test]
fn stable_identity_classification_has_no_locator_or_process_input() {
    let bytes = encoded_identity(11);
    let classify = || match classify_store_namespace(&NamespaceRootObservation::directory(
        canonical_entries(bytes.clone()),
    )) {
        StoreNamespaceClassification::Initialized { identity, .. } => identity,
        other => panic!("expected initialized namespace, got {other:?}"),
    };
    assert_eq!(classify().bytes(), [11; 16]);
    assert_eq!(classify(), classify());
}
