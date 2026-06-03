use forge_query::facade::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQueryWorkspace,
};

use crate::construction::certification::{
    PrimitiveConstructionChosenIntentResolutionAuthority,
    PrimitiveConstructionChosenIntentResolutionRow,
    PrimitiveConstructionIntentArbitrationConflictClass,
    PrimitiveConstructionIntentArbitrationPolicyRow, PrimitiveConstructionObservedIntentRelation,
};
use crate::construction::digest::digest_owned_parts;
use worth_spatial::facade::arbitration::{SpatialIntentCandidate, SpatialIntentEscalation};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionIntentArbitrationQueryInspectionSurface {
    IntentArbitrationPolicyReportReceipt,
}

impl PrimitiveConstructionIntentArbitrationQueryInspectionSurface {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IntentArbitrationPolicyReportReceipt => {
                "intent_arbitration_policy_report_receipt"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionIntentArbitrationQueryReadSurface {
    IntentArbitrationPolicyInspection,
    ProjectionConsumptionFromIntentArbitrationPolicyReport,
}

impl PrimitiveConstructionIntentArbitrationQueryReadSurface {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IntentArbitrationPolicyInspection => "intent_arbitration_policy_inspection",
            Self::ProjectionConsumptionFromIntentArbitrationPolicyReport => {
                "projection_consumption_from_intent_arbitration_policy_report"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionIntentArbitrationQueryFactProvenance {
    DirectIntentArbitrationPolicyReport,
    EquivalentProjectionConsumptionFacts,
}

impl PrimitiveConstructionIntentArbitrationQueryFactProvenance {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DirectIntentArbitrationPolicyReport => "direct_intent_arbitration_policy_report",
            Self::EquivalentProjectionConsumptionFacts => "equivalent_projection_consumption_facts",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionIntentChosenTruth {
    Unresolved,
    Resolved {
        candidate: SpatialIntentCandidate,
        authority: PrimitiveConstructionChosenIntentResolutionAuthority,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionQueryIntentArbitrationParityReport {
    authored_act: worth_spatial::facade::arbitration::SpatialAuthoredActKind,
    observed_relations: Vec<PrimitiveConstructionObservedIntentRelation>,
    conflict_class: PrimitiveConstructionIntentArbitrationConflictClass,
    escalation: SpatialIntentEscalation,
    candidates: Vec<SpatialIntentCandidate>,
    blocked_candidates: Vec<(
        SpatialIntentCandidate,
        worth_spatial::facade::arbitration::SpatialBlockedCapability,
    )>,
    chosen_truth: PrimitiveConstructionIntentChosenTruth,
    query_contract_digest: String,
    required_query_families: Vec<ForgeQueryRuntimeFacadeFamily>,
    read_surface: PrimitiveConstructionIntentArbitrationQueryReadSurface,
    inspection_surface: PrimitiveConstructionIntentArbitrationQueryInspectionSurface,
    fact_provenance: PrimitiveConstructionIntentArbitrationQueryFactProvenance,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionQueryIntentArbitrationParityReport {
    fn new(
        query_contract_digest: String,
        policy_row: PrimitiveConstructionIntentArbitrationPolicyRow,
        chosen_row: Option<PrimitiveConstructionChosenIntentResolutionRow>,
        read_surface: PrimitiveConstructionIntentArbitrationQueryReadSurface,
        fact_provenance: PrimitiveConstructionIntentArbitrationQueryFactProvenance,
    ) -> Result<Self, PrimitiveConstructionQueryIntentArbitrationParityError> {
        let chosen_truth = match chosen_row {
            Some(row) => {
                if row.authored_act() != policy_row.authored_act()
                    || row.observed_relations() != policy_row.observed_relations()
                    || row.conflict_class() != policy_row.conflict_class()
                {
                    return Err(
                        PrimitiveConstructionQueryIntentArbitrationParityError::ChosenResolutionMismatch,
                    );
                }
                PrimitiveConstructionIntentChosenTruth::Resolved {
                    candidate: row.chosen_candidate(),
                    authority: row.authority(),
                }
            }
            None => match policy_row.chosen_candidate() {
                Some(candidate) => PrimitiveConstructionIntentChosenTruth::Resolved {
                    candidate,
                    authority:
                        PrimitiveConstructionChosenIntentResolutionAuthority::PolicyAutoResolve,
                },
                None => PrimitiveConstructionIntentChosenTruth::Unresolved,
            },
        };
        let required_query_families = vec![ForgeQueryRuntimeFacadeFamily::Inspect];
        let inspection_surface =
            PrimitiveConstructionIntentArbitrationQueryInspectionSurface::IntentArbitrationPolicyReportReceipt;
        let parity_verified = !query_contract_digest.is_empty()
            && matches!(
                (read_surface, fact_provenance),
                (
                    PrimitiveConstructionIntentArbitrationQueryReadSurface::IntentArbitrationPolicyInspection,
                    PrimitiveConstructionIntentArbitrationQueryFactProvenance::DirectIntentArbitrationPolicyReport
                ) | (
                    PrimitiveConstructionIntentArbitrationQueryReadSurface::ProjectionConsumptionFromIntentArbitrationPolicyReport,
                    PrimitiveConstructionIntentArbitrationQueryFactProvenance::EquivalentProjectionConsumptionFacts
                )
            )
            && match chosen_truth {
                PrimitiveConstructionIntentChosenTruth::Unresolved => policy_row.chosen_candidate().is_none(),
                PrimitiveConstructionIntentChosenTruth::Resolved { candidate, .. } => {
                    policy_row.candidates().contains(&candidate)
                }
            };
        let report_digest = digest_owned_parts(&[
            policy_row.authored_act().as_str().to_string(),
            format!("{:?}", policy_row.observed_relations()),
            format!("{:?}", policy_row.conflict_class()),
            format!("{:?}", policy_row.escalation()),
            format!("{:?}", policy_row.candidates()),
            format!("{:?}", policy_row.blocked_candidates()),
            format!("{chosen_truth:?}"),
            query_contract_digest.clone(),
            required_query_families
                .iter()
                .map(|family| format!("{family:?}"))
                .collect::<Vec<_>>()
                .join("|"),
            read_surface.as_str().to_string(),
            inspection_surface.as_str().to_string(),
            fact_provenance.as_str().to_string(),
            parity_verified.to_string(),
        ]);
        Ok(Self {
            authored_act: policy_row.authored_act(),
            observed_relations: policy_row.observed_relations().to_vec(),
            conflict_class: policy_row.conflict_class(),
            escalation: policy_row.escalation(),
            candidates: policy_row.candidates().to_vec(),
            blocked_candidates: policy_row.blocked_candidates().to_vec(),
            chosen_truth,
            query_contract_digest,
            required_query_families,
            read_surface,
            inspection_surface,
            fact_provenance,
            parity_verified,
            report_digest,
        })
    }

    pub fn authored_act(&self) -> worth_spatial::facade::arbitration::SpatialAuthoredActKind {
        self.authored_act
    }

    pub fn observed_relations(&self) -> &[PrimitiveConstructionObservedIntentRelation] {
        &self.observed_relations
    }

    pub fn conflict_class(&self) -> PrimitiveConstructionIntentArbitrationConflictClass {
        self.conflict_class
    }

    pub fn escalation(&self) -> SpatialIntentEscalation {
        self.escalation
    }

    pub fn candidates(&self) -> &[SpatialIntentCandidate] {
        &self.candidates
    }

    pub fn blocked_candidates(
        &self,
    ) -> &[(
        SpatialIntentCandidate,
        worth_spatial::facade::arbitration::SpatialBlockedCapability,
    )] {
        &self.blocked_candidates
    }

    pub fn chosen_truth(&self) -> PrimitiveConstructionIntentChosenTruth {
        self.chosen_truth
    }

    pub fn query_contract_digest(&self) -> &str {
        &self.query_contract_digest
    }

    pub fn required_query_families(&self) -> &[ForgeQueryRuntimeFacadeFamily] {
        &self.required_query_families
    }

    pub fn read_surface(&self) -> PrimitiveConstructionIntentArbitrationQueryReadSurface {
        self.read_surface
    }

    pub fn inspection_surface(
        &self,
    ) -> PrimitiveConstructionIntentArbitrationQueryInspectionSurface {
        self.inspection_surface
    }

    pub fn fact_provenance(&self) -> PrimitiveConstructionIntentArbitrationQueryFactProvenance {
        self.fact_provenance
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionQueryIntentArbitrationParityError {
    QueryRuntime(ForgeQueryRuntimeError),
    ChosenResolutionMismatch,
}

impl std::fmt::Display for PrimitiveConstructionQueryIntentArbitrationParityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueryRuntime(error) => write!(f, "{error}"),
            Self::ChosenResolutionMismatch => {
                write!(
                    f,
                    "chosen intent resolution does not match policy row truth"
                )
            }
        }
    }
}

impl std::error::Error for PrimitiveConstructionQueryIntentArbitrationParityError {}

pub fn prepare_primitive_construction_query_intent_arbitration_inspection_parity_report(
    workspace: &mut ForgeQueryWorkspace,
    policy_row: PrimitiveConstructionIntentArbitrationPolicyRow,
    chosen_row: Option<PrimitiveConstructionChosenIntentResolutionRow>,
) -> Result<
    PrimitiveConstructionQueryIntentArbitrationParityReport,
    PrimitiveConstructionQueryIntentArbitrationParityError,
> {
    let query_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Inspect)
        .map_err(PrimitiveConstructionQueryIntentArbitrationParityError::QueryRuntime)?
        .contract_digest()
        .to_string();
    PrimitiveConstructionQueryIntentArbitrationParityReport::new(
        query_contract_digest,
        policy_row,
        chosen_row,
        PrimitiveConstructionIntentArbitrationQueryReadSurface::IntentArbitrationPolicyInspection,
        PrimitiveConstructionIntentArbitrationQueryFactProvenance::DirectIntentArbitrationPolicyReport,
    )
}

pub fn prepare_primitive_construction_query_intent_arbitration_projection_consumption_receipt_report(
    workspace: &mut ForgeQueryWorkspace,
    policy_row: PrimitiveConstructionIntentArbitrationPolicyRow,
    chosen_row: Option<PrimitiveConstructionChosenIntentResolutionRow>,
) -> Result<
    PrimitiveConstructionQueryIntentArbitrationParityReport,
    PrimitiveConstructionQueryIntentArbitrationParityError,
> {
    let query_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Inspect)
        .map_err(PrimitiveConstructionQueryIntentArbitrationParityError::QueryRuntime)?
        .contract_digest()
        .to_string();
    PrimitiveConstructionQueryIntentArbitrationParityReport::new(
        query_contract_digest,
        policy_row,
        chosen_row,
        PrimitiveConstructionIntentArbitrationQueryReadSurface::ProjectionConsumptionFromIntentArbitrationPolicyReport,
        PrimitiveConstructionIntentArbitrationQueryFactProvenance::EquivalentProjectionConsumptionFacts,
    )
}

#[cfg(test)]
mod tests {
    use super::{
        prepare_primitive_construction_query_intent_arbitration_inspection_parity_report,
        prepare_primitive_construction_query_intent_arbitration_projection_consumption_receipt_report,
        PrimitiveConstructionIntentArbitrationQueryFactProvenance,
        PrimitiveConstructionIntentArbitrationQueryReadSurface,
        PrimitiveConstructionIntentChosenTruth,
    };
    use crate::construction::{
        prepare_primitive_chosen_intent_resolution_report,
        prepare_primitive_intent_arbitration_policy_report,
        PrimitiveConstructionChosenIntentResolutionCase,
        PrimitiveConstructionIntentArbitrationPolicyCase,
    };
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters,
    };
    use worth_spatial::facade::arbitration::SpatialIntentCandidate;

    #[test]
    fn query_arbitration_inspection_report_preserves_unresolved_conflict_truth() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.query-intent-arbitration".to_string(),
        )
        .expect("workspace");
        let policy = prepare_primitive_intent_arbitration_policy_report().expect("policy");
        let report =
            prepare_primitive_construction_query_intent_arbitration_inspection_parity_report(
                &mut workspace,
                policy
                    .row(PrimitiveConstructionIntentArbitrationPolicyCase::GrazingSnapConflict)
                    .expect("grazing")
                    .clone(),
                None,
            )
            .expect("inspection report");

        assert_eq!(
            report.read_surface(),
            PrimitiveConstructionIntentArbitrationQueryReadSurface::IntentArbitrationPolicyInspection
        );
        assert_eq!(
            report.chosen_truth(),
            PrimitiveConstructionIntentChosenTruth::Unresolved
        );
        assert!(report.parity_verified());
    }

