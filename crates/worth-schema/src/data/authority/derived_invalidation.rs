use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthDerivedInvalidationTarget {
    TopologyStructure,
    TopologyOwnership,
    TopologyBoundary,
    TopologyRadial,
    NamingPersistentName,
}

impl WorthDerivedInvalidationTarget {
    pub fn bridge_scope(self) -> &'static str {
        match self {
            Self::TopologyStructure => "worth.derived.topology.structure",
            Self::TopologyOwnership => "worth.derived.topology.ownership",
            Self::TopologyBoundary => "worth.derived.topology.boundary",
            Self::TopologyRadial => "worth.derived.topology.radial",
            Self::NamingPersistentName => "worth.derived.naming.persistent_name",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthDerivedTruthSurfaceKind {
    EntityField,
    EntityRelationEndpoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTruthToDerivedInvalidationDeclaration {
    pub declaration_id: &'static str,
    pub truth_patch_field: &'static str,
    pub truth_surface_kind: WorthDerivedTruthSurfaceKind,
    pub target: WorthDerivedInvalidationTarget,
}

pub fn worth_milestone_two_invalidation_declarations(
) -> Vec<WorthTruthToDerivedInvalidationDeclaration> {
    vec![
        WorthTruthToDerivedInvalidationDeclaration {
            declaration_id: "relation-source-structural",
            truth_patch_field: "source",
            truth_surface_kind: WorthDerivedTruthSurfaceKind::EntityRelationEndpoint,
            target: WorthDerivedInvalidationTarget::TopologyStructure,
        },
        WorthTruthToDerivedInvalidationDeclaration {
            declaration_id: "relation-target-structural",
            truth_patch_field: "target",
            truth_surface_kind: WorthDerivedTruthSurfaceKind::EntityRelationEndpoint,
            target: WorthDerivedInvalidationTarget::TopologyStructure,
        },
        WorthTruthToDerivedInvalidationDeclaration {
            declaration_id: "topology-structural-lifecycle",
            truth_patch_field: "lifecycle",
            truth_surface_kind: WorthDerivedTruthSurfaceKind::EntityField,
            target: WorthDerivedInvalidationTarget::TopologyStructure,
        },
        WorthTruthToDerivedInvalidationDeclaration {
            declaration_id: "topology-structure",
            truth_patch_field: "topology.structure",
            truth_surface_kind: WorthDerivedTruthSurfaceKind::EntityField,
            target: WorthDerivedInvalidationTarget::TopologyStructure,
        },
        WorthTruthToDerivedInvalidationDeclaration {
            declaration_id: "topology-ownership",
            truth_patch_field: "topology.ownership",
            truth_surface_kind: WorthDerivedTruthSurfaceKind::EntityRelationEndpoint,
            target: WorthDerivedInvalidationTarget::TopologyOwnership,
        },
        WorthTruthToDerivedInvalidationDeclaration {
            declaration_id: "topology-boundary",
            truth_patch_field: "topology.boundary",
            truth_surface_kind: WorthDerivedTruthSurfaceKind::EntityRelationEndpoint,
            target: WorthDerivedInvalidationTarget::TopologyBoundary,
        },
        WorthTruthToDerivedInvalidationDeclaration {
            declaration_id: "topology-radial",
            truth_patch_field: "topology.radial",
            truth_surface_kind: WorthDerivedTruthSurfaceKind::EntityRelationEndpoint,
            target: WorthDerivedInvalidationTarget::TopologyRadial,
        },
        WorthTruthToDerivedInvalidationDeclaration {
            declaration_id: "naming-persistent-name",
            truth_patch_field: "naming.persistent_name",
            truth_surface_kind: WorthDerivedTruthSurfaceKind::EntityRelationEndpoint,
            target: WorthDerivedInvalidationTarget::NamingPersistentName,
        },
    ]
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{
        worth_milestone_two_invalidation_declarations, WorthDerivedInvalidationTarget,
    };

    #[test]
    fn milestone_two_invalidation_declarations_cover_unique_fields_and_targets() {
        let declarations = worth_milestone_two_invalidation_declarations();

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
        assert!(targets.contains(&WorthDerivedInvalidationTarget::TopologyStructure));
        assert!(targets.contains(&WorthDerivedInvalidationTarget::TopologyOwnership));
        assert!(targets.contains(&WorthDerivedInvalidationTarget::TopologyBoundary));
        assert!(targets.contains(&WorthDerivedInvalidationTarget::TopologyRadial));
        assert!(targets.contains(&WorthDerivedInvalidationTarget::NamingPersistentName));
    }
}
