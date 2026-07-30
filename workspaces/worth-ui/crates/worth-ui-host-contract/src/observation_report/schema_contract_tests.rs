use crate::{
    UiHostObservationSchemaVersion, UiHostProtocolContract, UiHostProtocolDenial,
    UiHostProtocolNegotiation, UiHostProtocolSchemaFamily,
};

#[test]
fn observation_schema_v4_is_required_before_batch_authority_exists() {
    let current = UiHostProtocolContract::current();
    assert_eq!(current.observation().revision(), 4);
    assert!(matches!(
        current.negotiate(),
        UiHostProtocolNegotiation::Compatible(_)
    ));
    assert_eq!(
        with_observation_revision(3).negotiate(),
        UiHostProtocolNegotiation::Incompatible(UiHostProtocolDenial::SchemaTooOld(
            UiHostProtocolSchemaFamily::Observation,
        ))
    );
    assert_eq!(
        with_observation_revision(5).negotiate(),
        UiHostProtocolNegotiation::Incompatible(UiHostProtocolDenial::SchemaTooNew(
            UiHostProtocolSchemaFamily::Observation,
        ))
    );
}

fn with_observation_revision(revision: u16) -> UiHostProtocolContract {
    let current = UiHostProtocolContract::current();
    UiHostProtocolContract::new(
        current.identity(),
        current.protocol(),
        current.mounted_frame(),
        current.mounted_presentation(),
        UiHostObservationSchemaVersion::new(revision),
        current.measurement(),
    )
}
