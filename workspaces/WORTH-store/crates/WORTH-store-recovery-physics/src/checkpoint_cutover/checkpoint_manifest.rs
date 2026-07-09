use super::{
    CheckpointCoveredLsnRange, CheckpointId, CheckpointPageLsnFrontier,
    CheckpointRecoveryCounterSnapshot, CheckpointRedoBoundary, CheckpointRootPosture,
    CheckpointValidationDenial, CheckpointValidationDenialKind,
    FuzzyCheckpointCertificationModeDenial, SharpCheckpointCertificationMode,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointManifest {
    checkpoint_id: CheckpointId,
    root_posture: CheckpointRootPosture,
    page_lsn_frontier: CheckpointPageLsnFrontier,
    covered_lsn_range: CheckpointCoveredLsnRange,
    redo_boundary: CheckpointRedoBoundary,
    counters: CheckpointRecoveryCounterSnapshot,
}

impl CheckpointManifest {
    pub fn sharp(
        root_posture: CheckpointRootPosture,
        page_lsn_frontier: CheckpointPageLsnFrontier,
        covered_lsn_range: CheckpointCoveredLsnRange,
        redo_boundary: CheckpointRedoBoundary,
        _mode: SharpCheckpointCertificationMode,
    ) -> Result<Self, CheckpointValidationDenial> {
        let counters = CheckpointRecoveryCounterSnapshot::new().with_manifest_validation();
        let root = match root_posture {
            CheckpointRootPosture::RootPresent(root) => root,
            CheckpointRootPosture::MissingRoot => {
                return Err(CheckpointValidationDenial::new(
                    CheckpointValidationDenialKind::MissingRoot,
                    counters,
                )
                .with_root_posture(root_posture));
            }
            CheckpointRootPosture::StaleRoot(_) => {
                return Err(CheckpointValidationDenial::new(
                    CheckpointValidationDenialKind::StaleRoot,
                    counters,
                )
                .with_root_posture(root_posture));
            }
        };
        if !covered_lsn_range.contains(redo_boundary.lsn()) {
            return Err(CheckpointValidationDenial::new(
                CheckpointValidationDenialKind::RedoBoundaryOutsideCoveredRange,
                counters,
            )
            .with_lsn_pair(
                covered_lsn_range.range().end_exclusive(),
                redo_boundary.lsn(),
            ));
        }
        page_lsn_frontier.require_covers_redo_boundary(redo_boundary, counters)?;
        Ok(Self {
            checkpoint_id: CheckpointId::from_basis(format!(
                "s4-checkpoint:{:?}:{:?}:{:?}:{:?}",
                root, page_lsn_frontier, covered_lsn_range, redo_boundary
            )),
            root_posture,
            page_lsn_frontier,
            covered_lsn_range,
            redo_boundary,
            counters,
        })
    }

    pub fn torn_manifest() -> Result<Self, CheckpointValidationDenial> {
        Err(CheckpointValidationDenial::new(
            CheckpointValidationDenialKind::TornManifest,
            CheckpointRecoveryCounterSnapshot::new().with_manifest_validation(),
        ))
    }

    pub fn fuzzy_checkpoint_attempt(
        _denial: FuzzyCheckpointCertificationModeDenial,
    ) -> Result<Self, CheckpointValidationDenial> {
        Err(CheckpointValidationDenial::new(
            CheckpointValidationDenialKind::FuzzyCheckpointModeUnsupported,
            CheckpointRecoveryCounterSnapshot::new().with_manifest_validation(),
        ))
    }

    pub fn checkpoint_id(&self) -> &CheckpointId {
        &self.checkpoint_id
    }

    pub const fn root_posture(&self) -> CheckpointRootPosture {
        self.root_posture
    }

    pub const fn covered_lsn_range(&self) -> CheckpointCoveredLsnRange {
        self.covered_lsn_range
    }

    pub const fn redo_boundary(&self) -> CheckpointRedoBoundary {
        self.redo_boundary
    }

    pub fn page_lsn_frontier(&self) -> &CheckpointPageLsnFrontier {
        &self.page_lsn_frontier
    }

    pub const fn counters(&self) -> CheckpointRecoveryCounterSnapshot {
        self.counters
    }
}
