//! Typed command-line surface for the release ceremony.

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::denial::WorthQueryReleaseCeremonyError;
use crate::expectations::{
    ExpectedProvenance, ExpectedRelease, ExpectedSigner, ReleaseExpectations,
};
use crate::finalization::{
    finalize_release, ReleaseFinalization, ReleaseInputPaths, ReleaseOutputPaths,
};
use crate::preflight::{preflight_release, ReleasePreflight};

#[derive(Debug, Parser)]
#[command(name = "worth-query-release")]
#[command(about = "Preflight and assemble an externally signed Query release")]
pub(crate) struct WorthQueryReleaseCommand {
    #[command(subcommand)]
    operation: WorthQueryReleaseOperation,
}

#[derive(Debug, Subcommand)]
enum WorthQueryReleaseOperation {
    /// Canonically readmit and stage the exact bytes a host signer may sign.
    Preflight(PreflightArguments),
    /// Assemble an opaque external signature and emit an untrusted release artifact.
    Finalize(FinalizeArguments),
}

#[derive(Debug, Args)]
struct PreflightArguments {
    #[arg(long)]
    signing_payload: PathBuf,
    #[command(flatten)]
    expectations: ReleaseExpectationArguments,
    #[arg(long)]
    staged_signing_payload: PathBuf,
}

#[derive(Debug, Args)]
struct FinalizeArguments {
    #[arg(long)]
    signing_payload: PathBuf,
    #[arg(long)]
    signature: PathBuf,
    #[command(flatten)]
    expectations: ReleaseExpectationArguments,
    #[arg(long)]
    output: PathBuf,
    #[arg(long)]
    report: PathBuf,
}

#[derive(Debug, Args)]
struct ReleaseExpectationArguments {
    #[arg(long)]
    expected_package_identity: String,
    #[arg(long)]
    expected_release_name: String,
    #[arg(long)]
    expected_release_version: String,
    #[arg(long)]
    expected_source_repository: String,
    #[arg(long)]
    expected_source_revision: String,
    #[arg(long)]
    expected_source_reference: String,
    #[arg(long)]
    expected_signer_identity: String,
    #[arg(long)]
    expected_signature_protocol_identity: String,
    #[arg(long)]
    expected_signature_protocol_version: u32,
    #[arg(long)]
    expected_signature_bytes: u32,
}

impl WorthQueryReleaseCommand {
    pub(crate) fn execute(self) -> Result<(), WorthQueryReleaseCeremonyError> {
        match self.operation {
            WorthQueryReleaseOperation::Preflight(arguments) => arguments.execute(),
            WorthQueryReleaseOperation::Finalize(arguments) => arguments.execute(),
        }
    }
}

impl PreflightArguments {
    fn execute(self) -> Result<(), WorthQueryReleaseCeremonyError> {
        preflight_release(ReleasePreflight::new(
            self.signing_payload,
            self.expectations.parse()?,
            self.staged_signing_payload,
        ))
    }
}

impl FinalizeArguments {
    fn execute(self) -> Result<(), WorthQueryReleaseCeremonyError> {
        finalize_release(ReleaseFinalization::new(
            ReleaseInputPaths::new(self.signing_payload, self.signature),
            self.expectations.parse()?,
            ReleaseOutputPaths::new(self.output, self.report),
        ))
    }
}

impl ReleaseExpectationArguments {
    fn parse(self) -> Result<ReleaseExpectations, WorthQueryReleaseCeremonyError> {
        let expected_release = ExpectedRelease::parse(
            &self.expected_package_identity,
            self.expected_release_name,
            self.expected_release_version,
        )?;
        let expected_provenance = ExpectedProvenance::new(
            self.expected_source_repository,
            self.expected_source_revision,
            self.expected_source_reference,
        );
        let expected_signer = ExpectedSigner::new(
            self.expected_signer_identity,
            self.expected_signature_protocol_identity,
            self.expected_signature_protocol_version,
            self.expected_signature_bytes,
        );
        Ok(ReleaseExpectations::new(
            expected_release,
            expected_provenance,
            expected_signer,
        ))
    }
}
