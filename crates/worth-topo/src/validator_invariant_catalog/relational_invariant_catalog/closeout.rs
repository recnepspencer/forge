use forge_query::facade::ForgeQueryGraphObligationKind;

use crate::validator_invariant_catalog::relational_invariant_catalog::{
    select_relational_invariant_family_rows, WorthTopologyRelationalInvariantCatalogCounters,
    WorthTopologyRelationalInvariantCatalogDenial,
    WorthTopologyRelationalInvariantCatalogDenialKind,
    WorthTopologyRelationalInvariantCatalogPhaseSixSeed,
    WorthTopologyRelationalInvariantCatalogSourceFirewallReport,
    WorthTopologyRelationalInvariantOldPackResidueReport,
    WorthTopologyRelationalInvariantOrdinaryAuthorityAdmission,
    WorthTopologyRelationalInvariantQueryRegistrationArtifactProjection,
    WorthTopologyRelationalInvariantQueryRegistrationBundle,
    WorthTopologySelectedRelationalInvariantFamilyRow,
};
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalog, WorthTopologyLegalityCatalogError,
    WorthTopologySelectedLegalityObligationPlan, WorthTopologySelectedLegalityObligationRow,
    WorthTopologySelectedValidatorEnforcementPhaseFiveSeed,
};

#[derive(Clone, Debug)]
pub struct WorthTopologyRelationalInvariantCatalogCloseout {
    catalog_digest: String,
    selected_plan_digest: String,
    selected_invariant_family_rows: Vec<WorthTopologySelectedRelationalInvariantFamilyRow>,
    selected_validator_family_rows: Vec<WorthTopologySelectedLegalityObligationRow>,
    ordinary_authority_admission: WorthTopologyRelationalInvariantOrdinaryAuthorityAdmission,
    query_registration_bundle: WorthTopologyRelationalInvariantQueryRegistrationBundle,
    query_registration_artifact:
        WorthTopologyRelationalInvariantQueryRegistrationArtifactProjection,
    old_pack_residue: WorthTopologyRelationalInvariantOldPackResidueReport,
    source_firewall: WorthTopologyRelationalInvariantCatalogSourceFirewallReport,
    counters: WorthTopologyRelationalInvariantCatalogCounters,
    phase_six_seed: WorthTopologyRelationalInvariantCatalogPhaseSixSeed,
    closeout_digest: String,
}

