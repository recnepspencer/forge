//! Save and load operations for JSON model files.
//!
//! DOMAIN: File I/O for versioned JSON feature tree serialization.
//! DEPENDENCIES: `serde_json`, `forge-kernel` (FeatureTree<NativeFeature>), `IoError`

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use serde::Serialize;

use forge_kernel::engine::facade::FeatureTree;
use forge_kernel::registry::facade::NativeFeature;

use super::schema::{VersionedModel, SCHEMA_VERSION};
use crate::IoError;

/// Save a FeatureTree model to a versioned JSON file.
pub fn save_model<P: AsRef<Path>>(model: &FeatureTree<NativeFeature>, path: P) -> Result<(), IoError> {
    #[derive(Serialize)]
    struct Envelope<'a> {
        version: u32,
        tree: &'a FeatureTree<NativeFeature>,
    }

    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    let envelope = Envelope {
        version: SCHEMA_VERSION,
        tree: model,
    };
    serde_json::to_writer_pretty(writer, &envelope)?;
    Ok(())
}

/// Load a FeatureTree model from a versioned JSON file.
///
/// Returns `IoError::VersionMismatch` if the file's schema version
/// exceeds what this build supports.
pub fn load_model<P: AsRef<Path>>(path: P) -> Result<FeatureTree<NativeFeature>, IoError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let envelope: VersionedModel = serde_json::from_reader(reader)?;

    if envelope.version > SCHEMA_VERSION {
        return Err(IoError::VersionMismatch {
            found: envelope.version,
            supported: SCHEMA_VERSION,
        });
    }

    Ok(envelope.into_tree())
}
