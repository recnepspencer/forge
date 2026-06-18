use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::workload_vocabulary::{
    BooleanEvidenceReceipt, BooleanEvidenceRowAuthority, BooleanEvidenceStageKind,
    WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};

use super::{
    BuiltBooleanOperandPairRecipe, PlanarBooleanCommonPlaneLocalFrameSelectedRequest,
    PlanarBooleanCommonPlaneOperandAProjectedRequest,
    PlanarBooleanCommonPlaneOperandBProjectedRequest,
    PlanarBooleanCommonPlanePrecisionAgreedRequest,
    PlanarBooleanCommonPlaneReducedOperandPairRequest,
    PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest, PlanarBooleanDeclarationReceipt,
    PlanarBooleanEventExtractionRequest, PlanarBooleanOutcomeKind, PlanarBooleanOutcomeReceipt,
    PlanarBooleanSupportPosture, PlanarBooleanSupportReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOperandPairConstructionReceipt {
    construction_digest: String,
    operand_pair_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanBlockerEvidenceReceipt {
    blocker_digest: String,
    support: WorkloadEvidenceSupport,
}

impl PlanarBooleanOperandPairConstructionReceipt {
    pub(crate) fn from_built_recipe(recipe: &BuiltBooleanOperandPairRecipe) -> Self {
        Self {
            construction_digest: truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &[
                    "planar-boolean-operand-pair-construction".to_string(),
                    format!("recipe:{}", recipe.recipe().query_key()),
                    format!(
                        "catalog-declaration:{}",
                        recipe.declaration().query_declaration_digest()
                    ),
                    format!(
                        "catalog-support:{}",
                        recipe.support().query_support_digest()
                    ),
                    format!("pair:{}", recipe.operand_pair_identity()),
                    format!(
                        "left:{}",
                        recipe
                            .left()
                            .workload()
                            .response()
                            .identity()
                            .receipt_identity()
                    ),
                    format!(
                        "right:{}",
                        recipe
                            .right()
                            .workload()
                            .response()
                            .identity()
                            .receipt_identity()
                    ),
                ],
            ),
            operand_pair_identity: recipe.operand_pair_identity().to_string(),
        }
    }

    pub fn construction_digest(&self) -> &str {
        &self.construction_digest
    }

    pub fn operand_pair_identity(&self) -> &str {
        &self.operand_pair_identity
    }
}

impl PlanarBooleanBlockerEvidenceReceipt {
    pub fn from_outcome(outcome: &PlanarBooleanOutcomeReceipt) -> Option<Self> {
        let blocker = outcome.blocker_provenance()?;
        Some(Self {
            blocker_digest: blocker.provenance_digest().to_string(),
            support: match outcome.kind() {
                PlanarBooleanOutcomeKind::Unsupported
                | PlanarBooleanOutcomeKind::PolicyRequired => WorkloadEvidenceSupport::Unsupported,
                PlanarBooleanOutcomeKind::Blocked
                | PlanarBooleanOutcomeKind::Denied
                | PlanarBooleanOutcomeKind::IntegrityMismatch
                | PlanarBooleanOutcomeKind::NoOptions => WorkloadEvidenceSupport::Blocked,
                PlanarBooleanOutcomeKind::Admitted => return None,
            },
        })
    }

    pub fn blocker_digest(&self) -> &str {
        &self.blocker_digest
    }
}

impl BooleanEvidenceReceipt for PlanarBooleanDeclarationReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::DeclarationEntry
    }

    fn evidence_identity(&self) -> &str {
        self.query_declaration_digest()
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_declaration()
    }
}

impl BooleanEvidenceRowAuthority for PlanarBooleanDeclarationReceipt {}

impl BooleanEvidenceReceipt for PlanarBooleanSupportReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::RoutePlan
    }

    fn evidence_identity(&self) -> &str {
        self.query_support_digest()
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        match self.posture() {
            PlanarBooleanSupportPosture::Admitted => WorkloadEvidenceSupport::Admitted,
            PlanarBooleanSupportPosture::VisibleNotAdmitted => WorkloadEvidenceSupport::Unsupported,
        }
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_route()
    }
}

impl BooleanEvidenceRowAuthority for PlanarBooleanSupportReceipt {}

impl BooleanEvidenceReceipt for PlanarBooleanOperandPairConstructionReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::OperandPairConstruction
    }

    fn evidence_identity(&self) -> &str {
        self.construction_digest()
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_operand_pair()
    }
}

impl BooleanEvidenceRowAuthority for PlanarBooleanOperandPairConstructionReceipt {}

impl BooleanEvidenceReceipt for PlanarBooleanBlockerEvidenceReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::BlockerProvenance
    }

    fn evidence_identity(&self) -> &str {
        self.blocker_digest()
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        self.support
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_blocker()
    }
}

impl BooleanEvidenceRowAuthority for PlanarBooleanBlockerEvidenceReceipt {}

impl BooleanEvidenceReceipt for PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::SharedPlaneIdentity
    }

    fn evidence_identity(&self) -> &str {
        self.shared_plane_identified_request_identity()
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_shared_plane_identity()
    }
}

impl BooleanEvidenceRowAuthority for PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest {}

impl BooleanEvidenceReceipt for PlanarBooleanCommonPlaneLocalFrameSelectedRequest {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::LocalFrameSelection
    }

    fn evidence_identity(&self) -> &str {
        self.local_frame_selection_identity()
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_local_frame_selection()
    }
}

impl BooleanEvidenceRowAuthority for PlanarBooleanCommonPlaneLocalFrameSelectedRequest {}

impl BooleanEvidenceReceipt for PlanarBooleanCommonPlaneOperandAProjectedRequest {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::OperandAProjectionConsumption
    }

    fn evidence_identity(&self) -> &str {
        self.operand_a_projection_identity()
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_operand_a_projection_consumption()
    }
}

impl BooleanEvidenceRowAuthority for PlanarBooleanCommonPlaneOperandAProjectedRequest {}

impl BooleanEvidenceReceipt for PlanarBooleanCommonPlaneOperandBProjectedRequest {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::OperandBProjectionConsumption
    }

    fn evidence_identity(&self) -> &str {
        self.operand_b_projection_identity()
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_operand_b_projection_consumption()
    }
}

impl BooleanEvidenceRowAuthority for PlanarBooleanCommonPlaneOperandBProjectedRequest {}

impl BooleanEvidenceReceipt for PlanarBooleanCommonPlaneReducedOperandPairRequest {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::ReducedOperandPair
    }

    fn evidence_identity(&self) -> &str {
        self.reduced_operand_pair_request_identity()
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_reduced_operand_pair()
    }
}

impl BooleanEvidenceRowAuthority for PlanarBooleanCommonPlaneReducedOperandPairRequest {}

impl BooleanEvidenceReceipt for PlanarBooleanEventExtractionRequest {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::EventExtractionRequest
    }

    fn evidence_identity(&self) -> &str {
        self.event_extraction_request_identity()
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_event_extraction_request()
    }
}

impl BooleanEvidenceRowAuthority for PlanarBooleanEventExtractionRequest {}

impl BooleanEvidenceReceipt for PlanarBooleanCommonPlanePrecisionAgreedRequest {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::PrecisionAgreement
    }

    fn evidence_identity(&self) -> &str {
        self.precision_agreement_identity()
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_precision_agreement()
    }
}

impl BooleanEvidenceRowAuthority for PlanarBooleanCommonPlanePrecisionAgreedRequest {}
