use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DerivedInvalidationTarget {
    TopologyStructure,
    TopologyOwnership,
    TopologyBoundary,
    TopologyRadial,
    NamingPersistentName,
}

impl DerivedInvalidationTarget {
    pub fn bridge_scope(self) -> &'static str {
        match self {
            Self::TopologyStructure => ".derived.topology.structure",
            Self::TopologyOwnership => ".derived.topology.ownership",
            Self::TopologyBoundary => ".derived.topology.boundary",
            Self::TopologyRadial => ".derived.topology.radial",
            Self::NamingPersistentName => ".derived.naming.persistent_name",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DerivedTruthSurfaceKind {
    EntityField,
    EntityRelationEndpoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TruthToDerivedInvalidationDeclaration {
    pub declaration_id: &'static str,
    pub truth_patch_field: &'static str,
    pub truth_surface_kind: DerivedTruthSurfaceKind,
    pub target: DerivedInvalidationTarget,
}

pub fn milestone_two_invalidation_declarations() -> Vec<TruthToDerivedInvalidationDeclaration> {
    vec![
        TruthToDerivedInvalidationDeclaration {
            declaration_id: "relation-source-structural",
            truth_patch_field: "source",
            truth_surface_kind: DerivedTruthSurfaceKind::EntityRelationEndpoint,
            target: DerivedInvalidationTarget::TopologyStructure,
        },
        TruthToDerivedInvalidationDeclaration {
            declaration_id: "relation-target-structural",
            truth_patch_field: "target",
            truth_surface_kind: DerivedTruthSurfaceKind::EntityRelationEndpoint,
            target: DerivedInvalidationTarget::TopologyStructure,
        },
        TruthToDerivedInvalidationDeclaration {
            declaration_id: "topology-structural-lifecycle",
            truth_patch_field: "lifecycle",
            truth_surface_kind: DerivedTruthSurfaceKind::EntityField,
            target: DerivedInvalidationTarget::TopologyStructure,
        },
        TruthToDerivedInvalidationDeclaration {
            declaration_id: "topology-structure",
            truth_patch_field: "topology.structure",
            truth_surface_kind: DerivedTruthSurfaceKind::EntityField,
            target: DerivedInvalidationTarget::TopologyStructure,
        },
        TruthToDerivedInvalidationDeclaration {
            declaration_id: "topology-ownership",
            truth_patch_field: "topology.ownership",
            truth_surface_kind: DerivedTruthSurfaceKind::EntityRelationEndpoint,
            target: DerivedInvalidationTarget::TopologyOwnership,
        },
        TruthToDerivedInvalidationDeclaration {
            declaration_id: "topology-boundary",
            truth_patch_field: "topology.boundary",
            truth_surface_kind: DerivedTruthSurfaceKind::EntityRelationEndpoint,
            target: DerivedInvalidationTarget::TopologyBoundary,
        },
        TruthToDerivedInvalidationDeclaration {
            declaration_id: "topology-radial",
            truth_patch_field: "topology.radial",
            truth_surface_kind: DerivedTruthSurfaceKind::EntityRelationEndpoint,
            target: DerivedInvalidationTarget::TopologyRadial,
        },
        TruthToDerivedInvalidationDeclaration {
            declaration_id: "naming-persistent-name",
            truth_patch_field: "naming.persistent_name",
            truth_surface_kind: DerivedTruthSurfaceKind::EntityRelationEndpoint,
            target: DerivedInvalidationTarget::NamingPersistentName,
        },
    ]
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{milestone_two_invalidation_declarations, DerivedInvalidationTarget};

    #[test]
    fn milestone_two_invalidation_declarations_cover_unique_fields_and_targets() {
        let declarations = milestone_two_invalidation_declarations();

        assert_eq!(declarations.len(), 8);
        let unique_ids = declarations
            .iter()
            .map(|declaration| declaration.declaration_id)
            .collect::<BTreeSet<_>>();
        let unique_fields = declarations
            .iter()
            .map(|declaration| declaration.truth_patch_field)
            .collect::<BTreeSet<_>>();
        let targets = declarations
            .iter()
            .map(|declaration| declaration.target)
            .collect::<BTreeSet<_>>();

        assert_eq!(unique_ids.len(), declarations.len());
        assert_eq!(unique_fields.len(), declarations.len());
        assert!(targets.contains(&DerivedInvalidationTarget::TopologyStructure));
        assert!(targets.contains(&DerivedInvalidationTarget::TopologyOwnership));
        assert!(targets.contains(&DerivedInvalidationTarget::TopologyBoundary));
        assert!(targets.contains(&DerivedInvalidationTarget::TopologyRadial));
        assert!(targets.contains(&DerivedInvalidationTarget::NamingPersistentName));
    }
}
