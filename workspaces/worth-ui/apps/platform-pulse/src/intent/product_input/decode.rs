use std::path::Path;
use std::thread;
use std::time::Duration;

use serde::Deserialize;

use super::{
    PlatformPulseExecutorGatePosture, PlatformPulseIntentInputOperability,
    PlatformPulseIntentInputRecord, PlatformPulseIntentInputWatchDenial, INPUT_BYTE_LIMIT,
    INPUT_IDENTITY, INPUT_SCHEMA_VERSION,
};

const READ_SETTLEMENT_INTERVAL: Duration = Duration::from_millis(5);
const MAXIMUM_READ_SETTLEMENT_ATTEMPTS: usize = 8;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoredPlatformPulseIntentInput {
    protocol: String,
    schema_version: u16,
    revision: u64,
    operability: PlatformPulseIntentInputOperability,
    executor_gate: PlatformPulseExecutorGatePosture,
}

pub(super) fn read_record(
    target: &Path,
) -> Result<PlatformPulseIntentInputRecord, PlatformPulseIntentInputWatchDenial> {
    let bytes = settle_exact_target_read(target)?;
    if bytes.len() > INPUT_BYTE_LIMIT {
        return Err(PlatformPulseIntentInputWatchDenial::InputTooLarge {
            observed: bytes.len(),
            maximum: INPUT_BYTE_LIMIT,
        });
    }
    let authored: AuthoredPlatformPulseIntentInput = serde_json::from_slice(&bytes)
        .map_err(|error| PlatformPulseIntentInputWatchDenial::Decode(error.to_string()))?;
    if authored.protocol != INPUT_IDENTITY {
        return Err(PlatformPulseIntentInputWatchDenial::UnsupportedProtocol);
    }
    if authored.schema_version != INPUT_SCHEMA_VERSION {
        return Err(PlatformPulseIntentInputWatchDenial::UnsupportedVersion {
            observed: authored.schema_version,
        });
    }
    if authored.revision == 0 {
        return Err(PlatformPulseIntentInputWatchDenial::InvalidRevision);
    }
    Ok(PlatformPulseIntentInputRecord {
        revision: authored.revision,
        operability: authored.operability,
        executor_gate: authored.executor_gate,
    })
}

fn settle_exact_target_read(target: &Path) -> Result<Vec<u8>, PlatformPulseIntentInputWatchDenial> {
    for attempt in 0..MAXIMUM_READ_SETTLEMENT_ATTEMPTS {
        match std::fs::read(target) {
            Ok(bytes) => return Ok(bytes),
            Err(error)
                if transient_read_error(&error)
                    && attempt + 1 < MAXIMUM_READ_SETTLEMENT_ATTEMPTS =>
            {
                thread::sleep(READ_SETTLEMENT_INTERVAL);
            }
            Err(error) => return Err(PlatformPulseIntentInputWatchDenial::Read(error.to_string())),
        }
    }
    unreachable!("the bounded read-settlement loop always returns")
}

fn transient_read_error(error: &std::io::Error) -> bool {
    if error.kind() == std::io::ErrorKind::NotFound {
        return true;
    }
    #[cfg(windows)]
    {
        matches!(error.raw_os_error(), Some(32 | 33))
    }
    #[cfg(not(windows))]
    {
        false
    }
}
