use forge_query::facade::{
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::anchor_selection::SpatialAnchorSelectionDeclarationFamily;
use crate::bindings::query_native::{
    PrimitiveAnchorBindingDeclarationFamily, PrimitiveBindingDeclarationFamily,
};
use crate::bindings::query_native_geometry_inventory::GeometryPublicSurface;
use crate::bindings::query_native_rebinding::PrimitiveRebindingDeclarationFamily;
use crate::bindings::query_native_rebinding_neighborhood_replacement::TopologyNeighborhoodReplacementDeclarationFamily;
use crate::bindings::query_native_rebinding_projection_consumption::GeometryProjectionConsumptionDeclarationFamily;
use crate::bindings::query_native_retained_geometry::{
    BranchLocalGeometryInspectionDeclarationFamily, GeometryRecoveryActionDeclarationFamily,
    GeometryReplayParityDeclarationFamily, HistoricalGeometryInspectionDeclarationFamily,
};
use crate::bindings::query_native_tolerance_precision::ToleranceAndPrecisionCertificationDeclarationFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeometryPublicSupportStatus {
    Supported,
}

impl GeometryPublicSupportStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeometryPublicSupportRow {
    surface: GeometryPublicSurface,
    status: GeometryPublicSupportStatus,
    declared_family_key: Option<&'static str>,
    legality_contract: Option<ForgeQueryDeclarationLegalityContract>,
    route_contract: Option<ForgeQueryDeclarationRouteContract>,
    admission_rule: &'static str,
    row_digest: String,
}

impl GeometryPublicSupportRow {
    fn new(surface: GeometryPublicSurface) -> Self {
        let declared_family_key = declared_family_key_for(surface);
        let legality_contract = legality_contract_for(surface);
        let route_contract = route_contract_for(surface);
        let admission_rule = admission_rule_for(surface);
        let row_digest = hash_parts(&[
            format!("surface:{}", surface.as_str()),
            format!("status:{}", GeometryPublicSupportStatus::Supported.as_str()),
            format!("family:{}", declared_family_key.unwrap_or("surface-only")),
            format!(
                "legality:{}",
                legality_contract
                    .map(|value| format!("{value:?}"))
                    .unwrap_or_else(|| "not-applicable".to_string())
            ),
            format!(
                "route:{}",
                route_contract
                    .map(|value| format!("{value:?}"))
                    .unwrap_or_else(|| "not-applicable".to_string())
            ),
            format!("rule:{admission_rule}"),
        ]);
        Self {
            surface,
            status: GeometryPublicSupportStatus::Supported,
            declared_family_key,
            legality_contract,
            route_contract,
            admission_rule,
            row_digest,
        }
    }

    pub fn surface(&self) -> GeometryPublicSurface {
        self.surface
    }

    pub fn status(&self) -> GeometryPublicSupportStatus {
        self.status
    }

    pub fn declared_family_key(&self) -> Option<&'static str> {
        self.declared_family_key
    }

    pub fn legality_contract(&self) -> Option<ForgeQueryDeclarationLegalityContract> {
        self.legality_contract
    }

    pub fn route_contract(&self) -> Option<ForgeQueryDeclarationRouteContract> {
        self.route_contract
    }

    pub fn admission_rule(&self) -> &'static str {
        self.admission_rule
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeometryPublicSupportMatrix {
    rows: Vec<GeometryPublicSupportRow>,
    matrix_digest: String,
}

impl GeometryPublicSupportMatrix {
    pub fn rows(&self) -> &[GeometryPublicSupportRow] {
        &self.rows
    }

    pub fn matrix_digest(&self) -> &str {
        &self.matrix_digest
    }

    pub fn row_for_surface(
        &self,
        surface: GeometryPublicSurface,
    ) -> Option<&GeometryPublicSupportRow> {
        self.rows.iter().find(|row| row.surface() == surface)
    }

    pub fn row(&self, surface: &str) -> Option<&GeometryPublicSupportRow> {
        self.rows
            .iter()
            .find(|row| row.surface().as_str() == surface)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeometryPublicFamilyAdmission {
    surface: GeometryPublicSurface,
    support_row_digest: String,
    matrix_digest: String,
}

impl GeometryPublicFamilyAdmission {
    pub fn surface(&self) -> GeometryPublicSurface {
        self.surface
    }

    pub fn support_row_digest(&self) -> &str {
        &self.support_row_digest
    }

    pub fn matrix_digest(&self) -> &str {
        &self.matrix_digest
    }
}

pub fn geometry_public_support_matrix() -> GeometryPublicSupportMatrix {
    let rows = GeometryPublicSurface::all()
        .into_iter()
        .map(GeometryPublicSupportRow::new)
        .collect::<Vec<_>>();
    let matrix_digest = hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    GeometryPublicSupportMatrix {
        rows,
        matrix_digest,
    }
}

pub fn admit_geometry_public_surface(
    surface: GeometryPublicSurface,
) -> GeometryPublicFamilyAdmission {
    let matrix = geometry_public_support_matrix();
    let row = matrix
        .row_for_surface(surface)
        .expect("public geometry support matrix must cover every admitted surface");
    GeometryPublicFamilyAdmission {
        surface,
        support_row_digest: row.row_digest().to_string(),
        matrix_digest: matrix.matrix_digest().to_string(),
    }
}

fn declared_family_key_for(surface: GeometryPublicSurface) -> Option<&'static str> {
    match surface {
        GeometryPublicSurface::GeometryTargetIdentity => None,
        GeometryPublicSurface::SpatialAnchorSelection => {
            Some(SpatialAnchorSelectionDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PrimitiveBinding => {
            Some(PrimitiveBindingDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PrimitiveAnchorBinding => {
            Some(PrimitiveAnchorBindingDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::PrimitiveRebinding => {
            Some(PrimitiveRebindingDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::TopologyNeighborhoodReplacement => {
            Some(TopologyNeighborhoodReplacementDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::ToleranceAndPrecisionCertification => {
            Some(ToleranceAndPrecisionCertificationDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::HistoricalGeometryInspection => {
            Some(HistoricalGeometryInspectionDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::BranchLocalGeometryInspection => {
            Some(BranchLocalGeometryInspectionDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::GeometryReplayParity => {
            Some(GeometryReplayParityDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::GeometryRecoveryAction => {
            Some(GeometryRecoveryActionDeclarationFamily::semantic_family_key())
        }
        GeometryPublicSurface::GeometryProjectionConsumption => {
            Some(GeometryProjectionConsumptionDeclarationFamily::semantic_family_key())
        }
    }
}

fn legality_contract_for(
    surface: GeometryPublicSurface,
) -> Option<ForgeQueryDeclarationLegalityContract> {
    match surface {
        GeometryPublicSurface::GeometryTargetIdentity => None,
        GeometryPublicSurface::SpatialAnchorSelection => {
            Some(SpatialAnchorSelectionDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PrimitiveBinding => {
            Some(PrimitiveBindingDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PrimitiveAnchorBinding => {
            Some(PrimitiveAnchorBindingDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::PrimitiveRebinding => {
            Some(PrimitiveRebindingDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::TopologyNeighborhoodReplacement => {
            Some(TopologyNeighborhoodReplacementDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::ToleranceAndPrecisionCertification => {
            Some(ToleranceAndPrecisionCertificationDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::HistoricalGeometryInspection => {
            Some(HistoricalGeometryInspectionDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::BranchLocalGeometryInspection => {
            Some(BranchLocalGeometryInspectionDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::GeometryReplayParity => {
            Some(GeometryReplayParityDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::GeometryRecoveryAction => {
            Some(GeometryRecoveryActionDeclarationFamily::legality_contract())
        }
        GeometryPublicSurface::GeometryProjectionConsumption => {
            Some(GeometryProjectionConsumptionDeclarationFamily::legality_contract())
        }
    }
}

fn route_contract_for(
    surface: GeometryPublicSurface,
) -> Option<ForgeQueryDeclarationRouteContract> {
    match surface {
        GeometryPublicSurface::GeometryTargetIdentity => None,
        GeometryPublicSurface::SpatialAnchorSelection => {
            Some(SpatialAnchorSelectionDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PrimitiveBinding => {
            Some(PrimitiveBindingDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PrimitiveAnchorBinding => {
            Some(PrimitiveAnchorBindingDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::PrimitiveRebinding => {
            Some(PrimitiveRebindingDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::TopologyNeighborhoodReplacement => {
            Some(TopologyNeighborhoodReplacementDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::ToleranceAndPrecisionCertification => {
            Some(ToleranceAndPrecisionCertificationDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::HistoricalGeometryInspection => {
            Some(HistoricalGeometryInspectionDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::BranchLocalGeometryInspection => {
            Some(BranchLocalGeometryInspectionDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::GeometryReplayParity => {
            Some(GeometryReplayParityDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::GeometryRecoveryAction => {
            Some(GeometryRecoveryActionDeclarationFamily::route_contract())
        }
        GeometryPublicSurface::GeometryProjectionConsumption => {
            Some(GeometryProjectionConsumptionDeclarationFamily::route_contract())
        }
    }
}

fn admission_rule_for(surface: GeometryPublicSurface) -> &'static str {
    match surface {
        GeometryPublicSurface::GeometryTargetIdentity => {
            "support comes from declaration-backed target identity facts over admitted binding and anchor workflows"
        }
        GeometryPublicSurface::SpatialAnchorSelection => {
            "support comes from admitted spatial anchor selection declaration family workflow"
        }
        GeometryPublicSurface::PrimitiveBinding => {
            "support comes from admitted primitive binding declaration family workflow"
        }
        GeometryPublicSurface::PrimitiveAnchorBinding => {
            "support comes from admitted primitive anchor binding declaration family workflow"
        }
        GeometryPublicSurface::PrimitiveRebinding => {
            "support comes from projection-backed primitive rebinding declaration family workflow"
        }
        GeometryPublicSurface::TopologyNeighborhoodReplacement => {
            "support comes from admitted topology neighborhood replacement declaration family workflow"
        }
        GeometryPublicSurface::ToleranceAndPrecisionCertification => {
            "support comes from admitted tolerance and precision certification declaration family workflow"
        }
        GeometryPublicSurface::HistoricalGeometryInspection => {
            "support comes from retained-fact historical geometry inspection declaration family workflow"
        }
        GeometryPublicSurface::BranchLocalGeometryInspection => {
            "support comes from retained-fact branch-local geometry inspection declaration family workflow"
        }
        GeometryPublicSurface::GeometryReplayParity => {
            "support comes from retained-fact geometry replay parity declaration family workflow"
        }
        GeometryPublicSurface::GeometryRecoveryAction => {
            "support comes from admitted geometry recovery action declaration family workflow"
        }
        GeometryPublicSurface::GeometryProjectionConsumption => {
            "support comes from receipt-backed geometry projection consumption declaration family workflow"
        }
    }
}

fn hash_parts(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
