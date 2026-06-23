use forge_runtime_bridge::facade::{
    BridgeIdentityEvidence, RelationalBridgeRecordIdentityKind,
    RelationalBridgeRecordIdentityParts, RelationalBridgeSnapshotIdentityParts,
    TruthCommitIdentity, TruthSnapshotIdentity,
};

use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForgeQueryCommitIdentity {
    Absent,
    RelationalBridge {
        bridge_identity: TruthCommitIdentity,
    },
    Preview {
        evidence_identity: ForgeQueryEvidenceIdentity,
    },
}

impl ForgeQueryCommitIdentity {
    pub fn from_relational_commit_id(commit_id: u64) -> Self {
        Self::RelationalBridge {
            bridge_identity: TruthCommitIdentity::from_relational_commit_id(commit_id),
        }
    }

    pub fn preview(evidence_identity: ForgeQueryEvidenceIdentity) -> Self {
        Self::Preview { evidence_identity }
    }

    pub fn admit_external_token(
        token: crate::identity_authority::QueryExternalIdentityToken<
            std::sync::Arc<str>,
            crate::identity_authority::QueryCommitIdentityKind,
        >,
    ) -> Self {
        super::truth_identity_admission::admit_external_commit_token(token)
    }

    pub fn absent() -> Self {
        Self::Absent
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Absent)
    }

    pub fn bridge_identity(&self) -> Option<&TruthCommitIdentity> {
        match self {
            Self::Absent => None,
            Self::RelationalBridge { bridge_identity } => Some(bridge_identity),
            Self::Preview { .. } => None,
        }
    }

    pub fn evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        match self {
            Self::Absent => ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::WriteReceiptCommitIdentity,
            )
            .field_shape(ForgeQueryEvidenceTag::new("commit_state"), "absent")
            .seal(),
            Self::RelationalBridge { bridge_identity } => {
                let Some(commit_id) = bridge_identity.relational_commit_id() else {
                    panic!("forge-query commit identity must retain relational commit payload");
                };
                ForgeQueryEvidenceIdentity::compose(
                    ForgeQueryEvidenceScope::WriteReceiptCommitIdentity,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("relational_commit_id"),
                    commit_id as usize,
                )
                .seal()
            }
            Self::Preview { evidence_identity } => evidence_identity.clone(),
        }
    }

    pub fn bridge_admission_evidence(&self) -> BridgeIdentityEvidence {
        match self {
            Self::RelationalBridge { bridge_identity } => {
                bridge_identity.bridge_admission_evidence()
            }
            Self::Absent | Self::Preview { .. } => {
                self.evidence_identity().bridge_evidence_identity()
            }
        }
    }

    pub fn terminal_projection_for_reporting(&self) -> String {
        self.evidence_identity()
            .terminal_projection_for_reporting()
            .to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForgeQuerySnapshotIdentity {
    EmptyRelationalState,
    RelationalBridge {
        bridge_identity: TruthSnapshotIdentity,
        parts: RelationalBridgeSnapshotIdentityParts,
    },
    Preview {
        evidence_identity: ForgeQueryEvidenceIdentity,
    },
}

impl ForgeQuerySnapshotIdentity {
    pub fn empty_relational_state() -> Self {
        Self::EmptyRelationalState
    }

    pub fn from_relational_snapshot(parts: RelationalBridgeSnapshotIdentityParts) -> Self {
        Self::RelationalBridge {
            bridge_identity: TruthSnapshotIdentity::from_relational_snapshot(parts),
            parts,
        }
    }

    pub fn preview(evidence_identity: ForgeQueryEvidenceIdentity) -> Self {
        Self::Preview { evidence_identity }
    }

    pub fn admit_external_token(
        token: crate::identity_authority::QueryExternalIdentityToken<
            std::sync::Arc<str>,
            crate::identity_authority::QuerySnapshotIdentityKind,
        >,
    ) -> Self {
        super::truth_identity_admission::admit_external_snapshot_token(token)
    }

    pub fn bridge_identity(&self) -> Option<&TruthSnapshotIdentity> {
        match self {
            Self::EmptyRelationalState => None,
            Self::RelationalBridge {
                bridge_identity, ..
            } => Some(bridge_identity),
            Self::Preview { .. } => None,
        }
    }

    pub fn relational_parts(&self) -> Option<RelationalBridgeSnapshotIdentityParts> {
        match self {
            Self::EmptyRelationalState => None,
            Self::RelationalBridge { parts, .. } => Some(*parts),
            Self::Preview { .. } => None,
        }
    }

    pub fn evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        match self {
            Self::EmptyRelationalState => ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::WriteReceiptSnapshotIdentity,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("snapshot_state"),
                "empty-relational",
            )
            .seal(),
            Self::RelationalBridge { parts, .. } => ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::WriteReceiptSnapshotIdentity,
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("relational_snapshot_id"),
                parts.snapshot_id() as usize,
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("relational_version_id"),
                parts.version_id() as usize,
            )
            .seal(),
            Self::Preview { evidence_identity } => evidence_identity.clone(),
        }
    }

    pub fn bridge_admission_evidence(&self) -> BridgeIdentityEvidence {
        match self {
            Self::RelationalBridge {
                bridge_identity, ..
            } => bridge_identity.bridge_admission_evidence(),
            Self::EmptyRelationalState | Self::Preview { .. } => {
                self.evidence_identity().bridge_evidence_identity()
            }
        }
    }

    pub fn terminal_projection_for_reporting(&self) -> String {
        self.evidence_identity()
            .terminal_projection_for_reporting()
            .to_string()
    }

    pub(crate) fn matches_declared_historical_basis_label(
        &self,
        declared_basis_label: &str,
    ) -> bool {
        if declared_basis_label == self.evidence_identity().as_str() {
            return true;
        }
        declared_basis_label
            == self
                .bridge_admission_evidence()
                .terminal_projection_for_reporting()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForgeQueryEntityIdentity {
    RelationalRecord {
        relational_record: RelationalBridgeRecordIdentityParts,
    },
    Preview {
        evidence_identity: ForgeQueryEvidenceIdentity,
    },
}

impl ForgeQueryEntityIdentity {
    pub fn from_relational_record(relational_record: RelationalBridgeRecordIdentityParts) -> Self {
        Self::RelationalRecord { relational_record }
    }

    pub fn preview(evidence_identity: ForgeQueryEvidenceIdentity) -> Self {
        Self::Preview { evidence_identity }
    }

    pub fn admit_authored_entity_token(
        token: crate::identity_authority::QueryExternalIdentityToken<
            std::sync::Arc<str>,
            crate::identity_authority::QueryEntityIdentityKind,
        >,
    ) -> Self {
        super::truth_identity_admission::admit_authored_entity_token(token)
    }

    pub fn relational_record_parts(&self) -> Option<RelationalBridgeRecordIdentityParts> {
        match self {
            Self::RelationalRecord { relational_record } => Some(*relational_record),
            Self::Preview { .. } => None,
        }
    }

    pub fn relational_entity_record_parts(&self) -> Option<RelationalBridgeRecordIdentityParts> {
        self.relational_record_parts()
            .filter(|parts| parts.kind() == RelationalBridgeRecordIdentityKind::Entity)
    }

    pub fn evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        match self {
            Self::RelationalRecord { relational_record } => {
                let kind = match relational_record.kind() {
                    RelationalBridgeRecordIdentityKind::Entity => "entity",
                    RelationalBridgeRecordIdentityKind::Relation => "relation",
                };
                ForgeQueryEvidenceIdentity::compose(
                    ForgeQueryEvidenceScope::WriteReceiptEntityIdentity,
                )
                .field_shape(ForgeQueryEvidenceTag::new("record_kind"), kind)
                .field_usize(
                    ForgeQueryEvidenceTag::new("partition_id"),
                    relational_record.partition_id() as usize,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("local_slot"),
                    relational_record.local_slot() as usize,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("generation"),
                    relational_record.generation() as usize,
                )
                .seal()
            }
            Self::Preview { evidence_identity } => evidence_identity.clone(),
        }
    }

    pub(crate) fn terminal_projection_for_reporting(&self) -> String {
        self.evidence_identity()
            .terminal_projection_for_reporting()
            .to_string()
    }
}

impl PartialOrd for ForgeQueryEntityIdentity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ForgeQueryEntityIdentity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.terminal_projection_for_reporting()
            .cmp(&other.terminal_projection_for_reporting())
    }
}

impl Hash for ForgeQueryEntityIdentity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.terminal_projection_for_reporting().hash(state);
    }
}
