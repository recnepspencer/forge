use crate::construction::tests::support::runtime_truth::PrimitiveConstructionCertificationRuntimeTruth;

use super::super::schema::{
    PrimitiveConstructionCompoundRowClass, PrimitiveConstructionCompoundTopologyClass,
    PrimitiveConstructionCompoundWorkloadFamily,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PrimitiveConstructionCompoundRow {
    scenario_id: String,
    workload_family: PrimitiveConstructionCompoundWorkloadFamily,
    topology_class: PrimitiveConstructionCompoundTopologyClass,
    row_class: PrimitiveConstructionCompoundRowClass,
    runtime_truth: PrimitiveConstructionCertificationRuntimeTruth,
}

impl PrimitiveConstructionCompoundRow {
    pub fn new(
        scenario_id: String,
        workload_family: PrimitiveConstructionCompoundWorkloadFamily,
        topology_class: PrimitiveConstructionCompoundTopologyClass,
        row_class: PrimitiveConstructionCompoundRowClass,
        runtime_truth: PrimitiveConstructionCertificationRuntimeTruth,
    ) -> Self {
        Self {
            scenario_id,
            workload_family,
            topology_class,
            row_class,
            runtime_truth,
        }
    }

    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }

    pub fn workload_family(&self) -> PrimitiveConstructionCompoundWorkloadFamily {
        self.workload_family
    }

    pub fn topology_class(&self) -> PrimitiveConstructionCompoundTopologyClass {
        self.topology_class
    }

    pub fn row_class(&self) -> PrimitiveConstructionCompoundRowClass {
        self.row_class
    }

    pub fn runtime_truth(&self) -> &PrimitiveConstructionCertificationRuntimeTruth {
        &self.runtime_truth
    }
}
