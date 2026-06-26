use std::collections::BTreeSet;

use forge_query::facade::ForgeQueryGraphObligationSupportStatus;

use crate::validation_authority_inventory::{
    WorthValidationAuthorityInventory, WorthValidationAuthorityMilestoneEightSeedSummary,
};
use crate::validator_invariant_catalog::family_record::WorthTopologyLegalityFamilyRecordCounters;
#[cfg(test)]
use crate::validator_invariant_catalog::family_record::WorthTopologyLegalityFamilyRecordInput;
use crate::validator_invariant_catalog::query_lowering::WorthTopologyQueryGraphObligationCatalogProjection;
use crate::validator_invariant_catalog::source_catalog::{
    current_invariant_family_inputs, current_validator_family_inputs,
    WorthTopologyLegalityFamilySourceProof,
};
use crate::validator_invariant_catalog::{
    WorthTopologyInvariantFamilyRecord, WorthTopologyLegalityCatalogError,
    WorthTopologyLegalityFamilyRecord, WorthTopologyValidatorFamilyRecord,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthTopologyLegalityCatalog {
    records: Vec<WorthTopologyLegalityFamilyRecord>,
    source_proofs: Vec<WorthTopologyLegalityFamilySourceProof>,
    query_projection: WorthTopologyQueryGraphObligationCatalogProjection,
    counters: WorthTopologyLegalityFamilyRecordCounters,
    catalog_digest: String,
}

impl WorthTopologyLegalityCatalog {
    pub fn from_phase_one_inventory_and_milestone_eight_summary(
        phase_one_inventory: &WorthValidationAuthorityInventory,
        milestone_eight_summary: &WorthValidationAuthorityMilestoneEightSeedSummary,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        validate_phase_two_handoff_authority(phase_one_inventory, milestone_eight_summary)?;
        Self::from_milestone_eight_posture_digest(milestone_eight_summary.seed_digest())
    }

    pub(super) fn from_milestone_eight_posture_digest(
        milestone_eight_posture_digest: &str,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let (records, source_proofs) =
            collect_family_records_from_current_sources(milestone_eight_posture_digest)?;
        reject_conflicting_family_identities(&records)?;
        let counters = counters_from_records(&records);
        let query_projection =
            WorthTopologyQueryGraphObligationCatalogProjection::from_family_records(&records)?;
        let catalog_digest = catalog_digest(
            &records,
            &source_proofs,
            query_projection.projection_digest(),
        );
        Ok(Self {
            records,
            source_proofs,
            query_projection,
            counters,
            catalog_digest,
        })
    }

    #[cfg(test)]
    pub(crate) fn from_test_family_records(
        records: Vec<WorthTopologyLegalityFamilyRecord>,
        source_proofs: Vec<WorthTopologyLegalityFamilySourceProof>,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        reject_conflicting_family_identities(&records)?;
        let counters = counters_from_records(&records);
        let query_projection =
            WorthTopologyQueryGraphObligationCatalogProjection::from_family_records(&records)?;
        let catalog_digest = catalog_digest(
            &records,
            &source_proofs,
            query_projection.projection_digest(),
        );
        Ok(Self {
            records,
            source_proofs,
            query_projection,
            counters,
            catalog_digest,
        })
    }

    #[cfg(test)]
    pub(in crate::validator_invariant_catalog) fn validator_record_from_input_for_tests(
        input: WorthTopologyLegalityFamilyRecordInput<
            crate::validator_invariant_catalog::WorthTopologyValidatorFamilyIdentity,
        >,
    ) -> Result<WorthTopologyValidatorFamilyRecord, WorthTopologyLegalityCatalogError> {
        WorthTopologyValidatorFamilyRecord::from_input(input)
    }

    #[cfg(test)]
    pub(in crate::validator_invariant_catalog) fn invariant_record_from_input_for_tests(
        input: WorthTopologyLegalityFamilyRecordInput<
            crate::validator_invariant_catalog::WorthTopologyInvariantFamilyIdentity,
        >,
    ) -> Result<WorthTopologyInvariantFamilyRecord, WorthTopologyLegalityCatalogError> {
        WorthTopologyInvariantFamilyRecord::from_input(input)
    }

    pub fn records(&self) -> &[WorthTopologyLegalityFamilyRecord] {
        &self.records
    }

    pub fn source_proofs(&self) -> &[WorthTopologyLegalityFamilySourceProof] {
        &self.source_proofs
    }

    pub const fn query_projection(&self) -> &WorthTopologyQueryGraphObligationCatalogProjection {
        &self.query_projection
    }

    pub const fn validator_family_count(&self) -> usize {
        self.counters.validator_family_count
    }

    pub const fn invariant_family_count(&self) -> usize {
        self.counters.invariant_family_count
    }

    pub const fn supported_family_count(&self) -> usize {
        self.counters.supported_family_count
    }

    pub const fn unsupported_family_count(&self) -> usize {
        self.counters.unsupported_family_count
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }
}

fn validate_phase_two_handoff_authority(
    phase_one_inventory: &WorthValidationAuthorityInventory,
    milestone_eight_summary: &WorthValidationAuthorityMilestoneEightSeedSummary,
) -> Result<(), WorthTopologyLegalityCatalogError> {
    if !phase_one_inventory
        .cut_line()
        .ready_for_parallel_catalog_lane()
    {
        return Err(WorthTopologyLegalityCatalogError::MissingMilestoneEightReceiptContext);
    }
    if milestone_eight_summary.claims_validator_selection() {
        return Err(
            WorthTopologyLegalityCatalogError::MilestoneEightSeedClaimsValidatorSelection(
                milestone_eight_summary.seed_digest().to_string(),
            ),
        );
    }
    if !milestone_eight_summary.receipt_context_present()
        || !milestone_eight_summary.posture_context_present()
    {
        return Err(WorthTopologyLegalityCatalogError::MissingMilestoneEightReceiptContext);
    }
    Ok(())
}

fn collect_family_records_from_current_sources(
    milestone_eight_posture_digest: &str,
) -> Result<
    (
        Vec<WorthTopologyLegalityFamilyRecord>,
        Vec<WorthTopologyLegalityFamilySourceProof>,
    ),
    WorthTopologyLegalityCatalogError,
> {
    let mut records = Vec::new();
    let mut source_proofs = Vec::new();
    collect_validator_family_records(
        milestone_eight_posture_digest,
        &mut records,
        &mut source_proofs,
    )?;
    collect_invariant_family_records(
        milestone_eight_posture_digest,
        &mut records,
        &mut source_proofs,
    )?;
    Ok((records, source_proofs))
}

fn collect_validator_family_records(
    milestone_eight_posture_digest: &str,
    records: &mut Vec<WorthTopologyLegalityFamilyRecord>,
    source_proofs: &mut Vec<WorthTopologyLegalityFamilySourceProof>,
) -> Result<(), WorthTopologyLegalityCatalogError> {
    for row in current_validator_family_inputs(milestone_eight_posture_digest)? {
        source_proofs.push(row.source_proof);
        records.push(WorthTopologyLegalityFamilyRecord::Validator(
            WorthTopologyValidatorFamilyRecord::from_input(row.input)?,
        ));
    }
    Ok(())
}

fn collect_invariant_family_records(
    milestone_eight_posture_digest: &str,
    records: &mut Vec<WorthTopologyLegalityFamilyRecord>,
    source_proofs: &mut Vec<WorthTopologyLegalityFamilySourceProof>,
) -> Result<(), WorthTopologyLegalityCatalogError> {
    for row in current_invariant_family_inputs(milestone_eight_posture_digest)? {
        source_proofs.push(row.source_proof);
        records.push(WorthTopologyLegalityFamilyRecord::Invariant(
            WorthTopologyInvariantFamilyRecord::from_input(row.input)?,
        ));
    }
    Ok(())
}

fn reject_conflicting_family_identities(
    records: &[WorthTopologyLegalityFamilyRecord],
) -> Result<(), WorthTopologyLegalityCatalogError> {
    let mut identities = BTreeSet::new();
    for record in records {
        let identity = record.identity();
        if !identities.insert(identity.identity_digest().to_string()) {
            return Err(
                WorthTopologyLegalityCatalogError::ConflictingFamilyIdentity(
                    identity.stable_key().to_string(),
                ),
            );
        }
    }
    Ok(())
}

fn counters_from_records(
    records: &[WorthTopologyLegalityFamilyRecord],
) -> WorthTopologyLegalityFamilyRecordCounters {
    let mut counters = WorthTopologyLegalityFamilyRecordCounters::default();
    for record in records {
        match record {
            WorthTopologyLegalityFamilyRecord::Validator(_) => counters.validator_family_count += 1,
            WorthTopologyLegalityFamilyRecord::Invariant(_) => counters.invariant_family_count += 1,
        }
        match record.query_support_posture().status() {
            ForgeQueryGraphObligationSupportStatus::Supported => {
                counters.supported_family_count += 1
            }
            ForgeQueryGraphObligationSupportStatus::Unsupported
            | ForgeQueryGraphObligationSupportStatus::NotApplicable
            | ForgeQueryGraphObligationSupportStatus::DiagnosticOnly
            | ForgeQueryGraphObligationSupportStatus::DeferredToBackstop => {
                counters.unsupported_family_count += 1
            }
        }
    }
    counters
}

fn catalog_digest(
    records: &[WorthTopologyLegalityFamilyRecord],
    source_proofs: &[WorthTopologyLegalityFamilySourceProof],
    query_projection_digest: &str,
) -> String {
    let mut parts = vec![
        "worth-topo-legality-catalog-v1".to_string(),
        format!("query:{query_projection_digest}"),
    ];
    parts.extend(
        records
            .iter()
            .map(|record| format!("family:{}", record.family_digest())),
    );
    parts.extend(
        source_proofs
            .iter()
            .map(|proof| format!("source:{}", proof.proof_digest())),
    );
    parts.join("|")
}
