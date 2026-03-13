use serde::{Deserialize, Serialize};

/// Semantic boundaries that must remain equivalent for artifact reuse to be legal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ArtifactSemanticBoundary {
    TopologyRegime,
    ToleranceRegime,
    SemanticRegionIdentity,
    AuthorityLane,
    SnapshotLineage,
}

/// Declarative equivalence boundaries for artifact reuse legality.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactEquivalenceContract {
    #[serde(default = "ArtifactEquivalenceContract::default_boundaries")]
    pub required_boundaries: Vec<ArtifactSemanticBoundary>,
    #[serde(default)]
    pub allows_snapshot_restore_reuse: bool,
    #[serde(default)]
    pub allows_authority_reconciliation_reuse: bool,
}

impl ArtifactEquivalenceContract {
    pub fn strict() -> Self {
        Self {
            required_boundaries: Self::default_boundaries(),
            allows_snapshot_restore_reuse: false,
            allows_authority_reconciliation_reuse: false,
        }
    }

    fn default_boundaries() -> Vec<ArtifactSemanticBoundary> {
        vec![
            ArtifactSemanticBoundary::TopologyRegime,
            ArtifactSemanticBoundary::ToleranceRegime,
            ArtifactSemanticBoundary::SemanticRegionIdentity,
        ]
    }
}

impl Default for ArtifactEquivalenceContract {
    fn default() -> Self {
        Self::strict()
    }
}

/// Node-level declarative posture for artifact reuse and certification retention.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeReuseContract {
    #[serde(default)]
    pub equivalence: ArtifactEquivalenceContract,
    #[serde(default)]
    pub retain_certification: bool,
}

impl NodeReuseContract {
    pub fn strict() -> Self {
        Self {
            equivalence: ArtifactEquivalenceContract::strict(),
            retain_certification: true,
        }
    }
}

impl Default for NodeReuseContract {
    fn default() -> Self {
        Self::strict()
    }
}
