use super::{
    support_matrix_rows, CertifiedPlaneSupport, CertifiedSurfaceSupport, SurfaceFamily,
    SurfaceSupportGeometrySnapshot, SurfaceSupportReceiptSet, UnsupportedSurfaceSupport,
    UnsupportedSurfaceSupportReasonCode, UnsupportedSurfaceSupportReceipt,
};
use crate::workload_platform::geometry_binding::BoundGeometryWorkload;

pub struct SurfaceSupportWorkload {
    bound_geometry: BoundGeometryWorkload,
    declaration: String,
    surface_family: Option<SurfaceFamily>,
}

impl SurfaceSupportWorkload {
    pub fn for_bound_geometry(bound_geometry: BoundGeometryWorkload) -> Self {
        Self {
            bound_geometry,
            declaration: "surface support workload".to_string(),
            surface_family: None,
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn with_surface_family(mut self, family: SurfaceFamily) -> Self {
        self.surface_family = Some(family);
        self
    }

    pub fn certify(self) -> Result<CertifiedSurfaceSupport, UnsupportedSurfaceSupport> {
        let matrix_rows = support_matrix_rows();
        if self.declaration.trim().is_empty() {
            return Err(self.deny_surface_support(
                None,
                UnsupportedSurfaceSupportReasonCode::MissingDeclaration,
                "Surface support requires a human-readable declaration before certification.",
                matrix_rows,
            ));
        }
        if !self.bound_geometry.can_enter_surface_support() {
            return Err(self.deny_surface_support(
                self.surface_family,
                UnsupportedSurfaceSupportReasonCode::MissingGeometryBindingReceipt,
                "Surface support requires a complete geometry binding receipt.",
                matrix_rows,
            ));
        }

        let Some(family) = self.surface_family else {
            return Err(self.deny_surface_support(
                None,
                UnsupportedSurfaceSupportReasonCode::MissingSurfaceFamily,
                "Surface support requires an explicit surface family.",
                matrix_rows,
            ));
        };

        if !family.is_certified_in_milestone() {
            return Err(self.deny_surface_support(
                Some(family),
                UnsupportedSurfaceSupportReasonCode::FamilyNotAdmitted,
                format!(
                    "{} is not admitted for M6.5 surface support.",
                    family.human_label()
                ),
                matrix_rows,
            ));
        }

        self.certify_plane_support(matrix_rows)
    }

    fn certify_plane_support(
        self,
        matrix_rows: Vec<super::SurfaceSupportMatrixRow>,
    ) -> Result<CertifiedSurfaceSupport, UnsupportedSurfaceSupport> {
        let geometry_binding_identity = self
            .bound_geometry
            .receipts()
            .stage_identity()
            .receipt_identity();
        let topology_query_surface = self
            .bound_geometry
            .receipts()
            .topology_query_surface()
            .to_string();
        let upstream_geometry_carriers = self
            .bound_geometry
            .receipts()
            .counters()
            .geometry_carriers();
        let geometry_snapshot =
            SurfaceSupportGeometrySnapshot::from_bound_geometry(&self.bound_geometry);
        let stage_receipt =
            crate::workload_platform::vocabulary::SurfaceSupportWorkload::for_geometry_binding(
                self.bound_geometry.receipts().stage_receipt(),
            )
            .declared(self.declaration)
            .admit()
            .map_err(|_| {
                UnsupportedSurfaceSupport::new(
                    Some(SurfaceFamily::Plane),
                    UnsupportedSurfaceSupportReasonCode::MissingGeometryBindingReceipt,
                    "Surface support could not produce a stage receipt from geometry binding.",
                    Some(geometry_binding_identity.clone()),
                    Some(topology_query_surface.clone()),
                    matrix_rows.clone(),
                    None,
                )
            })?;
        let receipts = SurfaceSupportReceiptSet::new(
            stage_receipt,
            geometry_binding_identity.clone(),
            topology_query_surface.clone(),
            matrix_rows,
            upstream_geometry_carriers,
        );
        let plane_support =
            CertifiedPlaneSupport::new(geometry_binding_identity, topology_query_surface);
        Ok(CertifiedSurfaceSupport::new(
            plane_support,
            receipts,
            geometry_snapshot,
        ))
    }

    fn deny_surface_support(
        &self,
        family: Option<SurfaceFamily>,
        reason_code: UnsupportedSurfaceSupportReasonCode,
        human_reason: impl Into<String>,
        matrix_rows: Vec<super::SurfaceSupportMatrixRow>,
    ) -> UnsupportedSurfaceSupport {
        let human_reason = human_reason.into();
        let geometry_binding_identity = self
            .bound_geometry
            .receipts()
            .stage_identity()
            .receipt_identity();
        let topology_query_surface = self
            .bound_geometry
            .receipts()
            .topology_query_surface()
            .to_string();
        let receipt = self.unsupported_surface_support_receipt(
            family,
            reason_code,
            human_reason.clone(),
            geometry_binding_identity.clone(),
            matrix_rows.clone(),
        );
        UnsupportedSurfaceSupport::new(
            family,
            reason_code,
            human_reason,
            Some(geometry_binding_identity),
            Some(topology_query_surface),
            matrix_rows,
            receipt,
        )
    }

    fn unsupported_surface_support_receipt(
        &self,
        family: Option<SurfaceFamily>,
        reason_code: UnsupportedSurfaceSupportReasonCode,
        human_reason: String,
        upstream_geometry_binding_identity: String,
        matrix_rows: Vec<super::SurfaceSupportMatrixRow>,
    ) -> Option<UnsupportedSurfaceSupportReceipt> {
        if self.declaration.trim().is_empty() {
            return None;
        }

        Some(UnsupportedSurfaceSupportReceipt::new(
            self.declaration.clone(),
            upstream_geometry_binding_identity,
            family,
            reason_code,
            human_reason,
            matrix_rows,
            self.bound_geometry
                .receipts()
                .counters()
                .geometry_carriers(),
        ))
    }
}
