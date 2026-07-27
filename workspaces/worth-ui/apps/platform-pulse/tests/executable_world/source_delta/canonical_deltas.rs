use std::fmt;

use crate::installation::{CanonicalPlatformPulse, IsolatedPulseInstallation};

use super::atomic_replacement::{self, AppliedPulseSourceDelta, PulseSourceActionFailure};
use super::PulseSourceDeltaIdentity;

const BLUE_TOKEN: &[u8] = b"theme.platform_pulse.blue";
const GREEN_TOKEN: &[u8] = b"theme.platform_pulse.green";
const MALFORMED_SOURCE: &[u8] = b"component platform.pulse.component.seed {";

#[derive(Debug)]
pub(crate) struct GreenPulseSourceDelta {
    bytes: Box<[u8]>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct MalformedPulseSourceDelta {
    _private: (),
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CanonicalBlueRecoverySourceDelta {
    canonical: CanonicalPlatformPulse,
}

#[derive(Debug)]
pub(crate) enum PulseSourceDeltaDefinitionFailure {
    CanonicalBlueTokenMissing,
    CanonicalBlueTokenAmbiguous(usize),
}

impl fmt::Display for PulseSourceDeltaDefinitionFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CanonicalBlueTokenMissing => {
                formatter.write_str("canonical pulse has no blue token")
            }
            Self::CanonicalBlueTokenAmbiguous(count) => {
                write!(formatter, "canonical pulse has {count} blue tokens")
            }
        }
    }
}

impl GreenPulseSourceDelta {
    pub(crate) fn from_checked_in(
        canonical: CanonicalPlatformPulse,
    ) -> Result<Self, PulseSourceDeltaDefinitionFailure> {
        let source = canonical.source_bytes();
        let offsets = source
            .windows(BLUE_TOKEN.len())
            .enumerate()
            .filter_map(|(offset, candidate)| (candidate == BLUE_TOKEN).then_some(offset))
            .collect::<Vec<_>>();
        let [offset] = offsets.as_slice() else {
            return Err(match offsets.len() {
                0 => PulseSourceDeltaDefinitionFailure::CanonicalBlueTokenMissing,
                count => PulseSourceDeltaDefinitionFailure::CanonicalBlueTokenAmbiguous(count),
            });
        };
        let mut bytes = Vec::with_capacity(source.len() - BLUE_TOKEN.len() + GREEN_TOKEN.len());
        bytes.extend_from_slice(&source[..*offset]);
        bytes.extend_from_slice(GREEN_TOKEN);
        bytes.extend_from_slice(&source[*offset + BLUE_TOKEN.len()..]);
        Ok(Self {
            bytes: bytes.into_boxed_slice(),
        })
    }

    pub(crate) fn apply(
        self,
        installation: &IsolatedPulseInstallation,
    ) -> Result<AppliedPulseSourceDelta<Self>, PulseSourceActionFailure> {
        atomic_replacement::apply(installation, PulseSourceDeltaIdentity::Green, &self.bytes)
    }

    pub(crate) fn source_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl MalformedPulseSourceDelta {
    pub(crate) fn stable() -> Self {
        Self { _private: () }
    }

    pub(crate) fn apply(
        self,
        installation: &IsolatedPulseInstallation,
    ) -> Result<AppliedPulseSourceDelta<Self>, PulseSourceActionFailure> {
        atomic_replacement::apply(
            installation,
            PulseSourceDeltaIdentity::Malformed,
            MALFORMED_SOURCE,
        )
    }

    pub(crate) fn source_bytes(self) -> &'static [u8] {
        MALFORMED_SOURCE
    }
}

impl CanonicalBlueRecoverySourceDelta {
    pub(crate) fn exact(canonical: CanonicalPlatformPulse) -> Self {
        Self { canonical }
    }

    pub(crate) fn apply(
        self,
        installation: &IsolatedPulseInstallation,
    ) -> Result<AppliedPulseSourceDelta<Self>, PulseSourceActionFailure> {
        atomic_replacement::apply(
            installation,
            PulseSourceDeltaIdentity::CanonicalBlueRecovery,
            self.canonical.source_bytes(),
        )
    }

    pub(crate) fn source_bytes(self) -> &'static [u8] {
        self.canonical.source_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CanonicalBlueRecoverySourceDelta, GreenPulseSourceDelta, MalformedPulseSourceDelta,
        BLUE_TOKEN, GREEN_TOKEN,
    };
    use crate::installation::{CanonicalPlatformPulse, IsolatedPulseInstallation};

    #[test]
    fn named_atomic_deltas_mutate_only_the_isolated_entry_and_recover_exact_canonical_bytes() {
        let canonical = CanonicalPlatformPulse::checked_in();
        let checkout_before = canonical.source_bytes().to_vec();
        let green = GreenPulseSourceDelta::from_checked_in(canonical).expect("green delta");
        assert_eq!(count(green.source_bytes(), BLUE_TOKEN), 0);
        assert_eq!(count(green.source_bytes(), GREEN_TOKEN), 1);

        let mut installation =
            IsolatedPulseInstallation::install(canonical).expect("isolated installation");
        let green_bytes = green.source_bytes().to_vec();
        let green_receipt = green.apply(&installation).expect("atomic green edit");
        assert_eq!(
            std::fs::read(installation.entry_source()).expect("green source"),
            green_bytes
        );

        let malformed = MalformedPulseSourceDelta::stable();
        let malformed_bytes = malformed.source_bytes();
        let malformed_receipt = malformed
            .apply(&installation)
            .expect("atomic malformed edit");
        assert_eq!(
            std::fs::read(installation.entry_source()).expect("malformed source"),
            malformed_bytes
        );

        let recovery = CanonicalBlueRecoverySourceDelta::exact(canonical);
        assert_eq!(recovery.source_bytes(), canonical.source_bytes());
        let recovery_receipt = recovery.apply(&installation).expect("atomic recovery");
        assert_eq!(
            std::fs::read(installation.entry_source()).expect("recovered source"),
            canonical.source_bytes()
        );
        assert_eq!(green_receipt.action_count(), 1);
        assert_eq!(malformed_receipt.action_count(), 1);
        assert_eq!(recovery_receipt.action_count(), 1);
        assert_eq!(canonical.source_bytes(), checkout_before);
        installation.close().expect("explicit cleanup");
    }

    fn count(source: &[u8], token: &[u8]) -> usize {
        source
            .windows(token.len())
            .filter(|candidate| *candidate == token)
            .count()
    }
}
