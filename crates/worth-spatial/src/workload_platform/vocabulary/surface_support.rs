use super::geometry_binding::GeometryBindingWorkloadReceipt;
use super::stage_contract::{
    certify_stage, SpatialWorkloadStage, WorkloadStageDenial, WorkloadStageEnvelope,
    WorkloadStageIdentity, WorkloadStageSupport,
};

pub struct SurfaceSupportWorkload<'a> {
    upstream: &'a GeometryBindingWorkloadReceipt,
    declaration: String,
}

impl<'a> SurfaceSupportWorkload<'a> {
    pub fn for_geometry_binding(upstream: &'a GeometryBindingWorkloadReceipt) -> Self {
        Self {
            upstream,
            declaration: "surface support workload".to_string(),
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn admit(self) -> Result<SurfaceSupportWorkloadReceipt, WorkloadStageDenial> {
        let envelope = certify_stage(
            SpatialWorkloadStage::SurfaceSupport,
            self.declaration,
            self.upstream.identity().receipt_identity(),
            WorkloadStageSupport::Admitted,
            "surface support workload is admitted",
        )?;
        Ok(SurfaceSupportWorkloadReceipt { envelope })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceSupportWorkloadReceipt {
    envelope: WorkloadStageEnvelope,
}

impl SurfaceSupportWorkloadReceipt {
    pub fn identity(&self) -> &WorkloadStageIdentity {
        self.envelope.identity()
    }

    pub fn envelope(&self) -> &WorkloadStageEnvelope {
        &self.envelope
    }
}
