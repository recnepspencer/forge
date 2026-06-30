use super::super::WorkloadEvidenceStage;
use super::WorkloadEvidenceStageCounters;

impl WorkloadEvidenceStageCounters {
    pub fn has_receipt_backed_counter_for_stage(self, stage: WorkloadEvidenceStage) -> bool {
        self.receipt_backed_counter_for_stage(stage) > 0
    }

    fn receipt_backed_counter_for_stage(self, stage: WorkloadEvidenceStage) -> usize {
        match stage {
            WorkloadEvidenceStage::Topology => self.topology_entity_count,
            WorkloadEvidenceStage::GeometryBinding => self.binding_target_count,
            WorkloadEvidenceStage::SurfaceSupport => self.surface_support_count,
            WorkloadEvidenceStage::Projection => self.projected_entity_count,
            WorkloadEvidenceStage::Transform => self.transform_step_count,
            WorkloadEvidenceStage::RetainedReplay => self.retained_artifact_count,
            WorkloadEvidenceStage::BatchAdmissionExecution => {
                self.batch_admission_execution_count
            }
            WorkloadEvidenceStage::Diagnostics => self.diagnostic_count,
            WorkloadEvidenceStage::Response => self.user_outcome_count,
            WorkloadEvidenceStage::Operator => self.operator_receipt_count,
            WorkloadEvidenceStage::BooleanDeclarationEntry => self.boolean_declaration_count,
            WorkloadEvidenceStage::BooleanRoutePlan => self.boolean_route_count,
            WorkloadEvidenceStage::BooleanOperandPairConstruction => {
                self.boolean_operand_pair_count
            }
            WorkloadEvidenceStage::BooleanBlockerProvenance => self.boolean_blocker_count,
            WorkloadEvidenceStage::BooleanPrecisionAgreement => {
                self.boolean_precision_agreement_count
            }
            WorkloadEvidenceStage::BooleanSharedPlaneIdentity => {
                self.boolean_shared_plane_identity_count
            }
            WorkloadEvidenceStage::BooleanLocalFrameSelection => {
                self.boolean_local_frame_selection_count
            }
            WorkloadEvidenceStage::BooleanOperandAProjectionConsumption => {
                self.boolean_operand_a_projection_consumption_count
            }
            WorkloadEvidenceStage::BooleanOperandBProjectionConsumption => {
                self.boolean_operand_b_projection_consumption_count
            }
            WorkloadEvidenceStage::BooleanReducedOperandPair => {
                self.boolean_reduced_operand_pair_count
            }
            WorkloadEvidenceStage::BooleanEventExtractionRequest => {
                self.boolean_event_extraction_request_count
            }
            WorkloadEvidenceStage::BooleanSegmentPairEnumeration => {
                self.boolean_segment_pair_enumeration_count
            }
            WorkloadEvidenceStage::BooleanEventLedger => self.boolean_event_ledger_count,
            WorkloadEvidenceStage::BooleanSplit => self.boolean_split_count,
            WorkloadEvidenceStage::BooleanLoopReconstruction => {
                self.boolean_loop_reconstruction_count
            }
            WorkloadEvidenceStage::BooleanClassify => self.boolean_classify_count,
            WorkloadEvidenceStage::BooleanAssemble => self.boolean_assemble_count,
            WorkloadEvidenceStage::BooleanCleanup => self.boolean_cleanup_count,
        }
    }
}
