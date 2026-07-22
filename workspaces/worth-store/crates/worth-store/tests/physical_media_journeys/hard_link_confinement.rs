use worth_proof::TransitionOutcome;
use worth_store_physical_backend::{
    FilesystemMediaOwnerAdmissionDenial, MediaQualificationDenial, MutationOwnershipDenial,
    NamespaceConfinementDenialKind,
};

use super::{admit_runtime, media_admission, MediaShutdownOutcome};

#[test]
fn mutation_lock_hard_link_cannot_modify_an_outside_sentinel() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let initialized = match admit_runtime(&root)
        .try_admit_filesystem_media(media_admission())
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("the specimen store must initialize"),
    };
    assert!(matches!(
        initialized.close(),
        MediaShutdownOutcome::Released(_)
    ));

    let sentinel = parent.path().join("outside-sentinel");
    let sentinel_bytes = b"outside bytes must survive";
    std::fs::write(&sentinel, sentinel_bytes).unwrap();
    let lock = root.join("namespace/mutation.lock");
    std::fs::remove_file(&lock).unwrap();
    std::fs::hard_link(&sentinel, &lock).unwrap();

    let outcome = admit_runtime(&root)
        .try_admit_filesystem_media(media_admission())
        .into_raw();
    let TransitionOutcome::Denied(denial) = outcome else {
        panic!("a multi-link mutation lock must deny before publication")
    };
    assert!(matches!(
        denial.reason(),
        MediaQualificationDenial::OwnerPreEffect {
            denial: FilesystemMediaOwnerAdmissionDenial::Ownership(
                MutationOwnershipDenial::Confinement(confinement)
            ),
            ..
        } if confinement.kind() == NamespaceConfinementDenialKind::MultipleLinks
    ));
    assert_eq!(std::fs::read(&sentinel).unwrap(), sentinel_bytes);
    assert_eq!(std::fs::read(&lock).unwrap(), sentinel_bytes);
    denial.into_runtime().abort();
}
