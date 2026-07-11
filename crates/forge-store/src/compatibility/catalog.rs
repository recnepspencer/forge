use super::manifests::{
    ArtifactCompatibilityWindow, ArtifactFamilyId, AuthoritativeCompatibilityManifest,
    DerivedCompatibilityManifest,
};
use forge_store_contracts::{
    CompatibilityAuthorityClassification, CompatibilityFamilyKind,
    FIRST_SHIP_COMPATIBILITY_FAMILIES, FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT,
};
use serde::{Deserialize, Serialize};

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

const fn family_posture(kind: CompatibilityFamilyKind) -> FamilyPosture {
    match kind {
        CompatibilityFamilyKind::CommitEnvelope
        | CompatibilityFamilyKind::BranchVersionDagRecord
        | CompatibilityFamilyKind::WalRestartRecord
        | CompatibilityFamilyKind::SchemaLineageCursorCheckpointSupport
        | CompatibilityFamilyKind::EmbeddedCheckpointAuthority => {
            FamilyPosture::authoritative(kind.label())
        }
        CompatibilityFamilyKind::SnapshotRecord
        | CompatibilityFamilyKind::DeltaRecord
        | CompatibilityFamilyKind::Milestone6LayoutBlockChunkRecord
        | CompatibilityFamilyKind::Milestone8BasisContinuationDescriptor
        | CompatibilityFamilyKind::Milestone9BulkRecord
        | CompatibilityFamilyKind::Milestone10RetentionRebuildRecord
        | CompatibilityFamilyKind::Milestone11MaintenanceRecord
        | CompatibilityFamilyKind::Milestone13TieringRecord => FamilyPosture::derived(kind.label()),
    }
}

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
        let posture = family_posture(kind);
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