impl WorthTopologyRelationalInvariantCatalogCloseout {
    pub fn from_catalog_selected_plan_and_validator_seed(
        catalog: &WorthTopologyLegalityCatalog,
        selected_plan: &WorthTopologySelectedLegalityObligationPlan,
        validator_phase_five_seed: &WorthTopologySelectedValidatorEnforcementPhaseFiveSeed,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        validate_validator_seed_matches_plan(selected_plan, validator_phase_five_seed)?;
        let selected_invariant_family_rows =
            select_relational_invariant_family_rows(catalog, selected_plan);
        let selected_validator_family_rows = selected_validator_family_rows(selected_plan);
        let query_registration_artifact =
            WorthTopologyRelationalInvariantQueryRegistrationArtifactProjection::from_catalog(
                catalog,
            )?;
        let query_registration_bundle =
            WorthTopologyRelationalInvariantQueryRegistrationBundle::from_catalog(catalog)?;
        let ordinary_authority_admission =
            WorthTopologyRelationalInvariantOrdinaryAuthorityAdmission::from_query_registered_catalog(
                selected_plan,
                validator_phase_five_seed,
                &query_registration_bundle,
            )
            .map_err(WorthTopologyLegalityCatalogError::RelationalInvariantCatalog)?;
        let old_pack_residue =
            WorthTopologyRelationalInvariantOldPackResidueReport::from_current_sources()?;
        let source_firewall =
            WorthTopologyRelationalInvariantCatalogSourceFirewallReport::for_relational_invariant_catalog_lane()?;
        let counters = WorthTopologyRelationalInvariantCatalogCounters::from_parts(
            catalog.invariant_family_count(),
            selected_invariant_family_rows.len(),
            query_registration_bundle.graph_obligation_registration_count(),
            old_pack_residue.source_pack_registration_count(),
            old_pack_residue.ordinary_path_count(),
            source_firewall.violations().len(),
        );
        reject_invalid_closeout_state(
            catalog,
            selected_plan,
            selected_invariant_family_rows.len(),
            &old_pack_residue,
            &source_firewall,
        )?;
        let phase_six_seed = WorthTopologyRelationalInvariantCatalogPhaseSixSeed::from_parts(
            catalog.catalog_digest(),
            selected_plan.selected_plan_digest(),
            selected_plan.routing_closure_digest(),
            validator_phase_five_seed.seed_digest(),
            query_registration_artifact.projection_digest(),
            query_registration_bundle.bundle_digest(),
            ordinary_authority_admission.admission_digest(),
            old_pack_residue.report_digest(),
            source_firewall.report_digest(),
            counters.counters_digest(),
            selected_invariant_family_rows.len(),
            selected_validator_family_rows.len(),
            counters.execution_receipt_count(),
            selected_invariant_family_rows
                .iter()
                .map(|row| row.row_digest().to_string())
                .collect(),
            selected_validator_family_rows
                .iter()
                .map(|row| row.row_digest().to_string())
                .collect(),
            query_registration_bundle
                .graph_obligation_registrations()
                .iter()
                .map(|registration| registration.registration_digest().to_string())
                .collect(),
        );
        let closeout_digest = relational_invariant_closeout_digest(
            catalog.catalog_digest(),
            selected_plan.selected_plan_digest(),
            ordinary_authority_admission.admission_digest(),
            query_registration_bundle.bundle_digest(),
            query_registration_artifact.projection_digest(),
            old_pack_residue.report_digest(),
            source_firewall.report_digest(),
            counters.counters_digest(),
            phase_six_seed.seed_digest(),
            &selected_invariant_family_rows,
            &selected_validator_family_rows,
        );
        Ok(Self {
            catalog_digest: catalog.catalog_digest().to_string(),
            selected_plan_digest: selected_plan.selected_plan_digest().to_string(),
            selected_invariant_family_rows,
            selected_validator_family_rows,
            ordinary_authority_admission,
            query_registration_bundle,
            query_registration_artifact,
            old_pack_residue,
            source_firewall,
            counters,
            phase_six_seed,
            closeout_digest,
        })
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn selected_invariant_family_rows(
        &self,
    ) -> &[WorthTopologySelectedRelationalInvariantFamilyRow] {
        &self.selected_invariant_family_rows
    }

    pub fn selected_validator_family_rows(&self) -> &[WorthTopologySelectedLegalityObligationRow] {
        &self.selected_validator_family_rows
    }

    pub const fn ordinary_authority_admission(
        &self,
    ) -> &WorthTopologyRelationalInvariantOrdinaryAuthorityAdmission {
        &self.ordinary_authority_admission
    }

    pub const fn query_registration_bundle(
        &self,
    ) -> &WorthTopologyRelationalInvariantQueryRegistrationBundle {
        &self.query_registration_bundle
    }

    pub const fn query_registration_artifact(
        &self,
    ) -> &WorthTopologyRelationalInvariantQueryRegistrationArtifactProjection {
        &self.query_registration_artifact
    }

    pub const fn old_pack_residue(&self) -> &WorthTopologyRelationalInvariantOldPackResidueReport {
        &self.old_pack_residue
    }

    pub const fn source_firewall(
        &self,
    ) -> &WorthTopologyRelationalInvariantCatalogSourceFirewallReport {
        &self.source_firewall
    }

    pub const fn counters(&self) -> &WorthTopologyRelationalInvariantCatalogCounters {
        &self.counters
    }

    pub const fn phase_six_seed(&self) -> &WorthTopologyRelationalInvariantCatalogPhaseSixSeed {
        &self.phase_six_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub const fn claims_invariant_execution_receipts(&self) -> bool {
        false
    }
}

fn selected_validator_family_rows(
    selected_plan: &WorthTopologySelectedLegalityObligationPlan,
) -> Vec<WorthTopologySelectedLegalityObligationRow> {
    selected_plan
        .selected_obligation_rows()
        .iter()
        .filter(|row| {
            row.query_obligation_kind() == ForgeQueryGraphObligationKind::SchemaContractValidator
        })
        .cloned()
        .collect()
}

fn validate_validator_seed_matches_plan(
    selected_plan: &WorthTopologySelectedLegalityObligationPlan,
    validator_phase_five_seed: &WorthTopologySelectedValidatorEnforcementPhaseFiveSeed,
) -> Result<(), WorthTopologyLegalityCatalogError> {
    if validator_phase_five_seed.selected_plan_digest() != selected_plan.selected_plan_digest() {
        return Err(WorthTopologyLegalityCatalogError::RelationalInvariantCatalog(
            WorthTopologyRelationalInvariantCatalogDenial::new(
                WorthTopologyRelationalInvariantCatalogDenialKind::ValidatorSeedMismatch,
                validator_phase_five_seed.seed_digest(),
                "Phase 5 invariant catalog must consume the same selected plan as Phase 4 validator enforcement",
            ),
        ));
    }
    Ok(())
}

fn reject_invalid_closeout_state(
    catalog: &WorthTopologyLegalityCatalog,
    selected_plan: &WorthTopologySelectedLegalityObligationPlan,
    selected_invariant_family_count: usize,
    old_pack_residue: &WorthTopologyRelationalInvariantOldPackResidueReport,
    source_firewall: &WorthTopologyRelationalInvariantCatalogSourceFirewallReport,
) -> Result<(), WorthTopologyLegalityCatalogError> {
    if catalog.invariant_family_count() == 0 {
        return Err(relational_denial(
            WorthTopologyRelationalInvariantCatalogDenialKind::NoInvariantFamilies,
            catalog.catalog_digest(),
            "Phase 5 cannot close without Phase 2 invariant family records",
        ));
    }
    if selected_invariant_family_count == 0 {
        return Err(relational_denial(
            WorthTopologyRelationalInvariantCatalogDenialKind::NoSelectedInvariantFamilies,
            selected_plan.selected_plan_digest(),
            "Phase 5 requires a Query-selected touched obligation plan before handoff",
        ));
    }
    if old_pack_residue.ordinary_path_count() != 0 {
        return Err(relational_denial(
            WorthTopologyRelationalInvariantCatalogDenialKind::OldPackOrdinaryPathResidue,
            old_pack_residue.report_digest(),
            "old milestone_one invariant pack may remain only as certified source intake",
        ));
    }
    if !source_firewall.violations().is_empty() {
        return Err(relational_denial(
            WorthTopologyRelationalInvariantCatalogDenialKind::SourceFirewallViolation,
            source_firewall.report_digest(),
            "relational invariant catalog lane contains forbidden old runtime authority",
        ));
    }
    Ok(())
}

fn relational_denial(
    kind: WorthTopologyRelationalInvariantCatalogDenialKind,
    subject_digest: &str,
    detail: &str,
) -> WorthTopologyLegalityCatalogError {
    WorthTopologyLegalityCatalogError::RelationalInvariantCatalog(
        WorthTopologyRelationalInvariantCatalogDenial::new(kind, subject_digest, detail),
    )
}

fn relational_invariant_closeout_digest(
    catalog_digest: &str,
    selected_plan_digest: &str,
    ordinary_authority_admission_digest: &str,
    query_registration_bundle_digest: &str,
    query_registration_artifact_digest: &str,
    old_pack_residue_digest: &str,
    source_firewall_digest: &str,
    counters_digest: &str,
    phase_six_seed_digest: &str,
    selected_invariant_rows: &[WorthTopologySelectedRelationalInvariantFamilyRow],
    selected_validator_rows: &[WorthTopologySelectedLegalityObligationRow],
) -> String {
    let mut parts = vec![
        "worth-topo-relational-invariant-catalog-closeout-v1".to_string(),
        format!("catalog:{catalog_digest}"),
        format!("selected-plan:{selected_plan_digest}"),
        format!("ordinary-authority-admission:{ordinary_authority_admission_digest}"),
        format!("query-registration-bundle:{query_registration_bundle_digest}"),
        format!("query-registration-artifact:{query_registration_artifact_digest}"),
        format!("old-pack-residue:{old_pack_residue_digest}"),
        format!("source-firewall:{source_firewall_digest}"),
        format!("counters:{counters_digest}"),
        format!("phase-six-seed:{phase_six_seed_digest}"),
    ];
    parts.extend(
        selected_invariant_rows
            .iter()
            .map(|row| format!("selected-invariant:{}", row.row_digest())),
    );
    parts.extend(
        selected_validator_rows
            .iter()
            .map(|row| format!("selected-validator:{}", row.row_digest())),
    );
    parts.join("|")
}
