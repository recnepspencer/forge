use super::*;
use worth_foundational::facade::{
    AspectBinding, AspectContractRevision, AspectIdentity, AspectKey,
    AuthoritativeAspectChangeKind, FieldKey,
};

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

#[test]
fn declared_field_to_whole_widening_selects_sibling_roles() {
    let aspect = AspectKey::new("Portfolio.Facts").unwrap();
    let identity = AspectIdentity(17);
    let revision = AspectContractRevision(4);
    let contract_key = (aspect.clone(), identity, revision);
    let index = WorthQuerySemanticImpactIndex {
        native_contract: HashMap::from([(
            contract_key,
            vec![
                WorthQuerySemanticDependencyRole::ProjectedValue,
                WorthQuerySemanticDependencyRole::Ordering,
                WorthQuerySemanticDependencyRole::Grouping,
            ],
        )]),
        native_whole: HashMap::new(),
        native_field: HashMap::new(),
        collection_aspect: HashMap::new(),
        collection_field: HashMap::new(),
        conditional: HashMap::new(),
        structural_membership: false,
        window_on_ordering: true,
        workflow_effect_receipts: HashSet::new(),
        mask_propagation_edges: 0,
    };
    let change = BridgeSemanticAspectChange::from_declared_authoritative_widening(
        aspect,
        identity,
        revision,
        AspectBinding::EntityField {
            field: FieldKey::new("reported").unwrap(),
        },
        AuthoritativeAspectChangeKind::FieldSet,
        Some(CanonicalFieldPath::new([FieldKey::new("reported").unwrap()]).unwrap()),
        worth_runtime_bridge::facade::BridgeAspectChangeWideningCause::FieldToWholeAspect,
    );
    let mut roles = index.semantic_roles(&change).roles;
    roles.sort_unstable();
    roles.dedup();
    assert_eq!(
        roles,
        [
            WorthQuerySemanticDependencyRole::Ordering,
            WorthQuerySemanticDependencyRole::ProjectedValue,
            WorthQuerySemanticDependencyRole::Grouping,
            WorthQuerySemanticDependencyRole::WindowBoundary,
        ]
    );
}
