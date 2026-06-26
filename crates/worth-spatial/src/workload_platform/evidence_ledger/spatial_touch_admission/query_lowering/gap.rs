use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_ledger::SpatialEvidenceSurfaceOwner;

const DECLARED_MUTATION_GAP_BLOCKER: &str =
    "Spatial evidence touch authority is read-family evidence, not graph mutation meaning.";
const DECLARED_MUTATION_GAP_REMOVAL_TRIGGER: &str =
    "Milestone 5 introduces a Query-owned obligation selection lane that needs declared mutation semantics for spatial evidence.";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialEvidenceQueryGapRow {
    kind: SpatialEvidenceQueryGapKind,
    owner: SpatialEvidenceSurfaceOwner,
    cap: &'static str,
    blocker: &'static str,
    removal_trigger: &'static str,
    gap_digest: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialEvidenceQueryGapKind {
    DeclaredMutationCollectionNotExpressed,
}

impl SpatialEvidenceQueryGapRow {
    fn declared_mutation_not_expressed() -> Self {
        Self::new(
            SpatialEvidenceQueryGapKind::DeclaredMutationCollectionNotExpressed,
            "declared mutation collection selector is capped because this phase lowers spatial evidence as Query read-family touch only",
            DECLARED_MUTATION_GAP_BLOCKER,
            DECLARED_MUTATION_GAP_REMOVAL_TRIGGER,
        )
    }

    fn new(
        kind: SpatialEvidenceQueryGapKind,
        cap: &'static str,
        blocker: &'static str,
        removal_trigger: &'static str,
    ) -> Self {
        let owner = SpatialEvidenceSurfaceOwner::WorthSpatial;
        let gap_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "spatial-evidence-query-gap".to_string(),
                format!("kind:{}", kind.as_str()),
                "owner:worth-spatial".to_string(),
                format!("cap:{cap}"),
                format!("blocker:{blocker}"),
                format!("removal-trigger:{removal_trigger}"),
            ],
        );
        Self {
            kind,
            owner,
            cap,
            blocker,
            removal_trigger,
            gap_digest,
        }
    }

    pub fn kind(&self) -> SpatialEvidenceQueryGapKind {
        self.kind
    }

    pub fn owner(&self) -> SpatialEvidenceSurfaceOwner {
        self.owner
    }

    pub fn cap(&self) -> &'static str {
        self.cap
    }

    pub fn blocker(&self) -> &'static str {
        self.blocker
    }

    pub fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }

    pub fn gap_digest(&self) -> &str {
        &self.gap_digest
    }
}

impl SpatialEvidenceQueryGapKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeclaredMutationCollectionNotExpressed => {
                "declared-mutation-collection-not-expressed"
            }
        }
    }
}

pub(super) fn declared_mutation_query_gap_rows() -> Vec<SpatialEvidenceQueryGapRow> {
    vec![SpatialEvidenceQueryGapRow::declared_mutation_not_expressed()]
}
