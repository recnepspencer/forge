use super::stage_contract::{
    certify_stage, SpatialWorkloadStage, WorkloadStageDenial, WorkloadStageEnvelope,
    WorkloadStageIdentity, WorkloadStageSupport,
};
use topology::facade::TopologyWorkloadReceipt;

pub struct GeometryBindingWorkload {
    topology_receipt: String,
    declaration: String,
}

impl GeometryBindingWorkload {
    pub fn for_topology_receipt(topology_receipt: &TopologyWorkloadReceipt) -> Self {
        Self {
            topology_receipt: topology_receipt.identity().name().to_string(),
            declaration: "geometry binding workload".to_string(),
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn admit(self) -> Result<GeometryBindingWorkloadReceipt, WorkloadStageDenial> {
        let envelope = certify_stage(
            SpatialWorkloadStage::GeometryBinding,
            self.declaration,
            self.topology_receipt,
            WorkloadStageSupport::Admitted,
            "geometry binding workload is admitted",
        )?;
        Ok(GeometryBindingWorkloadReceipt { envelope })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeometryBindingWorkloadReceipt {
    envelope: WorkloadStageEnvelope,
}

impl GeometryBindingWorkloadReceipt {
    pub fn identity(&self) -> &WorkloadStageIdentity {
        self.envelope.identity()
    }

    pub fn envelope(&self) -> &WorkloadStageEnvelope {
        &self.envelope
    }
}
