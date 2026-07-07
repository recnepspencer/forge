use crate::workload_composition::planner_owned_routing::current_worth_workload_ordinary_consumer_cutover;
use crate::workload_composition::planner_owned_routing::public_proof::{
    WorthTouchedGraphConflictResidueBoundaryPosture, WorthTouchedGraphConflictResidueChain,
    WorthTouchedGraphConflictResidueDisposition, WorthTouchedGraphConflictResidueRow,
};

const PUBLIC_FACADE_RESIDUE_ROOTS: &[&str] = &[
    "crates/worth-kernel/src/workload_composition/planner_owned_routing/public_facade/",
    "crates/worth-kernel/src/workload_composition/public_closeout/",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublicCloseoutConsumerResidueOwner {
    WorthKernel,
    WorthTopo,
    WorthSpatial,
    ForgeQuery,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublicCloseoutConsumerResidueDisposition {
    ExplicitResidue,
    QueryGap,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublicCloseoutConsumerResidueBoundaryPosture {
    QueryProofAccompanimentOnly,
    ReplayUndoCloseoutOnly,
    QueryGapSupportGap,
    CoveredOrdinaryConsumerDependency,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicCloseoutConsumerResidueManifestError {
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicCloseoutConsumerResidueRow {
    source_path: String,
    current_surface: String,
    owner: PublicCloseoutConsumerResidueOwner,
    disposition: PublicCloseoutConsumerResidueDisposition,
    blocker: String,
    removal_trigger: String,
    boundary_posture: PublicCloseoutConsumerResidueBoundaryPosture,
}

impl PublicCloseoutConsumerResidueRow {
    fn from_live_residue_row(row: &WorthTouchedGraphConflictResidueRow) -> Self {
        Self {
            source_path: row.source_path().to_string(),
            current_surface: row.surface_name().to_string(),
            owner: PublicCloseoutConsumerResidueOwner::from_live_row_owner(row.owner()),
            disposition: PublicCloseoutConsumerResidueDisposition::from_live_row_disposition(
                row.disposition(),
            ),
            blocker: row.blocker().to_string(),
            removal_trigger: row.removal_trigger().to_string(),
            boundary_posture: PublicCloseoutConsumerResidueBoundaryPosture::from_live_row_posture(
                row.boundary_posture(),
            ),
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn current_surface(&self) -> &str {
        &self.current_surface
    }

    pub const fn owner(&self) -> PublicCloseoutConsumerResidueOwner {
        self.owner
    }

    pub const fn disposition(&self) -> PublicCloseoutConsumerResidueDisposition {
        self.disposition
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    #[cfg(test)]
    pub const fn boundary_posture(&self) -> PublicCloseoutConsumerResidueBoundaryPosture {
        self.boundary_posture
    }
}

pub fn current_public_closeout_consumer_residue_manifest(
) -> Result<Vec<PublicCloseoutConsumerResidueRow>, PublicCloseoutConsumerResidueManifestError> {
    let cutover = current_worth_workload_ordinary_consumer_cutover().map_err(|error| {
        PublicCloseoutConsumerResidueManifestError::new(format!(
            "current public-closeout residue manifest requires the current ordinary-consumer cutover: {error:?}"
        ))
    })?;
    let residue_chain =
        WorthTouchedGraphConflictResidueChain::from_current_live_surfaces(cutover.rows());
    Ok(public_closeout_consumer_residue_manifest_from_chain(
        &residue_chain,
    ))
}

pub(crate) fn public_closeout_consumer_residue_manifest_from_chain(
    residue_chain: &WorthTouchedGraphConflictResidueChain,
) -> Vec<PublicCloseoutConsumerResidueRow> {
    residue_chain
        .rows()
        .iter()
        .filter(|row| is_public_closeout_local_residue_row(row))
        .map(PublicCloseoutConsumerResidueRow::from_live_residue_row)
        .collect()
}

fn is_public_closeout_local_residue_row(row: &WorthTouchedGraphConflictResidueRow) -> bool {
    PUBLIC_FACADE_RESIDUE_ROOTS
        .iter()
        .any(|root| row.source_path().starts_with(root))
}

impl PublicCloseoutConsumerResidueOwner {
    fn from_live_row_owner(owner: &str) -> Self {
        match owner {
            "worth-kernel" => Self::WorthKernel,
            "worth-topo" => Self::WorthTopo,
            "worth-spatial" => Self::WorthSpatial,
            "forge-query" => Self::ForgeQuery,
            other => panic!("unrecognized public-closeout residue owner `{other}`"),
        }
    }
}

impl PublicCloseoutConsumerResidueDisposition {
    fn from_live_row_disposition(disposition: WorthTouchedGraphConflictResidueDisposition) -> Self {
        match disposition {
            WorthTouchedGraphConflictResidueDisposition::ExplicitResidue => Self::ExplicitResidue,
            WorthTouchedGraphConflictResidueDisposition::QueryGap => Self::QueryGap,
        }
    }
}

impl PublicCloseoutConsumerResidueBoundaryPosture {
    fn from_live_row_posture(posture: WorthTouchedGraphConflictResidueBoundaryPosture) -> Self {
        match posture {
            WorthTouchedGraphConflictResidueBoundaryPosture::QueryProofAccompanimentOnly => {
                Self::QueryProofAccompanimentOnly
            }
            WorthTouchedGraphConflictResidueBoundaryPosture::ReplayUndoCloseoutOnly => {
                Self::ReplayUndoCloseoutOnly
            }
            WorthTouchedGraphConflictResidueBoundaryPosture::QueryGapSupportGap => {
                Self::QueryGapSupportGap
            }
            WorthTouchedGraphConflictResidueBoundaryPosture::CoveredOrdinaryConsumerDependency => {
                Self::CoveredOrdinaryConsumerDependency
            }
        }
    }
}

impl PublicCloseoutConsumerResidueManifestError {
    fn new(detail: String) -> Self {
        Self { detail }
    }
}
