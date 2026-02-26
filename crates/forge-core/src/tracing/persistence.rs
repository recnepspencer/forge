//! Trace persistence — auto-persisting decision logs to disk.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use super::decision_log::DecisionLog;
use super::adjunct::TraceAdjunctRecord;

/// Typed trace persistence failure for explicit emit paths.
#[derive(Debug)]
pub enum TracePersistenceError {
    Io(std::io::Error),
    Serde(serde_json::Error),
}

impl From<std::io::Error> for TracePersistenceError {
    fn from(value: std::io::Error) -> Self { Self::Io(value) }
}

impl From<serde_json::Error> for TracePersistenceError {
    fn from(value: serde_json::Error) -> Self { Self::Serde(value) }
}

/// Resolve the trace output directory (cached, checked once per process).
///
/// Priority:
/// 1. `FORGE_TRACE_DIR` env var (explicit override)
/// 2. In debug builds: `{workspace_root}/traces` (auto-detected from crate location)
/// 3. `None` in release builds without the env var
pub fn resolve_trace_dir() -> Option<PathBuf> {
    static TRACE_DIR: OnceLock<Option<PathBuf>> = OnceLock::new();

    TRACE_DIR.get_or_init(|| {
        if let Ok(dir) = std::env::var("FORGE_TRACE_DIR") {
            return Some(PathBuf::from(dir));
        }

        #[cfg(debug_assertions)]
        {
            let workspace_traces = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../traces");
            if workspace_traces.exists() || std::fs::create_dir_all(&workspace_traces).is_ok() {
                return workspace_traces.canonicalize().ok();
            }
        }

        None
    }).clone()
}

/// Write a trace entry to `trace.json`, accumulating all traces from
/// this process invocation into a single file.
///
/// On the first call in a process, the file is truncated (fresh start).
/// Subsequent calls (from parallel test threads) append to the same file.
/// This ensures `cargo test` produces one file with ALL test traces,
/// while re-running overwrites the previous batch.
pub fn write_trace_file(dir: &Path, log: &DecisionLog, state_hash: u128, status: &str) {
    write_trace_file_with_adjuncts(dir, log, &[], state_hash, status);
}

/// Write a trace entry with typed adjunct payloads to `trace.json`.
///
/// Adjunct records are stored alongside the `DecisionLog` and preserved for
/// trace readers/viewers. Callers should provide adjuncts in deterministic
/// order (or use `TraceAdjunctSet`).
pub fn write_trace_file_with_adjuncts(
    dir: &Path,
    log: &DecisionLog,
    adjuncts: &[TraceAdjunctRecord],
    state_hash: u128,
    status: &str,
) {
    let _ = try_write_trace_file_with_adjuncts(dir, log, adjuncts, state_hash, status);
}

/// Explicit (typed) trace write path for operation finalization and tests.
pub fn try_write_trace_file_with_adjuncts(
    dir: &Path,
    log: &DecisionLog,
    adjuncts: &[TraceAdjunctRecord],
    state_hash: u128,
    status: &str,
) -> Result<(), TracePersistenceError> {
    use std::sync::Mutex;

    /// Process-level guard: `None` = first write hasn't happened yet.
    static FILE_LOCK: OnceLock<Mutex<bool>> = OnceLock::new();

    std::fs::create_dir_all(dir)?;

    let test_name = std::thread::current()
        .name()
        .unwrap_or("unknown")
        .to_string();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);

    let path = dir.join("trace.json");
    let lock = FILE_LOCK.get_or_init(|| Mutex::new(false));

    let mut initialized = lock
        .lock()
        .map_err(|_| std::io::Error::other("trace persistence lock poisoned"))?;

    #[derive(Serialize, Deserialize)]
    struct TraceEntry {
        name: String,
        timestamp: String,
        state_hash: u128,
        status: String,
        log: DecisionLog,
        #[serde(default)]
        adjuncts: Vec<TraceAdjunctRecord>,
    }

    let mut entries: Vec<TraceEntry> = if *initialized {
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        *initialized = true;
        Vec::new()
    };

    entries.push(TraceEntry {
        name: test_name,
        timestamp: format!("{}", timestamp),
        state_hash,
        status: status.to_string(),
        log: log.clone(),
        adjuncts: adjuncts.to_vec(),
    });

    let json = serde_json::to_string_pretty(&entries)?;
    std::fs::write(&path, json)?;
    Ok(())
}
