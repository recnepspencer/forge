use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimePerformanceCounters {
    crossing_inventory_width: usize,
    compatibility_debt_width: usize,
    route_plan_width: usize,
    boundary_evidence_width: usize,
    support_width: usize,
    deferred_width: usize,
    capability_eligibility_operations: usize,
    route_plan_assembly_operations: usize,
    boundary_receipt_assembly_operations: usize,
    boundary_envelope_assembly_operations: usize,
    support_lookup_operations: usize,
    debt_registry_lookup_operations: usize,
    counter_snapshot_digest: String,
}

impl ForgeQueryLowerRuntimePerformanceCounters {
    pub(crate) fn new(
        crossing_inventory_width: usize,
        compatibility_debt_width: usize,
        route_plan_width: usize,
        boundary_evidence_width: usize,
        support_width: usize,
        deferred_width: usize,
        capability_eligibility_operations: usize,
        route_plan_assembly_operations: usize,
        boundary_receipt_assembly_operations: usize,
        boundary_envelope_assembly_operations: usize,
        support_lookup_operations: usize,
        debt_registry_lookup_operations: usize,
    ) -> Self {
        let counter_snapshot_digest = hash_parts(&[
            format!("crossings:{crossing_inventory_width}"),
            format!("compatibility_debt:{compatibility_debt_width}"),
            format!("route_plans:{route_plan_width}"),
            format!("boundary_evidence:{boundary_evidence_width}"),
            format!("support:{support_width}"),
            format!("deferred:{deferred_width}"),
            format!("capability_eligibility_ops:{capability_eligibility_operations}"),
            format!("route_plan_ops:{route_plan_assembly_operations}"),
            format!("boundary_receipt_ops:{boundary_receipt_assembly_operations}"),
            format!("boundary_envelope_ops:{boundary_envelope_assembly_operations}"),
            format!("support_lookup_ops:{support_lookup_operations}"),
            format!("debt_lookup_ops:{debt_registry_lookup_operations}"),
        ]);
        Self {
            crossing_inventory_width,
            compatibility_debt_width,
            route_plan_width,
            boundary_evidence_width,
            support_width,
            deferred_width,
            capability_eligibility_operations,
            route_plan_assembly_operations,
            boundary_receipt_assembly_operations,
            boundary_envelope_assembly_operations,
            support_lookup_operations,
            debt_registry_lookup_operations,
            counter_snapshot_digest,
        }
    }

    pub fn crossing_inventory_width(&self) -> usize {
        self.crossing_inventory_width
    }

    pub fn compatibility_debt_width(&self) -> usize {
        self.compatibility_debt_width
    }

    pub fn route_plan_width(&self) -> usize {
        self.route_plan_width
    }

    pub fn boundary_evidence_width(&self) -> usize {
        self.boundary_evidence_width
    }

    pub fn support_width(&self) -> usize {
        self.support_width
    }

    pub fn deferred_width(&self) -> usize {
        self.deferred_width
    }

    pub fn capability_eligibility_operations(&self) -> usize {
        self.capability_eligibility_operations
    }

    pub fn route_plan_assembly_operations(&self) -> usize {
        self.route_plan_assembly_operations
    }

    pub fn boundary_receipt_assembly_operations(&self) -> usize {
        self.boundary_receipt_assembly_operations
    }

    pub fn boundary_envelope_assembly_operations(&self) -> usize {
        self.boundary_envelope_assembly_operations
    }

    pub fn support_lookup_operations(&self) -> usize {
        self.support_lookup_operations
    }

    pub fn debt_registry_lookup_operations(&self) -> usize {
        self.debt_registry_lookup_operations
    }

    pub fn counter_snapshot_digest(&self) -> &str {
        &self.counter_snapshot_digest
    }
}
