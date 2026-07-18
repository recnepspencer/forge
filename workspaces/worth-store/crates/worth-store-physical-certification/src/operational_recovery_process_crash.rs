use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

use worth_store_operations::{OperationalControlSessionObservation, OperationalControlStore};

use crate::certification_child_process::{
    decode_hex_32, encode_hex_32, fresh_challenge, validated_current_executable,
};
use crate::{
    OperationalRecoveryCrashCutDenial, OperationalRecoveryCrashCutEvidence,
    OperationalRecoveryDriverTrace, OperationalRecoveryYieldpoint,
};

const CUT_ROLE: &str = "cut";
const REOPEN_ROLE: &str = "reopen";
const CRASH_EXIT_CODE: i32 = 73;
mod wire;
use wire::{read_report, write_report, ProcessObservationReport};
pub const PROCESS_CRASH_ROLE_ENV: &str = "WORTH_STORE_S10_CRASH_ROLE";
pub const PROCESS_CRASH_REPORT_ENV: &str = "WORTH_STORE_S10_CRASH_REPORT";
pub const PROCESS_CRASH_CHALLENGE_ENV: &str = "WORTH_STORE_S10_CRASH_CHALLENGE";
pub const PROCESS_CRASH_YIELDPOINT_ENV: &str = "WORTH_STORE_S10_CRASH_YIELDPOINT";

#[derive(Debug, Clone)]
pub struct OperationalRecoveryProcessCrashConfig {
    yieldpoint: OperationalRecoveryYieldpoint,
    report_path: PathBuf,
    challenge: [u8; 32],
}

#[derive(Debug)]
pub struct OperationalRecoveryFreshProcessRunner {
    evidence_directory: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationalRecoveryProcessCrashDenial {
    InvalidEnvironment,
    CutProcessLaunch,
    ReopenProcessLaunch,
    CutProcessDidNotCrash(Option<i32>),
    ReopenProcessFailed(Option<i32>),
    MissingOrMalformedReport,
    ChallengeMismatch,
    YieldpointMismatch,
    TraceMismatch,
    ExecutableMismatch,
    CrashCut(OperationalRecoveryCrashCutDenial),
}

impl OperationalRecoveryProcessCrashConfig {
    pub fn from_environment() -> Result<Option<Self>, OperationalRecoveryProcessCrashDenial> {
        if std::env::var(PROCESS_CRASH_ROLE_ENV).ok().as_deref() != Some(CUT_ROLE) {
            return Ok(None);
        }
        let report_path = std::env::var_os(PROCESS_CRASH_REPORT_ENV)
            .map(PathBuf::from)
            .ok_or(OperationalRecoveryProcessCrashDenial::InvalidEnvironment)?;
        let challenge = decode_hex_32(
            &std::env::var(PROCESS_CRASH_CHALLENGE_ENV)
                .map_err(|_| OperationalRecoveryProcessCrashDenial::InvalidEnvironment)?,
        )
        .ok_or(OperationalRecoveryProcessCrashDenial::InvalidEnvironment)?;
        let token = std::env::var(PROCESS_CRASH_YIELDPOINT_ENV)
            .map_err(|_| OperationalRecoveryProcessCrashDenial::InvalidEnvironment)?;
        let yieldpoint = OperationalRecoveryYieldpoint::from_token(&token)
            .ok_or(OperationalRecoveryProcessCrashDenial::InvalidEnvironment)?;
        Ok(Some(Self {
            yieldpoint,
            report_path,
            challenge,
        }))
    }

    pub const fn yieldpoint(&self) -> OperationalRecoveryYieldpoint {
        self.yieldpoint
    }

    pub(crate) fn crash_with_control_observation(
        &self,
        observation: OperationalControlSessionObservation,
        trace: &OperationalRecoveryDriverTrace,
    ) -> ! {
        let report = ProcessObservationReport {
            challenge: self.challenge,
            yieldpoint: self.yieldpoint,
            observation,
            trace_identity: trace.evidence_identity(),
            operations: trace.operation_identities().to_vec(),
        };
        write_report(&self.report_path, &report).expect("write durable S10 crash-cut report");
        std::process::exit(CRASH_EXIT_CODE)
    }
}

pub fn write_reopen_observation_from_environment(
    store: &OperationalControlStore,
) -> Result<bool, OperationalRecoveryProcessCrashDenial> {
    if std::env::var(PROCESS_CRASH_ROLE_ENV).ok().as_deref() != Some(REOPEN_ROLE) {
        return Ok(false);
    }
    let report_path = std::env::var_os(PROCESS_CRASH_REPORT_ENV)
        .map(PathBuf::from)
        .ok_or(OperationalRecoveryProcessCrashDenial::InvalidEnvironment)?;
    let challenge = decode_hex_32(
        &std::env::var(PROCESS_CRASH_CHALLENGE_ENV)
            .map_err(|_| OperationalRecoveryProcessCrashDenial::InvalidEnvironment)?,
    )
    .ok_or(OperationalRecoveryProcessCrashDenial::InvalidEnvironment)?;
    let token = std::env::var(PROCESS_CRASH_YIELDPOINT_ENV)
        .map_err(|_| OperationalRecoveryProcessCrashDenial::InvalidEnvironment)?;
    let yieldpoint = OperationalRecoveryYieldpoint::from_token(&token)
        .ok_or(OperationalRecoveryProcessCrashDenial::InvalidEnvironment)?;
    write_report(
        &report_path,
        &ProcessObservationReport {
            challenge,
            yieldpoint,
            observation: store
                .session_observation()
                .map_err(|_| OperationalRecoveryProcessCrashDenial::MissingOrMalformedReport)?,
            trace_identity: [0; 32],
            operations: Vec::new(),
        },
    )
    .map_err(|_| OperationalRecoveryProcessCrashDenial::MissingOrMalformedReport)?;
    Ok(true)
}

impl OperationalRecoveryFreshProcessRunner {
    pub fn new(evidence_directory: impl Into<PathBuf>) -> Self {
        Self {
            evidence_directory: evidence_directory.into(),
        }
    }

