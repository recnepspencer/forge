use super::super::*;

#[test]
fn profile_preserves_indeterminate_support_and_rejects_invalid_cardinality() {
    let observations = MediaCapability::ALL.map(|capability| {
        let support = if capability == MediaCapability::DirectIo {
            CapabilitySupport::Indeterminate
        } else {
            CapabilitySupport::Supported
        };
        MediaCapabilityObservation::new(capability, support)
    });
    let profile = FilesystemBackendProfile::from_observations(&observations)
        .expect("one observation for every capability");
    assert_eq!(
        profile.support(MediaCapability::DirectIo),
        CapabilitySupport::Indeterminate
    );
    assert_eq!(
        profile.support(MediaCapability::Append),
        CapabilitySupport::Supported
    );
    assert_eq!(
        profile.support(MediaCapability::DirectorySynchronization),
        CapabilitySupport::Supported
    );
    assert_eq!(
        profile.support(MediaCapability::AtomicSameNamespaceReplacement),
        CapabilitySupport::Supported
    );

    let duplicate = [MediaCapabilityObservation::new(
        MediaCapability::OrdinaryFile,
        CapabilitySupport::Supported,
    ); MediaCapability::ALL.len()];
    assert_eq!(
        FilesystemBackendProfile::from_observations(&duplicate),
        Err(CapabilityProfileError::Duplicate(
            MediaCapability::OrdinaryFile
        ))
    );

    assert_eq!(
        FilesystemBackendProfile::from_observations(&observations[..observations.len() - 1]),
        Err(CapabilityProfileError::Missing(MediaCapability::DirectIo))
    );
}
