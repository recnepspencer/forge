//! Tests for JSON serialization.

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, BufWriter};

    use crate::json::{load_model, save_model, VersionedModel, SCHEMA_VERSION};
    use crate::IoError;
    use forge_kernel::boolean::BooleanOp;
    use forge_kernel::engine::facade::FeatureTree;
    use forge_kernel::primitives::MakePrimitiveFeature;
    use forge_kernel::registry::facade::NativeFeature;
    use tempfile::tempdir;

    #[test]
    fn round_trip_preserves_structure() {
        let mut tree = FeatureTree::new();

        let cube = MakePrimitiveFeature::cube("Cube", [0.0, 0.0, 0.0], 10.0);
        let cube_id = tree
            .register_feature(NativeFeature::primitive("Cube", cube))
            .unwrap();

        let tool = MakePrimitiveFeature::cube("Tool", [5.0, 5.0, 5.0], 5.0);
        let tool_id = tree
            .register_feature(NativeFeature::primitive("Tool", tool))
            .unwrap();

        let _cut_id = tree
            .register_feature(NativeFeature::boolean(
                "Cut",
                BooleanOp::Subtraction,
                cube_id,
                tool_id,
            ))
            .unwrap();

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

        let raw: serde_json::Value =
            serde_json::from_reader(BufReader::new(File::open(&path).unwrap())).unwrap();

        assert_eq!(raw["version"], SCHEMA_VERSION);
        assert!(raw["tree"].is_object());
    }

    #[test]
    fn future_version_rejected() {
        let tree = FeatureTree::new();
        let future_envelope = VersionedModel { version: 999, tree };

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
        let cube = MakePrimitiveFeature::cube("Box1", [1.0, 2.0, 3.0], 4.0);
        tree.register_feature(NativeFeature::primitive("Box1", cube))
            .unwrap();

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
        let cube = MakePrimitiveFeature::cube("Base", [0.0, 0.0, 0.0], 10.0);
        tree.register_feature(NativeFeature::primitive("Base", cube))
            .unwrap();
        let tool = MakePrimitiveFeature::cube("Cutter", [3.0, 3.0, 3.0], 5.0);
        tree.register_feature(NativeFeature::primitive("Cutter", tool))
            .unwrap();

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
        let cube = MakePrimitiveFeature::cube("Body", [0.0, 0.0, 0.0], 10.0);
        let cube_id = tree
            .register_feature(NativeFeature::primitive("Body", cube))
            .unwrap();
        let tool = MakePrimitiveFeature::cube("Hole", [2.0, 2.0, 2.0], 4.0);
        let tool_id = tree
            .register_feature(NativeFeature::primitive("Hole", tool))
            .unwrap();
        tree.register_feature(NativeFeature::boolean(
            "Pocket",
            BooleanOp::Subtraction,
            cube_id,
            tool_id,
        ))
        .unwrap();

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
        let cube = MakePrimitiveFeature::cube("Part", [1.0, 2.0, 3.0], 7.0);
        let cube_id = tree
            .register_feature(NativeFeature::primitive("Part", cube))
            .unwrap();
        let tool = MakePrimitiveFeature::cube("Drill", [2.0, 3.0, 4.0], 3.0);
        let tool_id = tree
            .register_feature(NativeFeature::primitive("Drill", tool))
            .unwrap();
        tree.register_feature(NativeFeature::boolean(
            "Op",
            BooleanOp::Intersection,
            cube_id,
            tool_id,
        ))
        .unwrap();

        let dir = tempdir().unwrap();
        let path_a = dir.path().join("diff_a.json");
        let path_b = dir.path().join("diff_b.json");

        save_model(&tree, &path_a).unwrap();
        save_model(&tree, &path_b).unwrap();

        let bytes_a = std::fs::read(&path_a).unwrap();
        let bytes_b = std::fs::read(&path_b).unwrap();
        assert_eq!(
            bytes_a, bytes_b,
            "Two serializations of the same model must be byte-identical"
        );
    }
}
