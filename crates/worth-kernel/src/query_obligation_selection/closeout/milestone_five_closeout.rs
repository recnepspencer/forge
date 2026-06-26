use std::collections::BTreeSet;

use super::super::selection_substrate::{
    QueryObligationSelectionAuthorityKind, QuerySelectedGraphObligationCloseout,
    QuerySelectorPrecisionPosture,
};
use super::MilestoneFiveQueryObligationSelectionCloseoutReport;

#[derive(Clone, Debug)]
pub struct MilestoneFiveQueryObligationSelectionCloseout {
    selected_count: usize,
    execution_row_count: usize,
    selected_registration_count: usize,
    topology_selected_count: usize,
    spatial_selected_count: usize,
    broad_selector_residue_count: usize,
    query_selector_gap_count: usize,
    authority_digests: Vec<String>,
    touch_descriptor_digests: Vec<String>,
    selected_registration_digests: Vec<String>,
    residue_manifest_digests: Vec<String>,
    execution_proof_digests: Vec<String>,
    adoption_manifest_digests: Vec<String>,
    selector_precision_report_digests: Vec<String>,
    closeout_report: MilestoneFiveQueryObligationSelectionCloseoutReport,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneSixGraphReadInventorySeed {
    selected_count: usize,
    selected_registration_count: usize,
    execution_row_count: usize,
    authority_digests: Vec<String>,
    touch_descriptor_digests: Vec<String>,
    selected_registration_digests: Vec<String>,
    residue_manifest_digests: Vec<String>,
    execution_proof_digests: Vec<String>,
    adoption_manifest_digests: Vec<String>,
    selector_precision_report_digests: Vec<String>,
    graph_read_access_planning_claimed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MilestoneFiveQueryObligationSelectionCloseoutError {
    MissingSelectedObligations,
    MissingExecutionRows,
    MissingSelectedRegistrationIdentity,
    MissingTopologySelection,
    MissingSpatialSelection,
    GraphReadAccessPlanningClaimed,
    UnboundedSelectorPrecision,
    LocalCeremonyNotClean,
    MissingResidueManifestDigest,
    MissingExecutionProofDigest,
    MissingAdoptionManifestDigest,
    MissingSelectorPrecisionDigest,
    CloseoutOpenFindings,
}

impl MilestoneFiveQueryObligationSelectionCloseout {
    pub fn from_selected_closeouts<I>(
        closeouts: I,
    ) -> Result<Self, MilestoneFiveQueryObligationSelectionCloseoutError>
    where
        I: IntoIterator<Item = QuerySelectedGraphObligationCloseout>,
    {
        let closeouts = closeouts.into_iter().collect::<Vec<_>>();
        require_milestone_five_closeout_rows(&closeouts)?;
        let selected_registration_digests = selected_registration_digest_set(&closeouts)
            .into_iter()
            .collect::<Vec<_>>();
        let selected_registration_count = selected_registration_digests.len();
        let closeout_report =
            MilestoneFiveQueryObligationSelectionCloseoutReport::from_closeout_rows(&closeouts);
        if closeout_report.open_finding_count() != 0 {
            return Err(MilestoneFiveQueryObligationSelectionCloseoutError::CloseoutOpenFindings);
        }

        Ok(Self {
            selected_count: selected_registration_count,
            execution_row_count: closeouts
                .iter()
                .map(QuerySelectedGraphObligationCloseout::execution_row_count)
                .sum(),
            selected_registration_count,
            topology_selected_count: count_authority(
                &closeouts,
                QueryObligationSelectionAuthorityKind::TopologyTouchedBasis,
            ),
            spatial_selected_count: count_authority(
                &closeouts,
                QueryObligationSelectionAuthorityKind::SpatialQueryDescriptor,
            ),
            broad_selector_residue_count: closeouts
                .iter()
                .map(|closeout| closeout.broad_selector_residue_rows().len())
                .sum(),
            query_selector_gap_count: closeouts
                .iter()
                .map(|closeout| closeout.query_selector_gap_rows().len())
                .sum(),
            authority_digests: closeouts
                .iter()
                .map(|closeout| closeout.authority_digest().to_string())
                .collect(),
            touch_descriptor_digests: closeouts
                .iter()
                .map(|closeout| closeout.touch_descriptor_digest().to_string())
                .collect(),
            selected_registration_digests,
            residue_manifest_digests: closeouts
                .iter()
                .map(|closeout| closeout.residue_manifest_digest().to_string())
                .collect(),
            execution_proof_digests: closeouts
                .iter()
                .map(|closeout| closeout.execution_proof_digest().to_string())
                .collect(),
            adoption_manifest_digests: closeouts
                .iter()
                .map(|closeout| closeout.adoption_manifest_digest().to_string())
                .collect(),
            selector_precision_report_digests: closeouts
                .iter()
                .map(|closeout| {
                    closeout
                        .selector_precision_report()
                        .report_digest()
                        .to_string()
                })
                .collect(),
            closeout_report,
        })
    }

    pub const fn is_closed(&self) -> bool {
        true
    }

    pub const fn selected_obligation_count(&self) -> usize {
        self.selected_count
    }

    pub const fn execution_row_count(&self) -> usize {
        self.execution_row_count
    }

    pub const fn selected_registration_count(&self) -> usize {
        self.selected_registration_count
    }

    pub const fn topology_selected_count(&self) -> usize {
        self.topology_selected_count
    }

    pub const fn spatial_selected_count(&self) -> usize {
        self.spatial_selected_count
    }

    pub const fn broad_selector_residue_count(&self) -> usize {
        self.broad_selector_residue_count
    }

    pub const fn query_selector_gap_count(&self) -> usize {
        self.query_selector_gap_count
    }

    pub const fn graph_read_access_planning_claimed(&self) -> bool {
        false
    }

    pub fn authority_digests(&self) -> &[String] {
        &self.authority_digests
    }

    pub fn touch_descriptor_digests(&self) -> &[String] {
        &self.touch_descriptor_digests
    }

    pub fn selected_registration_digests(&self) -> &[String] {
        &self.selected_registration_digests
    }

    pub const fn closeout_report(&self) -> &MilestoneFiveQueryObligationSelectionCloseoutReport {
        &self.closeout_report
    }

    pub const fn open_finding_count(&self) -> usize {
        self.closeout_report.open_finding_count()
    }

    pub fn into_graph_read_inventory_seed(self) -> MilestoneSixGraphReadInventorySeed {
        let graph_read_access_planning_claimed = self.graph_read_access_planning_claimed();
        MilestoneSixGraphReadInventorySeed {
            selected_count: self.selected_count,
            selected_registration_count: self.selected_registration_count,
            execution_row_count: self.execution_row_count,
            authority_digests: self.authority_digests,
            touch_descriptor_digests: self.touch_descriptor_digests,
            selected_registration_digests: self.selected_registration_digests,
            residue_manifest_digests: self.residue_manifest_digests,
            execution_proof_digests: self.execution_proof_digests,
            adoption_manifest_digests: self.adoption_manifest_digests,
            selector_precision_report_digests: self.selector_precision_report_digests,
            graph_read_access_planning_claimed,
        }
    }
}

impl MilestoneSixGraphReadInventorySeed {
    pub const fn selected_obligation_count(&self) -> usize {
        self.selected_count
    }

    pub const fn selected_registration_count(&self) -> usize {
        self.selected_registration_count
    }

    pub const fn execution_row_count(&self) -> usize {
        self.execution_row_count
    }

    pub const fn requires_graph_read_access_planning(&self) -> bool {
        !self.graph_read_access_planning_claimed
    }

    pub const fn graph_read_access_planning_claimed(&self) -> bool {
        self.graph_read_access_planning_claimed
    }

    pub fn authority_digests(&self) -> &[String] {
        &self.authority_digests
    }

    pub fn touch_descriptor_digests(&self) -> &[String] {
        &self.touch_descriptor_digests
    }

    pub fn selected_registration_digests(&self) -> &[String] {
        &self.selected_registration_digests
    }

    pub fn residue_manifest_digests(&self) -> &[String] {
        &self.residue_manifest_digests
    }

    pub fn execution_proof_digests(&self) -> &[String] {
        &self.execution_proof_digests
    }

    pub fn adoption_manifest_digests(&self) -> &[String] {
        &self.adoption_manifest_digests
    }

    pub fn selector_precision_report_digests(&self) -> &[String] {
        &self.selector_precision_report_digests
    }
}

fn require_milestone_five_closeout_rows(
    closeouts: &[QuerySelectedGraphObligationCloseout],
) -> Result<(), MilestoneFiveQueryObligationSelectionCloseoutError> {
    if closeouts.is_empty()
        || closeouts
            .iter()
            .any(|closeout| closeout.selected_obligation_count() == 0)
    {
        return Err(MilestoneFiveQueryObligationSelectionCloseoutError::MissingSelectedObligations);
    }
    if closeouts
        .iter()
        .any(|closeout| closeout.execution_row_count() == 0)
    {
        return Err(MilestoneFiveQueryObligationSelectionCloseoutError::MissingExecutionRows);
    }
    if closeouts
        .iter()
        .any(|closeout| closeout.selected_registration_digests().is_empty())
    {
        return Err(
            MilestoneFiveQueryObligationSelectionCloseoutError::MissingSelectedRegistrationIdentity,
        );
    }
    if count_authority(
        closeouts,
        QueryObligationSelectionAuthorityKind::TopologyTouchedBasis,
    ) == 0
    {
        return Err(MilestoneFiveQueryObligationSelectionCloseoutError::MissingTopologySelection);
    }
    if count_authority(
        closeouts,
        QueryObligationSelectionAuthorityKind::SpatialQueryDescriptor,
    ) == 0
    {
        return Err(MilestoneFiveQueryObligationSelectionCloseoutError::MissingSpatialSelection);
    }
    for closeout in closeouts {
        require_closeout_row_is_ready(closeout)?;
    }
    Ok(())
}

fn require_closeout_row_is_ready(
    closeout: &QuerySelectedGraphObligationCloseout,
) -> Result<(), MilestoneFiveQueryObligationSelectionCloseoutError> {
    if closeout.graph_read_access_planning_claimed() {
        return Err(
            MilestoneFiveQueryObligationSelectionCloseoutError::GraphReadAccessPlanningClaimed,
        );
    }
    if closeout.selector_precision_report().posture()
        == QuerySelectorPrecisionPosture::CounterEvidenceUnbounded
    {
        return Err(MilestoneFiveQueryObligationSelectionCloseoutError::UnboundedSelectorPrecision);
    }
    if !closeout.local_ceremony_closeout().is_clean() {
        return Err(MilestoneFiveQueryObligationSelectionCloseoutError::LocalCeremonyNotClean);
    }
    require_non_empty_digest(
        closeout.residue_manifest_digest(),
        MilestoneFiveQueryObligationSelectionCloseoutError::MissingResidueManifestDigest,
    )?;
    require_non_empty_digest(
        closeout.execution_proof_digest(),
        MilestoneFiveQueryObligationSelectionCloseoutError::MissingExecutionProofDigest,
    )?;
    require_non_empty_digest(
        closeout.adoption_manifest_digest(),
        MilestoneFiveQueryObligationSelectionCloseoutError::MissingAdoptionManifestDigest,
    )?;
    require_non_empty_digest(
        closeout.selector_precision_report().report_digest(),
        MilestoneFiveQueryObligationSelectionCloseoutError::MissingSelectorPrecisionDigest,
    )
}

fn require_non_empty_digest(
    digest: &str,
    error: MilestoneFiveQueryObligationSelectionCloseoutError,
) -> Result<(), MilestoneFiveQueryObligationSelectionCloseoutError> {
    if digest.is_empty() {
        return Err(error);
    }
    Ok(())
}

fn count_authority(
    closeouts: &[QuerySelectedGraphObligationCloseout],
    authority: QueryObligationSelectionAuthorityKind,
) -> usize {
    closeouts
        .iter()
        .filter(|closeout| closeout.authority_kind() == authority)
        .count()
}

fn selected_registration_digest_set(
    closeouts: &[QuerySelectedGraphObligationCloseout],
) -> BTreeSet<String> {
    closeouts
        .iter()
        .flat_map(QuerySelectedGraphObligationCloseout::selected_registration_digests)
        .cloned()
        .collect()
}
