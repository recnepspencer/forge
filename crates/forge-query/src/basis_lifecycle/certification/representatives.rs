use crate::identity::hash_parts;

use super::{BasisLifecycleCertificationLane, BasisLifecycleCertificationRow};
use crate::basis_lifecycle::{
    admit_basis_capability, discover_basis_lifecycle_support, emit_observation_basis_receipt,
    envelope_basis_use, evaluate_basis_certification_eligibility,
    evaluate_basis_inspection_advisory_eligibility, evaluate_basis_materialization_eligibility,
    evaluate_basis_observation_eligibility, normalize_raw_basis_intent,
    readmit_lower_runtime_evidence, scope_basis_for_observation, BasisFamily, BasisOperationLane,
    CertificationLaneWitness, DeniedBasisCapabilityKind, InspectionLaneWitness,
    LowerRuntimeBasisEvidence, MaterializationLaneWitness, ObservationLaneWitness, RawBasisIntent,
};

pub(super) fn certification_rows() -> Vec<BasisLifecycleCertificationRow> {
    vec![
        admitted_certification_row(),
        advisory_certification_row(),
        denied_certification_row(),
        lower_runtime_mismatch_certification_row(),
        future_neighbor_certification_row(),
        performance_certification_row(),
    ]
}

fn admitted_certification_row() -> BasisLifecycleCertificationRow {
    let lane = <ObservationLaneWitness as BasisOperationLane>::lane_name();
    let normalized = normalize_raw_basis_intent(RawBasisIntent::CurrentHead, lane)
        .expect("certification current-head normalization must succeed");
    let normalized_digest = normalized.normalized_digest().to_string();
    let normalized_counter_digest = normalized.counters().digest();
    let eligibility =
        evaluate_basis_observation_eligibility(normalized).expect("certification must admit");
    let eligibility_counter_digest = eligibility.counters().digest();
    let decision_trace_digest = eligibility.decision_trace().trace_digest().to_string();
    let capability = admit_basis_capability(eligibility);
    let capability_digest = capability.capability_digest().to_string();
    let scoped = scope_basis_for_observation(capability);
    let scoped_digest = scoped.scoped_basis_digest().to_string();
    let scoped_counter_digest = scoped.counters().digest();
    let bound = readmit_lower_runtime_evidence(
        scoped,
        LowerRuntimeBasisEvidence::from_runtime_basis("runtime-current-head", "runtime-cert", 1),
    )
    .expect("certification lower-runtime readmission must bind");
    let bound_counter_digest = bound.counters().digest();
    let receipt = emit_observation_basis_receipt(bound);
    let receipt_digest = receipt.receipt_digest().to_string();
    let receipt_counter_digest = receipt.counters().digest();
    let envelope = envelope_basis_use(receipt);
    let envelope_counter_digest = envelope.counters().digest();

    BasisLifecycleCertificationRow::new(
        BasisLifecycleCertificationLane::Admitted,
        BasisFamily::CurrentHead,
        lane,
        hash_parts(&[
            normalized_digest,
            decision_trace_digest,
            capability_digest,
            scoped_digest,
            receipt_digest,
            envelope.envelope_digest().to_string(),
        ]),
        None,
        hash_parts(&[
            normalized_counter_digest,
            eligibility_counter_digest,
            scoped_counter_digest,
            bound_counter_digest,
            receipt_counter_digest,
            envelope_counter_digest,
        ]),
    )
}

fn advisory_certification_row() -> BasisLifecycleCertificationRow {
    let lane = <InspectionLaneWitness as BasisOperationLane>::lane_name();
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::PreviewDerived {
            preview_identity: "preview-cert".to_string(),
            source_basis_identity: "branch-cert".to_string(),
        },
        lane,
    )
    .expect("certification advisory normalization must succeed");
    let advisory = evaluate_basis_inspection_advisory_eligibility(normalized)
        .expect("certification advisory eligibility must succeed");

    BasisLifecycleCertificationRow::new(
        BasisLifecycleCertificationLane::Advisory,
        BasisFamily::PreviewDerived,
        lane,
        advisory.decision_trace().trace_digest().to_string(),
        None,
        hash_parts(&["advisory_no_operational_receipt".to_string()]),
    )
}

