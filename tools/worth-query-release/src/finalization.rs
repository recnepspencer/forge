//! Ordered external-signature assembly and independent semantic re-entry.

use std::path::PathBuf;

use worth_query_package_archive::facade::{
    assemble_untrusted_package_release_envelope, encode_package_release_envelope,
    WorthQueryPackageEnvelopeLimits, WorthQueryPackageReleaseEnvelopeSignature,
};

use crate::denial::WorthQueryReleaseCeremonyError as Error;
use crate::expectations::ReleaseExpectations;
use crate::input::read_bounded_input;
use crate::output::write_release_outputs;
use crate::readmission::readmit_exact_release;
use crate::report::WorthQueryReleaseCeremonyReport;

pub(crate) struct ReleaseInputPaths {
    signing_payload: PathBuf,
    signature: PathBuf,
}

impl ReleaseInputPaths {
    pub(crate) const fn new(signing_payload: PathBuf, signature: PathBuf) -> Self {
        Self {
            signing_payload,
            signature,
        }
    }
}

pub(crate) struct ReleaseOutputPaths {
    envelope: PathBuf,
    report: PathBuf,
}

impl ReleaseOutputPaths {
    pub(crate) const fn new(envelope: PathBuf, report: PathBuf) -> Self {
        Self { envelope, report }
    }
}

pub(crate) struct ReleaseFinalization {
    inputs: ReleaseInputPaths,
    expectations: ReleaseExpectations,
    outputs: ReleaseOutputPaths,
}

impl ReleaseFinalization {
    pub(crate) const fn new(
        inputs: ReleaseInputPaths,
        expectations: ReleaseExpectations,
        outputs: ReleaseOutputPaths,
    ) -> Self {
        Self {
            inputs,
            expectations,
            outputs,
        }
    }
}

pub(crate) fn finalize_release(request: ReleaseFinalization) -> Result<(), Error> {
    let limits = WorthQueryPackageEnvelopeLimits::DEFAULT;
    let signing_payload = read_bounded_input(
        &request.inputs.signing_payload,
        limits.maximum_envelope_bytes(),
    )?;
    let signature = WorthQueryPackageReleaseEnvelopeSignature::new(read_bounded_input(
        &request.inputs.signature,
        u64::from(limits.maximum_signature_bytes()),
    )?)
    .map_err(|denial| Error::Archive {
        stage: "signature intake",
        denial,
    })?;
    request
        .expectations
        .admit_signature_bytes(signature.bytes().len())?;
    let envelope = assemble_untrusted_package_release_envelope(&signing_payload, signature, limits)
        .map_err(|denial| Error::Archive {
            stage: "external-signature assembly",
            denial,
        })?;
    request.expectations.admit(envelope.envelope().unsigned())?;
    let freshly_validated_identity = readmit_exact_release(
        envelope.envelope().unsigned(),
        request.expectations.package_identity(),
    )?;
    let envelope_bytes =
        encode_package_release_envelope(envelope.envelope(), limits).map_err(|denial| {
            Error::Archive {
                stage: "canonical output encoding",
                denial,
            }
        })?;
    let report = WorthQueryReleaseCeremonyReport::derive(
        &envelope,
        &freshly_validated_identity,
        &envelope_bytes,
    );
    let report_bytes = serde_json::to_vec_pretty(&report).map_err(Error::ReportEncoding)?;
    write_release_outputs(
        &request.outputs.envelope,
        &envelope_bytes,
        &request.outputs.report,
        &report_bytes,
    )
}
