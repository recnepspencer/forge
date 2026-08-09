use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crate::external_observation::PlatformPulseLifecycleStream;

#[cfg(target_os = "windows")]
use super::kill_on_close_job::KillOnCloseJob;
use super::native_desktop_lease::NativeDesktopLease;

const NATIVE_DESKTOP_LEASE_DEADLINE: Duration = Duration::from_secs(30);

#[derive(Debug)]
pub(crate) struct CargoBuiltPlatformPulse {
    executable: PathBuf,
}

pub(crate) struct PlatformPulseProcessLaunch {
    pub(crate) process: LivePlatformPulseProcess,
    pub(crate) lifecycle: PlatformPulseLifecycleStream,
    pub(crate) launch_started: Instant,
}

pub(crate) struct NativePhase2ProcessLaunch {
    pub(crate) process: LivePlatformPulseProcess,
    pub(crate) stdout: std::process::ChildStdout,
}

pub(crate) struct LivePlatformPulseProcess {
    _native_desktop_lease: NativeDesktopLease,
    child: Child,
    #[cfg(target_os = "windows")]
    _kill_on_close_job: KillOnCloseJob,
    fallback_termination_required: bool,
    exit_status: Option<ExitStatus>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct EmergencyPlatformPulseExit {
    status: ExitStatus,
    forced_termination: bool,
    poll_count: u32,
}

#[derive(Debug)]
pub(crate) enum PlatformPulseProcessLaunchFailure {
    CargoExecutableNotAbsolute(PathBuf),
    CargoExecutableMissing(PathBuf),
    NativeDesktopLease,
    Spawn(std::io::Error),
    #[cfg(target_os = "windows")]
    KillOnCloseJob(String),
    MissingStdout {
        teardown: Result<EmergencyPlatformPulseExit, EmergencyPlatformPulseExitFailure>,
    },
    Poll(std::io::Error),
}

#[derive(Debug)]
pub(crate) enum EmergencyPlatformPulseExitFailure {
    Poll(std::io::Error),
    Terminate(std::io::Error),
    Deadline,
}

impl fmt::Display for PlatformPulseProcessLaunchFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CargoExecutableNotAbsolute(path) => write!(
                formatter,
                "Cargo executable is not absolute: {}",
                path.display()
            ),
            Self::CargoExecutableMissing(path) => {
                write!(formatter, "Cargo executable is missing: {}", path.display())
            }
            Self::NativeDesktopLease => {
                formatter.write_str("exclusive native desktop lease deadline elapsed")
            }
            Self::Spawn(error) => write!(formatter, "spawn product process: {error}"),
            #[cfg(target_os = "windows")]
            Self::KillOnCloseJob(error) => write!(formatter, "contain product process: {error}"),
            Self::MissingStdout { teardown } => {
                write!(
                    formatter,
                    "product stdout was not piped; process teardown: "
                )?;
                format_emergency_exit_result(formatter, teardown)
            }
            Self::Poll(error) => write!(formatter, "poll product process: {error}"),
        }
    }
}

impl fmt::Display for EmergencyPlatformPulseExitFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Poll(error) => write!(formatter, "poll process during teardown: {error}"),
            Self::Terminate(error) => write!(formatter, "terminate failed process: {error}"),
            Self::Deadline => formatter.write_str("failed-process termination deadline elapsed"),
        }
    }
}

impl CargoBuiltPlatformPulse {
    pub(crate) fn exact() -> Result<Self, PlatformPulseProcessLaunchFailure> {
        let executable = PathBuf::from(env!("CARGO_BIN_EXE_worth-ui-platform-pulse"));
        if !executable.is_absolute() {
            return Err(PlatformPulseProcessLaunchFailure::CargoExecutableNotAbsolute(executable));
        }
        if !executable.is_file() {
            return Err(PlatformPulseProcessLaunchFailure::CargoExecutableMissing(
                executable,
            ));
        }
        Ok(Self { executable })
    }

    pub(crate) fn launch(
        self,
        source_root: &Path,
        query_source_root: &Path,
        intent_source_root: &Path,
    ) -> Result<PlatformPulseProcessLaunch, PlatformPulseProcessLaunchFailure> {
        let desktop_wait_started = Instant::now();
        let desktop_deadline = desktop_wait_started
            .checked_add(NATIVE_DESKTOP_LEASE_DEADLINE)
            .ok_or(PlatformPulseProcessLaunchFailure::NativeDesktopLease)?;
        let native_desktop_lease = NativeDesktopLease::acquire(desktop_deadline)
            .map_err(|_| PlatformPulseProcessLaunchFailure::NativeDesktopLease)?;
        let launch_started = Instant::now();
        let mut child = Command::new(&self.executable)
            .arg("--source-root")
            .arg(source_root)
            .arg("--query-source-root")
            .arg(query_source_root)
            .arg("--intent-source-root")
            .arg(intent_source_root)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(PlatformPulseProcessLaunchFailure::Spawn)?;
        #[cfg(target_os = "windows")]
        let kill_on_close_job = assign_kill_on_close_job(&mut child)?;
        let mut process = LivePlatformPulseProcess {
            _native_desktop_lease: native_desktop_lease,
            child,
            #[cfg(target_os = "windows")]
            _kill_on_close_job: kill_on_close_job,
            fallback_termination_required: true,
            exit_status: None,
        };
        let Some(stdout) = process.child.stdout.take() else {
            let teardown = process.terminate_after_failure(Instant::now() + Duration::from_secs(5));
            return Err(PlatformPulseProcessLaunchFailure::MissingStdout { teardown });
        };
        Ok(PlatformPulseProcessLaunch {
            process,
            lifecycle: PlatformPulseLifecycleStream::read(stdout),
            launch_started,
        })
    }

