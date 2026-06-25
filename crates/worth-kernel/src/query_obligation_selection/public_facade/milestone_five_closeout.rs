use super::selected_closeout::WorthQuerySelectedGraphObligationCloseout;
use crate::query_obligation_selection::closeout::{
    MilestoneFiveQueryObligationSelectionCloseout,
    MilestoneFiveQueryObligationSelectionCloseoutError, MilestoneSixGraphReadInventorySeed,
};

#[derive(Clone, Debug)]
pub struct WorthQueryObligationSelectionMilestoneFiveCloseout {
    closeout: MilestoneFiveQueryObligationSelectionCloseout,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryObligationSelectionMilestoneSixSeed {
    seed: MilestoneSixGraphReadInventorySeed,
}

pub type WorthQueryObligationSelectionMilestoneFiveCloseoutError =
    MilestoneFiveQueryObligationSelectionCloseoutError;

impl WorthQueryObligationSelectionMilestoneFiveCloseout {
    pub fn from_selected_closeouts<I>(
        closeouts: I,
    ) -> Result<Self, WorthQueryObligationSelectionMilestoneFiveCloseoutError>
    where
        I: IntoIterator<Item = WorthQuerySelectedGraphObligationCloseout>,
    {
        let internal_closeouts = closeouts
            .into_iter()
            .map(WorthQuerySelectedGraphObligationCloseout::into_closeout);
        Ok(Self {
            closeout: MilestoneFiveQueryObligationSelectionCloseout::from_selected_closeouts(
                internal_closeouts,
            )?,
        })
    }

    pub const fn is_closed(&self) -> bool {
        self.closeout.is_closed()
    }

    pub const fn selected_obligation_count(&self) -> usize {
        self.closeout.selected_obligation_count()
    }

    pub const fn execution_row_count(&self) -> usize {
        self.closeout.execution_row_count()
    }

    pub const fn selected_registration_count(&self) -> usize {
        self.closeout.selected_registration_count()
    }

    pub const fn topology_selected_count(&self) -> usize {
        self.closeout.topology_selected_count()
    }

    pub const fn spatial_selected_count(&self) -> usize {
        self.closeout.spatial_selected_count()
    }

    pub const fn broad_selector_residue_count(&self) -> usize {
        self.closeout.broad_selector_residue_count()
    }

    pub const fn query_selector_gap_count(&self) -> usize {
        self.closeout.query_selector_gap_count()
    }

    pub const fn graph_read_access_planning_claimed(&self) -> bool {
        self.closeout.graph_read_access_planning_claimed()
    }

    pub fn authority_digests(&self) -> &[String] {
        self.closeout.authority_digests()
    }

    pub fn touch_descriptor_digests(&self) -> &[String] {
        self.closeout.touch_descriptor_digests()
    }

    pub fn selected_registration_digests(&self) -> &[String] {
        self.closeout.selected_registration_digests()
    }

    pub const fn open_finding_count(&self) -> usize {
        self.closeout.open_finding_count()
    }

    pub const fn topology_lane_count(&self) -> usize {
        self.closeout.closeout_report().topology_lane_count()
    }

    pub const fn spatial_lane_count(&self) -> usize {
        self.closeout.closeout_report().spatial_lane_count()
    }

    pub const fn capped_broad_selector_residue_count(&self) -> usize {
        self.closeout
            .closeout_report()
            .capped_broad_selector_residue_count()
    }

    pub const fn uncapped_broad_selector_residue_count(&self) -> usize {
        self.closeout
            .closeout_report()
            .uncapped_broad_selector_residue_count()
    }

    pub const fn owned_query_gap_count(&self) -> usize {
        self.closeout.closeout_report().owned_query_gap_count()
    }

    pub const fn incomplete_query_gap_count(&self) -> usize {
        self.closeout.closeout_report().incomplete_query_gap_count()
    }

    pub const fn graph_read_access_planning_claimed_count(&self) -> usize {
        self.closeout
            .closeout_report()
            .graph_read_access_planning_claimed_count()
    }

    pub fn into_graph_read_inventory_seed(self) -> WorthQueryObligationSelectionMilestoneSixSeed {
        WorthQueryObligationSelectionMilestoneSixSeed {
            seed: self.closeout.into_graph_read_inventory_seed(),
        }
    }
}

impl WorthQueryObligationSelectionMilestoneSixSeed {
    pub const fn selected_obligation_count(&self) -> usize {
        self.seed.selected_obligation_count()
    }

    pub const fn selected_registration_count(&self) -> usize {
        self.seed.selected_registration_count()
    }

    pub const fn execution_row_count(&self) -> usize {
        self.seed.execution_row_count()
    }

    pub const fn requires_graph_read_access_planning(&self) -> bool {
        self.seed.requires_graph_read_access_planning()
    }

    pub const fn graph_read_access_planning_claimed(&self) -> bool {
        self.seed.graph_read_access_planning_claimed()
    }

    pub fn authority_digests(&self) -> &[String] {
        self.seed.authority_digests()
    }

    pub fn touch_descriptor_digests(&self) -> &[String] {
        self.seed.touch_descriptor_digests()
    }

    pub fn selected_registration_digests(&self) -> &[String] {
        self.seed.selected_registration_digests()
    }

    pub fn residue_manifest_digests(&self) -> &[String] {
        self.seed.residue_manifest_digests()
    }

    pub fn execution_proof_digests(&self) -> &[String] {
        self.seed.execution_proof_digests()
    }

    pub fn adoption_manifest_digests(&self) -> &[String] {
        self.seed.adoption_manifest_digests()
    }

    pub fn selector_precision_report_digests(&self) -> &[String] {
        self.seed.selector_precision_report_digests()
    }
}
