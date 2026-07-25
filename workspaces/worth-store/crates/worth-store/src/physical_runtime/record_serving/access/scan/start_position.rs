use worth_store_physical_format::PersistedRecordIdentity;

use super::{
    manifest_error, scan_manifest_error, ManifestRangeCursor, PhysicalRecordReader,
    RecordScanCounterSnapshot, RecordScanDenial, RecordScanError,
};
use crate::physical_runtime::record_serving::access::{
    manifest_routing::ManifestReader, scan_observation::manifest_snapshot,
};

pub(super) struct PositionedScanStart {
    pub(super) cursor: ManifestRangeCursor<'static>,
    pub(super) complete: bool,
    pub(super) observation: RecordScanCounterSnapshot,
}

pub(super) fn position_scan_start(
    reader: &PhysicalRecordReader,
    first: Option<PersistedRecordIdentity>,
    runtime: &crate::physical_runtime::instance::PhysicalStoreWorkRuntime,
) -> Result<PositionedScanStart, RecordScanError> {
    let manifest = ManifestReader::serving(
        reader.frame_ports.clone(),
        reader.source.clone(),
        reader.format,
        reader.access,
        reader.current_root.clone(),
    );
    let mut cursor = ManifestRangeCursor::new(manifest);
    let positioned = cursor
        .seek(reader.current_root.routing_root(), first)
        .map_err(|failure| observe_manifest_failure(&cursor, failure, runtime))?;
    require_resume_position(&mut cursor, first, positioned, runtime)?;
    Ok(PositionedScanStart {
        observation: manifest_snapshot(cursor.counters()),
        cursor,
        complete: !positioned && first.is_none(),
    })
}

fn require_resume_position(
    cursor: &mut ManifestRangeCursor<'static>,
    first: Option<PersistedRecordIdentity>,
    positioned: bool,
    runtime: &crate::physical_runtime::instance::PhysicalStoreWorkRuntime,
) -> Result<(), RecordScanError> {
    let Some(expected) = first else {
        return Ok(());
    };
    if !positioned {
        return Err(manifest_error(
            cursor,
            RecordScanDenial::CursorPositionNotFound,
        ));
    }
    let found = cursor
        .next()
        .map_err(|failure| observe_manifest_failure(cursor, failure, runtime))?;
    if found.map(|placement| placement.record()) != Some(expected) {
        return Err(manifest_error(
            cursor,
            RecordScanDenial::CursorPositionNotFound,
        ));
    }
    Ok(())
}

fn observe_manifest_failure(
    cursor: &ManifestRangeCursor<'_>,
    failure: super::ManifestLookupFailure,
    runtime: &crate::physical_runtime::instance::PhysicalStoreWorkRuntime,
) -> RecordScanError {
    let error = scan_manifest_error(cursor, failure);
    runtime.health.observe_scan_denial(error.denial);
    error
}
