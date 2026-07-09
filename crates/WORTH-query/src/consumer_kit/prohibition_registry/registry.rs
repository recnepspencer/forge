use super::enforcement::WorthQueryProhibitionEnforcementTier;
use super::row::WorthQueryProhibitionRegistryRow;
use super::seam::WorthQueryProhibitedSeam;

const WORKSPACE_SUBMISSION_LANE: &str = "WorthQueryWorkspace::submissions";
const GRAPH_OR_PROBE_INTENT_LANE: &str =
    "typed existing-truth binding artifact plus graph composition or probe intent lane";
const ADMITTED_EXISTING_TRUTH_MUTATION_LANE: &str =
    "graph composition or admitted existing-truth mutation lane";

static HARD_PROHIBITION_ROWS: &[WorthQueryProhibitionRegistryRow] = &[
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::WorkspaceDirectWrite,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        WORKSPACE_SUBMISSION_LANE,
        "direct workspace writes bypass the explicit submission/admission lane",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::WorkspaceDirectBatch,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        WORKSPACE_SUBMISSION_LANE,
        "direct workspace batches bypass the explicit submission/admission lane",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthBindEntity,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        GRAPH_OR_PROBE_INTENT_LANE,
        "workspace binding helpers hide the typed binding artifact boundary",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthBindRelation,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        GRAPH_OR_PROBE_INTENT_LANE,
        "workspace binding helpers hide the typed binding artifact boundary",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthProbe,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        GRAPH_OR_PROBE_INTENT_LANE,
        "existing-truth probes must pass through intent admission before execution",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthUpdate,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        ADMITTED_EXISTING_TRUTH_MUTATION_LANE,
        "existing-truth mutation must not be caller-assembled from a direct binding",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthAssert,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        ADMITTED_EXISTING_TRUTH_MUTATION_LANE,
        "existing-truth assertion must stay inside the admitted runtime lane",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthVerify,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        ADMITTED_EXISTING_TRUTH_MUTATION_LANE,
        "existing-truth verification must stay inside the admitted runtime lane",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthUpdateVerified,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        ADMITTED_EXISTING_TRUTH_MUTATION_LANE,
        "verified existing-truth mutation must be planned by the owning lane",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthDelete,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        ADMITTED_EXISTING_TRUTH_MUTATION_LANE,
        "existing-truth deletion must not be caller-assembled from a direct binding",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthDeleteWith,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        ADMITTED_EXISTING_TRUTH_MUTATION_LANE,
        "existing-truth deletion must not be caller-assembled from a direct binding",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::WorkspaceExistingTruthDeleteVerified,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        ADMITTED_EXISTING_TRUTH_MUTATION_LANE,
        "verified existing-truth deletion must be planned by the owning lane",
    ),
];

#[derive(Clone, Copy, Debug)]
pub struct WorthQueryProhibitionRegistry {
    rows: &'static [WorthQueryProhibitionRegistryRow],
}

impl WorthQueryProhibitionRegistry {
    pub(crate) const fn new(rows: &'static [WorthQueryProhibitionRegistryRow]) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &'static [WorthQueryProhibitionRegistryRow] {
        self.rows
    }

    pub fn contains_seam(&self, seam: WorthQueryProhibitedSeam) -> bool {
        self.row(seam).is_some()
    }

    pub fn row(
        &self,
        seam: WorthQueryProhibitedSeam,
    ) -> Option<&'static WorthQueryProhibitionRegistryRow> {
        self.rows.iter().find(|row| row.seam() == seam)
    }
}

pub fn hard_prohibition_registry() -> WorthQueryProhibitionRegistry {
    WorthQueryProhibitionRegistry::new(HARD_PROHIBITION_ROWS)
}
