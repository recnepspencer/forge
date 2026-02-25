//! Persistence helpers for versioned audit records and trace bundles.

use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::IoError;

use super::schema::{
    AUDIT_SCHEMA_VERSION, AuditBundleFiles, AuditBundleManifest,
    VersionedAuditRecord,
};

/// Save a versioned audit record to a JSON file.
pub fn save_audit_record<P, T>(
    record: &VersionedAuditRecord<T>,
    path: P,
) -> Result<(), IoError>
where
    P: AsRef<Path>,
    T: Serialize,
{
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, record)?;
    Ok(())
}

/// Load a versioned audit record from a JSON file.
///
/// Rejects envelopes whose schema version exceeds the current supported version.
pub fn load_audit_record<P, T>(path: P) -> Result<VersionedAuditRecord<T>, IoError>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let envelope: VersionedAuditRecord<T> = serde_json::from_reader(reader)?;
    if envelope.schema_version > AUDIT_SCHEMA_VERSION {
        return Err(IoError::VersionMismatch {
            found: envelope.schema_version,
            supported: AUDIT_SCHEMA_VERSION,
        });
    }
    Ok(envelope)
}

/// Append one versioned audit record as a JSON line to an append-only log file.
///
/// The file is created if missing. Each call appends exactly one line and does
/// not rewrite previous entries.
pub fn append_audit_record_jsonl<P, T>(
    record: &VersionedAuditRecord<T>,
    path: P,
) -> Result<(), IoError>
where
    P: AsRef<Path>,
    T: Serialize,
{
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    serde_json::to_writer(&mut file, record)?;
    file.write_all(b"\n")?;
    file.flush()?;
    Ok(())
}

/// Write a per-operation audit bundle directory containing:
/// - `manifest.json`
/// - `operation.json`
/// - `trace.json` (optional)
///
/// Fails if the operation-id directory already exists (append-only bundle semantics).
pub fn write_audit_bundle<P, T, U>(
    root_dir: P,
    operation_id: &str,
    record: &VersionedAuditRecord<T>,
    trace: Option<&U>,
) -> Result<AuditBundleManifest, IoError>
where
    P: AsRef<Path>,
    T: Serialize,
    U: Serialize,
{
    let root = root_dir.as_ref();
    std::fs::create_dir_all(root)?;

    let bundle_dir = root.join(operation_id);
    std::fs::create_dir(&bundle_dir)?;

    let files = AuditBundleFiles {
        operation_json: "operation.json".to_string(),
        trace_json: trace.map(|_| "trace.json".to_string()),
    };

    save_audit_record(record, bundle_dir.join(&files.operation_json))?;
    if let Some(trace_payload) = trace {
        let trace_file = File::create(bundle_dir.join(files.trace_json.as_ref().expect("trace filename set")))?;
        serde_json::to_writer_pretty(BufWriter::new(trace_file), trace_payload)?;
    }

    let created_at_unix_millis = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let manifest = AuditBundleManifest::from_record(
        operation_id,
        record,
        files,
        created_at_unix_millis,
    );

    let manifest_file = File::create(bundle_dir.join("manifest.json"))?;
    serde_json::to_writer_pretty(BufWriter::new(manifest_file), &manifest)?;

    Ok(manifest)
}

#[cfg(test)]
pub(crate) fn read_jsonl_records<P, T>(path: P) -> Result<Vec<VersionedAuditRecord<T>>, IoError>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    use std::io::BufRead;

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut out = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let envelope: VersionedAuditRecord<T> = serde_json::from_str(&line)?;
        if envelope.schema_version > AUDIT_SCHEMA_VERSION {
            return Err(IoError::VersionMismatch {
                found: envelope.schema_version,
                supported: AUDIT_SCHEMA_VERSION,
            });
        }
        out.push(envelope);
    }
    Ok(out)
}
