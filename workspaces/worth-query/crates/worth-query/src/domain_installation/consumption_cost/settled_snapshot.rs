use worth_foundational::FoundationalPerformanceWorkClass;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumptionCostRow {
    name: String,
    work_class: FoundationalPerformanceWorkClass,
    observed_count: u64,
}

impl WorthQueryConsumptionCostRow {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn work_class(&self) -> FoundationalPerformanceWorkClass {
        self.work_class
    }

    pub const fn observed_count(&self) -> u64 {
        self.observed_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumptionCostSnapshot {
    rows: Vec<WorthQueryConsumptionCostRow>,
}

impl WorthQueryConsumptionCostSnapshot {
    pub fn rows(&self) -> &[WorthQueryConsumptionCostRow] {
        &self.rows
    }

    pub fn row(&self, name: &str) -> Option<&WorthQueryConsumptionCostRow> {
        self.rows.iter().find(|row| row.name == name)
    }

    pub(super) fn from_settled<D, O, F, L: crate::basis_lifecycle::BasisOperationLane>(
        settled: &crate::domain_installation::WorthQuerySettledDomainProjection<D, O, F, L>,
    ) -> Self {
        let mut rows = Vec::new();
        retain_lookup_rows(settled, &mut rows);
        retain_binding_rows(settled, &mut rows);
        retain_support_rows(settled, &mut rows);
        retain_execution_rows(settled, &mut rows);
        retain_direct_domain_evidence_rows(settled, &mut rows);
        retain_dependency_rows(settled, &mut rows);
        retain_native_binding_rows(settled, &mut rows);
        Self { rows }
    }

    pub(super) fn from_workflow<D, O, F, L: crate::basis_lifecycle::BasisOperationLane>(
        settled: &crate::domain_installation::WorthQuerySettledWorkflowProjection<D, O, F, L>,
    ) -> Self {
        let mut rows = Vec::new();
        retain_workflow_lookup_rows(settled, &mut rows);
        retain_workflow_binding_rows(settled, &mut rows);
        retain_workflow_support_rows(settled, &mut rows);
        retain_workflow_execution_rows(settled, &mut rows);
        retain_workflow_domain_evidence_rows(settled, &mut rows);
        if let Some(closure) = settled.trace().semantic_aspect_dependency_closure() {
            retain_dependency_counter_rows(closure.counters(), &mut rows);
        }
        if let Some(counters) = settled.native_access_binding_counters() {
            retain_native_binding_counter_rows(counters, &mut rows);
        }
        Self { rows }
    }
}

pub(super) const fn execution_work_class(
    posture: crate::domain_installation::WorthQueryBoundCommitPosture,
) -> FoundationalPerformanceWorkClass {
    match posture {
        crate::domain_installation::WorthQueryBoundCommitPosture::ReadOnly => {
            FoundationalPerformanceWorkClass::AuthoritativeObservation
        }
        crate::domain_installation::WorthQueryBoundCommitPosture::Atomic
        | crate::domain_installation::WorthQueryBoundCommitPosture::Compensated => {
            FoundationalPerformanceWorkClass::AuthoritativeMutation
        }
    }
}

impl<D, O, F, L: crate::basis_lifecycle::BasisOperationLane>
    crate::domain_installation::WorthQuerySettledWorkflowProjection<D, O, F, L>
{
    pub fn consumption_cost_snapshot(&self) -> WorthQueryConsumptionCostSnapshot {
        WorthQueryConsumptionCostSnapshot::from_workflow(self)
    }
}

impl<D, O, F, L: crate::basis_lifecycle::BasisOperationLane>
    crate::domain_installation::WorthQuerySettledDomainProjection<D, O, F, L>
{
    pub fn consumption_cost_snapshot(&self) -> WorthQueryConsumptionCostSnapshot {
        WorthQueryConsumptionCostSnapshot::from_settled(self)
    }
}

macro_rules! retain_rows {
    ($rows:expr, $prefix:literal, $class:expr, $counters:expr, [$($field:ident),+ $(,)?]) => {
        $(
            $rows.push(WorthQueryConsumptionCostRow {
                name: concat!($prefix, ".", stringify!($field)).into(),
                work_class: $class,
                observed_count: $counters.$field as u64,
            });
        )+
    };
}

mod direct_rows;
mod domain_evidence_rows;
mod workflow_rows;

use direct_rows::{
    retain_binding_rows, retain_dependency_counter_rows, retain_dependency_rows,
    retain_execution_rows, retain_lookup_rows, retain_native_binding_counter_rows,
    retain_native_binding_rows, retain_support_rows,
};
use domain_evidence_rows::{
    retain_direct_domain_evidence_rows, retain_workflow_domain_evidence_rows,
};
use workflow_rows::{
    retain_workflow_binding_rows, retain_workflow_execution_rows, retain_workflow_lookup_rows,
    retain_workflow_support_rows,
};
