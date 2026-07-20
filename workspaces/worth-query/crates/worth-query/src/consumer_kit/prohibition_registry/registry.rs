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
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::RawDigestMinting,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        "facade::identity_authority",
        "authority identities are minted only by sealed Query-owned admission",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::RawBasisIdentity,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        "facade::foundation::basis_lifecycle",
        "basis authority must originate from the declarative scoped lifecycle",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::UnscopedQueryContext,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        "scoped observation or materialization query context",
        "query execution must carry a scoped basis proof",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::RawIntentAdmissionRequest,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        "declarative intent authoring facade",
        "raw admission requests are internal lifecycle machinery",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::PostureAuthoredSubscription,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        "scoped subscription declaration and activation",
        "posture values cannot author subscription authority",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::ReceiptOnlyCausalInspection,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        "receipt plus ScopedInspectionBasis",
        "causal evidence does not independently authorize inspection",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::LegacyPreviewExecution,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        "ScopedPreviewLiveSessionPlanBinding",
        "preview execution and drift require the scoped live binding",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::DeepFacadeToolingImport,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        "facade::certification",
        "ordinary facade namespaces cannot expose certification or migration machinery",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::LegacyQueryBasisLifecycle,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        "facade::foundation::basis_lifecycle",
        "the deleted parallel lifecycle cannot be restored as competing authority",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::CrateRootPhaseMirror,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        "capability-oriented facade declaration and returned evidence",
        "crate-root mirrors cannot expose Query's internal phase artifacts",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::DeepPhaseModuleImport,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        "ordinary facade capability journey",
        "moving a transition into a deep module is not phase quarantine",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::OrdinaryFacadePhaseReexport,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        "facade::read declaration and outcome navigation",
        "ordinary namespaces expose desired capabilities rather than phase advancement",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::PhaseArtifactAlias,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        "sealed proof returned by an ordinary capability journey",
        "renaming a phase artifact cannot restore downstream access",
    ),
    WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::GenericPhaseConversion,
        WorthQueryProhibitionEnforcementTier::SealedByVisibility,
        "Query-owned phase progression",
        "generic conversion traits cannot mint or advance internal phase artifacts",
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
