use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_composition::worth_workload::{
    WorthWorkloadOrdinaryConsumerCutoverPosture, WorthWorkloadOrdinaryConsumerCutoverRow,
};
use topology::facade::{
    current_query_backed_consumer_residue_manifest, QueryBackedConsumerResidueDisposition,
    QueryBackedConsumerResidueRow,
};
use worth_spatial::facade::evidence_lookup_public_closeout::{
    current_evidence_lookup_public_closeout_residue_manifest,
    EvidenceLookupPublicCloseoutResidueDisposition, EvidenceLookupPublicCloseoutResidueRow,
};

const CUTOVER_RESIDUE_SOURCE_PATH: &str =
    "crates/worth-kernel/src/workload_composition/planner_owned_routing/public_proof/residue_chain.rs";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTouchedGraphConflictQueryGapKind {
    MissingArtifact,
    NotAdmittedOnSupportedPath,
    NotExposedAtBoundary,
    IdentitySemanticsInsufficient,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTouchedGraphConflictResidueBoundaryPosture {
    QueryProofAccompanimentOnly,
    ReplayUndoCloseoutOnly,
    QueryGapSupportGap,
    CoveredOrdinaryConsumerDependency,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTouchedGraphConflictResidueDisposition {
    ExplicitResidue,
    QueryGap,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictResidueRow {
    family_kind: TouchedGraphParityFamilyKind,
    source_path: String,
    surface_name: String,
    owner: String,
    disposition: WorthTouchedGraphConflictResidueDisposition,
    query_gap_kind: Option<WorthTouchedGraphConflictQueryGapKind>,
    blocker: String,
    removal_trigger: String,
    boundary_posture: WorthTouchedGraphConflictResidueBoundaryPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictResidueChain {
    rows: Vec<WorthTouchedGraphConflictResidueRow>,
    residue_digest: String,
}

impl WorthTouchedGraphConflictResidueChain {
    pub(crate) fn from_cutover_rows(rows: &[WorthWorkloadOrdinaryConsumerCutoverRow]) -> Self {
        let lowered = rows
            .iter()
            .filter_map(WorthTouchedGraphConflictResidueRow::from_cutover_row)
            .collect::<Vec<_>>();
        Self::from_rows(lowered)
    }

    pub(crate) fn from_current_live_surfaces(
        rows: &[WorthWorkloadOrdinaryConsumerCutoverRow],
    ) -> Self {
        let mut lowered = rows
            .iter()
            .filter_map(WorthTouchedGraphConflictResidueRow::from_cutover_row)
            .collect::<Vec<_>>();
        lowered.extend(
            current_query_backed_consumer_residue_manifest()
                .iter()
                .map(WorthTouchedGraphConflictResidueRow::from_query_backed_residue_row),
        );
        lowered.extend(
            current_evidence_lookup_public_closeout_residue_manifest()
                .iter()
                .map(WorthTouchedGraphConflictResidueRow::from_evidence_lookup_public_closeout_residue_row),
        );
        Self::from_rows(lowered)
    }

    pub(crate) fn from_rows(mut rows: Vec<WorthTouchedGraphConflictResidueRow>) -> Self {
        rows.sort_by(|left, right| left.surface_name.cmp(&right.surface_name));
        let residue_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &rows
                .iter()
                .map(|row| {
                    format!(
                        "{}:{}:{}:{}:{}:{}:{}:{}",
                        row.source_path,
                        row.surface_name,
                        row.owner,
                        row.disposition.as_str(),
                        row.query_gap_kind.map_or("none", |kind| kind.as_str()),
                        row.blocker,
                        row.removal_trigger,
                        row.boundary_posture.as_str()
                    )
                })
                .chain(std::iter::once(
                    "worth-kernel:touched-graph-conflict-residue-chain:v1".to_string(),
                ))
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            residue_digest,
        }
    }

    pub fn rows(&self) -> &[WorthTouchedGraphConflictResidueRow] {
        &self.rows
    }

    pub fn residue_digest(&self) -> &str {
        &self.residue_digest
    }

    pub fn ordinary_dependency_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| {
                row.boundary_posture
                    == WorthTouchedGraphConflictResidueBoundaryPosture::CoveredOrdinaryConsumerDependency
            })
            .count()
    }
}

impl WorthTouchedGraphConflictResidueRow {
    pub(crate) fn new(
        family_kind: TouchedGraphParityFamilyKind,
        source_path: impl Into<String>,
        surface_name: impl Into<String>,
        owner: impl Into<String>,
        disposition: WorthTouchedGraphConflictResidueDisposition,
        query_gap_kind: Option<WorthTouchedGraphConflictQueryGapKind>,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
        boundary_posture: WorthTouchedGraphConflictResidueBoundaryPosture,
    ) -> Self {
        Self {
            family_kind,
            source_path: source_path.into(),
            surface_name: surface_name.into(),
            owner: owner.into(),
            disposition,
            query_gap_kind,
            blocker: blocker.into(),
            removal_trigger: removal_trigger.into(),
            boundary_posture,
        }
    }

    fn from_cutover_row(row: &WorthWorkloadOrdinaryConsumerCutoverRow) -> Option<Self> {
        let boundary_posture = match row.posture() {
            WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer => {
                return None;
            }
            WorthWorkloadOrdinaryConsumerCutoverPosture::QueryProofAccompanimentOnly => {
                WorthTouchedGraphConflictResidueBoundaryPosture::QueryProofAccompanimentOnly
            }
            WorthWorkloadOrdinaryConsumerCutoverPosture::ReplayUndoCloseoutOnly => {
                WorthTouchedGraphConflictResidueBoundaryPosture::ReplayUndoCloseoutOnly
            }
            WorthWorkloadOrdinaryConsumerCutoverPosture::CoveredOrdinaryConsumerDependency => {
                WorthTouchedGraphConflictResidueBoundaryPosture::CoveredOrdinaryConsumerDependency
            }
        };
        Some(Self::new(
            family_kind_for_cutover_surface(row.surface_name()),
            CUTOVER_RESIDUE_SOURCE_PATH,
            row.surface_name(),
            row.owner(),
            WorthTouchedGraphConflictResidueDisposition::ExplicitResidue,
            None,
            row.blocker(),
            row.removal_trigger(),
            boundary_posture,
        ))
    }

    fn from_query_backed_residue_row(row: &QueryBackedConsumerResidueRow) -> Self {
        let (disposition, boundary_posture) = match row.disposition() {
            QueryBackedConsumerResidueDisposition::ExplicitResidue => (
                WorthTouchedGraphConflictResidueDisposition::ExplicitResidue,
                WorthTouchedGraphConflictResidueBoundaryPosture::QueryProofAccompanimentOnly,
            ),
            QueryBackedConsumerResidueDisposition::QueryGap => (
                WorthTouchedGraphConflictResidueDisposition::QueryGap,
                WorthTouchedGraphConflictResidueBoundaryPosture::QueryGapSupportGap,
            ),
        };
        let query_gap_kind = row.query_gap_kind().map(|kind| match kind.as_str() {
            "missing" => WorthTouchedGraphConflictQueryGapKind::MissingArtifact,
            "not-admitted" => {
                WorthTouchedGraphConflictQueryGapKind::NotAdmittedOnSupportedPath
            }
            "not-exposed" => WorthTouchedGraphConflictQueryGapKind::NotExposedAtBoundary,
            "identity-semantics-insufficient" => {
                WorthTouchedGraphConflictQueryGapKind::IdentitySemanticsInsufficient
            }
            other => panic!("unexpected topology query gap kind `{other}`"),
        });
        Self::new(
            TouchedGraphParityFamilyKind::ReadRouting,
            row.source_path(),
            row.current_surface(),
            match row.owner() {
                topology::facade::QueryBackedConsumerResidueOwner::WorthTopo => "worth-topo",
                topology::facade::QueryBackedConsumerResidueOwner::ForgeQuery => "forge-query",
            },
            disposition,
            query_gap_kind,
            row.blocker(),
            row.removal_trigger(),
            boundary_posture,
        )
    }

    fn from_evidence_lookup_public_closeout_residue_row(
        row: &EvidenceLookupPublicCloseoutResidueRow,
    ) -> Self {
        let (disposition, boundary_posture) = match row.disposition() {
            EvidenceLookupPublicCloseoutResidueDisposition::ExplicitResidue => (
                WorthTouchedGraphConflictResidueDisposition::ExplicitResidue,
                WorthTouchedGraphConflictResidueBoundaryPosture::QueryProofAccompanimentOnly,
            ),
            EvidenceLookupPublicCloseoutResidueDisposition::QueryGap => (
                WorthTouchedGraphConflictResidueDisposition::QueryGap,
                WorthTouchedGraphConflictResidueBoundaryPosture::QueryGapSupportGap,
            ),
        };
        let query_gap_kind = row.query_gap_kind().map(|kind| match kind.as_str() {
            "missing" => WorthTouchedGraphConflictQueryGapKind::MissingArtifact,
            "not-admitted" => {
                WorthTouchedGraphConflictQueryGapKind::NotAdmittedOnSupportedPath
            }
            "not-exposed" => WorthTouchedGraphConflictQueryGapKind::NotExposedAtBoundary,
            "identity-semantics-insufficient" => {
                WorthTouchedGraphConflictQueryGapKind::IdentitySemanticsInsufficient
            }
            other => panic!("unexpected spatial query gap kind `{other}`"),
        });
        Self::new(
            TouchedGraphParityFamilyKind::EvidenceLookup,
            row.source_path(),
            row.current_surface(),
            match row.owner() {
                worth_spatial::facade::evidence_lookup_public_closeout::EvidenceLookupPublicCloseoutResidueOwner::WorthSpatial => "worth-spatial",
                worth_spatial::facade::evidence_lookup_public_closeout::EvidenceLookupPublicCloseoutResidueOwner::WorthTopo => "worth-topo",
                worth_spatial::facade::evidence_lookup_public_closeout::EvidenceLookupPublicCloseoutResidueOwner::ForgeQuery => "forge-query",
            },
            disposition,
            query_gap_kind,
            row.blocker(),
            row.removal_trigger(),
            boundary_posture,
        )
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub const fn family_kind(&self) -> TouchedGraphParityFamilyKind {
        self.family_kind
    }

    pub fn surface_name(&self) -> &str {
        &self.surface_name
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub const fn disposition(&self) -> WorthTouchedGraphConflictResidueDisposition {
        self.disposition
    }

    pub const fn query_gap_kind(&self) -> Option<WorthTouchedGraphConflictQueryGapKind> {
        self.query_gap_kind
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub const fn boundary_posture(&self) -> WorthTouchedGraphConflictResidueBoundaryPosture {
        self.boundary_posture
    }
}

impl WorthTouchedGraphConflictResidueBoundaryPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueryProofAccompanimentOnly => "query-proof-accompaniment-only",
            Self::ReplayUndoCloseoutOnly => "replay-undo-closeout-only",
            Self::QueryGapSupportGap => "query-gap-support-gap",
            Self::CoveredOrdinaryConsumerDependency => "covered-ordinary-consumer-dependency",
        }
    }
}

impl WorthTouchedGraphConflictResidueDisposition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitResidue => "explicit-residue",
            Self::QueryGap => "query-gap",
        }
    }
}

impl WorthTouchedGraphConflictQueryGapKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingArtifact => "missing",
            Self::NotAdmittedOnSupportedPath => "not-admitted",
            Self::NotExposedAtBoundary => "not-exposed",
            Self::IdentitySemanticsInsufficient => "identity-semantics-insufficient",
        }
    }
}

fn family_kind_for_cutover_surface(surface_name: &str) -> TouchedGraphParityFamilyKind {
    match surface_name {
        "admit_boolean_split_replay_undo_boundary" | "BooleanChainIntegrationHandoff" => {
            TouchedGraphParityFamilyKind::ReplayUndo
        }
        "PlanarBooleanLoopRuntimeRegistrationProof" => {
            TouchedGraphParityFamilyKind::ConflictIndependenceBatchAdmission
        }
        other => panic!(
            "missing production-owned phase-3 residue family mapping for ordinary cutover surface `{other}`"
        ),
    }
}
