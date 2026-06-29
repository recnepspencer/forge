use schema::facade::platform::authority::touched_graph_conflict::ConflictOverlapCategory;
use schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingPosture;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::spatial_lowering::{route_selection_context, selected_plan_denial};
use crate::workload_composition::conflict_input::AdmittedSpatialConflictInput;
use crate::workload_composition::conflict_plan::{
    ConflictPlanDownstreamProofCategory, ConflictPlanExecutionAdmission,
};
use worth_spatial::touched_graph_conflict::{
    SpatialConflictDiagnosticWitness, SpatialConflictFamilyCatalogCloseout,
    SpatialConflictFamilyIdentity, SpatialConflictPriorProofPosture,
    SpatialConflictSelectionProductPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialConflictPlanDenialKind {
    NoMatchingFamily,
    MissingRequiredPriorProof,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialConflictPlanDenial {
    pub(super) kind: SpatialConflictPlanDenialKind,
    pub(super) downstream_proof_category: ConflictPlanDownstreamProofCategory,
    pub(super) detail: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpatialConflictPlanCounters {
    declared_family_count: usize,
    selected_family_count: usize,
    unselected_family_count: usize,
    denied_family_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedSpatialConflictFamilyRow {
    identity: SpatialConflictFamilyIdentity,
    declaration_digest: String,
    routing_posture: ConflictRoutingPosture,
    prior_proof_posture: SpatialConflictPriorProofPosture,
    diagnostic_witness: SpatialConflictDiagnosticWitness,
    selection_product_posture: SpatialConflictSelectionProductPosture,
    downstream_proof_category: ConflictPlanDownstreamProofCategory,
}

#[derive(Clone)]
pub struct SelectedSpatialConflictPlan<'a> {
    authority:
        &'a worth_spatial::facade::workload_vocabulary::SpatialGeometryEvidenceTouchAuthority,
    overlap_category: ConflictOverlapCategory,
    overlap_identity_digest: String,
    locality_footprint_digest: String,
    prior_proof_posture: SpatialConflictPriorProofPosture,
    downstream_proof_category: ConflictPlanDownstreamProofCategory,
    admitted_input_digest: String,
    catalog_digest: String,
    selected_families: Vec<SelectedSpatialConflictFamilyRow>,
    unselected_family_identities: Vec<SpatialConflictFamilyIdentity>,
    counters: SpatialConflictPlanCounters,
    execution_admission: ConflictPlanExecutionAdmission,
    denial: Option<SpatialConflictPlanDenial>,
    selected_plan_digest: String,
}

pub fn lower_selected_spatial_conflict_plan<'a>(
    catalog_closeout: &SpatialConflictFamilyCatalogCloseout,
    admitted_input: &AdmittedSpatialConflictInput<'a>,
) -> SelectedSpatialConflictPlan<'a> {
    let selection = route_selection_context(catalog_closeout, admitted_input);
    let selected_families = canonical_selected_rows(
        &selection.matching_families,
        selection.downstream_proof_category,
    );
    let unselected_family_identities =
        canonical_unselected_family_identities(catalog_closeout, &selected_families);
    let denial = selected_plan_denial(
        selection.miss,
        selection.downstream_proof_category,
        selected_families.is_empty(),
    );
    let counters = SpatialConflictPlanCounters {
        declared_family_count: catalog_closeout.catalog().declarations().len(),
        selected_family_count: selected_families.len(),
        unselected_family_count: unselected_family_identities.len(),
        denied_family_count: usize::from(denial.is_some()),
    };
    let execution_admission = ConflictPlanExecutionAdmission::from_denial(denial.is_some());
    let selected_plan_digest = selected_plan_digest(
        catalog_closeout.catalog_digest(),
        admitted_input.admission_digest(),
        admitted_input.routing_contract().contract_digest(),
        admitted_input
            .routing_contract()
            .overlap_identity()
            .category(),
        selection.admitted_prior_proof,
        &selected_families,
        counters,
        execution_admission,
        selection.downstream_proof_category,
        denial.as_ref().map(|row| row.kind()),
    );

    SelectedSpatialConflictPlan {
        authority: admitted_input.authority(),
        overlap_category: admitted_input
            .routing_contract()
            .overlap_identity()
            .category(),
        overlap_identity_digest: admitted_input
            .routing_contract()
            .overlap_identity()
            .overlap_identity_digest()
            .to_string(),
        locality_footprint_digest: admitted_input
            .routing_contract()
            .overlap_identity()
            .locality_identity()
            .expect("spatial conflict input requires locality identity")
            .locality_identity_digest()
            .to_string(),
        prior_proof_posture: selection.admitted_prior_proof,
        downstream_proof_category: selection.downstream_proof_category,
        admitted_input_digest: admitted_input.admission_digest().to_string(),
        catalog_digest: catalog_closeout.catalog_digest().to_string(),
        selected_families,
        unselected_family_identities,
        counters,
        execution_admission,
        denial,
        selected_plan_digest,
    }
}

impl SpatialConflictPlanDenial {
    pub const fn kind(&self) -> SpatialConflictPlanDenialKind {
        self.kind
    }

    pub const fn downstream_proof_category(&self) -> ConflictPlanDownstreamProofCategory {
        self.downstream_proof_category
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl SpatialConflictPlanCounters {
    pub const fn declared_family_count(&self) -> usize {
        self.declared_family_count
    }

    pub const fn selected_family_count(&self) -> usize {
        self.selected_family_count
    }

    pub const fn unselected_family_count(&self) -> usize {
        self.unselected_family_count
    }

    pub const fn denied_family_count(&self) -> usize {
        self.denied_family_count
    }
}

impl SelectedSpatialConflictFamilyRow {
    pub const fn identity(&self) -> SpatialConflictFamilyIdentity {
        self.identity
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub const fn routing_posture(&self) -> ConflictRoutingPosture {
        self.routing_posture
    }

    pub const fn prior_proof_posture(&self) -> SpatialConflictPriorProofPosture {
        self.prior_proof_posture
    }

    pub const fn diagnostic_witness(&self) -> SpatialConflictDiagnosticWitness {
        self.diagnostic_witness
    }

    pub const fn selection_product_posture(&self) -> SpatialConflictSelectionProductPosture {
        self.selection_product_posture
    }

    pub const fn downstream_proof_category(&self) -> ConflictPlanDownstreamProofCategory {
        self.downstream_proof_category
    }
}

impl<'a> SelectedSpatialConflictPlan<'a> {
    pub const fn authority(
        &self,
    ) -> &'a worth_spatial::facade::workload_vocabulary::SpatialGeometryEvidenceTouchAuthority {
        self.authority
    }

    pub const fn overlap_category(&self) -> ConflictOverlapCategory {
        self.overlap_category
    }

    pub fn overlap_identity_digest(&self) -> &str {
        &self.overlap_identity_digest
    }

    pub fn locality_footprint_digest(&self) -> &str {
        &self.locality_footprint_digest
    }

    pub const fn prior_proof_posture(&self) -> SpatialConflictPriorProofPosture {
        self.prior_proof_posture
    }

    pub const fn downstream_proof_category(&self) -> ConflictPlanDownstreamProofCategory {
        self.downstream_proof_category
    }

    pub fn admitted_input_digest(&self) -> &str {
        &self.admitted_input_digest
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }

    pub fn selected_families(&self) -> &[SelectedSpatialConflictFamilyRow] {
        &self.selected_families
    }

    pub fn unselected_family_identities(&self) -> &[SpatialConflictFamilyIdentity] {
        &self.unselected_family_identities
    }

    pub const fn counters(&self) -> SpatialConflictPlanCounters {
        self.counters
    }

    pub const fn execution_admission(&self) -> ConflictPlanExecutionAdmission {
        self.execution_admission
    }

    pub fn denial(&self) -> Option<&SpatialConflictPlanDenial> {
        self.denial.as_ref()
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }
}

fn canonical_selected_rows(
    matching_families: &[&worth_spatial::touched_graph_conflict::SpatialConflictFamilyDeclaration],
    downstream_proof_category: ConflictPlanDownstreamProofCategory,
) -> Vec<SelectedSpatialConflictFamilyRow> {
    let mut rows = matching_families
        .iter()
        .map(|declaration| SelectedSpatialConflictFamilyRow {
            identity: declaration.identity(),
            declaration_digest: declaration.declaration_digest().to_string(),
            routing_posture: declaration.routing_posture(),
            prior_proof_posture: declaration.prior_proof_posture(),
            diagnostic_witness: declaration.diagnostic_witness(),
            selection_product_posture: declaration.selection_product_posture(),
            downstream_proof_category,
        })
        .collect::<Vec<_>>();
    rows.sort_by_key(|row| row.identity.as_str());
    rows
}

fn canonical_unselected_family_identities(
    catalog_closeout: &SpatialConflictFamilyCatalogCloseout,
    selected_families: &[SelectedSpatialConflictFamilyRow],
) -> Vec<SpatialConflictFamilyIdentity> {
    let selected = selected_families
        .iter()
        .map(|row| row.identity.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut unselected = catalog_closeout
        .catalog()
        .declarations()
        .iter()
        .map(|declaration| declaration.identity())
        .filter(|identity| !selected.contains(identity.as_str()))
        .collect::<Vec<_>>();
    unselected.sort_by_key(|identity| identity.as_str());
    unselected
}

fn selected_plan_digest(
    catalog_digest: &str,
    admitted_input_digest: &str,
    routing_contract_digest: &str,
    overlap_category: ConflictOverlapCategory,
    prior_proof_posture: SpatialConflictPriorProofPosture,
    selected_families: &[SelectedSpatialConflictFamilyRow],
    counters: SpatialConflictPlanCounters,
    execution_admission: ConflictPlanExecutionAdmission,
    downstream_proof_category: ConflictPlanDownstreamProofCategory,
    denial_kind: Option<SpatialConflictPlanDenialKind>,
) -> String {
    let route_posture = match prior_proof_posture {
        SpatialConflictPriorProofPosture::NoPriorProofRequired => "no-prior-proof",
        SpatialConflictPriorProofPosture::ReplayUndoOrTransactionRequired => {
            "replay-undo-or-transaction"
        }
    };
    let mut parts = vec![
        "worth-kernel:selected-spatial-conflict-plan:v1".to_string(),
        format!("catalog:{catalog_digest}"),
        format!("admitted:{admitted_input_digest}"),
        format!("overlap:{overlap_category:?}"),
        format!("prior-proof:{route_posture}"),
        format!("downstream:{}", downstream_proof_category.as_str()),
        format!("execution:{}", execution_admission.as_str()),
        format!("routing:{routing_contract_digest}"),
        format!("declared:{}", counters.declared_family_count),
        format!("selected:{}", counters.selected_family_count),
        format!("unselected:{}", counters.unselected_family_count),
        format!("denied:{}", counters.denied_family_count),
    ];
    if let Some(kind) = denial_kind {
        let denial = match kind {
            SpatialConflictPlanDenialKind::NoMatchingFamily => "no-matching-family",
            SpatialConflictPlanDenialKind::MissingRequiredPriorProof => {
                "missing-required-prior-proof"
            }
        };
        parts.push(format!("denial-kind:{denial}"));
    }
    parts.extend(
        selected_families
            .iter()
            .map(|row| format!("selected:{}", row.identity.as_str())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
