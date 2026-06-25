use super::catalog::WorthGraphReadDeclarationCatalog;
use super::catalog_summary::WorthGraphReadDeclarationCatalogSummary;
use super::errors::WorthGraphReadAccessDeclarationPhaseTwoError;
use super::phase_three_seed::WorthGraphReadAccessDeclarationPhaseThreeSeed;
use crate::graph_read_access_declarations::{
    WorthGraphReadAccessDeclarationPhaseOneCloseout, WorthGraphReadTouchedAuthorityLoweringSummary,
};
use crate::graph_read_access_inventory::WorthGraphReadDeletionLedgerItem;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessDeclarationPhaseTwoCloseout {
    declaration_catalog: WorthGraphReadDeclarationCatalog,
    catalog_summary: WorthGraphReadDeclarationCatalogSummary,
    lowering_summary: WorthGraphReadTouchedAuthorityLoweringSummary,
    deletion_items: Vec<WorthGraphReadDeletionLedgerItem>,
    phase_three_seed: WorthGraphReadAccessDeclarationPhaseThreeSeed,
}

pub fn current_worth_graph_read_access_declaration_catalog_closeout(
    phase_one: &WorthGraphReadAccessDeclarationPhaseOneCloseout,
) -> Result<
    WorthGraphReadAccessDeclarationPhaseTwoCloseout,
    WorthGraphReadAccessDeclarationPhaseTwoError,
> {
    let declaration_catalog =
        WorthGraphReadDeclarationCatalog::from_candidates(phase_one.declaration_candidates())?;
    let catalog_summary =
        WorthGraphReadDeclarationCatalogSummary::from_catalog(&declaration_catalog);
    let lowering_summary = WorthGraphReadTouchedAuthorityLoweringSummary::from_lowered_authorities(
        declaration_catalog
            .records()
            .iter()
            .map(|record| record.key().lowered_authority()),
    );
    let phase_three_seed = WorthGraphReadAccessDeclarationPhaseThreeSeed::from_catalog(
        &declaration_catalog,
        phase_one.deletion_items().to_vec(),
    );
    Ok(WorthGraphReadAccessDeclarationPhaseTwoCloseout {
        declaration_catalog,
        catalog_summary,
        lowering_summary,
        deletion_items: phase_one.deletion_items().to_vec(),
        phase_three_seed,
    })
}

impl WorthGraphReadAccessDeclarationPhaseTwoCloseout {
    pub fn declaration_catalog(&self) -> &WorthGraphReadDeclarationCatalog {
        &self.declaration_catalog
    }

    pub fn catalog_summary(&self) -> &WorthGraphReadDeclarationCatalogSummary {
        &self.catalog_summary
    }

    pub fn lowering_summary(&self) -> &WorthGraphReadTouchedAuthorityLoweringSummary {
        &self.lowering_summary
    }

    pub fn deletion_items(&self) -> &[WorthGraphReadDeletionLedgerItem] {
        &self.deletion_items
    }

    pub fn milestone_seven_phase_three_seed(
        &self,
    ) -> &WorthGraphReadAccessDeclarationPhaseThreeSeed {
        &self.phase_three_seed
    }

    pub const fn claims_execution_authority(&self) -> bool {
        false
    }

    pub const fn claims_admitted_access_plans_complete(&self) -> bool {
        false
    }

    pub const fn claims_graph_read_receipts_complete(&self) -> bool {
        false
    }
}
