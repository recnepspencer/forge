use crate::validation_authority_inventory::{
    WorthValidationAuthorityInventory, WorthValidationAuthorityInventoryInput,
    WorthValidationAuthorityMilestoneEightSeedSummary,
};
use crate::validator_invariant_catalog::no_execution_proof::WorthTopologyLegalityCatalogNoExecutionProof;
use crate::validator_invariant_catalog::phase_two_seed::{
    WorthTopologyLegalityCatalogPhaseThreeSeed, WorthTopologyLegalityCatalogPhaseThreeSeedInput,
};
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalog, WorthTopologyLegalityCatalogError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthTopologyLegalityCatalogCloseout {
    catalog: WorthTopologyLegalityCatalog,
    no_execution_proof: WorthTopologyLegalityCatalogNoExecutionProof,
    phase_three_seed: WorthTopologyLegalityCatalogPhaseThreeSeed,
    closeout_digest: String,
}

pub fn current_worth_topology_legality_catalog_closeout(
) -> Result<WorthTopologyLegalityCatalogCloseout, WorthTopologyLegalityCatalogError> {
    let milestone_eight_summary =
        WorthValidationAuthorityMilestoneEightSeedSummary::current_imported_public_closeout();
    let phase_one_inventory = WorthValidationAuthorityInventory::from_current_sources_with_input(
        WorthValidationAuthorityInventoryInput::from_milestone_eight_seed_summary(
            milestone_eight_summary.clone(),
        ),
    )
    .map_err(|error| WorthTopologyLegalityCatalogError::QueryRegistration(error.to_string()))?;
    WorthTopologyLegalityCatalogCloseout::from_phase_one_inventory_and_milestone_eight_summary(
        &phase_one_inventory,
        phase_one_inventory
            .milestone_eight_seed_summary()
            .unwrap_or(&milestone_eight_summary),
    )
}

impl WorthTopologyLegalityCatalogCloseout {
    pub fn from_phase_one_inventory_and_milestone_eight_summary(
        phase_one_inventory: &WorthValidationAuthorityInventory,
        milestone_eight_summary: &WorthValidationAuthorityMilestoneEightSeedSummary,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let catalog =
            WorthTopologyLegalityCatalog::from_phase_one_inventory_and_milestone_eight_summary(
                phase_one_inventory,
                milestone_eight_summary,
            )?;
        let no_execution_proof =
            WorthTopologyLegalityCatalogNoExecutionProof::phase_two_catalog_only();
        let phase_three_seed = WorthTopologyLegalityCatalogPhaseThreeSeed::from_input(
            WorthTopologyLegalityCatalogPhaseThreeSeedInput {
                catalog_digest: catalog.catalog_digest().to_string(),
                query_registration_catalog_digest: catalog
                    .query_projection()
                    .query_catalog()
                    .catalog_digest()
                    .to_string(),
                validator_family_count: catalog.validator_family_count(),
                invariant_family_count: catalog.invariant_family_count(),
                supported_family_count: catalog.supported_family_count(),
                unsupported_family_count: catalog.unsupported_family_count(),
                no_execution_proof_digest: no_execution_proof.proof_digest().to_string(),
            },
        );
        let closeout_digest = format!(
            "worth-topo-legality-catalog-closeout:{}:{}",
            catalog.catalog_digest(),
            phase_three_seed.seed_digest()
        );
        Ok(Self {
            catalog,
            no_execution_proof,
            phase_three_seed,
            closeout_digest,
        })
    }

    #[cfg(test)]
    pub(crate) fn from_test_catalog(catalog: WorthTopologyLegalityCatalog) -> Self {
        let no_execution_proof =
            WorthTopologyLegalityCatalogNoExecutionProof::phase_two_catalog_only();
        let phase_three_seed = WorthTopologyLegalityCatalogPhaseThreeSeed::from_input(
            WorthTopologyLegalityCatalogPhaseThreeSeedInput {
                catalog_digest: catalog.catalog_digest().to_string(),
                query_registration_catalog_digest: catalog
                    .query_projection()
                    .query_catalog()
                    .catalog_digest()
                    .to_string(),
                validator_family_count: catalog.validator_family_count(),
                invariant_family_count: catalog.invariant_family_count(),
                supported_family_count: catalog.supported_family_count(),
                unsupported_family_count: catalog.unsupported_family_count(),
                no_execution_proof_digest: no_execution_proof.proof_digest().to_string(),
            },
        );
        let closeout_digest = format!(
            "worth-topo-legality-test-catalog-closeout:{}:{}",
            catalog.catalog_digest(),
            phase_three_seed.seed_digest()
        );
        Self {
            catalog,
            no_execution_proof,
            phase_three_seed,
            closeout_digest,
        }
    }

    pub const fn catalog(&self) -> &WorthTopologyLegalityCatalog {
        &self.catalog
    }

    pub const fn phase_three_seed(&self) -> &WorthTopologyLegalityCatalogPhaseThreeSeed {
        &self.phase_three_seed
    }

    pub const fn no_execution_proof(&self) -> &WorthTopologyLegalityCatalogNoExecutionProof {
        &self.no_execution_proof
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub const fn claims_selected_obligations(&self) -> bool {
        self.no_execution_proof.claims_selected_obligations()
    }

    pub const fn claims_enforcement_receipts(&self) -> bool {
        self.no_execution_proof.claims_enforcement_receipts()
    }
}
