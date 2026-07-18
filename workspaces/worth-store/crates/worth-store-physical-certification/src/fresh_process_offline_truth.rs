use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};
use worth_store_offline_verifier::OperationalTruthReport;

use crate::certification_child_process::{
    decode_hex_32, encode_hex_32, fresh_challenge, validated_current_executable,
};

mod wire;
use wire::{read_report, write_report, FreshProcessTruthReport, TruthRegionKind};

const OBSERVER_ROLE: &str = "offline-truth-observer";
pub const OFFLINE_TRUTH_ROLE_ENV: &str = "WORTH_STORE_S10_OFFLINE_TRUTH_ROLE";
pub const OFFLINE_TRUTH_REPORT_ENV: &str = "WORTH_STORE_S10_OFFLINE_TRUTH_REPORT";
pub const OFFLINE_TRUTH_CHALLENGE_ENV: &str = "WORTH_STORE_S10_OFFLINE_TRUTH_CHALLENGE";
pub const OFFLINE_TRUTH_TARGET_ENV: &str = "WORTH_STORE_S10_OFFLINE_TRUTH_TARGET";

#[derive(Debug)]
pub struct FreshProcessOfflineTruthBaseline {
    target: PathBuf,
    live_digest: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreshProcessDestroyedPrimaryEvidence {
    live_digest: [u8; 32],
    damaged_digest: [u8; 32],
    source_inspection_identity: [u8; 32],
    truth_evidence_identity: [u8; 32],
    observer_process_id: u32,
    evidence_identity: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreshProcessOfflineTruthDenial {
    BaselineUnavailable,
    InvalidEnvironment,
    ExecutableMismatch,
    ObserverLaunch,
    ObserverFailed(Option<i32>),
    MissingOrMalformedReport,
    ChallengeMismatch,
    SameProcessObservation,
    TargetNotDamaged,
    ChildDisagreedWithDamagedBytes,
    TargetRegionMissingOrAmbiguous,
    TargetRemainedTrustedAuthority,
    InvalidTruthIdentity,
    RegionCoverageMismatch,
}

#[derive(Debug)]
pub struct FreshProcessOfflineTruthRunner {
    evidence_directory: PathBuf,
}

impl FreshProcessOfflineTruthBaseline {
    pub fn capture(target: impl AsRef<Path>) -> Result<Self, FreshProcessOfflineTruthDenial> {
        let target = std::fs::canonicalize(target)
            .map_err(|_| FreshProcessOfflineTruthDenial::BaselineUnavailable)?;
        let live_digest =
            sha256_file(&target).ok_or(FreshProcessOfflineTruthDenial::BaselineUnavailable)?;
        Ok(Self {
            target,
            live_digest,
        })
    }

    pub const fn live_digest(&self) -> [u8; 32] {
        self.live_digest
    }
}

impl FreshProcessOfflineTruthRunner {
    pub fn new(evidence_directory: impl Into<PathBuf>) -> Self {
        Self {
            evidence_directory: evidence_directory.into(),
        }
    }

    pub fn certify_destroyed_primary(
        &self,
        baseline: &FreshProcessOfflineTruthBaseline,
        observer_command: &mut Command,
    ) -> Result<FreshProcessDestroyedPrimaryEvidence, FreshProcessOfflineTruthDenial> {
        std::fs::create_dir_all(&self.evidence_directory)
            .map_err(|_| FreshProcessOfflineTruthDenial::ObserverLaunch)?;
        let executable_identity = validated_current_executable(observer_command)
            .ok_or(FreshProcessOfflineTruthDenial::ExecutableMismatch)?;
        let damaged_digest = sha256_file(&baseline.target)
            .ok_or(FreshProcessOfflineTruthDenial::BaselineUnavailable)?;
        if damaged_digest == baseline.live_digest {
            return Err(FreshProcessOfflineTruthDenial::TargetNotDamaged);
        }
        let mut subject = [0_u8; 64];
        subject[..32].copy_from_slice(&baseline.live_digest);
        subject[32..].copy_from_slice(&damaged_digest);
        let challenge = fresh_challenge(
            b"worth-store-s10-fresh-process-offline-truth-v1",
            &subject,
            executable_identity,
        );
        let report_path = self
            .evidence_directory
            .join(format!("{}.truth", encode_hex_32(&challenge)));
        observer_command
            .env(OFFLINE_TRUTH_ROLE_ENV, OBSERVER_ROLE)
            .env(OFFLINE_TRUTH_REPORT_ENV, &report_path)
            .env(OFFLINE_TRUTH_CHALLENGE_ENV, encode_hex_32(&challenge))
            .env(OFFLINE_TRUTH_TARGET_ENV, &baseline.target);
        let status = observer_command
            .status()
            .map_err(|_| FreshProcessOfflineTruthDenial::ObserverLaunch)?;
        if !status.success() {
            return Err(FreshProcessOfflineTruthDenial::ObserverFailed(
                status.code(),
            ));
        }
        let report = read_report(&report_path)?;
        let _ = std::fs::remove_file(report_path);
        validate_report(
            report,
            challenge,
            baseline,
            damaged_digest,
            executable_identity,
        )
    }
}

pub fn write_offline_truth_observation_from_environment(
    truth: &OperationalTruthReport,
) -> Result<bool, FreshProcessOfflineTruthDenial> {
    if std::env::var(OFFLINE_TRUTH_ROLE_ENV).ok().as_deref() != Some(OBSERVER_ROLE) {
        return Ok(false);
    }
    let report_path = required_path(OFFLINE_TRUTH_REPORT_ENV)?;
    let target = std::fs::canonicalize(required_path(OFFLINE_TRUTH_TARGET_ENV)?)
        .map_err(|_| FreshProcessOfflineTruthDenial::InvalidEnvironment)?;
    let challenge = decode_hex_32(
        &std::env::var(OFFLINE_TRUTH_CHALLENGE_ENV)
            .map_err(|_| FreshProcessOfflineTruthDenial::InvalidEnvironment)?,
    )
    .ok_or(FreshProcessOfflineTruthDenial::InvalidEnvironment)?;
    let mut matches = truth.regions().iter().filter(|region| {
        std::fs::canonicalize(region.evidence().source()).is_ok_and(|source| source == target)
    });
    let region = matches
        .next()
        .ok_or(FreshProcessOfflineTruthDenial::TargetRegionMissingOrAmbiguous)?;
    if matches.next().is_some() {
        return Err(FreshProcessOfflineTruthDenial::TargetRegionMissingOrAmbiguous);
    }
    let (start, end) = region.evidence().range();
    write_report(
        &report_path,
        &FreshProcessTruthReport {
            challenge,
            observer_process_id: std::process::id(),
            source_inspection_identity: truth.source_inspection_identity(),
            truth_evidence_identity: truth.truth_evidence_identity(),
            observed_content_digest: region.evidence().content_digest(),
            region_kind: TruthRegionKind::from_region(region),
            start,
            end,
        },
    )?;
    Ok(true)
}

fn validate_report(
    report: FreshProcessTruthReport,
    challenge: [u8; 32],
    baseline: &FreshProcessOfflineTruthBaseline,
    damaged_digest: [u8; 32],
    executable_identity: [u8; 32],
) -> Result<FreshProcessDestroyedPrimaryEvidence, FreshProcessOfflineTruthDenial> {
    if report.challenge != challenge {
        return Err(FreshProcessOfflineTruthDenial::ChallengeMismatch);
    }
    if report.observer_process_id == 0 || report.observer_process_id == std::process::id() {
        return Err(FreshProcessOfflineTruthDenial::SameProcessObservation);
    }
    if report.observed_content_digest != damaged_digest {
        return Err(FreshProcessOfflineTruthDenial::ChildDisagreedWithDamagedBytes);
    }
    if report.region_kind == TruthRegionKind::TrustedAuthority {
        return Err(FreshProcessOfflineTruthDenial::TargetRemainedTrustedAuthority);
    }
    if report.source_inspection_identity == [0; 32] || report.truth_evidence_identity == [0; 32] {
        return Err(FreshProcessOfflineTruthDenial::InvalidTruthIdentity);
    }
    let length = std::fs::metadata(&baseline.target)
        .ok()
        .map(|metadata| metadata.len())
        .ok_or(FreshProcessOfflineTruthDenial::BaselineUnavailable)?;
    if report.start != 0 || report.end != length {
        return Err(FreshProcessOfflineTruthDenial::RegionCoverageMismatch);
    }
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-destroyed-primary-evidence-v1");
    digest.update(executable_identity);
    digest.update(challenge);
    digest.update(baseline.live_digest);
    digest.update(damaged_digest);
    digest.update(report.source_inspection_identity);
    digest.update(report.truth_evidence_identity);
    digest.update(report.observer_process_id.to_be_bytes());
    digest.update([report.region_kind as u8]);
    Ok(FreshProcessDestroyedPrimaryEvidence {
        live_digest: baseline.live_digest,
        damaged_digest,
        source_inspection_identity: report.source_inspection_identity,
        truth_evidence_identity: report.truth_evidence_identity,
        observer_process_id: report.observer_process_id,
        evidence_identity: digest.finalize().into(),
    })
}

fn required_path(name: &str) -> Result<PathBuf, FreshProcessOfflineTruthDenial> {
    std::env::var_os(name)
        .map(PathBuf::from)
        .ok_or(FreshProcessOfflineTruthDenial::InvalidEnvironment)
}

fn sha256_file(path: &Path) -> Option<[u8; 32]> {
    let mut file = std::fs::File::open(path).ok()?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer).ok()?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    Some(digest.finalize().into())
}

impl FreshProcessDestroyedPrimaryEvidence {
    pub const fn live_digest(self) -> [u8; 32] {
        self.live_digest
    }
    pub const fn damaged_digest(self) -> [u8; 32] {
        self.damaged_digest
    }
    pub const fn source_inspection_identity(self) -> [u8; 32] {
        self.source_inspection_identity
    }
    pub const fn truth_evidence_identity(self) -> [u8; 32] {
        self.truth_evidence_identity
    }
    pub const fn observer_process_id(self) -> u32 {
        self.observer_process_id
    }
    pub const fn evidence_identity(self) -> [u8; 32] {
        self.evidence_identity
    }
}

impl From<std::io::Error> for FreshProcessOfflineTruthDenial {
    fn from(_: std::io::Error) -> Self {
        Self::MissingOrMalformedReport
    }
}