    #[test]
    fn query_arbitration_projection_report_preserves_explicit_choice_truth() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.query-intent-arbitration-choice".to_string(),
        )
        .expect("workspace");
        let policy = prepare_primitive_intent_arbitration_policy_report().expect("policy");
        let chosen = prepare_primitive_chosen_intent_resolution_report().expect("chosen");
        let report = prepare_primitive_construction_query_intent_arbitration_projection_consumption_receipt_report(
            &mut workspace,
            policy
                .row(PrimitiveConstructionIntentArbitrationPolicyCase::GrazingSnapConflict)
                .expect("grazing")
                .clone(),
            Some(
                chosen
                    .row(PrimitiveConstructionChosenIntentResolutionCase::ExplicitSnapFlush)
                    .expect("chosen row")
                    .clone(),
            ),
        )
        .expect("projection report");

        assert_eq!(
            report.fact_provenance(),
            PrimitiveConstructionIntentArbitrationQueryFactProvenance::EquivalentProjectionConsumptionFacts
        );
        assert_eq!(
            report.chosen_truth(),
            PrimitiveConstructionIntentChosenTruth::Resolved {
                candidate: SpatialIntentCandidate::SnapFlush,
                authority: crate::construction::PrimitiveConstructionChosenIntentResolutionAuthority::ExplicitChoice,
            }
        );
        assert!(report.parity_verified());
    }
}
