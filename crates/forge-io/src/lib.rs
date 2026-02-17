//! # forge-io
//!
//! File format support for the Forge geometry kernel.
//! STEP, IGES, STL import/export, and native JSON serialization.

#![forbid(unsafe_code)]

pub mod diff;

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use serde::{Deserialize, Serialize};

use forge_kernel::features::tree::FeatureTree;

/// Current schema version for forward compatibility.
const SCHEMA_VERSION: u32 = 1;

/// Versioned envelope for serialized models.
///
/// Every JSON file starts with this wrapper so that future versions
/// can detect and migrate older schemas automatically.
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionedModel {
    /// Schema version (monotonically increasing).
    version: u32,
    /// The feature tree payload.
    tree: FeatureTree,
}

impl VersionedModel {
    /// Wrap a feature tree with the current schema version.
    pub fn wrap(tree: FeatureTree) -> Self {
        Self {
            version: SCHEMA_VERSION,
            tree,
        }
    }

    /// The schema version of this model.
    pub fn get_version(&self) -> u32 {
        self.version
    }

    /// Consume and return the inner feature tree.
    pub fn into_tree(self) -> FeatureTree {
        self.tree
    }
}

/// Error type for IO operations.
#[derive(Debug)]
pub enum IoError {
    /// Standard IO error.
    Io(std::io::Error),
    /// JSON serialization error.
    Json(serde_json::Error),
    /// Schema version mismatch.
    VersionMismatch {
        /// The version found in the file.
        found: u32,
        /// The maximum version this build supports.
        supported: u32,
    },
}

impl From<std::io::Error> for IoError {
    fn from(e: std::io::Error) -> Self {
        IoError::Io(e)
    }
}

impl From<serde_json::Error> for IoError {
    fn from(e: serde_json::Error) -> Self {
        IoError::Json(e)
    }
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoError::Io(e) => write!(f, "IO error: {}", e),
            IoError::Json(e) => write!(f, "JSON error: {}", e),
            IoError::VersionMismatch { found, supported } => {
                write!(f, "Schema version {} not supported (max: {})", found, supported)
            }
        }
    }
}

