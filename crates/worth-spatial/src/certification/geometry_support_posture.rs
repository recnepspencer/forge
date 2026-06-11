use forge_query::facade::{
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::query_native_geometry_inventory::GeometryPublicSurface;

#[path = "geometry_support_family_contracts.rs"]
mod geometry_support_family_contracts;

use geometry_support_family_contracts::{
    declared_family_key_for, legality_contract_for, route_contract_for,
};

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
        GeometryPublicSurface::PlanarPredicateAuthority => {
            "support comes from admitted planar predicate authority declaration family workflow backed by worth-math certified predicates"
        }
        GeometryPublicSurface::PlanarPrecisionCertification => {
            "support comes from admitted planar precision certification workflow consuming retained planar predicate receipts and local feature-scale basis"
        }
        GeometryPublicSurface::PlanarLocalFrameCertificate => {
            "support comes from admitted planar local-frame certificate workflow consuming retained precision basis and transform posture"
        }
        GeometryPublicSurface::ProjectPointToCertifiedPlane2D => {
            "support comes from admitted certified plane-to-2D point projection consuming retained local-frame certificates"
        }
        GeometryPublicSurface::CertifiedSegmentSegment2D => {
            "support comes from admitted certified segment classification consuming projected endpoint receipts and exact planar predicate authority"
        }
        GeometryPublicSurface::CertifiedPolygonWinding2D => {
            "support comes from admitted certified polygon winding consuming projected loop receipts, exact planar predicates, and segment contact certificates"
        }
        GeometryPublicSurface::CertifiedSignedArea2D => {
            "support comes from admitted certified signed-area classification consuming winding, projection, local-frame, and precision receipts without boolean repair"
        }
        GeometryPublicSurface::CoplanarOverlapContractExtractor => {
            "support comes from admitted coplanar overlap contract extraction consuming signed-area, winding, segment-contact, projection, local-frame, and movement receipts without imprinting"
        }
        GeometryPublicSurface::PlanarContractBundleValidator => {
            "support comes from admitted planar contract bundle validation consuming complete M6 receipts as boolean-readiness input without computing boolean topology"
        }
        GeometryPublicSurface::PredicateCertificateConsumptionValidator => {
            "support comes from admitted predicate certificate consumption validation proving retained planar classifications consumed worth-math certified signs and precision metadata"
        }
        GeometryPublicSurface::PlanarStructuralIdentity => {
            "support comes from admitted planar structural identity certification over boolean-readiness receipts and canonical transform basis, not topology names or final coordinates"
        }
        GeometryPublicSurface::PlanarMotionPosture => {
            "support comes from admitted planar motion posture certification over boolean-readiness receipts, typed movement, rotation, reorientation, cancellation, signal compatibility, and continuation basis"
        }
        GeometryPublicSurface::PlanarTopologyContractCompleteness => {
            "support comes from admitted topology-to-spatial completeness certification consuming Query-owned topology receipts, declared surfaces, validation facts, and planar neighborhood basis before planar fact emission"
        }
        GeometryPublicSurface::RetainedPlanarFacts => {
            "support comes from admitted retained planar fact certification freezing boolean-readiness, structural identity, movement/rotation posture, topology completeness, and retained family rows for historical and branch-local replay without live-state repair"
        }
        GeometryPublicSurface::ProjectionConsumedPlanarFacts => {
            "support comes from admitted projection-consumed planar fact certification consuming retained planar facts and exact bundle projection receipts for downstream boolean-readiness without payload spelunking or recomputation"
        }
        GeometryPublicSurface::PlanarRecoveryPosture => {
            "support comes from admitted planar recovery posture certification consuming typed planar blockers and basis receipts to produce next-step recovery without changing planar truth"
        }
        GeometryPublicSurface::PlanarDiagnosticBundle => {
            "support comes from admitted planar diagnostic bundle certification deriving machine-checkable locality and causal references from typed receipts without changing planar truth"
        }
        GeometryPublicSurface::PlanarLocalRebuildParity => {
            "support comes from admitted local planar rebuild parity certification consuming grouped neighborhood replacement, rebinding continuity, retained/projection-consumed facts, motion, recovery, and diagnostics without broad search"
        }
        GeometryPublicSurface::PlanarCleanFailBoundary => {
            "support comes from admitted planar clean-fail boundary certification consuming admission, movement/rotation, recovery, and diagnostics while proving no repair or bounded conversion was attempted"
        }
        GeometryPublicSurface::PlanarBooleanReadinessWorkload => {
            "support comes from admitted final-boss boolean-readiness workload certification consuming complete workload evidence, projection parity, diagnostics, and user-response blockers before M7 without boolean execution"
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
