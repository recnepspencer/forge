use schema::facade::platform::authority::touched_graph_conflict::ConflictOverlapCategory;
use schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingPosture;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::topology_lowering::{route_selection_context, selected_plan_denial};
use crate::workload_composition::conflict_input::AdmittedTopologyConflictInput;
use crate::workload_composition::conflict_plan::{
    ConflictPlanDownstreamProofCategory, ConflictPlanExecutionAdmission,
};
use topology::touched_graph_conflict::{
    TopologyConflictDiagnosticWitness, TopologyConflictFamilyCatalogCloseout,
    TopologyConflictFamilyIdentity, TopologyConflictPriorProofPosture,
    TopologyConflictSelectionProductPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyConflictPlanDenialKind {
    NoMatchingFamily,
    MissingRequiredPriorProof,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyConflictPlanDenial {
    pub(super) kind: TopologyConflictPlanDenialKind,
    pub(super) downstream_proof_category: ConflictPlanDownstreamProofCategory,
    pub(super) detail: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyConflictPlanCounters {
    declared_family_count: usize,
    selected_family_count: usize,
    unselected_family_count: usize,
    denied_family_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedTopologyConflictFamilyRow {
    identity: TopologyConflictFamilyIdentity,
    declaration_digest: String,
    routing_posture: ConflictRoutingPosture,
    prior_proof_posture: TopologyConflictPriorProofPosture,
    diagnostic_witness: TopologyConflictDiagnosticWitness,
    selection_product_posture: TopologyConflictSelectionProductPosture,
    downstream_proof_category: ConflictPlanDownstreamProofCategory,
}

#[derive(Clone)]
pub struct SelectedTopologyConflictPlan<'a> {
    touched_closure:
        &'a topology::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure,
    overlap_category: ConflictOverlapCategory,
    overlap_identity_digest: String,
    locality_footprint_digest: String,
    prior_proof_posture: TopologyConflictPriorProofPosture,
    downstream_proof_category: ConflictPlanDownstreamProofCategory,
    admitted_input_digest: String,
    catalog_digest: String,
    selected_families: Vec<SelectedTopologyConflictFamilyRow>,
    unselected_family_identities: Vec<TopologyConflictFamilyIdentity>,
    counters: TopologyConflictPlanCounters,
    execution_admission: ConflictPlanExecutionAdmission,
    denial: Option<TopologyConflictPlanDenial>,
    selected_plan_digest: String,
}

pub fn lower_selected_topology_conflict_plan<'a>(
    catalog_closeout: &TopologyConflictFamilyCatalogCloseout,
    admitted_input: &AdmittedTopologyConflictInput<'a>,
) -> SelectedTopologyConflictPlan<'a> {
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
    let counters = TopologyConflictPlanCounters {
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

    SelectedTopologyConflictPlan {
        touched_closure: admitted_input.touched_closure(),
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
            .expect("topology conflict input requires locality identity")
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

impl TopologyConflictPlanDenial {
    pub const fn kind(&self) -> TopologyConflictPlanDenialKind {
        self.kind
    }

    pub const fn downstream_proof_category(&self) -> ConflictPlanDownstreamProofCategory {
        self.downstream_proof_category
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl TopologyConflictPlanCounters {
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

impl SelectedTopologyConflictFamilyRow {
    pub const fn identity(&self) -> TopologyConflictFamilyIdentity {
        self.identity
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub const fn routing_posture(&self) -> ConflictRoutingPosture {
        self.routing_posture
    }

    pub const fn prior_proof_posture(&self) -> TopologyConflictPriorProofPosture {
        self.prior_proof_posture
    }

    pub const fn diagnostic_witness(&self) -> TopologyConflictDiagnosticWitness {
        self.diagnostic_witness
    }

    pub const fn selection_product_posture(&self) -> TopologyConflictSelectionProductPosture {
        self.selection_product_posture
    }

    pub const fn downstream_proof_category(&self) -> ConflictPlanDownstreamProofCategory {
        self.downstream_proof_category
    }
}

impl<'a> SelectedTopologyConflictPlan<'a> {
    pub const fn touched_closure(
        &self,
    ) -> &'a topology::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure {
        self.touched_closure
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

    pub const fn prior_proof_posture(&self) -> TopologyConflictPriorProofPosture {
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

    pub fn selected_families(&self) -> &[SelectedTopologyConflictFamilyRow] {
        &self.selected_families
    }

    pub fn unselected_family_identities(&self) -> &[TopologyConflictFamilyIdentity] {
        &self.unselected_family_identities
    }

    pub const fn counters(&self) -> TopologyConflictPlanCounters {
        self.counters
    }

    pub const fn execution_admission(&self) -> ConflictPlanExecutionAdmission {
        self.execution_admission
    }

    pub fn denial(&self) -> Option<&TopologyConflictPlanDenial> {
        self.denial.as_ref()
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }
}

fn canonical_selected_rows(
    matching_families: &[&topology::touched_graph_conflict::TopologyConflictFamilyDeclaration],
    downstream_proof_category: ConflictPlanDownstreamProofCategory,
) -> Vec<SelectedTopologyConflictFamilyRow> {
    let mut rows = matching_families
        .iter()
        .map(|declaration| SelectedTopologyConflictFamilyRow {
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
    catalog_closeout: &TopologyConflictFamilyCatalogCloseout,
    selected_families: &[SelectedTopologyConflictFamilyRow],
) -> Vec<TopologyConflictFamilyIdentity> {
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
    prior_proof_posture: TopologyConflictPriorProofPosture,
    selected_families: &[SelectedTopologyConflictFamilyRow],
    counters: TopologyConflictPlanCounters,
    execution_admission: ConflictPlanExecutionAdmission,
    downstream_proof_category: ConflictPlanDownstreamProofCategory,
    denial_kind: Option<TopologyConflictPlanDenialKind>,
) -> String {
    let route_posture = match prior_proof_posture {
        TopologyConflictPriorProofPosture::NoPriorProofRequired => "no-prior-proof",
        TopologyConflictPriorProofPosture::ReplayUndoOrTransactionRequired => {
            "replay-undo-or-transaction"
        }
    };
    let mut parts = vec![
        "worth-kernel:selected-topology-conflict-plan:v1".to_string(),
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
            TopologyConflictPlanDenialKind::NoMatchingFamily => "no-matching-family",
            TopologyConflictPlanDenialKind::MissingRequiredPriorProof => {
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