fn denied_certification_row() -> BasisLifecycleCertificationRow {
    let lane = <ObservationLaneWitness as BasisOperationLane>::lane_name();
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::PolicyScoped {
            policy_digest: "policy-cert".to_string(),
            tenant_identity: "tenant-cert".to_string(),
            branch_identity: "branch-cert".to_string(),
            schema_identity: "schema-cert".to_string(),
            tenant_schema_matches: true,
            policy_masks_operation: true,
            advisory_visibility: false,
        },
        lane,
    )
    .expect("certification denied normalization must succeed");
    let denial = evaluate_basis_observation_eligibility(normalized)
        .expect_err("certification policy-masked basis must deny");

    BasisLifecycleCertificationRow::new(
        BasisLifecycleCertificationLane::Denied,
        BasisFamily::PolicyScoped,
        lane,
        denial.decision_trace().trace_digest().to_string(),
        Some(denial_digest(denial.denial_kind())),
        denial.counters().digest(),
    )
}

fn lower_runtime_mismatch_certification_row() -> BasisLifecycleCertificationRow {
    let lane = <ObservationLaneWitness as BasisOperationLane>::lane_name();
    let normalized = normalize_raw_basis_intent(RawBasisIntent::CurrentHead, lane)
        .expect("certification lower-runtime normalization must succeed");
    let eligibility =
        evaluate_basis_observation_eligibility(normalized).expect("certification must admit");
    let capability = admit_basis_capability(eligibility);
    let scoped = scope_basis_for_observation(capability);
    let denial = readmit_lower_runtime_evidence(
        scoped,
        LowerRuntimeBasisEvidence::from_runtime_bridge_facade(
            "bridge-foreign",
            "bridge-foreign-evidence",
            2,
        ),
    )
    .expect_err("foreign bridge evidence must deny");

    BasisLifecycleCertificationRow::new(
        BasisLifecycleCertificationLane::LowerRuntimeMismatch,
        BasisFamily::CurrentHead,
        lane,
        denial.decision_trace().trace_digest().to_string(),
        Some(denial_digest(denial.denial_kind())),
        denial.counters().digest(),
    )
}

fn future_neighbor_certification_row() -> BasisLifecycleCertificationRow {
    let lane = <CertificationLaneWitness as BasisOperationLane>::lane_name();
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::DurableReload {
            reload_identity: "reload-cert".to_string(),
        },
        lane,
    )
    .expect("certification durable reload normalization must succeed");
    let denial = evaluate_basis_certification_eligibility(normalized)
        .expect_err("durable reload must remain deferred");

    BasisLifecycleCertificationRow::new(
        BasisLifecycleCertificationLane::FutureNeighborDenial,
        BasisFamily::DurableReload,
        lane,
        denial.decision_trace().trace_digest().to_string(),
        Some(denial_digest(denial.denial_kind())),
        denial.counters().digest(),
    )
}

fn performance_certification_row() -> BasisLifecycleCertificationRow {
    let support = discover_basis_lifecycle_support(
        BasisFamily::CurrentHead,
        <ObservationLaneWitness as BasisOperationLane>::lane_name(),
    );
    let materialization_support = discover_basis_lifecycle_support(
        BasisFamily::BranchHead,
        <MaterializationLaneWitness as BasisOperationLane>::lane_name(),
    );
    let materialization = normalize_raw_basis_intent(
        RawBasisIntent::BranchHead {
            branch_identity: "branch-cert".to_string(),
            accessible: true,
        },
        <MaterializationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("certification materialization normalization must succeed");
    let materialization_denial = evaluate_basis_materialization_eligibility(materialization)
        .expect_err("unsupported materialization lane must deny");

    BasisLifecycleCertificationRow::new(
        BasisLifecycleCertificationLane::Performance,
        BasisFamily::CurrentHead,
        <ObservationLaneWitness as BasisOperationLane>::lane_name(),
        hash_parts(&[
            support.discovery_digest().to_string(),
            materialization_support.discovery_digest().to_string(),
        ]),
        Some(denial_digest(materialization_denial.denial_kind())),
        hash_parts(&[
            support.counters().digest(),
            materialization_support.counters().digest(),
            materialization_denial.counters().digest(),
        ]),
    )
}

fn denial_digest(kind: DeniedBasisCapabilityKind) -> String {
    hash_parts(&[kind.as_str().to_string()])
}
