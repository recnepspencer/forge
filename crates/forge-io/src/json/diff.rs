//! Programmatic model diffing.
//!
//! Compares two serialized JSON model trees and produces a list
//! of structural changes (added/removed/modified features).
//!
//! DEPENDENCIES: serde_json (Value-level comparison)

use serde_json::Value;

/// A single change between two serialized models.
#[derive(Debug, Clone, PartialEq)]
pub enum ModelChange {
    /// A feature was added (present in `b` but not `a`).
    FeatureAdded {
        /// Name of the added feature.
        name: String,
    },
    /// A feature was removed (present in `a` but not `b`).
    FeatureRemoved {
        /// Name of the removed feature.
        name: String,
    },
    /// A feature parameter was modified.
    FeatureModified {
        /// Name of the modified feature.
        name: String,
        /// JSON path to the changed field (dot-separated).
        field: String,
        /// Old value.
        old: Value,
        /// New value.
        new: Value,
    },
}

/// Compare two serialized model JSON values and produce a change list.
///
/// Expects both values to have a `tree.features` map (the standard
/// serialization format from `save_model`). Returns an empty vec
/// if the models are identical.
pub fn diff_models(a: &Value, b: &Value) -> Vec<ModelChange> {
    let mut changes = Vec::new();

    let features_a = extract_features(a);
    let features_b = extract_features(b);

    let (Some(map_a), Some(map_b)) = (features_a, features_b) else {
        return changes;
    };

    for (name, val_a) in &map_a {
        match map_b.get(name.as_str()) {
            None => changes.push(ModelChange::FeatureRemoved { name: name.clone() }),
            Some(val_b) => {
                diff_feature_fields(name, val_a, val_b, &mut changes);
            }
        }
    }

    for name in map_b.keys() {
        if !map_a.contains_key(name) {
            changes.push(ModelChange::FeatureAdded { name: name.clone() });
        }
    }

    changes
}

/// Extract the features map keyed by name from a serialized model.
///
/// Navigates `tree.names` to get `name → node_id` and `tree.features`
/// to get `node_id → feature_data`, returning `name → feature_data`.
fn extract_features(root: &Value) -> Option<std::collections::HashMap<String, &Value>> {
    let tree = root.get("tree").unwrap_or(root);
    let names = tree.get("names")?.as_object()?;
    let features = tree.get("features")?.as_object()?;

    let mut result = std::collections::HashMap::new();
    for (name, node_id_val) in names {
        let node_id_str = node_id_val.as_str()?;
        if let Some(feature_val) = features.get(node_id_str) {
            result.insert(name.clone(), feature_val);
        }
    }
    Some(result)
}

/// Compare two feature values field-by-field and emit changes.
fn diff_feature_fields(name: &str, a: &Value, b: &Value, changes: &mut Vec<ModelChange>) {
    if a == b {
        return;
    }

    match (a.as_object(), b.as_object()) {
        (Some(obj_a), Some(obj_b)) => {
            let all_keys: std::collections::HashSet<&String> =
                obj_a.keys().chain(obj_b.keys()).collect();

            for key in all_keys {
                let val_a = obj_a.get(key);
                let val_b = obj_b.get(key);
                match (val_a, val_b) {
                    (Some(va), Some(vb)) if va != vb => {
                        changes.push(ModelChange::FeatureModified {
                            name: name.to_string(),
                            field: key.clone(),
                            old: va.clone(),
                            new: vb.clone(),
                        });
                    }
                    (Some(va), None) => {
                        changes.push(ModelChange::FeatureModified {
                            name: name.to_string(),
                            field: key.clone(),
                            old: va.clone(),
                            new: Value::Null,
                        });
                    }
                    (None, Some(vb)) => {
                        changes.push(ModelChange::FeatureModified {
                            name: name.to_string(),
                            field: key.clone(),
                            old: Value::Null,
                            new: vb.clone(),
                        });
                    }
                    _ => {}
                }
            }
        }
        _ => {
            changes.push(ModelChange::FeatureModified {
                name: name.to_string(),
                field: String::new(),
                old: a.clone(),
                new: b.clone(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn identical_models_produce_no_changes() {
        let model = json!({
            "tree": {
                "names": {"Cube": "n0"},
                "features": {"n0": {"type": "MakeCube", "size": 10.0}}
            }
        });
        let changes = diff_models(&model, &model);
        assert!(changes.is_empty());
    }

    #[test]
    fn added_feature_detected() {
        let a = json!({
            "tree": {
                "names": {"Cube": "n0"},
                "features": {"n0": {"type": "MakeCube"}}
            }
        });
        let b = json!({
            "tree": {
                "names": {"Cube": "n0", "Tool": "n1"},
                "features": {"n0": {"type": "MakeCube"}, "n1": {"type": "MakeCube"}}
            }
        });
        let changes = diff_models(&a, &b);
        assert_eq!(changes.len(), 1);
        assert!(matches!(&changes[0], ModelChange::FeatureAdded { name } if name == "Tool"));
    }

    #[test]
    fn removed_feature_detected() {
        let a = json!({
            "tree": {
                "names": {"Cube": "n0", "Tool": "n1"},
                "features": {"n0": {"type": "MakeCube"}, "n1": {"type": "MakeCube"}}
            }
        });
        let b = json!({
            "tree": {
                "names": {"Cube": "n0"},
                "features": {"n0": {"type": "MakeCube"}}
            }
        });
        let changes = diff_models(&a, &b);
        assert_eq!(changes.len(), 1);
        assert!(matches!(&changes[0], ModelChange::FeatureRemoved { name } if name == "Tool"));
    }

    #[test]
    fn modified_parameter_detected() {
        let a = json!({
            "tree": {
                "names": {"Cube": "n0"},
                "features": {"n0": {"type": "MakeCube", "size": 10.0}}
            }
        });
        let b = json!({
            "tree": {
                "names": {"Cube": "n0"},
                "features": {"n0": {"type": "MakeCube", "size": 20.0}}
            }
        });
        let changes = diff_models(&a, &b);
        assert_eq!(changes.len(), 1);
        assert!(
            matches!(&changes[0], ModelChange::FeatureModified { name, field, .. }
            if name == "Cube" && field == "size")
        );
    }
}
