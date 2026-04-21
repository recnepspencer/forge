use super::manifests::{
    ArtifactCompatibilityWindow, ArtifactFamilyId, AuthoritativeCompatibilityManifest,
    DerivedCompatibilityManifest,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CompatibilityAuthorityClassification {
    Authoritative,
    Derived,
}

impl CompatibilityAuthorityClassification {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::Derived => "derived",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CompatibilityFamilyKind {
    CommitEnvelope,
    BranchVersionDagRecord,
    WalRestartRecord,
    SchemaLineageCursorCheckpointSupport,
    EmbeddedCheckpointAuthority,
    SnapshotRecord,
    DeltaRecord,
    Milestone6LayoutBlockChunkRecord,
    Milestone8BasisContinuationDescriptor,
    Milestone9BulkRecord,
    Milestone10RetentionRebuildRecord,
    Milestone11MaintenanceRecord,
    Milestone13TieringRecord,
}

impl CompatibilityFamilyKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::CommitEnvelope => "commit_envelope",
            Self::BranchVersionDagRecord => "branch_version_dag_record",
            Self::WalRestartRecord => "wal_restart_record",
            Self::SchemaLineageCursorCheckpointSupport => {
                "schema_lineage_cursor_checkpoint_support"
            }
            Self::EmbeddedCheckpointAuthority => "embedded_checkpoint_authority",
            Self::SnapshotRecord => "snapshot_record",
            Self::DeltaRecord => "delta_record",
            Self::Milestone6LayoutBlockChunkRecord => "milestone_6_layout_block_chunk_record",
            Self::Milestone8BasisContinuationDescriptor => {
                "milestone_8_basis_continuation_descriptor"
            }
            Self::Milestone9BulkRecord => "milestone_9_bulk_record",
            Self::Milestone10RetentionRebuildRecord => "milestone_10_retention_rebuild_record",
            Self::Milestone11MaintenanceRecord => "milestone_11_maintenance_record",
            Self::Milestone13TieringRecord => "milestone_13_tiering_record",
        }
    }

    pub fn family_id(self) -> ArtifactFamilyId {
        ArtifactFamilyId::new(self.label())
    }

    pub const fn authority_classification(self) -> CompatibilityAuthorityClassification {
        self.posture().authority_classification
    }

    const fn posture(self) -> FamilyPosture {
        match self {
            Self::CommitEnvelope
            | Self::BranchVersionDagRecord
            | Self::WalRestartRecord
            | Self::SchemaLineageCursorCheckpointSupport
            | Self::EmbeddedCheckpointAuthority => FamilyPosture::authoritative(self.label()),
            Self::SnapshotRecord
            | Self::DeltaRecord
            | Self::Milestone6LayoutBlockChunkRecord
            | Self::Milestone8BasisContinuationDescriptor
            | Self::Milestone9BulkRecord
            | Self::Milestone10RetentionRebuildRecord
            | Self::Milestone11MaintenanceRecord
            | Self::Milestone13TieringRecord => FamilyPosture::derived(self.label()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FamilyPosture {
    authority_classification: CompatibilityAuthorityClassification,
    restore_posture: &'static str,
    rolling_posture: &'static str,
    counter_family_id: &'static str,
    certification_lane_id: &'static str,
}

impl FamilyPosture {
    const fn authoritative(label: &'static str) -> Self {
        Self {
            authority_classification: CompatibilityAuthorityClassification::Authoritative,
            restore_posture: label,
            rolling_posture: label,
            counter_family_id: label,
            certification_lane_id: label,
        }
    }

    const fn derived(label: &'static str) -> Self {
        Self {
            authority_classification: CompatibilityAuthorityClassification::Derived,
            restore_posture: label,
            rolling_posture: label,
            counter_family_id: label,
            certification_lane_id: label,
        }
    }
}

pub const FIRST_SHIP_COMPATIBILITY_FAMILIES: [CompatibilityFamilyKind; 13] = [
    CompatibilityFamilyKind::CommitEnvelope,
    CompatibilityFamilyKind::BranchVersionDagRecord,
    CompatibilityFamilyKind::WalRestartRecord,
    CompatibilityFamilyKind::SchemaLineageCursorCheckpointSupport,
    CompatibilityFamilyKind::EmbeddedCheckpointAuthority,
    CompatibilityFamilyKind::SnapshotRecord,
    CompatibilityFamilyKind::DeltaRecord,
    CompatibilityFamilyKind::Milestone6LayoutBlockChunkRecord,
    CompatibilityFamilyKind::Milestone8BasisContinuationDescriptor,
    CompatibilityFamilyKind::Milestone9BulkRecord,
    CompatibilityFamilyKind::Milestone10RetentionRebuildRecord,
    CompatibilityFamilyKind::Milestone11MaintenanceRecord,
    CompatibilityFamilyKind::Milestone13TieringRecord,
];

pub const FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT: usize = FIRST_SHIP_COMPATIBILITY_FAMILIES.len();

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompatibilityManifestDeclaration {
    Authoritative(AuthoritativeCompatibilityManifest),
    Derived(DerivedCompatibilityManifest),
}

impl CompatibilityManifestDeclaration {
    pub fn family_id(&self) -> &ArtifactFamilyId {
        match self {
            Self::Authoritative(manifest) => manifest.family_id(),
            Self::Derived(manifest) => manifest.family_id(),
        }
    }

    pub fn window(&self) -> &ArtifactCompatibilityWindow {
        match self {
            Self::Authoritative(manifest) => manifest.window(),
            Self::Derived(manifest) => manifest.window(),
        }
    }

    pub fn digest(&self) -> &super::manifests::CompatibilityManifestDigest {
        match self {
            Self::Authoritative(manifest) => manifest.digest(),
            Self::Derived(manifest) => manifest.digest(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityFamilyDeclaration {
    kind: CompatibilityFamilyKind,
    family_id: ArtifactFamilyId,
    authority_classification: CompatibilityAuthorityClassification,
    manifest: CompatibilityManifestDeclaration,
    restore_posture: String,
    rolling_posture: String,
    counter_family_id: String,
    certification_lane_id: String,
}

impl CompatibilityFamilyDeclaration {
    pub(crate) fn new(
        kind: CompatibilityFamilyKind,
        manifest: CompatibilityManifestDeclaration,
        restore_posture: impl Into<String>,
        rolling_posture: impl Into<String>,
        counter_family_id: impl Into<String>,
        certification_lane_id: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            family_id: kind.family_id(),
            authority_classification: kind.authority_classification(),
            manifest,
            restore_posture: restore_posture.into(),
            rolling_posture: rolling_posture.into(),
            counter_family_id: counter_family_id.into(),
            certification_lane_id: certification_lane_id.into(),
        }
    }

    pub fn kind(&self) -> CompatibilityFamilyKind {
        self.kind
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn authority_classification(&self) -> CompatibilityAuthorityClassification {
        self.authority_classification
    }

    pub fn manifest(&self) -> &CompatibilityManifestDeclaration {
        &self.manifest
    }

    pub fn restore_posture(&self) -> &str {
        &self.restore_posture
    }

    pub fn rolling_posture(&self) -> &str {
        &self.rolling_posture
    }

    pub fn counter_family_id(&self) -> &str {
        &self.counter_family_id
    }

    pub fn certification_lane_id(&self) -> &str {
        &self.certification_lane_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoritativeFamilyDeclaration {
    declaration: CompatibilityFamilyDeclaration,
}

impl AuthoritativeFamilyDeclaration {
    pub(crate) fn new(declaration: CompatibilityFamilyDeclaration) -> Self {
        debug_assert_eq!(
            declaration.authority_classification(),
            CompatibilityAuthorityClassification::Authoritative
        );
        Self { declaration }
    }

    pub fn declaration(&self) -> &CompatibilityFamilyDeclaration {
        &self.declaration
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedFamilyDeclaration {
    declaration: CompatibilityFamilyDeclaration,
}

impl DerivedFamilyDeclaration {
    pub(crate) fn new(declaration: CompatibilityFamilyDeclaration) -> Self {
        debug_assert_eq!(
            declaration.authority_classification(),
            CompatibilityAuthorityClassification::Derived
        );
        Self { declaration }
    }

    pub fn declaration(&self) -> &CompatibilityFamilyDeclaration {
        &self.declaration
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityRegistrySnapshot {
    declarations: Vec<CompatibilityFamilyDeclaration>,
}

impl CompatibilityRegistrySnapshot {
    pub(crate) fn new(mut declarations: Vec<CompatibilityFamilyDeclaration>) -> Self {
        declarations.sort_by_key(|declaration| declaration.kind().label());
        Self { declarations }
    }

    pub fn declarations(&self) -> &[CompatibilityFamilyDeclaration] {
        &self.declarations
    }

    pub fn get(&self, kind: CompatibilityFamilyKind) -> Option<&CompatibilityFamilyDeclaration> {
        self.declarations
            .iter()
            .find(|declaration| declaration.kind() == kind)
    }
}

#[derive(Debug, Default)]
pub struct CompatibilityRegistry {
    declarations: Vec<CompatibilityFamilyDeclaration>,
}

impl CompatibilityRegistry {
    pub fn first_ship() -> CompatibilityRegistrySnapshot {
        let mut registry = Self::default();
        for kind in FIRST_SHIP_COMPATIBILITY_FAMILIES {
            registry.declare_first_ship(kind);
        }
        registry.snapshot()
    }

    fn declare_first_ship(&mut self, kind: CompatibilityFamilyKind) {
        let family_id = kind.family_id();
        let window = ArtifactCompatibilityWindow::native(1);
        let posture = kind.posture();
        let manifest = match kind.authority_classification() {
            CompatibilityAuthorityClassification::Authoritative => {
                CompatibilityManifestDeclaration::Authoritative(
                    AuthoritativeCompatibilityManifest::new(family_id, window),
                )
            }
            CompatibilityAuthorityClassification::Derived => {
                CompatibilityManifestDeclaration::Derived(DerivedCompatibilityManifest::new(
                    family_id, window,
                ))
            }
        };
        self.declarations.push(CompatibilityFamilyDeclaration::new(
            kind,
            manifest,
            format!("restore.posture.{}", posture.restore_posture),
            format!("rolling.posture.{}", posture.rolling_posture),
            format!("counter.family.{}", posture.counter_family_id),
            format!("certification.lane.{}", posture.certification_lane_id),
        ));
    }

    pub fn snapshot(self) -> CompatibilityRegistrySnapshot {
        CompatibilityRegistrySnapshot::new(self.declarations)
    }
}
