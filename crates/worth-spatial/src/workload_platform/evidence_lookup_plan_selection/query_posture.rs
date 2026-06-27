use crate::workload_platform::evidence_lookup_family_catalog::{
    EvidenceLookupFamilyQueryPosture, EvidenceLookupProjectionFactFamily,
    EvidenceLookupQueryImportEvidence,
};
use crate::workload_platform::evidence_lookup_input_admission::{
    EvidenceLookupQueryAdmissionSupport, EvidenceLookupQuerySupportState,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceLookupPlanQueryPostureState {
    NotRequired,
    NotEvaluatedForUnaffectedFamily,
    Satisfied {
        imported_evidence_digest: String,
        surface: EvidenceLookupPlanQuerySurface,
        query_surface_type_name: &'static str,
        projection_fact_family: Option<EvidenceLookupProjectionFactFamily>,
    },
    RequiredButMissing {
        required_evidence_digest: String,
        surface: EvidenceLookupPlanQuerySurface,
        query_surface_type_name: &'static str,
        projection_fact_family: Option<EvidenceLookupProjectionFactFamily>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupPlanQuerySurface {
    NotRequired,
    NotEvaluatedForUnaffectedFamily,
    ConsumerKitSupportPin,
    ProjectionConsumptionReceipt,
    LowerRuntimeBoundaryEnvelope,
    RequiredButMissing,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupPlanQueryPosture {
    state: EvidenceLookupPlanQueryPostureState,
}

impl EvidenceLookupPlanQueryPosture {
    pub(crate) const fn not_evaluated_for_unaffected_family() -> Self {
        Self {
            state: EvidenceLookupPlanQueryPostureState::NotEvaluatedForUnaffectedFamily,
        }
    }

    pub(crate) fn from_family_and_admission(
        family_posture: &EvidenceLookupFamilyQueryPosture,
        support: Option<&EvidenceLookupQueryAdmissionSupport>,
    ) -> Self {
        if !family_posture.requires_query_evidence() {
            return Self {
                state: EvidenceLookupPlanQueryPostureState::NotRequired,
            };
        }
        let Some(required_evidence) = family_posture.imported_evidence() else {
            return Self {
                state: EvidenceLookupPlanQueryPostureState::RequiredButMissing {
                    required_evidence_digest: "missing-query-import-declaration".to_string(),
                    surface: EvidenceLookupPlanQuerySurface::RequiredButMissing,
                    query_surface_type_name: "missing-query-import-declaration",
                    projection_fact_family: None,
                },
            };
        };
        let Some(support) = support else {
            return Self {
                state: EvidenceLookupPlanQueryPostureState::RequiredButMissing {
                    required_evidence_digest: required_evidence.evidence_digest().to_string(),
                    surface: surface_from_imported_evidence(required_evidence),
                    query_surface_type_name: required_evidence.query_surface_type_name(),
                    projection_fact_family: required_evidence.projection_fact_family(),
                },
            };
        };
        match support.state() {
            EvidenceLookupQuerySupportState::Satisfied {
                imported_evidence_digest,
                query_surface_type_name,
            } if imported_evidence_digest == required_evidence.evidence_digest()
                && *query_surface_type_name == required_evidence.query_surface_type_name() =>
            {
                Self {
                    state: EvidenceLookupPlanQueryPostureState::Satisfied {
                        imported_evidence_digest: imported_evidence_digest.clone(),
                        surface: surface_from_imported_evidence(required_evidence),
                        query_surface_type_name,
                        projection_fact_family: required_evidence.projection_fact_family(),
                    },
                }
            }
            _ => Self {
                state: EvidenceLookupPlanQueryPostureState::RequiredButMissing {
                    required_evidence_digest: required_evidence.evidence_digest().to_string(),
                    surface: surface_from_imported_evidence(required_evidence),
                    query_surface_type_name: required_evidence.query_surface_type_name(),
                    projection_fact_family: required_evidence.projection_fact_family(),
                },
            },
        }
    }

    pub const fn state(&self) -> &EvidenceLookupPlanQueryPostureState {
        &self.state
    }

    pub const fn is_missing_required_query_posture(&self) -> bool {
        matches!(
            self.state,
            EvidenceLookupPlanQueryPostureState::RequiredButMissing { .. }
        )
    }

    pub const fn surface(&self) -> EvidenceLookupPlanQuerySurface {
        match &self.state {
            EvidenceLookupPlanQueryPostureState::NotRequired => {
                EvidenceLookupPlanQuerySurface::NotRequired
            }
            EvidenceLookupPlanQueryPostureState::NotEvaluatedForUnaffectedFamily => {
                EvidenceLookupPlanQuerySurface::NotEvaluatedForUnaffectedFamily
            }
            EvidenceLookupPlanQueryPostureState::Satisfied { surface, .. } => *surface,
            EvidenceLookupPlanQueryPostureState::RequiredButMissing { .. } => {
                EvidenceLookupPlanQuerySurface::RequiredButMissing
            }
        }
    }

    pub const fn projection_fact_family(&self) -> Option<EvidenceLookupProjectionFactFamily> {
        match &self.state {
            EvidenceLookupPlanQueryPostureState::Satisfied {
                projection_fact_family,
                ..
            }
            | EvidenceLookupPlanQueryPostureState::RequiredButMissing {
                projection_fact_family,
                ..
            } => *projection_fact_family,
            EvidenceLookupPlanQueryPostureState::NotRequired
            | EvidenceLookupPlanQueryPostureState::NotEvaluatedForUnaffectedFamily => None,
        }
    }

    pub const fn requires_projection_consumption_receipt(&self) -> bool {
        matches!(
            self.surface(),
            EvidenceLookupPlanQuerySurface::ProjectionConsumptionReceipt
        )
    }

    pub fn satisfied_digest_summary(&self) -> Option<String> {
        match &self.state {
            EvidenceLookupPlanQueryPostureState::Satisfied {
                imported_evidence_digest,
                query_surface_type_name,
                ..
            } => Some(format!(
                "{query_surface_type_name}:{imported_evidence_digest}"
            )),
            EvidenceLookupPlanQueryPostureState::NotRequired
            | EvidenceLookupPlanQueryPostureState::NotEvaluatedForUnaffectedFamily
            | EvidenceLookupPlanQueryPostureState::RequiredButMissing { .. } => None,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        match &self.state {
            EvidenceLookupPlanQueryPostureState::NotRequired => "query:not-required".to_string(),
            EvidenceLookupPlanQueryPostureState::NotEvaluatedForUnaffectedFamily => {
                "query:not-evaluated-unaffected-family".to_string()
            }
            EvidenceLookupPlanQueryPostureState::Satisfied {
                imported_evidence_digest,
                query_surface_type_name,
                ..
            } => format!("query:satisfied:{query_surface_type_name}:{imported_evidence_digest}"),
            EvidenceLookupPlanQueryPostureState::RequiredButMissing {
                required_evidence_digest,
                query_surface_type_name,
                ..
            } => format!(
                "query:required-missing:{query_surface_type_name}:{required_evidence_digest}"
            ),
        }
    }
}

fn surface_from_imported_evidence(
    required_evidence: &EvidenceLookupQueryImportEvidence,
) -> EvidenceLookupPlanQuerySurface {
    match required_evidence {
        EvidenceLookupQueryImportEvidence::ConsumerKitSupportPin { .. } => {
            EvidenceLookupPlanQuerySurface::ConsumerKitSupportPin
        }
        EvidenceLookupQueryImportEvidence::ProjectionConsumptionReceipt { .. } => {
            EvidenceLookupPlanQuerySurface::ProjectionConsumptionReceipt
        }
        EvidenceLookupQueryImportEvidence::LowerRuntimeBoundaryEnvelope { .. } => {
            EvidenceLookupPlanQuerySurface::LowerRuntimeBoundaryEnvelope
        }
    }
}
