use super::*;
use worth_foundational::facade::FieldKey;

#[test]
fn indexed_dependency_edges_use_canonical_path_overlap_without_sibling_leakage() {
    let path = |fields: &[&str]| {
        CanonicalFieldPath::new(fields.iter().map(|field| FieldKey::new(*field).unwrap())).unwrap()
    };
    let mut index =
        crate::canonical_field_path_overlap_index::WorthQueryCanonicalPathOverlapIndex::default();
    index.insert(
        &path(&["address"]),
        IndexedRoleEdge {
            dependency_ordinal: 1,
            role: WorthQuerySemanticDependencyRole::ProjectedValue,
        },
    );
    index.insert(
        &path(&["address", "city"]),
        IndexedRoleEdge {
            dependency_ordinal: 2,
            role: WorthQuerySemanticDependencyRole::Ordering,
        },
    );
    index.insert(
        &path(&["address", "postal"]),
        IndexedRoleEdge {
            dependency_ordinal: 3,
            role: WorthQuerySemanticDependencyRole::Grouping,
        },
    );

    let mut roles = Vec::new();
    let mut lookups = 0;
    extend_overlapping(
        &mut roles,
        Some(&index),
        &path(&["address", "city", "name"]),
        &mut lookups,
    );
    assert_eq!(
        roles,
        [
            WorthQuerySemanticDependencyRole::ProjectedValue,
            WorthQuerySemanticDependencyRole::Ordering,
        ]
    );
    assert_eq!(lookups, 4, "one map lookup plus three path-node probes");
}
