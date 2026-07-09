use worth_store_contracts::StableDigest;
use worth_store_physical_format::{PhysicalGenerationOwner, RootManifestIntegrityPosture};
use worth_store_physical_integrity::{
    DamageClassification, ManifestIntegrityDenial, ManifestIntegrityDenialKind, QuarantineRecord,
    WalFrameDamageDenial, WalFrameDamageDenialKind, WalTailIntegrityPosture,
};

use crate::{S4IntegrityHandoffDenial, S4IntegrityHandoffDenialKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryBlockingIntegritySource {
    WalFrame,
    CheckpointAdjacentRecord,
    ManifestRoot,
    UnresolvedAuthorityDamage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryBlockedByIntegrityDamage {
    source: RecoveryBlockingIntegritySource,
    basis: StableDigest,
    locality: Option<PhysicalGenerationOwner>,
    wal_kind: Option<WalFrameDamageDenialKind>,
    tail_posture: Option<WalTailIntegrityPosture>,
    manifest_kind: Option<ManifestIntegrityDenialKind>,
    root_posture: Option<RootManifestIntegrityPosture>,
}

impl RecoveryBlockedByIntegrityDamage {
    pub fn damaged_wal_frame(denial: &WalFrameDamageDenial) -> Self {
        Self {
            source: RecoveryBlockingIntegritySource::WalFrame,
            basis: digest(format!(
                "s4-blocked-wal:{:?}:{:?}:{:?}",
                denial.kind(),
                denial.basis(),
                denial.counters()
            )),
            locality: denial.locality(),
            wal_kind: Some(denial.kind()),
            tail_posture: Some(denial.tail_posture()),
            manifest_kind: None,
            root_posture: None,
        }
    }

    pub fn checkpoint_adjacent_damage(denial: &WalFrameDamageDenial) -> Self {
        Self {
            source: RecoveryBlockingIntegritySource::CheckpointAdjacentRecord,
            basis: digest(format!(
                "s4-blocked-checkpoint:{:?}:{:?}:{:?}",
                denial.kind(),
                denial.checkpoint_adjacent_damage(),
                denial.counters()
            )),
            locality: denial.locality(),
            wal_kind: Some(denial.kind()),
            tail_posture: Some(denial.tail_posture()),
            manifest_kind: None,
            root_posture: None,
        }
    }

    pub fn damaged_manifest_root(denial: &ManifestIntegrityDenial) -> Self {
        Self {
            source: RecoveryBlockingIntegritySource::ManifestRoot,
            basis: digest(format!(
                "s4-blocked-manifest:{:?}:{:?}:{:?}",
                denial.kind(),
                denial.posture(),
                denial.counters()
            )),
            locality: denial.locality(),
            wal_kind: None,
            tail_posture: None,
            manifest_kind: Some(denial.kind()),
            root_posture: Some(denial.posture()),
        }
    }

    pub fn unresolved_authority_damage(
        record: &QuarantineRecord,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        match record.damage_classification() {
            DamageClassification::UnrecoverableAuthorityDamage(damage) => Ok(Self {
                source: RecoveryBlockingIntegritySource::UnresolvedAuthorityDamage,
                basis: digest(format!(
                    "s4-blocked-unresolved-authority:{:?}:{:?}:{:?}",
                    damage.boundary(),
                    record.locality(),
                    record.receipt().foundational_basis().digest()
                )),
                locality: damage.locality(),
                wal_kind: None,
                tail_posture: None,
                manifest_kind: None,
                root_posture: None,
            }),
            _ => Err(S4IntegrityHandoffDenial::new(
                S4IntegrityHandoffDenialKind::UnresolvedAuthorityDamageRequiresAuthorityClassification,
            )),
        }
    }

    pub const fn source(&self) -> RecoveryBlockingIntegritySource {
        self.source
    }

    pub fn basis(&self) -> &StableDigest {
        &self.basis
    }

    pub const fn locality(&self) -> Option<PhysicalGenerationOwner> {
        self.locality
    }

    pub const fn wal_kind(&self) -> Option<WalFrameDamageDenialKind> {
        self.wal_kind
    }

    pub const fn tail_posture(&self) -> Option<WalTailIntegrityPosture> {
        self.tail_posture
    }

    pub const fn manifest_kind(&self) -> Option<ManifestIntegrityDenialKind> {
        self.manifest_kind
    }

    pub const fn root_posture(&self) -> Option<RootManifestIntegrityPosture> {
        self.root_posture
    }
}

fn digest(value: impl Into<String>) -> StableDigest {
    StableDigest::new(value).expect("S.4 recovery-blocking digest basis is non-empty")
}