    pub fn certify_control_cut(
        &self,
        cut_command: &mut Command,
        reopen_command: &mut Command,
        yieldpoint: OperationalRecoveryYieldpoint,
        uninterrupted_trace: &OperationalRecoveryDriverTrace,
    ) -> Result<OperationalRecoveryCrashCutEvidence, OperationalRecoveryProcessCrashDenial> {
        std::fs::create_dir_all(&self.evidence_directory)
            .map_err(|_| OperationalRecoveryProcessCrashDenial::CutProcessLaunch)?;
        let executable_identity = validated_executable_identity(cut_command, reopen_command)?;
        let challenge = fresh_challenge(
            b"worth-store-s10-process-crash-challenge-v1",
            yieldpoint.token().as_bytes(),
            executable_identity,
        );
        let stem = encode_hex_32(&challenge);
        let cut_path = self.evidence_directory.join(format!("{stem}.cut"));
        let reopen_path = self.evidence_directory.join(format!("{stem}.reopen"));
        configure_command(cut_command, CUT_ROLE, &cut_path, challenge, yieldpoint);
        configure_command(
            reopen_command,
            REOPEN_ROLE,
            &reopen_path,
            challenge,
            yieldpoint,
        );
        let cut_status = cut_command
            .status()
            .map_err(|_| OperationalRecoveryProcessCrashDenial::CutProcessLaunch)?;
        require_crash_exit(cut_status)?;
        let reopen_status = reopen_command
            .status()
            .map_err(|_| OperationalRecoveryProcessCrashDenial::ReopenProcessLaunch)?;
        if !reopen_status.success() {
            return Err(OperationalRecoveryProcessCrashDenial::ReopenProcessFailed(
                reopen_status.code(),
            ));
        }
        let cut = read_report(&cut_path)?;
        let reopened = read_report(&reopen_path)?;
        let _ = std::fs::remove_file(cut_path);
        let _ = std::fs::remove_file(reopen_path);
        if cut.challenge != challenge || reopened.challenge != challenge {
            return Err(OperationalRecoveryProcessCrashDenial::ChallengeMismatch);
        }
        if cut.yieldpoint != yieldpoint || reopened.yieldpoint != yieldpoint {
            return Err(OperationalRecoveryProcessCrashDenial::YieldpointMismatch);
        }
        if cut.trace_identity == [0; 32]
            || cut.operations.is_empty()
            || cut.operations.iter().any(|operation| {
                !uninterrupted_trace
                    .operation_identities()
                    .contains(operation)
            })
        {
            return Err(OperationalRecoveryProcessCrashDenial::TraceMismatch);
        }
        OperationalRecoveryCrashCutEvidence::from_external_process_reopen(
            yieldpoint,
            cut.trace_identity,
            cut.operations,
            cut.observation,
            reopened.observation,
            challenge,
        )
        .map_err(OperationalRecoveryProcessCrashDenial::CrashCut)
    }
}

fn configure_command(
    command: &mut Command,
    role: &str,
    report_path: &Path,
    challenge: [u8; 32],
    yieldpoint: OperationalRecoveryYieldpoint,
) {
    command
        .env(PROCESS_CRASH_ROLE_ENV, role)
        .env(PROCESS_CRASH_REPORT_ENV, report_path)
        .env(PROCESS_CRASH_CHALLENGE_ENV, encode_hex_32(&challenge))
        .env(PROCESS_CRASH_YIELDPOINT_ENV, yieldpoint.token());
}

fn require_crash_exit(status: ExitStatus) -> Result<(), OperationalRecoveryProcessCrashDenial> {
    if status.code() == Some(CRASH_EXIT_CODE) {
        Ok(())
    } else {
        Err(OperationalRecoveryProcessCrashDenial::CutProcessDidNotCrash(status.code()))
    }
}

fn validated_executable_identity(
    cut_command: &Command,
    reopen_command: &Command,
) -> Result<[u8; 32], OperationalRecoveryProcessCrashDenial> {
    let cut = validated_current_executable(cut_command)
        .ok_or(OperationalRecoveryProcessCrashDenial::ExecutableMismatch)?;
    let reopened = validated_current_executable(reopen_command)
        .ok_or(OperationalRecoveryProcessCrashDenial::ExecutableMismatch)?;
    if cut == reopened {
        Ok(cut)
    } else {
        Err(OperationalRecoveryProcessCrashDenial::ExecutableMismatch)
    }
}