    pub(crate) fn launch_native_phase2(
        self,
    ) -> Result<NativePhase2ProcessLaunch, PlatformPulseProcessLaunchFailure> {
        let deadline = Instant::now()
            .checked_add(NATIVE_DESKTOP_LEASE_DEADLINE)
            .ok_or(PlatformPulseProcessLaunchFailure::NativeDesktopLease)?;
        let native_desktop_lease = NativeDesktopLease::acquire(deadline)
            .map_err(|_| PlatformPulseProcessLaunchFailure::NativeDesktopLease)?;
        let mut child = Command::new(&self.executable)
            .arg("--worth-ui-native-phase2-world")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(PlatformPulseProcessLaunchFailure::Spawn)?;
        #[cfg(target_os = "windows")]
        let kill_on_close_job = assign_kill_on_close_job(&mut child)?;
        let mut process = LivePlatformPulseProcess {
            _native_desktop_lease: native_desktop_lease,
            child,
            #[cfg(target_os = "windows")]
            _kill_on_close_job: kill_on_close_job,
            fallback_termination_required: true,
            exit_status: None,
        };
        let Some(stdout) = process.child.stdout.take() else {
            let teardown = process.terminate_after_failure(Instant::now() + Duration::from_secs(5));
            return Err(PlatformPulseProcessLaunchFailure::MissingStdout { teardown });
        };
        Ok(NativePhase2ProcessLaunch { process, stdout })
    }
}

impl LivePlatformPulseProcess {
    pub(crate) fn id(&self) -> u32 {
        self.child.id()
    }

    pub(crate) fn observed_exit(
        &mut self,
    ) -> Result<Option<ExitStatus>, PlatformPulseProcessLaunchFailure> {
        if let Some(status) = self.exit_status {
            return Ok(Some(status));
        }
        let status = self
            .child
            .try_wait()
            .map_err(PlatformPulseProcessLaunchFailure::Poll)?;
        if let Some(status) = status {
            self.exit_status = Some(status);
            self.fallback_termination_required = false;
        }
        Ok(status)
    }

    pub(crate) fn terminate_after_failure(
        &mut self,
        deadline: Instant,
    ) -> Result<EmergencyPlatformPulseExit, EmergencyPlatformPulseExitFailure> {
        let initial_poll_count = 1_u32;
        if let Some(status) = self.poll_exit_after_failure()? {
            return Ok(emergency_exit(status, false, initial_poll_count));
        }
        if let Err(error) = self.child.kill() {
            let race_poll_count = initial_poll_count.saturating_add(1);
            if let Some(status) = self.poll_exit_after_failure()? {
                return Ok(emergency_exit(status, false, race_poll_count));
            }
            return Err(EmergencyPlatformPulseExitFailure::Terminate(error));
        }
        self.await_forced_exit(deadline, initial_poll_count)
    }

    fn await_forced_exit(
        &mut self,
        deadline: Instant,
        prior_poll_count: u32,
    ) -> Result<EmergencyPlatformPulseExit, EmergencyPlatformPulseExitFailure> {
        let mut poll_count = prior_poll_count;
        loop {
            poll_count = poll_count.saturating_add(1);
            if let Some(status) = self.poll_exit_after_failure()? {
                return Ok(emergency_exit(status, true, poll_count));
            }
            if Instant::now() >= deadline {
                return Err(EmergencyPlatformPulseExitFailure::Deadline);
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    fn poll_exit_after_failure(
        &mut self,
    ) -> Result<Option<ExitStatus>, EmergencyPlatformPulseExitFailure> {
        self.observed_exit().map_err(|failure| match failure {
            PlatformPulseProcessLaunchFailure::Poll(error) => {
                EmergencyPlatformPulseExitFailure::Poll(error)
            }
            _ => unreachable!("observed_exit returns only Poll"),
        })
    }
}

impl EmergencyPlatformPulseExit {
    pub(crate) fn forced_termination(self) -> bool {
        self.forced_termination
    }
}

#[cfg(target_os = "windows")]
fn assign_kill_on_close_job(
    child: &mut Child,
) -> Result<KillOnCloseJob, PlatformPulseProcessLaunchFailure> {
    KillOnCloseJob::assign(child).map_err(|error| {
        let _ = child.kill();
        let _ = child.wait();
        PlatformPulseProcessLaunchFailure::KillOnCloseJob(error)
    })
}

fn emergency_exit(
    status: ExitStatus,
    forced_termination: bool,
    poll_count: u32,
) -> EmergencyPlatformPulseExit {
    EmergencyPlatformPulseExit {
        status,
        forced_termination,
        poll_count,
    }
}

fn format_emergency_exit_result(
    formatter: &mut fmt::Formatter<'_>,
    result: &Result<EmergencyPlatformPulseExit, EmergencyPlatformPulseExitFailure>,
) -> fmt::Result {
    match result {
        Ok(exit) => write!(
            formatter,
            "released(status={}, forced={}, polls={})",
            exit.status, exit.forced_termination, exit.poll_count
        ),
        Err(failure) => write!(formatter, "failed({failure})"),
    }
}

impl Drop for LivePlatformPulseProcess {
    fn drop(&mut self) {
        if self.fallback_termination_required {
            if let Err(failure) =
                self.terminate_after_failure(Instant::now() + Duration::from_secs(2))
            {
                eprintln!(
                    "fallback Platform Pulse teardown deferred to kill-on-close containment: {failure}"
                );
            }
        }
    }
}
