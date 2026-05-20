use crate::identity::hash_parts;
use crate::lower_runtime_routing::{
    forge_query_lower_runtime_direct_import_audit, ForgeQueryLowerRuntimeDirectImportPosture,
    ForgeQueryLowerRuntimeSeamKey,
};

use super::public_surface::{
    forge_query_lower_runtime_public_surface_inventory, ForgeQueryLowerRuntimePublicSurfaceKind,
};

const REMAINING_PHASE_SIX_BOUNDARY_SEAMS: &[ForgeQueryLowerRuntimeSeamKey] = &[
    ForgeQueryLowerRuntimeSeamKey::RuntimeBackendBoundaryModules,
    ForgeQueryLowerRuntimeSeamKey::ExecuteRuntimeCurrentReadGraph,
    ForgeQueryLowerRuntimeSeamKey::ExecuteRuntimeBasisContextReadGraph,
    ForgeQueryLowerRuntimeSeamKey::SubscriptionContinuity,
    ForgeQueryLowerRuntimeSeamKey::BasisReadmissionFromTruthViewEvidence,
    ForgeQueryLowerRuntimeSeamKey::BasisReadmissionFromSubscriptionEvidence,
    ForgeQueryLowerRuntimeSeamKey::HistoricalBridgeLowering,
    ForgeQueryLowerRuntimeSeamKey::EffectBackedRelationalMutation,
    ForgeQueryLowerRuntimeSeamKey::EffectBackedRelationalMerge,
    ForgeQueryLowerRuntimeSeamKey::EffectBackedBridgeWriteback,
    ForgeQueryLowerRuntimeSeamKey::RuntimeIntentAuthorityAdapter,
    ForgeQueryLowerRuntimeSeamKey::IntentRuntimeExecution,
    ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromQueryReceipts,
    ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromRelationalArtifacts,
    ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromBridgeArtifacts,
    ForgeQueryLowerRuntimeSeamKey::CausalBridgeMaterialization,
    ForgeQueryLowerRuntimeSeamKey::FrontierEvidenceIntake,
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeBoundaryReconciliationRow {
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    surface_label: &'static str,
    implementation_path: &'static str,
    surface_kind: ForgeQueryLowerRuntimePublicSurfaceKind,
    direct_import_posture: Option<ForgeQueryLowerRuntimeDirectImportPosture>,
}

impl ForgeQueryLowerRuntimeBoundaryReconciliationRow {
    fn new(
        seam_key: ForgeQueryLowerRuntimeSeamKey,
        surface_label: &'static str,
        implementation_path: &'static str,
        surface_kind: ForgeQueryLowerRuntimePublicSurfaceKind,
        direct_import_posture: Option<ForgeQueryLowerRuntimeDirectImportPosture>,
    ) -> Self {
        Self {
            seam_key,
            surface_label,
            implementation_path,
            surface_kind,
            direct_import_posture,
        }
    }

    pub fn seam_key(&self) -> ForgeQueryLowerRuntimeSeamKey {
        self.seam_key
    }

    pub fn surface_label(&self) -> &'static str {
        self.surface_label
    }

    pub fn implementation_path(&self) -> &'static str {
        self.implementation_path
    }

    pub fn surface_kind(&self) -> ForgeQueryLowerRuntimePublicSurfaceKind {
        self.surface_kind
    }

    pub fn direct_import_posture(&self) -> Option<ForgeQueryLowerRuntimeDirectImportPosture> {
        self.direct_import_posture
    }

    fn row_digest(&self) -> String {
        hash_parts(&[
            self.seam_key.as_str().to_string(),
            self.surface_label.to_string(),
            self.implementation_path.to_string(),
            self.surface_kind.as_str().to_string(),
            self.direct_import_posture
                .map(|posture| posture.as_str().to_string())
                .unwrap_or_else(|| "none".to_string()),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeBoundaryReconciliationReport {
    rows: Vec<ForgeQueryLowerRuntimeBoundaryReconciliationRow>,
    report_digest: String,
}

impl ForgeQueryLowerRuntimeBoundaryReconciliationReport {
    fn new(rows: Vec<ForgeQueryLowerRuntimeBoundaryReconciliationRow>) -> Self {
        let report_digest = hash_parts(
            &rows
                .iter()
                .map(ForgeQueryLowerRuntimeBoundaryReconciliationRow::row_digest)
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            report_digest,
        }
    }

    pub fn rows(&self) -> &[ForgeQueryLowerRuntimeBoundaryReconciliationRow] {
        &self.rows
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn forge_query_lower_runtime_boundary_reconciliation_report(
) -> ForgeQueryLowerRuntimeBoundaryReconciliationReport {
    let audit = forge_query_lower_runtime_direct_import_audit();
    let rows = forge_query_lower_runtime_public_surface_inventory()
        .rows()
        .iter()
        .map(|row| {
            let direct_import_posture = audit
                .rows()
                .iter()
                .find(|audit_row| audit_row.module_path() == row.implementation_path())
                .map(|audit_row| audit_row.posture());
            ForgeQueryLowerRuntimeBoundaryReconciliationRow::new(
                row.seam_key(),
                row.surface_label(),
                row.implementation_path(),
                row.surface_kind(),
                direct_import_posture,
            )
        })
        .collect::<Vec<_>>();

    for seam_key in REMAINING_PHASE_SIX_BOUNDARY_SEAMS {
        assert!(
            rows.iter().any(|row| row.seam_key == *seam_key),
            "remaining phase six seam {} must stay boundary-reconciled",
            seam_key.as_str()
        );
    }

    ForgeQueryLowerRuntimeBoundaryReconciliationReport::new(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reconciliation_report_covers_remaining_phase_six_boundary_seams() {
        let report = forge_query_lower_runtime_boundary_reconciliation_report();

        for seam_key in REMAINING_PHASE_SIX_BOUNDARY_SEAMS {
            assert!(report.rows().iter().any(|row| row.seam_key() == *seam_key));
        }
    }

    #[test]
    fn allowed_boundary_adapters_stay_audit_backed_inside_reconciliation_report() {
        let report = forge_query_lower_runtime_boundary_reconciliation_report();

        for row in report.rows() {
            match row.surface_kind() {
                ForgeQueryLowerRuntimePublicSurfaceKind::AllowedBoundaryAdapter => {
                    assert_eq!(
                        row.direct_import_posture(),
                        Some(ForgeQueryLowerRuntimeDirectImportPosture::AllowedAdapter)
                    );
                }
                ForgeQueryLowerRuntimePublicSurfaceKind::RuntimeBackendBoundary => {
                    assert_eq!(
                        row.direct_import_posture(),
                        Some(ForgeQueryLowerRuntimeDirectImportPosture::RuntimeBackendBoundary)
                    );
                }
                _ => {}
            }
        }
    }
}
