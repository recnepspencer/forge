use super::{ForgeQueryEphemeralGraphIndexPlan, ForgeQueryEphemeralGraphIndexScope};
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEphemeralGraphIndex {
    index_digest: String,
    plan_digest: String,
    scope_digest: String,
    rebuild_basis_digest: String,
    allocated_bytes: usize,
    allocation_row_count: usize,
    touched_node_count: usize,
    touched_edge_count: usize,
}

impl ForgeQueryEphemeralGraphIndex {
    pub fn index_digest(&self) -> &str {
        &self.index_digest
    }

    pub fn plan_digest(&self) -> &str {
        &self.plan_digest
    }

    pub fn scope_digest(&self) -> &str {
        &self.scope_digest
    }

    pub fn rebuild_basis_digest(&self) -> &str {
        &self.rebuild_basis_digest
    }

    pub fn allocated_bytes(&self) -> usize {
        self.allocated_bytes
    }

    pub fn allocation_row_count(&self) -> usize {
        self.allocation_row_count
    }

    pub fn touched_node_count(&self) -> usize {
        self.touched_node_count
    }

    pub fn touched_edge_count(&self) -> usize {
        self.touched_edge_count
    }

    pub(in crate::runtime::graph_read_access::ephemeral_index_provisioning) fn build_from_plan_and_scope(
        plan: &ForgeQueryEphemeralGraphIndexPlan,
        scope: &ForgeQueryEphemeralGraphIndexScope,
    ) -> Self {
        let plan_digest = plan.digest().to_string();
        let scope_digest = scope.digest().to_string();
        let rebuild_basis_digest = hash_parts(&[
            "forge_query_ephemeral_graph_index_rebuild_basis_v1".to_string(),
            format!("requirements:{}", plan.requirement_set_digest()),
            format!("scope:{scope_digest}"),
        ]);
        let allocated_bytes = observed_allocated_bytes(plan);
        let allocation_row_count = plan.required_allocations().len();
        let touched_node_count = plan.estimated_touched_nodes();
        let touched_edge_count = plan.estimated_touched_edges();
        let index_digest = hash_parts(&[
            "forge_query_ephemeral_graph_index_v1".to_string(),
            format!("plan:{plan_digest}"),
            format!("scope:{scope_digest}"),
            format!("rebuild_basis:{rebuild_basis_digest}"),
            format!("allocated:{allocated_bytes}"),
            format!("allocation_rows:{allocation_row_count}"),
            format!("touched_nodes:{touched_node_count}"),
            format!("touched_edges:{touched_edge_count}"),
        ]);
        Self {
            index_digest,
            plan_digest,
            scope_digest,
            rebuild_basis_digest,
            allocated_bytes,
            allocation_row_count,
            touched_node_count,
            touched_edge_count,
        }
    }
}

fn observed_allocated_bytes(plan: &ForgeQueryEphemeralGraphIndexPlan) -> usize {
    let allocation_row_bytes = plan
        .required_allocations()
        .iter()
        .map(|row| row.estimated_bytes())
        .sum::<usize>();
    if allocation_row_bytes == 0 {
        plan.estimated_index_bytes()
    } else {
        allocation_row_bytes
    }
}
