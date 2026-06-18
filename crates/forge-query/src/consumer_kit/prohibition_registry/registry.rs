use super::enforcement::ForgeQueryProhibitionEnforcementTier;
use super::row::ForgeQueryProhibitionRegistryRow;
use super::seam::ForgeQueryProhibitedSeam;

const WORKSPACE_SUBMISSION_LANE: &str = "ForgeQueryWorkspace::submissions";
const GRAPH_OR_PROBE_INTENT_LANE: &str =
    "typed existing-truth binding artifact plus graph composition or probe intent lane";
const ADMITTED_EXISTING_TRUTH_MUTATION_LANE: &str =
    "graph composition or admitted existing-truth mutation lane";

static HARD_PROHIBITION_ROWS: &[ForgeQueryProhibitionRegistryRow] = &[
    ForgeQueryProhibitionRegistryRow::new(
        ForgeQueryProhibitedSeam::WorkspaceDirectWrite,
        ForgeQueryProhibitionEnforcementTier::SealedByVisibility,
        WORKSPACE_SUBMISSION_LANE,
        "direct workspace writes bypass the explicit submission/admission lane",
    ),
    ForgeQueryProhibitionRegistryRow::new(
        ForgeQueryProhibitedSeam::WorkspaceDirectBatch,
        ForgeQueryProhibitionEnforcementTier::SealedByVisibility,
        WORKSPACE_SUBMISSION_LANE,
        "direct workspace batches bypass the explicit submission/admission lane",
    ),
    ForgeQueryProhibitionRegistryRow::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthBindEntity,
        ForgeQueryProhibitionEnforcementTier::SealedByVisibility,
        GRAPH_OR_PROBE_INTENT_LANE,
        "workspace binding helpers hide the typed binding artifact boundary",
    ),
    ForgeQueryProhibitionRegistryRow::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthBindRelation,
        ForgeQueryProhibitionEnforcementTier::SealedByVisibility,
        GRAPH_OR_PROBE_INTENT_LANE,
        "workspace binding helpers hide the typed binding artifact boundary",
    ),
    ForgeQueryProhibitionRegistryRow::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthProbe,
        ForgeQueryProhibitionEnforcementTier::SealedByVisibility,
        GRAPH_OR_PROBE_INTENT_LANE,
        "existing-truth probes must pass through intent admission before execution",
    ),
    ForgeQueryProhibitionRegistryRow::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthUpdate,
        ForgeQueryProhibitionEnforcementTier::SealedByVisibility,
        ADMITTED_EXISTING_TRUTH_MUTATION_LANE,
        "existing-truth mutation must not be caller-assembled from a direct binding",
    ),
    ForgeQueryProhibitionRegistryRow::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthAssert,
        ForgeQueryProhibitionEnforcementTier::SealedByVisibility,
        ADMITTED_EXISTING_TRUTH_MUTATION_LANE,
        "existing-truth assertion must stay inside the admitted runtime lane",
    ),
    ForgeQueryProhibitionRegistryRow::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthVerify,
        ForgeQueryProhibitionEnforcementTier::SealedByVisibility,
        ADMITTED_EXISTING_TRUTH_MUTATION_LANE,
        "existing-truth verification must stay inside the admitted runtime lane",
    ),
    ForgeQueryProhibitionRegistryRow::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthUpdateVerified,
        ForgeQueryProhibitionEnforcementTier::SealedByVisibility,
        ADMITTED_EXISTING_TRUTH_MUTATION_LANE,
        "verified existing-truth mutation must be planned by the owning lane",
    ),
    ForgeQueryProhibitionRegistryRow::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthDelete,
        ForgeQueryProhibitionEnforcementTier::SealedByVisibility,
        ADMITTED_EXISTING_TRUTH_MUTATION_LANE,
        "existing-truth deletion must not be caller-assembled from a direct binding",
    ),
    ForgeQueryProhibitionRegistryRow::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthDeleteWith,
        ForgeQueryProhibitionEnforcementTier::SealedByVisibility,
        ADMITTED_EXISTING_TRUTH_MUTATION_LANE,
        "existing-truth deletion must not be caller-assembled from a direct binding",
    ),
    ForgeQueryProhibitionRegistryRow::new(
        ForgeQueryProhibitedSeam::WorkspaceExistingTruthDeleteVerified,
        ForgeQueryProhibitionEnforcementTier::SealedByVisibility,
        ADMITTED_EXISTING_TRUTH_MUTATION_LANE,
        "verified existing-truth deletion must be planned by the owning lane",
    ),
];

#[derive(Clone, Copy, Debug)]
pub struct ForgeQueryProhibitionRegistry {
    rows: &'static [ForgeQueryProhibitionRegistryRow],
}

impl ForgeQueryProhibitionRegistry {
    pub(crate) const fn new(rows: &'static [ForgeQueryProhibitionRegistryRow]) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &'static [ForgeQueryProhibitionRegistryRow] {
        self.rows
    }

    pub fn contains_seam(&self, seam: ForgeQueryProhibitedSeam) -> bool {
        self.row(seam).is_some()
    }

    pub fn row(
        &self,
        seam: ForgeQueryProhibitedSeam,
    ) -> Option<&'static ForgeQueryProhibitionRegistryRow> {
        self.rows.iter().find(|row| row.seam() == seam)
    }
}

pub fn hard_prohibition_registry() -> ForgeQueryProhibitionRegistry {
    ForgeQueryProhibitionRegistry::new(HARD_PROHIBITION_ROWS)
}
