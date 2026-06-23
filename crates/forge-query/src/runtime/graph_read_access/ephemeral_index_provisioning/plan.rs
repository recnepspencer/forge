use super::ForgeQueryEphemeralGraphIndexScopeKind;
use crate::identity::hash_parts;
use crate::runtime::{
    ForgeQueryGraphReadAccessAdmission, ForgeQueryGraphReadAccessAdmissionPosture,
    ForgeQueryGraphReadAccessRequirementKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEphemeralGraphIndexAllocationRow {
    requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
    requirement_row_digest: String,
    estimated_bytes: usize,
}

impl ForgeQueryEphemeralGraphIndexAllocationRow {
    pub fn requirement_kind(&self) -> &ForgeQueryGraphReadAccessRequirementKind {
        &self.requirement_kind
    }

    pub fn requirement_row_digest(&self) -> &str {
        &self.requirement_row_digest
    }

    pub fn estimated_bytes(&self) -> usize {
        self.estimated_bytes
    }

    fn new(
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
        requirement_row_digest: impl Into<String>,
        estimated_bytes: usize,
    ) -> Self {
        Self {
            requirement_kind,
            requirement_row_digest: requirement_row_digest.into(),
            estimated_bytes,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "allocation:{}:{}:{}",
            self.requirement_kind.as_str(),
            self.requirement_row_digest,
            self.estimated_bytes
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEphemeralGraphIndexPlan {
    digest: String,
    admission_digest: String,
    requirement_set_digest: String,
    estimated_index_bytes: usize,
    admitted_byte_budget: usize,
    estimated_touched_nodes: usize,
    estimated_touched_edges: usize,
    required_scope_kind: ForgeQueryEphemeralGraphIndexScopeKind,
    allocation_rows: Vec<ForgeQueryEphemeralGraphIndexAllocationRow>,
}

impl ForgeQueryEphemeralGraphIndexPlan {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub fn requirement_set_digest(&self) -> &str {
        &self.requirement_set_digest
    }

    pub fn estimated_index_bytes(&self) -> usize {
        self.estimated_index_bytes
    }

    pub fn admitted_byte_budget(&self) -> usize {
        self.admitted_byte_budget
    }

    pub fn estimated_touched_nodes(&self) -> usize {
        self.estimated_touched_nodes
    }

    pub fn estimated_touched_edges(&self) -> usize {
        self.estimated_touched_edges
    }

    pub fn required_scope_kind(&self) -> &ForgeQueryEphemeralGraphIndexScopeKind {
        &self.required_scope_kind
    }

    pub fn required_allocations(&self) -> &[ForgeQueryEphemeralGraphIndexAllocationRow] {
        &self.allocation_rows
    }

    pub(crate) fn from_admission(admission: &ForgeQueryGraphReadAccessAdmission) -> Option<Self> {
        if admission.posture() != &ForgeQueryGraphReadAccessAdmissionPosture::BoundedEphemeralIndex
        {
            return None;
        }
        let estimated_index_bytes = admission.cost_estimate().supported().index_bytes();
        let allocation_source_rows = admission
            .graph_index_inventory_match_report()
            .matches()
            .iter()
            .filter(|row| {
                row.resolved_admission_posture()
                    == &ForgeQueryGraphReadAccessAdmissionPosture::BoundedEphemeralIndex
            })
            .collect::<Vec<_>>();
        let allocation_count = allocation_source_rows.len().max(1);
        let per_row_estimated_bytes = estimated_index_bytes.div_ceil(allocation_count);
        let allocation_rows = allocation_source_rows
            .into_iter()
            .map(|row| {
                ForgeQueryEphemeralGraphIndexAllocationRow::new(
                    row.requirement_kind().clone(),
                    row.requirement_row_digest(),
                    per_row_estimated_bytes,
                )
            })
            .collect::<Vec<_>>();
        let requirement_set_digest = admission.requirement_set().digest().as_str().to_string();
        Some(Self::new(
            admission.digest(),
            requirement_set_digest,
            estimated_index_bytes,
            admission.budget_check().max_inline_index_bytes(),
            admission.cost_estimate().intrinsic().candidate_roots()
                + admission.cost_estimate().intrinsic().frontier_breadth(),
            admission.cost_estimate().intrinsic().edge_touches(),
            allocation_rows,
        ))
    }

    fn new(
        admission_digest: impl Into<String>,
        requirement_set_digest: String,
        estimated_index_bytes: usize,
        admitted_byte_budget: usize,
        estimated_touched_nodes: usize,
        estimated_touched_edges: usize,
        allocation_rows: Vec<ForgeQueryEphemeralGraphIndexAllocationRow>,
    ) -> Self {
        let admission_digest = admission_digest.into();
        let required_scope_kind = ForgeQueryEphemeralGraphIndexScopeKind::ReadExecution;
        let digest = hash_parts(
            &[
                "forge_query_ephemeral_graph_index_plan_v1".to_string(),
                format!("admission:{admission_digest}"),
                format!("requirements:{requirement_set_digest}"),
                format!("estimated_index_bytes:{estimated_index_bytes}"),
                format!("admitted_byte_budget:{admitted_byte_budget}"),
                format!("estimated_touched_nodes:{estimated_touched_nodes}"),
                format!("estimated_touched_edges:{estimated_touched_edges}"),
                format!("scope:{}", required_scope_kind.as_str()),
            ]
            .into_iter()
            .chain(
                allocation_rows
                    .iter()
                    .map(ForgeQueryEphemeralGraphIndexAllocationRow::digest_part),
            )
            .collect::<Vec<_>>(),
        );
        Self {
            digest,
            admission_digest,
            requirement_set_digest,
            estimated_index_bytes,
            admitted_byte_budget,
            estimated_touched_nodes,
            estimated_touched_edges,
            required_scope_kind,
            allocation_rows,
        }
    }
}
