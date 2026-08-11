//! Sole derivation site for architectural-law-14 aftermath posture names.

use super::correction_authority::InstalledCorrectionAuthority;
use super::correction_mechanism::InstalledCorrectionMechanism;
use super::denial::{
    WorthQueryAftermathInstallationDenial, WorthQueryAftermathInstallationDenialKind,
};

/// Published aftermath posture derived from the installed authority/mechanism pair.
///
/// This is the only module that produces these four names. Declarations may not
/// state a posture directly.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PublishedAftermathPosture {
    Reversible,
    Compensatable,
    Reconcilable,
    Irreversible,
}

pub fn derive_published_posture(
    authority: InstalledCorrectionAuthority,
    mechanism: Option<&InstalledCorrectionMechanism>,
) -> Result<PublishedAftermathPosture, WorthQueryAftermathInstallationDenial> {
    match (authority, mechanism) {
        (InstalledCorrectionAuthority::NotCorrectable, None) => {
            Ok(PublishedAftermathPosture::Irreversible)
        }
        (InstalledCorrectionAuthority::NotCorrectable, Some(_)) => {
            Err(WorthQueryAftermathInstallationDenial::new(
                WorthQueryAftermathInstallationDenialKind::MechanismPresentForNotCorrectable,
                "not-correctable-rejects-mechanism",
            ))
        }
        (
            InstalledCorrectionAuthority::RuntimeAlone,
            Some(InstalledCorrectionMechanism::RecordedInverse(_)),
        ) => Ok(PublishedAftermathPosture::Reversible),
        (
            InstalledCorrectionAuthority::RuntimeAlone,
            Some(InstalledCorrectionMechanism::Compensation(_)),
        ) => Ok(PublishedAftermathPosture::Compensatable),
        (InstalledCorrectionAuthority::RuntimeWithExternalOwner, Some(_)) => {
            Ok(PublishedAftermathPosture::Reconcilable)
        }
        (InstalledCorrectionAuthority::RuntimeAlone, None)
        | (InstalledCorrectionAuthority::RuntimeWithExternalOwner, None) => {
            Err(WorthQueryAftermathInstallationDenial::new(
                WorthQueryAftermathInstallationDenialKind::MechanismRequired,
                "correctable-authority-requires-mechanism",
            ))
        }
    }
}
