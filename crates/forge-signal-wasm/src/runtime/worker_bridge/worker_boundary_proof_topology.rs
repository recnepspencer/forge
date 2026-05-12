use serde::Serialize;
use std::any::type_name;

use crate::runtime::placement::declaration_classification::PlacementClassificationOutcome;
use crate::runtime::placement::lowering::lowered_plan_proof::{
    LoweredMainThreadHostedExecutionPlanProof, LoweredWorkerExecutionPlanProof,
};
use crate::runtime::placement::raw_declaration_proof::RawPlacementProof;
use crate::runtime::worker_bridge::worker_boundary_readmission_proof::BoundaryBridgedWorkerEnvelopeReadmissionProof;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerBoundaryProofStageSummary {
    pub label: &'static str,
    pub forge_proof_stage: &'static str,
    pub owner: &'static str,
    pub rust_type: &'static str,
}

pub(in crate::runtime::worker_bridge) fn worker_boundary_proof_stages(
) -> Vec<WorkerBoundaryProofStageSummary> {
    vec![
        WorkerBoundaryProofStageSummary {
            label: "rawPlacementDeclaration",
            forge_proof_stage: "Recipe<Unresolved, RawPlacementDeclaration>",
            owner: "runtime/placement/raw_declaration_proof",
            rust_type: type_name::<RawPlacementProof>(),
        },
        WorkerBoundaryProofStageSummary {
            label: "placementClassifiedDeclaration",
            forge_proof_stage:
                "TransitionOutcome<PlacementClassifiedDeclaration, PlacementDenialArtifact>",
            owner: "runtime/placement/declaration_classification",
            rust_type: type_name::<PlacementClassificationOutcome>(),
        },
        WorkerBoundaryProofStageSummary {
            label: "loweredWorkerExecutionPlan",
            forge_proof_stage: "Recipe<Lowered, LoweredWorkerExecutionPlan>",
            owner: "runtime/placement/lowering/lowered_plan_proof",
            rust_type: type_name::<LoweredWorkerExecutionPlanProof>(),
        },
        WorkerBoundaryProofStageSummary {
            label: "loweredMainThreadHostedExecutionPlan",
            forge_proof_stage: "Recipe<Lowered, LoweredMainThreadHostedExecutionPlan>",
            owner: "runtime/placement/lowering/lowered_plan_proof",
            rust_type: type_name::<LoweredMainThreadHostedExecutionPlanProof>(),
        },
        WorkerBoundaryProofStageSummary {
            label: "boundaryBridgedReadmission",
            forge_proof_stage: "Recipe<Admitted, BoundaryBridgedWorkerEnvelope>",
            owner: "runtime/worker_bridge/readmission_proof",
            rust_type: type_name::<BoundaryBridgedWorkerEnvelopeReadmissionProof>(),
        },
    ]
}
