use crate::workload_platform::surface_support::CertifiedSurfaceSupport;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalFrameBasis {
    identity: String,
    human_label: String,
    basis_parts: Vec<String>,
}

impl LocalFrameBasis {
    pub fn from_certified_plane() -> Self {
        Self {
            identity: "local-frame:certified-plane:origin-u-v-normal".to_string(),
            human_label: "certified plane local frame".to_string(),
            basis_parts: vec![
                "origin".to_string(),
                "u-axis".to_string(),
                "v-axis".to_string(),
                "normal".to_string(),
            ],
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn human_label(&self) -> &str {
        &self.human_label
    }

    pub fn basis_parts(&self) -> &[String] {
        &self.basis_parts
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertifiedLocalFrameReceipt {
    surface_support_identity: String,
    certified_plane_support_identity: String,
    local_basis_identity: String,
    basis_parts: Vec<String>,
}

impl CertifiedLocalFrameReceipt {
    pub(crate) fn new(
        surface_support: &CertifiedSurfaceSupport,
        local_basis: &LocalFrameBasis,
    ) -> Self {
        Self {
            surface_support_identity: surface_support
                .receipts()
                .stage_identity()
                .receipt_identity(),
            certified_plane_support_identity: surface_support
                .certified_plane_support()
                .upstream_geometry_binding_identity()
                .to_string(),
            local_basis_identity: local_basis.identity().to_string(),
            basis_parts: local_basis.basis_parts().to_vec(),
        }
    }

    pub fn surface_support_identity(&self) -> &str {
        &self.surface_support_identity
    }

    pub fn certified_plane_support_identity(&self) -> &str {
        &self.certified_plane_support_identity
    }

    pub fn local_basis_identity(&self) -> &str {
        &self.local_basis_identity
    }

    pub fn basis_parts(&self) -> &[String] {
        &self.basis_parts
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertifiedLocalFrameWorkload {
    receipt: CertifiedLocalFrameReceipt,
}

impl CertifiedLocalFrameWorkload {
    pub(crate) fn new(receipt: CertifiedLocalFrameReceipt) -> Self {
        Self { receipt }
    }

    pub fn receipt(&self) -> &CertifiedLocalFrameReceipt {
        &self.receipt
    }
}