/// Save a FeatureTree model to a versioned JSON file.
pub fn save_model<P: AsRef<Path>>(model: &FeatureTree, path: P) -> Result<(), IoError> {
    #[derive(Serialize)]
    struct Envelope<'a> {
        version: u32,
        tree: &'a FeatureTree,
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
pub fn load_model<P: AsRef<Path>>(path: P) -> Result<FeatureTree, IoError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use forge_kernel::features::tree::{FeatureTree, NativeFeature};
    use forge_kernel::features::wrappers::{BooleanFeature, MakeCubeFeature};
    use forge_kernel::boolean::BooleanOp;
    use tempfile::tempdir;

    #[test]
    fn round_trip_preserves_structure() {
        let mut tree = FeatureTree::new();

        let cube = MakeCubeFeature::new("Cube", [0.0, 0.0, 0.0], 10.0);
        let cube_id = tree.register_feature(NativeFeature::MakeCube(cube)).unwrap();

        let tool = MakeCubeFeature::new("Tool", [5.0, 5.0, 5.0], 5.0);
        let tool_id = tree.register_feature(NativeFeature::MakeCube(tool)).unwrap();

        let cut = BooleanFeature::new("Cut", BooleanOp::Subtraction, cube_id, tool_id);
        let _cut_id = tree.register_feature(NativeFeature::Boolean(cut)).unwrap();

        let dir = tempdir().unwrap();
        let path = dir.path().join("model.json");
        save_model(&tree, &path).expect("Failed to save model");

        let loaded = load_model(&path).expect("Failed to load model");

        assert!(loaded.get_node_by_name("Cube").is_some());
        assert!(loaded.get_node_by_name("Tool").is_some());
        assert!(loaded.get_node_by_name("Cut").is_some());
    }

    #[test]
    fn version_header_is_present() {
        let tree = FeatureTree::new();
        let dir = tempdir().unwrap();
        let path = dir.path().join("versioned.json");
        save_model(&tree, &path).expect("Failed to save");

        let raw: serde_json::Value = serde_json::from_reader(
            BufReader::new(File::open(&path).unwrap())
        ).unwrap();

        assert_eq!(raw["version"], SCHEMA_VERSION);
        assert!(raw["tree"].is_object());
    }

    #[test]
    fn future_version_rejected() {
        let tree = FeatureTree::new();
        let future_envelope = VersionedModel {
            version: 999,
            tree,
        };

        let dir = tempdir().unwrap();
        let path = dir.path().join("future.json");
        let file = File::create(&path).unwrap();
        serde_json::to_writer_pretty(BufWriter::new(file), &future_envelope).unwrap();

        let result = load_model(&path);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            IoError::VersionMismatch { found: 999, .. }
        ));
    }

    #[test]
    fn tc01_empty_tree_round_trip() {
        let tree = FeatureTree::new();
        let dir = tempdir().unwrap();
        let path = dir.path().join("tc01.json");

        save_model(&tree, &path).unwrap();
        let loaded = load_model(&path).unwrap();

        let json_a = serde_json::to_string(&tree).unwrap();
        let json_b = serde_json::to_string(&loaded).unwrap();
        assert_eq!(json_a, json_b);
    }

    #[test]
    fn tc02_single_cube_round_trip() {
        let mut tree = FeatureTree::new();
        let cube = MakeCubeFeature::new("Box1", [1.0, 2.0, 3.0], 4.0);
        tree.register_feature(NativeFeature::MakeCube(cube)).unwrap();

        let dir = tempdir().unwrap();
        let path = dir.path().join("tc02.json");
        save_model(&tree, &path).unwrap();
        let loaded = load_model(&path).unwrap();

        assert!(loaded.get_node_by_name("Box1").is_some());
        let json_a = serde_json::to_string(&tree).unwrap();
        let json_b = serde_json::to_string(&loaded).unwrap();
        assert_eq!(json_a, json_b);
    }

    #[test]
    fn tc03_two_features_round_trip() {
        let mut tree = FeatureTree::new();
        let cube = MakeCubeFeature::new("Base", [0.0, 0.0, 0.0], 10.0);
        tree.register_feature(NativeFeature::MakeCube(cube)).unwrap();
        let tool = MakeCubeFeature::new("Cutter", [3.0, 3.0, 3.0], 5.0);
        tree.register_feature(NativeFeature::MakeCube(tool)).unwrap();

        let dir = tempdir().unwrap();
        let path = dir.path().join("tc03.json");
        save_model(&tree, &path).unwrap();
        let loaded = load_model(&path).unwrap();

        assert!(loaded.get_node_by_name("Base").is_some());
        assert!(loaded.get_node_by_name("Cutter").is_some());
        let json_a: serde_json::Value = serde_json::to_value(&tree).unwrap();
        let json_b: serde_json::Value = serde_json::to_value(&loaded).unwrap();
        assert_eq!(json_a, json_b);
    }

    #[test]
    fn tc04_boolean_subtraction_round_trip() {
        let mut tree = FeatureTree::new();
        let cube = MakeCubeFeature::new("Body", [0.0, 0.0, 0.0], 10.0);
        let cube_id = tree.register_feature(NativeFeature::MakeCube(cube)).unwrap();
        let tool = MakeCubeFeature::new("Hole", [2.0, 2.0, 2.0], 4.0);
        let tool_id = tree.register_feature(NativeFeature::MakeCube(tool)).unwrap();
        let cut = BooleanFeature::new("Pocket", BooleanOp::Subtraction, cube_id, tool_id);
        tree.register_feature(NativeFeature::Boolean(cut)).unwrap();

        let dir = tempdir().unwrap();
        let path = dir.path().join("tc04.json");
        save_model(&tree, &path).unwrap();
        let loaded = load_model(&path).unwrap();

        assert!(loaded.get_node_by_name("Body").is_some());
        assert!(loaded.get_node_by_name("Hole").is_some());
        assert!(loaded.get_node_by_name("Pocket").is_some());
        let json_a: serde_json::Value = serde_json::to_value(&tree).unwrap();
        let json_b: serde_json::Value = serde_json::to_value(&loaded).unwrap();
        assert_eq!(json_a, json_b);
    }

    #[test]
    fn tc05_diffability_byte_identical() {
        let mut tree = FeatureTree::new();
        let cube = MakeCubeFeature::new("Part", [1.0, 2.0, 3.0], 7.0);
        let cube_id = tree.register_feature(NativeFeature::MakeCube(cube)).unwrap();
        let tool = MakeCubeFeature::new("Drill", [2.0, 3.0, 4.0], 3.0);
        let tool_id = tree.register_feature(NativeFeature::MakeCube(tool)).unwrap();
        let cut = BooleanFeature::new("Op", BooleanOp::Intersection, cube_id, tool_id);
        tree.register_feature(NativeFeature::Boolean(cut)).unwrap();

        let dir = tempdir().unwrap();
        let path_a = dir.path().join("diff_a.json");
        let path_b = dir.path().join("diff_b.json");

        save_model(&tree, &path_a).unwrap();
        save_model(&tree, &path_b).unwrap();

        let bytes_a = std::fs::read(&path_a).unwrap();
        let bytes_b = std::fs::read(&path_b).unwrap();
        assert_eq!(bytes_a, bytes_b, "Two serializations of the same model must be byte-identical");
    }
}
