use serde::Serialize;

use super::{
    DerivedInvalidationDensityPolicy, DerivedInvalidationExecutionAdmission,
    DerivedInvalidationLegalitySupportEvidence, DerivedInvalidationPhaseFourSeed,
    DerivedInvalidationQuerySupportEvidence, DerivedInvalidationSelectionCounters,
    DerivedInvalidationSelectionError, DerivedInvalidationTouchedClosure,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedInvalidationFamilyCatalogCloseout;
use crate::derived_topology::invalidation_plan::selection::row::{
    DerivedInvalidationDenialRow, DerivedInvalidationResidueRow, DerivedInvalidationSelectedRow,
    DerivedInvalidationUnaffectedRow,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationSelectedPlan {
    phase_three_seed_digest: String,
    catalog_digest: String,
    touched_closure_digest: String,
    query_support_digest: String,
    legality_support_digest: String,
    density_policy: DerivedInvalidationDensityPolicy,
    selected_rows: Vec<DerivedInvalidationSelectedRow>,
    unaffected_rows: Vec<DerivedInvalidationUnaffectedRow>,
    denied_rows: Vec<DerivedInvalidationDenialRow>,
    residue_rows: Vec<DerivedInvalidationResidueRow>,
    counters: DerivedInvalidationSelectionCounters,
    execution_admission: DerivedInvalidationExecutionAdmission,
    phase_four_seed: DerivedInvalidationPhaseFourSeed,
    selected_plan_digest: String,
}

impl DerivedInvalidationSelectedPlan {
    pub fn lower(
        catalog_closeout: &DerivedInvalidationFamilyCatalogCloseout,
        touched_closure: &DerivedInvalidationTouchedClosure,
        query_support: &DerivedInvalidationQuerySupportEvidence,
        legality_support: &DerivedInvalidationLegalitySupportEvidence,
        density_policy: DerivedInvalidationDensityPolicy,
    ) -> Result<Self, DerivedInvalidationSelectionError> {
        super::lowering::lower_selected_invalidation_plan(
            catalog_closeout,
            touched_closure,
            query_support,
            legality_support,
            density_policy,
        )
    }

    pub(super) fn from_parts(input: DerivedInvalidationSelectedPlanInput) -> Self {
        let execution_admission =
            DerivedInvalidationExecutionAdmission::from_denial_count(input.denied_rows.len());
        let selected_plan_digest = selected_plan_digest(&input, execution_admission);
        let phase_four_seed = DerivedInvalidationPhaseFourSeed::from_selected_plan(
            &selected_plan_digest,
            &input.touched_closure_digest,
            &input.query_support_digest,
            &input.legality_support_digest,
            input.selected_rows.len(),
            input.denied_rows.len(),
            input.unaffected_rows.len(),
        );
        Self {
            phase_three_seed_digest: input.phase_three_seed_digest,
            catalog_digest: input.catalog_digest,
            touched_closure_digest: input.touched_closure_digest,
            query_support_digest: input.query_support_digest,
            legality_support_digest: input.legality_support_digest,
            density_policy: input.density_policy,
            selected_rows: input.selected_rows,
            unaffected_rows: input.unaffected_rows,
            denied_rows: input.denied_rows,
            residue_rows: input.residue_rows,
            counters: input.counters,
            execution_admission,
            phase_four_seed,
            selected_plan_digest,
        }
    }

    pub fn phase_three_seed_digest(&self) -> &str {
        &self.phase_three_seed_digest
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }

    pub fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub fn legality_support_digest(&self) -> &str {
        &self.legality_support_digest
    }

    pub const fn density_policy(&self) -> DerivedInvalidationDensityPolicy {
        self.density_policy
    }

    pub fn selected_rows(&self) -> &[DerivedInvalidationSelectedRow] {
        &self.selected_rows
    }

    pub fn unaffected_rows(&self) -> &[DerivedInvalidationUnaffectedRow] {
        &self.unaffected_rows
    }

    pub fn denied_rows(&self) -> &[DerivedInvalidationDenialRow] {
        &self.denied_rows
    }

    pub fn residue_rows(&self) -> &[DerivedInvalidationResidueRow] {
        &self.residue_rows
    }

    pub const fn counters(&self) -> &DerivedInvalidationSelectionCounters {
        &self.counters
    }

    pub const fn execution_admission(&self) -> DerivedInvalidationExecutionAdmission {
        self.execution_admission
    }

    pub const fn phase_four_seed(&self) -> &DerivedInvalidationPhaseFourSeed {
        &self.phase_four_seed
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }
}

pub(super) struct DerivedInvalidationSelectedPlanInput {
    pub(super) phase_three_seed_digest: String,
    pub(super) catalog_digest: String,
    pub(super) touched_closure_digest: String,
    pub(super) query_support_digest: String,
    pub(super) legality_support_digest: String,
    pub(super) density_policy: DerivedInvalidationDensityPolicy,
    pub(super) selected_rows: Vec<DerivedInvalidationSelectedRow>,
    pub(super) unaffected_rows: Vec<DerivedInvalidationUnaffectedRow>,
    pub(super) denied_rows: Vec<DerivedInvalidationDenialRow>,
    pub(super) residue_rows: Vec<DerivedInvalidationResidueRow>,
    pub(super) counters: DerivedInvalidationSelectionCounters,
}

fn selected_plan_digest(
    input: &DerivedInvalidationSelectedPlanInput,
    execution_admission: DerivedInvalidationExecutionAdmission,
) -> String {
    let mut parts = vec![
        "worth-topo:derived-invalidation-selected-plan:v1".to_string(),
        format!("phase-three-seed:{}", input.phase_three_seed_digest),
        format!("catalog:{}", input.catalog_digest),
        format!("touched-closure:{}", input.touched_closure_digest),
        format!("query-support:{}", input.query_support_digest),
        format!("legality-support:{}", input.legality_support_digest),
        format!("density:{}", input.density_policy.as_str()),
        format!("admission:{}", execution_admission.as_str()),
        format!("counters:{}", input.counters.counters_digest()),
    ];
    parts.extend(
        input
            .selected_rows
            .iter()
            .map(|row| format!("selected:{}", row.row_digest())),
    );
    parts.extend(
        input
            .unaffected_rows
            .iter()
            .map(|row| format!("unaffected:{}", row.row_digest())),
    );
    parts.extend(
        input
            .denied_rows
            .iter()
            .map(|row| format!("denied:{}", row.denial_digest())),
    );
    parts.extend(
        input
            .residue_rows
            .iter()
            .map(|row| format!("residue:{}", row.row_digest())),
    );
    super::super::catalog::catalog_digest(parts)
}
