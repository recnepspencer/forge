use std::path::PathBuf;
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const PROCESS_LANE_NAME: &str = "phase-eight-fresh-process-exclusive-disk-lane";
const PROCESS_LANE_BUDGET: Duration = Duration::from_secs(30 * 60);
const PROCESS_LANE_ACQUIRE_BUDGET: Duration = Duration::from_secs(30 * 60);
const PROCESS_LANE_STALE_AFTER: Duration = Duration::from_secs(45 * 60);
const PROCESS_LANE_EMPTY_GRACE: Duration = Duration::from_secs(5);
const OWNER_FILE: &str = ".owner";

static PROCESS_LANE: OnceLock<Mutex<()>> = OnceLock::new();

pub(super) struct ProcessLaneGuard {
    _local: MutexGuard<'static, ()>,
    interprocess_path: PathBuf,
    owner_path: PathBuf,
    owner_token: String,
    started: Instant,
    closed: bool,
}

pub(super) fn acquire() -> Result<ProcessLaneGuard, String> {
    let local = PROCESS_LANE
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let interprocess_path = std::env::temp_dir().join(PROCESS_LANE_NAME);
    let deadline = Instant::now() + PROCESS_LANE_ACQUIRE_BUDGET;
    loop {
        match std::fs::create_dir(&interprocess_path) {
            Ok(()) => break,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                reclaim_stale_lane(&interprocess_path)?;
                if Instant::now() >= deadline {
                    return Err(format!(
                        "timed out acquiring {PROCESS_LANE_NAME} at {}",
                        interprocess_path.display()
                    ));
                }
                std::thread::sleep(Duration::from_millis(25));
            }
            Err(error) => {
                return Err(format!(
                    "create {PROCESS_LANE_NAME} interprocess gate {}: {error}",
                    interprocess_path.display()
                ))
            }
        }
    }
    let owner_path = interprocess_path.join(OWNER_FILE);
    let owner_token = owner_token();
    if let Err(error) = std::fs::write(&owner_path, &owner_token) {
        let cleanup = std::fs::remove_dir(&interprocess_path)
            .err()
            .map(|cleanup| format!("; cleanup failed: {cleanup}"))
            .unwrap_or_default();
        return Err(format!(
            "write {PROCESS_LANE_NAME} owner {}: {error}{cleanup}",
            owner_path.display()
        ));
    }
    Ok(ProcessLaneGuard {
        _local: local,
        interprocess_path,
        owner_path,
        owner_token,
        started: Instant::now(),
        closed: false,
    })
}

impl Drop for ProcessLaneGuard {
    fn drop(&mut self) {
        if !self.closed {
            if let Err(error) =
                close_owned_lane(&self.interprocess_path, &self.owner_path, &self.owner_token)
            {
                eprintln!("Phase 8 emergency process-lane cleanup failed: {error}");
            }
        }
    }
}

fn owner_token() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after the Unix epoch")
        .as_nanos();
    format!("pid={} started={timestamp}\n", std::process::id())
}

fn reclaim_stale_lane(path: &PathBuf) -> Result<(), String> {
    let owner = path.join(OWNER_FILE);
    let age = std::fs::metadata(path)
        .ok()
        .and_then(|metadata| metadata.modified().ok())
        .and_then(|modified| modified.elapsed().ok());
    let entries = std::fs::read_dir(path)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    if entries.is_empty() && age.is_some_and(|age| age >= PROCESS_LANE_EMPTY_GRACE) {
        remove_if_present(path, false)?;
        return Ok(());
    }
    if entries.iter().any(|entry| entry.path() != owner) {
        return Ok(());
    }
    let owner_alive = owner_pid(&owner).is_some_and(owner_process_is_alive);
    let stale = age.is_some_and(|age| age >= PROCESS_LANE_STALE_AFTER);
    if owner_alive && !stale {
        return Ok(());
    }
    remove_if_present(&owner, true)?;
    remove_if_present(path, false)
}

fn remove_if_present(path: &PathBuf, file: bool) -> Result<(), String> {
    let result = if file {
        std::fs::remove_file(path)
    } else {
        std::fs::remove_dir(path)
    };
    match result {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!(
            "remove stale process-lane {}: {error}",
            path.display()
        )),
    }
}

fn owner_pid(path: &PathBuf) -> Option<u32> {
    std::fs::read_to_string(path)
        .ok()?
        .split_whitespace()
        .find_map(|field| field.strip_prefix("pid=")?.parse::<u32>().ok())
}

fn owner_process_is_alive(pid: u32) -> bool {
    if pid == std::process::id() {
        return true;
    }
    #[cfg(windows)]
    {
        return Command::new("tasklist")
            .args(["/FI", &format!("PID eq {pid}"), "/NH"])
            .output()
            .ok()
            .is_some_and(|output| {
                String::from_utf8_lossy(&output.stdout).contains(&pid.to_string())
            });
    }
    #[cfg(unix)]
    {
        return Command::new("kill")
            .args(["-0", &pid.to_string()])
            .status()
            .is_ok_and(|status| status.success());
    }
    #[cfg(not(any(windows, unix)))]
    {
        true
    }
}

pub(super) const fn lane_name() -> &'static str {
    PROCESS_LANE_NAME
}

impl ProcessLaneGuard {
    pub(super) fn close(mut self) -> Result<(), String> {
        close_owned_lane(&self.interprocess_path, &self.owner_path, &self.owner_token)?;
        self.closed = true;
        Ok(())
    }

    pub(super) fn assert_within_budget(&self, owner: &str) {
        let elapsed = self.started.elapsed();
        assert!(
            elapsed <= PROCESS_LANE_BUDGET,
            "{owner} exceeded the serialized {PROCESS_LANE_NAME} budget of {:?}: {:?}",
            PROCESS_LANE_BUDGET,
            elapsed
        );
    }
}

fn close_owned_lane(path: &PathBuf, owner: &PathBuf, expected: &str) -> Result<(), String> {
    let actual = std::fs::read_to_string(owner).map_err(|error| {
        format!(
            "read {PROCESS_LANE_NAME} owner {}: {error}",
            owner.display()
        )
    })?;
    if actual != expected {
        return Err(format!(
            "{PROCESS_LANE_NAME} owner token changed before close"
        ));
    }
    std::fs::remove_file(owner).map_err(|error| {
        format!(
            "remove {PROCESS_LANE_NAME} owner {}: {error}",
            owner.display()
        )
    })?;
    std::fs::remove_dir(path).map_err(|error| {
        format!(
            "remove {PROCESS_LANE_NAME} directory {}: {error}",
            path.display()
        )
    })?;
    if owner.exists() || path.exists() {
        return Err(format!(
            "{PROCESS_LANE_NAME} residue survived explicit close"
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "process_lane/tests.rs"]
mod tests;
