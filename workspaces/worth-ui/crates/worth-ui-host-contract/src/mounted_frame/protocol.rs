#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiHostProtocolIdentity(u128);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiHostProtocolVersion(u16);

macro_rules! schema_version {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
        pub struct $name(u16);

        impl $name {
            pub const fn new(revision: u16) -> Self {
                Self(revision)
            }

            pub const fn revision(self) -> u16 {
                self.0
            }
        }
    };
}

schema_version!(UiMountedFrameSchemaVersion);
schema_version!(UiMountedPresentationSchemaVersion);
schema_version!(UiHostObservationSchemaVersion);
schema_version!(UiHostMeasurementSchemaVersion);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostProtocolContract {
    identity: UiHostProtocolIdentity,
    protocol: UiHostProtocolVersion,
    mounted_frame: UiMountedFrameSchemaVersion,
    mounted_presentation: UiMountedPresentationSchemaVersion,
    observation: UiHostObservationSchemaVersion,
    measurement: UiHostMeasurementSchemaVersion,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostProtocolAgreement {
    contract: UiHostProtocolContract,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostProtocolNegotiation {
    Compatible(UiHostProtocolAgreement),
    Incompatible(UiHostProtocolDenial),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostProtocolSchemaFamily {
    MountedFrame,
    MountedPresentation,
    Observation,
    Measurement,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostProtocolDenial {
    ForeignIdentity,
    ProtocolTooOld,
    ProtocolTooNew,
    SchemaTooOld(UiHostProtocolSchemaFamily),
    SchemaTooNew(UiHostProtocolSchemaFamily),
}

impl UiHostProtocolIdentity {
    const WORTH_UI: u128 = 0x574f_5254_482d_5549_2d48_4f53_542d_5631;

    pub const fn worth_ui() -> Self {
        Self(Self::WORTH_UI)
    }

    pub const fn from_untrusted(value: u128) -> Self {
        Self(value)
    }

    pub const fn diagnostic_value(self) -> u128 {
        self.0
    }
}

impl UiHostProtocolVersion {
    pub const fn new(revision: u16) -> Self {
        Self(revision)
    }

    pub const fn revision(self) -> u16 {
        self.0
    }
}

impl UiHostProtocolContract {
    const COMPATIBLE_FLOOR: u16 = 1;
    const CURRENT: u16 = 3;
    const CURRENT_OBSERVATION_SCHEMA: u16 = 4;

    pub const fn current() -> Self {
        Self::new(
            UiHostProtocolIdentity::worth_ui(),
            UiHostProtocolVersion::new(Self::CURRENT),
            UiMountedFrameSchemaVersion::new(Self::CURRENT),
            UiMountedPresentationSchemaVersion::new(Self::CURRENT),
            UiHostObservationSchemaVersion::new(Self::CURRENT_OBSERVATION_SCHEMA),
            UiHostMeasurementSchemaVersion::new(Self::CURRENT),
        )
    }

    pub const fn new(
        identity: UiHostProtocolIdentity,
        protocol: UiHostProtocolVersion,
        mounted_frame: UiMountedFrameSchemaVersion,
        mounted_presentation: UiMountedPresentationSchemaVersion,
        observation: UiHostObservationSchemaVersion,
        measurement: UiHostMeasurementSchemaVersion,
    ) -> Self {
        Self {
            identity,
            protocol,
            mounted_frame,
            mounted_presentation,
            observation,
            measurement,
        }
    }

    pub const fn identity(self) -> UiHostProtocolIdentity {
        self.identity
    }

    pub const fn protocol(self) -> UiHostProtocolVersion {
        self.protocol
    }

    pub const fn mounted_frame(self) -> UiMountedFrameSchemaVersion {
        self.mounted_frame
    }

    pub const fn mounted_presentation(self) -> UiMountedPresentationSchemaVersion {
        self.mounted_presentation
    }

    pub const fn observation(self) -> UiHostObservationSchemaVersion {
        self.observation
    }

    pub const fn measurement(self) -> UiHostMeasurementSchemaVersion {
        self.measurement
    }

    pub fn negotiate(self) -> UiHostProtocolNegotiation {
        let denial = self.compatibility_denial();
        match denial {
            Some(denial) => UiHostProtocolNegotiation::Incompatible(denial),
            None => {
                UiHostProtocolNegotiation::Compatible(UiHostProtocolAgreement { contract: self })
            }
        }
    }

    fn compatibility_denial(self) -> Option<UiHostProtocolDenial> {
        if self.identity != UiHostProtocolIdentity::worth_ui() {
            return Some(UiHostProtocolDenial::ForeignIdentity);
        }
        revision_denial(self.protocol.revision()).map_or_else(
            || {
                [
                    (
                        UiHostProtocolSchemaFamily::MountedFrame,
                        self.mounted_frame.revision(),
                    ),
                    (
                        UiHostProtocolSchemaFamily::MountedPresentation,
                        self.mounted_presentation.revision(),
                    ),
                    (
                        UiHostProtocolSchemaFamily::Observation,
                        self.observation.revision(),
                    ),
                    (
                        UiHostProtocolSchemaFamily::Measurement,
                        self.measurement.revision(),
                    ),
                ]
                .into_iter()
                .find_map(|(family, revision)| schema_denial(family, revision))
            },
            |ordering| {
                Some(match ordering {
                    RevisionDenial::TooOld => UiHostProtocolDenial::ProtocolTooOld,
                    RevisionDenial::TooNew => UiHostProtocolDenial::ProtocolTooNew,
                })
            },
        )
    }
}

impl UiHostProtocolAgreement {
    pub const fn contract(self) -> UiHostProtocolContract {
        self.contract
    }
}

#[derive(Clone, Copy)]
enum RevisionDenial {
    TooOld,
    TooNew,
}

fn revision_denial(revision: u16) -> Option<RevisionDenial> {
    if revision < UiHostProtocolContract::COMPATIBLE_FLOOR {
        Some(RevisionDenial::TooOld)
    } else if revision > UiHostProtocolContract::CURRENT {
        Some(RevisionDenial::TooNew)
    } else {
        None
    }
}

fn schema_denial(
    family: UiHostProtocolSchemaFamily,
    revision: u16,
) -> Option<UiHostProtocolDenial> {
    let denial = if family == UiHostProtocolSchemaFamily::Observation {
        if revision < UiHostProtocolContract::CURRENT_OBSERVATION_SCHEMA {
            Some(RevisionDenial::TooOld)
        } else if revision > UiHostProtocolContract::CURRENT_OBSERVATION_SCHEMA {
            Some(RevisionDenial::TooNew)
        } else {
            None
        }
    } else {
        revision_denial(revision)
    };
    denial.map(|denial| match denial {
        RevisionDenial::TooOld => UiHostProtocolDenial::SchemaTooOld(family),
        RevisionDenial::TooNew => UiHostProtocolDenial::SchemaTooNew(family),
    })
}

#[cfg(test)]
mod tests {
    use super::{
        UiHostMeasurementSchemaVersion, UiHostObservationSchemaVersion, UiHostProtocolContract,
        UiHostProtocolDenial, UiHostProtocolIdentity, UiHostProtocolNegotiation,
        UiHostProtocolSchemaFamily, UiHostProtocolVersion, UiMountedFrameSchemaVersion,
        UiMountedPresentationSchemaVersion,
    };

    #[test]
    fn protocol_and_each_schema_family_negotiate_without_cross_family_substitution() {
        let current = UiHostProtocolContract::current();
        let protocol = current.protocol().revision();
        let frame = current.mounted_frame().revision();
        let presentation = current.mounted_presentation().revision();
        let observation = current.observation().revision();
        let measurement = current.measurement().revision();
        assert!(matches!(
            current.negotiate(),
            UiHostProtocolNegotiation::Compatible(_)
        ));
        assert!(matches!(
            contract(1, 1, 1, observation, 1).negotiate(),
            UiHostProtocolNegotiation::Compatible(_)
        ));
        assert_denial(
            contract(0, frame, presentation, observation, measurement),
            UiHostProtocolDenial::ProtocolTooOld,
        );
        assert_denial(
            contract(protocol + 1, frame, presentation, observation, measurement),
            UiHostProtocolDenial::ProtocolTooNew,
        );
        for (contract, family) in [
            (
                contract(protocol, 0, presentation, observation, measurement),
                UiHostProtocolSchemaFamily::MountedFrame,
            ),
            (
                contract(protocol, frame, 0, observation, measurement),
                UiHostProtocolSchemaFamily::MountedPresentation,
            ),
            (
                contract(protocol, frame, presentation, observation - 1, measurement),
                UiHostProtocolSchemaFamily::Observation,
            ),
            (
                contract(protocol, frame, presentation, observation, 0),
                UiHostProtocolSchemaFamily::Measurement,
            ),
        ] {
            assert_denial(contract, UiHostProtocolDenial::SchemaTooOld(family));
        }
        for (contract, family) in [
            (
                contract(protocol, frame + 1, presentation, observation, measurement),
                UiHostProtocolSchemaFamily::MountedFrame,
            ),
            (
                contract(protocol, frame, presentation + 1, observation, measurement),
                UiHostProtocolSchemaFamily::MountedPresentation,
            ),
            (
                contract(protocol, frame, presentation, observation + 1, measurement),
                UiHostProtocolSchemaFamily::Observation,
            ),
            (
                contract(protocol, frame, presentation, observation, measurement + 1),
                UiHostProtocolSchemaFamily::Measurement,
            ),
        ] {
            assert_denial(contract, UiHostProtocolDenial::SchemaTooNew(family));
        }
        let foreign = UiHostProtocolContract::new(
            UiHostProtocolIdentity::from_untrusted(7),
            UiHostProtocolVersion::new(protocol),
            UiMountedFrameSchemaVersion::new(frame),
            UiMountedPresentationSchemaVersion::new(presentation),
            UiHostObservationSchemaVersion::new(observation),
            UiHostMeasurementSchemaVersion::new(measurement),
        );
        assert_denial(foreign, UiHostProtocolDenial::ForeignIdentity);
    }

    fn contract(
        protocol: u16,
        frame: u16,
        presentation: u16,
        observation: u16,
        measurement: u16,
    ) -> UiHostProtocolContract {
        UiHostProtocolContract::new(
            UiHostProtocolIdentity::worth_ui(),
            UiHostProtocolVersion::new(protocol),
            UiMountedFrameSchemaVersion::new(frame),
            UiMountedPresentationSchemaVersion::new(presentation),
            UiHostObservationSchemaVersion::new(observation),
            UiHostMeasurementSchemaVersion::new(measurement),
        )
    }

    fn assert_denial(contract: UiHostProtocolContract, expected: UiHostProtocolDenial) {
        assert_eq!(
            contract.negotiate(),
            UiHostProtocolNegotiation::Incompatible(expected)
        );
    }
}
