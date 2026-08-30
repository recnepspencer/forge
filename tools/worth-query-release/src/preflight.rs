//! Pre-effect admission and no-overwrite staging for one signing payload.

use std::path::PathBuf;

use worth_query_package_archive::facade::{
    decode_package_release_signing_payload, WorthQueryPackageEnvelopeLimits,
    WorthQueryUntrustedPackageReleaseSigningPayload,
};

use crate::denial::WorthQueryReleaseCeremonyError as Error;
use crate::expectations::ReleaseExpectations;
use crate::input::read_bounded_input;
use crate::output::write_new_output;
use crate::readmission::readmit_exact_release;

pub(crate) struct ReleasePreflight {
    signing_payload: PathBuf,
    expectations: ReleaseExpectations,
    staged_signing_payload: PathBuf,
}

impl ReleasePreflight {
    pub(crate) const fn new(
        signing_payload: PathBuf,
        expectations: ReleaseExpectations,
        staged_signing_payload: PathBuf,
    ) -> Self {
        Self {
            signing_payload,
            expectations,
            staged_signing_payload,
        }
    }
}

pub(crate) fn preflight_release(request: ReleasePreflight) -> Result<(), Error> {
    let limits = WorthQueryPackageEnvelopeLimits::DEFAULT;
    let source = read_bounded_input(&request.signing_payload, limits.maximum_envelope_bytes())?;
    let payload = decode_package_release_signing_payload(&source, limits).map_err(|denial| {
        Error::Archive {
            stage: "signing-payload preflight",
            denial,
        }
    })?;
    request.expectations.admit(payload.unsigned())?;
    readmit_exact_release(payload.unsigned(), request.expectations.package_identity())?;
    admit_expected_signature_capacity(&payload, request.expectations.signature_bytes(), limits)?;
    write_new_output(&request.staged_signing_payload, payload.signing_payload())
}

fn admit_expected_signature_capacity(
    payload: &WorthQueryUntrustedPackageReleaseSigningPayload,
    expected_signature_bytes: u32,
    limits: WorthQueryPackageEnvelopeLimits,
) -> Result<(), Error> {
    payload
        .unsigned()
        .require_external_signature_capacity(expected_signature_bytes, limits)
        .map_err(|denial| Error::Archive {
            stage: "expected signature capacity",
            denial,
        })
}
