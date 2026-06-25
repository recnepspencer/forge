use crate::runtime_support::milestone_one_invariant_registrations;
use crate::validator_invariant_catalog::WorthTopologyLegalityCatalogError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTopologyRelationalInvariantOldPackResidueStatus {
    CertificationOnlySourceIntake,
    CappedCompatibilityResidue,
    DeletedOrdinaryPath,
}

impl WorthTopologyRelationalInvariantOldPackResidueStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertificationOnlySourceIntake => "certification-only-source-intake",
            Self::CappedCompatibilityResidue => "capped-compatibility-residue",
            Self::DeletedOrdinaryPath => "deleted-ordinary-path",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyRelationalInvariantOldPackResidueRow {
    source_path: String,
    status: WorthTopologyRelationalInvariantOldPackResidueStatus,
    owner: String,
    blocker: String,
    removal_trigger: String,
    ordinary_path_count: usize,
    registration_count: usize,
    row_digest: String,
}

impl WorthTopologyRelationalInvariantOldPackResidueRow {
    fn new(
        source_path: impl Into<String>,
        status: WorthTopologyRelationalInvariantOldPackResidueStatus,
        owner: impl Into<String>,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
        ordinary_path_count: usize,
        registration_count: usize,
    ) -> Self {
        let source_path = source_path.into();
        let owner = owner.into();
        let blocker = blocker.into();
        let removal_trigger = removal_trigger.into();
        let row_digest = [
            "worth-topo-relational-invariant-old-pack-residue-row-v1",
            source_path.as_str(),
            status.as_str(),
            owner.as_str(),
            blocker.as_str(),
            removal_trigger.as_str(),
            &ordinary_path_count.to_string(),
            &registration_count.to_string(),
        ]
        .join("|");
        Self {
            source_path,
            status,
            owner,
            blocker,
            removal_trigger,
            ordinary_path_count,
            registration_count,
            row_digest,
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub const fn status(&self) -> WorthTopologyRelationalInvariantOldPackResidueStatus {
        self.status
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub const fn ordinary_path_count(&self) -> usize {
        self.ordinary_path_count
    }

    pub const fn registration_count(&self) -> usize {
        self.registration_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyRelationalInvariantOldPackResidueReport {
    rows: Vec<WorthTopologyRelationalInvariantOldPackResidueRow>,
    source_pack_registration_count: usize,
    ordinary_path_count: usize,
    report_digest: String,
}

impl WorthTopologyRelationalInvariantOldPackResidueReport {
    pub(in crate::validator_invariant_catalog) fn from_current_sources(
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let source_pack_registration_count = milestone_one_invariant_registrations()
            .map_err(|error| {
                WorthTopologyLegalityCatalogError::InvariantRegistration(format!("{error:?}"))
            })?
            .len();
        let rows = vec![
            WorthTopologyRelationalInvariantOldPackResidueRow::new(
                "runtime_support::milestone_one_invariant_registrations",
                WorthTopologyRelationalInvariantOldPackResidueStatus::CertificationOnlySourceIntake,
                "worth-topo.validator-invariant-catalog",
                "source truth for current custom invariant family parity until Phase 8 hard deletion",
                "Phase 8 public closeout proves graph-scoped custom invariant registration and removes public ordinary pack exposure",
                0,
                source_pack_registration_count,
            ),
            WorthTopologyRelationalInvariantOldPackResidueRow::new(
                "validation::reference_integrity::milestone_one_runtime_builder",
                WorthTopologyRelationalInvariantOldPackResidueStatus::CappedCompatibilityResidue,
                "worth-topo.runtime-support",
                "legacy certification/runtime comparison path remains below ordinary Query registration authority",
                "Phase 8 deletes or test-only gates legacy runtime builder helpers after Phase 6 execution receipts exist",
                0,
                0,
            ),
        ];
        let ordinary_path_count = rows.iter().map(|row| row.ordinary_path_count()).sum();
        let report_digest = old_pack_residue_report_digest(
            source_pack_registration_count,
            ordinary_path_count,
            &rows,
        );
        Ok(Self {
            rows,
            source_pack_registration_count,
            ordinary_path_count,
            report_digest,
        })
    }

    pub fn rows(&self) -> &[WorthTopologyRelationalInvariantOldPackResidueRow] {
        &self.rows
    }

    pub const fn source_pack_registration_count(&self) -> usize {
        self.source_pack_registration_count
    }

    pub const fn ordinary_path_count(&self) -> usize {
        self.ordinary_path_count
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

fn old_pack_residue_report_digest(
    source_pack_registration_count: usize,
    ordinary_path_count: usize,
    rows: &[WorthTopologyRelationalInvariantOldPackResidueRow],
) -> String {
    let mut parts = vec![
        "worth-topo-relational-invariant-old-pack-residue-report-v1".to_string(),
        format!("source-pack-registration-count:{source_pack_registration_count}"),
        format!("ordinary-path-count:{ordinary_path_count}"),
    ];
    parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
    parts.join("|")
}
