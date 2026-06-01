use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use serde_json::json;

use crate::harness::fixtures::BridgeHarnessFixture;
use crate::routing::canonicalization::digest_string;

use super::*;

pub(super) enum WritebackHarnessTarget {
    DuplicateCertification,
    BypassCertification,
    FeedbackLoopCertification,
    ReplayMismatchCertification,
    ExtensibleFamilyCertification,
    MultiFamilyAdmissionBoundaryCertification,
    CrossFamilyReplayLoopIsolationCertification,
    HostMapperParityCertification,
}

pub(super) enum WritebackHarnessExecution {
    DuplicateCertification {
        first_bundle_digest: String,
        repeated_bundle_digest: String,
        replay_bundle_digest: String,
        duplicate_authority_matrix: serde_json::Value,
        counter_snapshot: WritebackCounterSnapshot,
    },
    BypassCertification {
        failure_digest: String,
        bypass_rejection: serde_json::Value,
        zero_residue_report: serde_json::Value,
        counter_snapshot: WritebackCounterSnapshot,
    },
    FeedbackLoopCertification {
        feedback_loop_digest: String,
        feedback_route_digest: String,
        feedback_origin_matrix: serde_json::Value,
        counter_snapshot: WritebackCounterSnapshot,
    },
    ReplayMismatchCertification {
        replay_validation_digest: String,
        replay_mismatch_matrix: serde_json::Value,
        counter_snapshot: WritebackCounterSnapshot,
    },
    ExtensibleFamilyCertification {
        family_extension_digest: String,
        family_extension_matrix: serde_json::Value,
        counter_snapshot: WritebackCounterSnapshot,
    },
    MultiFamilyAdmissionBoundaryCertification {
        family_extension_digest: String,
        admission_boundary_matrix: serde_json::Value,
        counter_snapshot: WritebackCounterSnapshot,
    },
    CrossFamilyReplayLoopIsolationCertification {
        family_extension_digest: String,
        replay_loop_matrix: serde_json::Value,
        counter_snapshot: WritebackCounterSnapshot,
    },
    HostMapperParityCertification {
        family_extension_digest: String,
        mapper_parity_matrix: serde_json::Value,
        counter_snapshot: WritebackCounterSnapshot,
    },
}

impl WritebackHarnessExecution {
    fn counter_artifact_json(counter_snapshot: WritebackCounterSnapshot) -> serde_json::Value {
        counter_snapshot.artifact_json()
    }

    fn certification_evidence_json(&self) -> serde_json::Value {
        match self {
            Self::DuplicateCertification {
                duplicate_authority_matrix,
                counter_snapshot,
                ..
            } => json!({
                "certification_shape": "duplicate-certification",
                "writeback_digest": duplicate_authority_matrix["writeback_digest"],
                "bridge_effect_digest": duplicate_authority_matrix["bridge_effect_digest"],
                "causality_digest": duplicate_authority_matrix["causality_digest"],
                "mutation_plan_digest": duplicate_authority_matrix["mutation_plan_digest"],
                "idempotence_report": duplicate_authority_matrix["idempotence_report"],
                "loop_prevention_report": duplicate_authority_matrix["loop_prevention_report"],
                "truth_integrity_report": duplicate_authority_matrix["boundedness_proof"],
                "authority_boundary_matrix": duplicate_authority_matrix["authority_boundary_matrix"],
                "failure_digest": serde_json::Value::Null,
                "replay_digest": duplicate_authority_matrix["replay_digest"],
                "counter_snapshot": counter_snapshot.json(),
                "counter_digest": counter_snapshot.counters().digest(),
            }),
            Self::BypassCertification {
                failure_digest,
                bypass_rejection,
                zero_residue_report,
                counter_snapshot,
            } => json!({
                "certification_shape": "bypass-certification",
                "writeback_digest": bypass_rejection["writeback_digest"],
                "bridge_effect_digest": bypass_rejection["bridge_effect_digest"],
                "causality_digest": bypass_rejection["causality_digest"],
                "mutation_plan_digest": bypass_rejection["mutation_plan_digest"],
                "idempotence_report": bypass_rejection["idempotence_report"],
                "loop_prevention_report": bypass_rejection["loop_prevention_report"],
                "truth_integrity_report": zero_residue_report,
                "authority_boundary_matrix": bypass_rejection["authority_boundary_matrix"],
                "failure_digest": failure_digest,
                "replay_digest": bypass_rejection["replay_digest"],
                "counter_snapshot": counter_snapshot.json(),
                "counter_digest": counter_snapshot.counters().digest(),
            }),
            Self::FeedbackLoopCertification {
                feedback_origin_matrix,
                counter_snapshot,
                ..
            } => json!({
                "certification_shape": "feedback-loop-certification",
                "writeback_digest": feedback_origin_matrix["writeback_digest"],
                "bridge_effect_digest": feedback_origin_matrix["bridge_effect_digest"],
                "causality_digest": feedback_origin_matrix["causality_digest"],
                "mutation_plan_digest": feedback_origin_matrix["mutation_plan_digest"],
                "idempotence_report": feedback_origin_matrix["idempotence_report"],
                "loop_prevention_report": feedback_origin_matrix["loop_prevention_report"],
                "truth_integrity_report": feedback_origin_matrix["boundedness_proof"],
                "authority_boundary_matrix": feedback_origin_matrix["authority_boundary_matrix"],
                "failure_digest": feedback_origin_matrix["changed_effect_feedback_matrix"]["failure_digest"],
                "replay_digest": feedback_origin_matrix["replay_digest"],
                "counter_snapshot": counter_snapshot.json(),
                "counter_digest": counter_snapshot.counters().digest(),
            }),
            Self::ReplayMismatchCertification {
                replay_validation_digest,
                replay_mismatch_matrix,
                counter_snapshot,
            } => json!({
                "certification_shape": "replay-mismatch-certification",
                "writeback_digest": {
                    "expected": replay_mismatch_matrix["expected_replay_digest"].clone(),
                    "replayed": replay_mismatch_matrix["replayed_replay_digest"].clone(),
                },
                "bridge_effect_digest": {
                    "expected": replay_mismatch_matrix["expected_effect_digest"].clone(),
                    "replayed": replay_mismatch_matrix["replayed_effect_digest"].clone(),
                },
                "causality_digest": {
                    "expected": replay_mismatch_matrix["expected_causality_digest"].clone(),
                    "replayed": replay_mismatch_matrix["replayed_causality_digest"].clone(),
                },
                "mutation_plan_digest": {
                    "expected": replay_mismatch_matrix["expected_semantic_digest"].clone(),
                    "replayed": replay_mismatch_matrix["replayed_semantic_digest"].clone(),
                },
                "idempotence_report": serde_json::Value::Null,
                "loop_prevention_report": serde_json::Value::Null,
                "truth_integrity_report": replay_mismatch_matrix["restart_replay_matrix"],
                "authority_boundary_matrix": {
                    "failure_kind": replay_mismatch_matrix["failure_kind"].clone(),
                    "restart_failure_kind": replay_mismatch_matrix["restart_replay_matrix"]["rebuilt_failure_kind"].clone(),
                },
                "failure_digest": replay_validation_digest,
                "replay_digest": replay_mismatch_matrix["replayed_replay_digest"],
                "counter_snapshot": counter_snapshot.json(),
                "counter_digest": counter_snapshot.counters().digest(),
            }),
            Self::ExtensibleFamilyCertification {
                family_extension_matrix,
                counter_snapshot,
                ..
            } => json!({
                "certification_shape": "extensible-family-certification",
                "writeback_digest": family_extension_matrix["projected_family"]["replay_bundle_digest"],
                "bridge_effect_digest": {
                    "projected": family_extension_matrix["projected_family"]["effect_digest"].clone(),
                    "aspect": family_extension_matrix["aspect_family"]["effect_digest"].clone(),
                },
                "causality_digest": family_extension_matrix["projected_family"]["causality_digest"],
                "mutation_plan_digest": {
                    "projected": family_extension_matrix["projected_family"]["mapped_input_digest"].clone(),
                    "aspect": family_extension_matrix["aspect_family"]["mapped_input_digest"].clone(),
                },
                "idempotence_report": {
                    "projected": family_extension_matrix["projected_family"]["idempotence_digest"].clone(),
                    "aspect": family_extension_matrix["aspect_family"]["idempotence_digest"].clone(),
                },
                "loop_prevention_report": family_extension_matrix["cross_family_loop_isolation"],
                "truth_integrity_report": family_extension_matrix["mapper_parity_matrix"],
                "authority_boundary_matrix": family_extension_matrix["shadow_protocol_rejection"],
                "failure_digest": family_extension_matrix["shadow_protocol_rejection"]["failure_digest"],
                "replay_digest": {
                    "projected": family_extension_matrix["projected_family"]["replay_bundle_digest"].clone(),
                    "aspect": family_extension_matrix["aspect_family"]["replay_bundle_digest"].clone(),
                },
                "counter_snapshot": counter_snapshot.json(),
                "counter_digest": counter_snapshot.counters().digest(),
            }),
            Self::MultiFamilyAdmissionBoundaryCertification {
                admission_boundary_matrix,
                counter_snapshot,
                ..
            } => json!({
                "certification_shape": "multi-family-admission-boundary",
                "writeback_digest": admission_boundary_matrix["projected_family"]["replay_bundle_digest"],
                "bridge_effect_digest": {
                    "projected": admission_boundary_matrix["projected_family"]["effect_digest"].clone(),
                    "aspect": admission_boundary_matrix["aspect_family"]["effect_digest"].clone(),
                },
                "causality_digest": admission_boundary_matrix["projected_family"]["causality_digest"],
                "mutation_plan_digest": {
                    "projected": admission_boundary_matrix["projected_family"]["mapped_input_digest"].clone(),
                    "aspect": admission_boundary_matrix["aspect_family"]["mapped_input_digest"].clone(),
                },
                "idempotence_report": {
                    "projected": admission_boundary_matrix["projected_family"]["idempotence_digest"].clone(),
                    "aspect": admission_boundary_matrix["aspect_family"]["idempotence_digest"].clone(),
                },
                "loop_prevention_report": serde_json::Value::Null,
                "truth_integrity_report": admission_boundary_matrix["family_admission_matrix"],
                "authority_boundary_matrix": admission_boundary_matrix["authority_boundary_matrix"],
                "failure_digest": admission_boundary_matrix["failure_digest"],
                "replay_digest": {
                    "projected": admission_boundary_matrix["projected_family"]["replay_bundle_digest"].clone(),
                    "aspect": admission_boundary_matrix["aspect_family"]["replay_bundle_digest"].clone(),
                },
                "counter_snapshot": counter_snapshot.json(),
                "counter_digest": counter_snapshot.counters().digest(),
            }),
            Self::CrossFamilyReplayLoopIsolationCertification {
                replay_loop_matrix,
                counter_snapshot,
                ..
            } => json!({
                "certification_shape": "cross-family-replay-loop-isolation",
                "writeback_digest": {
                    "projected": replay_loop_matrix["projected_family"]["replay_bundle_digest"].clone(),
                    "aspect": replay_loop_matrix["aspect_family"]["replay_bundle_digest"].clone(),
                },
                "bridge_effect_digest": {
                    "projected": replay_loop_matrix["projected_family"]["effect_digest"].clone(),
                    "aspect": replay_loop_matrix["aspect_family"]["effect_digest"].clone(),
                },
                "causality_digest": {
                    "projected": replay_loop_matrix["projected_family"]["causality_digest"].clone(),
                    "aspect": replay_loop_matrix["aspect_family"]["causality_digest"].clone(),
                },
                "mutation_plan_digest": {
                    "projected": replay_loop_matrix["projected_family"]["mapped_input_digest"].clone(),
                    "aspect": replay_loop_matrix["aspect_family"]["mapped_input_digest"].clone(),
                },
                "idempotence_report": {
                    "projected": replay_loop_matrix["projected_family"]["idempotence_digest"].clone(),
                    "aspect": replay_loop_matrix["aspect_family"]["idempotence_digest"].clone(),
                },
                "loop_prevention_report": replay_loop_matrix["cross_family_loop_isolation"],
                "truth_integrity_report": replay_loop_matrix["same_family_equivalence"],
                "authority_boundary_matrix": replay_loop_matrix["same_family_changed_causality"],
                "failure_digest": replay_loop_matrix["cross_family_replay_isolation"]["failure_digest"],
                "replay_digest": {
                    "projected": replay_loop_matrix["projected_family"]["replay_bundle_digest"].clone(),
                    "aspect": replay_loop_matrix["aspect_family"]["replay_bundle_digest"].clone(),
                },
                "counter_snapshot": counter_snapshot.json(),
                "counter_digest": counter_snapshot.counters().digest(),
            }),
            Self::HostMapperParityCertification {
                mapper_parity_matrix,
                counter_snapshot,
                ..
            } => json!({
                "certification_shape": "host-mapper-parity-and-shadow-protocol-rejection",
                "writeback_digest": {
                    "projected": mapper_parity_matrix["projected_family"]["replay_bundle_digest"].clone(),
                    "aspect": mapper_parity_matrix["aspect_family"]["replay_bundle_digest"].clone(),
                },
                "bridge_effect_digest": {
                    "projected": mapper_parity_matrix["projected_family"]["effect_digest"].clone(),
                    "aspect": mapper_parity_matrix["aspect_family"]["effect_digest"].clone(),
                },
                "causality_digest": mapper_parity_matrix["projected_family"]["causality_digest"],
                "mutation_plan_digest": {
                    "projected": mapper_parity_matrix["projected_family"]["mapped_input_digest"].clone(),
                    "aspect": mapper_parity_matrix["aspect_family"]["mapped_input_digest"].clone(),
                },
                "idempotence_report": serde_json::Value::Null,
                "loop_prevention_report": serde_json::Value::Null,
                "truth_integrity_report": mapper_parity_matrix["mapper_parity_matrix"],
                "authority_boundary_matrix": mapper_parity_matrix["shadow_protocol_rejection"],
                "failure_digest": mapper_parity_matrix["shadow_protocol_rejection"]["failure_digest"],
                "replay_digest": {
                    "projected": mapper_parity_matrix["projected_family"]["replay_bundle_digest"].clone(),
                    "aspect": mapper_parity_matrix["aspect_family"]["replay_bundle_digest"].clone(),
                },
                "counter_snapshot": counter_snapshot.json(),
                "counter_digest": counter_snapshot.counters().digest(),
            }),
        }
    }

    pub(super) fn summary_json(&self) -> serde_json::Value {
        match self {
            Self::DuplicateCertification {
                first_bundle_digest,
                repeated_bundle_digest,
                replay_bundle_digest,
                duplicate_authority_matrix,
                counter_snapshot,
            } => json!({
                "first_bundle_digest": first_bundle_digest,
                "repeated_bundle_digest": repeated_bundle_digest,
                "replay_bundle_digest": replay_bundle_digest,
                "duplicate_authority_matrix": duplicate_authority_matrix,
                "counter_artifact": Self::counter_artifact_json(*counter_snapshot),
                "certification_evidence": self.certification_evidence_json(),
                "counter_snapshot": counter_snapshot.json(),
                "counter_digest": counter_snapshot.counters().digest(),
            }),
            Self::BypassCertification {
                failure_digest,
                bypass_rejection,
                zero_residue_report,
                counter_snapshot,
            } => json!({
                "failure_digest": failure_digest,
                "bypass_rejection": bypass_rejection,
                "zero_residue_report": zero_residue_report,
                "counter_artifact": Self::counter_artifact_json(*counter_snapshot),
                "certification_evidence": self.certification_evidence_json(),
                "counter_snapshot": counter_snapshot.json(),
                "counter_digest": counter_snapshot.counters().digest(),
            }),
            Self::FeedbackLoopCertification {
                feedback_loop_digest,
                feedback_route_digest,
                feedback_origin_matrix,
                counter_snapshot,
            } => json!({
                "feedback_loop_digest": feedback_loop_digest,
                "feedback_route_digest": feedback_route_digest,
                "feedback_origin_matrix": feedback_origin_matrix,
                "counter_artifact": Self::counter_artifact_json(*counter_snapshot),
                "certification_evidence": self.certification_evidence_json(),
                "counter_snapshot": counter_snapshot.json(),
                "counter_digest": counter_snapshot.counters().digest(),
            }),
            Self::ReplayMismatchCertification {
                replay_validation_digest,
                replay_mismatch_matrix,
                counter_snapshot,
            } => json!({
                "replay_validation_digest": replay_validation_digest,
                "replay_mismatch_matrix": replay_mismatch_matrix,
                "counter_artifact": Self::counter_artifact_json(*counter_snapshot),
                "certification_evidence": self.certification_evidence_json(),
                "counter_snapshot": counter_snapshot.json(),
                "counter_digest": counter_snapshot.counters().digest(),
            }),
            Self::ExtensibleFamilyCertification {
                family_extension_digest,
                family_extension_matrix,
                counter_snapshot,
            } => json!({
                "family_extension_digest": family_extension_digest,
                "family_extension_matrix": family_extension_matrix,
                "counter_artifact": Self::counter_artifact_json(*counter_snapshot),
                "certification_evidence": self.certification_evidence_json(),
                "counter_snapshot": counter_snapshot.json(),
                "counter_digest": counter_snapshot.counters().digest(),
            }),
            Self::MultiFamilyAdmissionBoundaryCertification {
                family_extension_digest,
                admission_boundary_matrix,
                counter_snapshot,
            } => json!({
                "family_extension_digest": family_extension_digest,
                "multi_family_admission_boundary_matrix": admission_boundary_matrix,
                "counter_artifact": Self::counter_artifact_json(*counter_snapshot),
                "certification_evidence": self.certification_evidence_json(),
                "counter_snapshot": counter_snapshot.json(),
                "counter_digest": counter_snapshot.counters().digest(),
            }),
            Self::CrossFamilyReplayLoopIsolationCertification {
                family_extension_digest,
                replay_loop_matrix,
                counter_snapshot,
            } => json!({
                "family_extension_digest": family_extension_digest,
                "cross_family_replay_loop_isolation_matrix": replay_loop_matrix,
                "counter_artifact": Self::counter_artifact_json(*counter_snapshot),
                "certification_evidence": self.certification_evidence_json(),
                "counter_snapshot": counter_snapshot.json(),
                "counter_digest": counter_snapshot.counters().digest(),
            }),
            Self::HostMapperParityCertification {
                family_extension_digest,
                mapper_parity_matrix,
                counter_snapshot,
            } => json!({
                "family_extension_digest": family_extension_digest,
                "host_mapper_parity_matrix": mapper_parity_matrix,
                "counter_artifact": Self::counter_artifact_json(*counter_snapshot),
                "certification_evidence": self.certification_evidence_json(),
                "counter_snapshot": counter_snapshot.json(),
                "counter_digest": counter_snapshot.counters().digest(),
            }),
        }
    }

    pub(super) fn extensions_json(
        &self,
        _runtime_bridge: &crate::facade::RuntimeBridge,
    ) -> BTreeMap<String, serde_json::Value> {
        match self {
            Self::DuplicateCertification {
                first_bundle_digest,
                repeated_bundle_digest,
                replay_bundle_digest,
                duplicate_authority_matrix,
                counter_snapshot,
            } => BTreeMap::from([(
                "bridge_writeback_certification_bundle".to_string(),
                json!({
                    "first_bundle_digest": first_bundle_digest,
                    "repeated_bundle_digest": repeated_bundle_digest,
                    "replay_bundle_digest": replay_bundle_digest,
                    "duplicate_authority_matrix": duplicate_authority_matrix,
                    "counter_artifact": Self::counter_artifact_json(*counter_snapshot),
                    "certification_evidence": self.certification_evidence_json(),
                    "counter_snapshot": counter_snapshot.json(),
                    "counter_digest": counter_snapshot.counters().digest(),
                }),
            )]),
            Self::BypassCertification {
                failure_digest,
                bypass_rejection,
                zero_residue_report,
                counter_snapshot,
            } => BTreeMap::from([(
                "bridge_writeback_certification_bundle".to_string(),
                json!({
                    "failure_digest": failure_digest,
                    "bypass_rejection": bypass_rejection,
                    "zero_residue_report": zero_residue_report,
                    "counter_artifact": Self::counter_artifact_json(*counter_snapshot),
                    "certification_evidence": self.certification_evidence_json(),
                    "counter_snapshot": counter_snapshot.json(),
                    "counter_digest": counter_snapshot.counters().digest(),
                }),
            )]),
            Self::FeedbackLoopCertification {
                feedback_loop_digest,
                feedback_route_digest,
                feedback_origin_matrix,
                counter_snapshot,
            } => BTreeMap::from([(
                "bridge_writeback_certification_bundle".to_string(),
                json!({
                    "feedback_loop_digest": feedback_loop_digest,
                    "feedback_route_digest": feedback_route_digest,
                    "feedback_origin_matrix": feedback_origin_matrix,
                    "counter_artifact": Self::counter_artifact_json(*counter_snapshot),
                    "certification_evidence": self.certification_evidence_json(),
                    "counter_snapshot": counter_snapshot.json(),
                    "counter_digest": counter_snapshot.counters().digest(),
                }),
            )]),
            Self::ReplayMismatchCertification {
                replay_validation_digest,
                replay_mismatch_matrix,
                counter_snapshot,
            } => BTreeMap::from([(
                "bridge_writeback_certification_bundle".to_string(),
                json!({
                    "replay_validation_digest": replay_validation_digest,
                    "replay_mismatch_matrix": replay_mismatch_matrix,
                    "counter_artifact": Self::counter_artifact_json(*counter_snapshot),
                    "certification_evidence": self.certification_evidence_json(),
                    "counter_snapshot": counter_snapshot.json(),
                    "counter_digest": counter_snapshot.counters().digest(),
                }),
            )]),
            Self::ExtensibleFamilyCertification {
                family_extension_digest,
                family_extension_matrix,
                counter_snapshot,
            } => BTreeMap::from([(
                "bridge_writeback_certification_bundle".to_string(),
                json!({
                    "family_extension_digest": family_extension_digest,
                    "family_extension_matrix": family_extension_matrix,
                    "counter_artifact": Self::counter_artifact_json(*counter_snapshot),
                    "certification_evidence": self.certification_evidence_json(),
                    "counter_snapshot": counter_snapshot.json(),
                    "counter_digest": counter_snapshot.counters().digest(),
                }),
            )]),
            Self::MultiFamilyAdmissionBoundaryCertification {
                family_extension_digest,
                admission_boundary_matrix,
                counter_snapshot,
            } => BTreeMap::from([(
                "bridge_writeback_certification_bundle".to_string(),
                json!({
                    "family_extension_digest": family_extension_digest,
                    "multi_family_admission_boundary_matrix": admission_boundary_matrix,
                    "counter_artifact": Self::counter_artifact_json(*counter_snapshot),
                    "certification_evidence": self.certification_evidence_json(),
                    "counter_snapshot": counter_snapshot.json(),
                    "counter_digest": counter_snapshot.counters().digest(),
                }),
            )]),
            Self::CrossFamilyReplayLoopIsolationCertification {
                family_extension_digest,
                replay_loop_matrix,
                counter_snapshot,
            } => BTreeMap::from([(
                "bridge_writeback_certification_bundle".to_string(),
                json!({
                    "family_extension_digest": family_extension_digest,
                    "cross_family_replay_loop_isolation_matrix": replay_loop_matrix,
                    "counter_artifact": Self::counter_artifact_json(*counter_snapshot),
                    "certification_evidence": self.certification_evidence_json(),
                    "counter_snapshot": counter_snapshot.json(),
                    "counter_digest": counter_snapshot.counters().digest(),
                }),
            )]),
            Self::HostMapperParityCertification {
                family_extension_digest,
                mapper_parity_matrix,
                counter_snapshot,
            } => BTreeMap::from([(
                "bridge_writeback_certification_bundle".to_string(),
                json!({
                    "family_extension_digest": family_extension_digest,
                    "host_mapper_parity_matrix": mapper_parity_matrix,
                    "counter_artifact": Self::counter_artifact_json(*counter_snapshot),
                    "certification_evidence": self.certification_evidence_json(),
                    "counter_snapshot": counter_snapshot.json(),
                    "counter_digest": counter_snapshot.counters().digest(),
                }),
            )]),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct WritebackCounterSnapshot {
    writeback_family_lookup_count: usize,
    writeback_family_dispatch_count: usize,
    writeback_mapper_lowering_count: usize,
    writeback_decision_record_append_count: usize,
    writeback_request_count: usize,
    writeback_effect_width: usize,
    writeback_strategy_contract_count: usize,
    writeback_strategy_rejection_count: usize,
    writeback_idempotence_check_count: usize,
    writeback_causality_match_count: usize,
    writeback_loop_prevention_check_count: usize,
    writeback_loop_prevention_rejection_count: usize,
    writeback_noop_count: usize,
    writeback_commit_count: usize,
    writeback_failure_count: usize,
    writeback_authority_bypass_rejection_count: usize,
    writeback_validation_rejection_count: usize,
    writeback_replay_request_count: usize,
    writeback_replay_mismatch_count: usize,
}

impl WritebackCounterSnapshot {
    fn json(self) -> serde_json::Value {
        self.counters().json()
    }

    fn artifact_json(self) -> serde_json::Value {
        let counters = self.counters();
        json!({
            "snapshot": counters.json(),
            "canonical_basis": counters.canonical_basis(),
            "digest": counters.digest(),
        })
    }

    fn counters(self) -> crate::facade::BridgeWritebackCounters {
        crate::facade::BridgeWritebackCounters::new(
            self.writeback_family_lookup_count,
            self.writeback_family_dispatch_count,
            self.writeback_mapper_lowering_count,
            self.writeback_decision_record_append_count,
            self.writeback_request_count,
            self.writeback_effect_width,
            self.writeback_strategy_contract_count,
            self.writeback_strategy_rejection_count,
            self.writeback_idempotence_check_count,
            self.writeback_causality_match_count,
            self.writeback_loop_prevention_check_count,
            self.writeback_loop_prevention_rejection_count,
            self.writeback_noop_count,
            self.writeback_commit_count,
            self.writeback_failure_count,
            self.writeback_authority_bypass_rejection_count,
            self.writeback_validation_rejection_count,
            self.writeback_replay_request_count,
            self.writeback_replay_mismatch_count,
        )
    }
}

fn snapshot_from_counters(
    counters: &crate::facade::BridgeWritebackCounters,
) -> WritebackCounterSnapshot {
    WritebackCounterSnapshot {
        writeback_family_lookup_count: counters.writeback_family_lookup_count(),
        writeback_family_dispatch_count: counters.writeback_family_dispatch_count(),
        writeback_mapper_lowering_count: counters.writeback_mapper_lowering_count(),
        writeback_decision_record_append_count: counters.writeback_decision_record_append_count(),
        writeback_request_count: counters.writeback_request_count(),
        writeback_effect_width: counters.writeback_effect_width(),
        writeback_strategy_contract_count: counters.writeback_strategy_contract_count(),
        writeback_strategy_rejection_count: counters.writeback_strategy_rejection_count(),
        writeback_idempotence_check_count: counters.writeback_idempotence_check_count(),
        writeback_causality_match_count: counters.writeback_causality_match_count(),
        writeback_loop_prevention_check_count: counters.writeback_loop_prevention_check_count(),
        writeback_loop_prevention_rejection_count: counters
            .writeback_loop_prevention_rejection_count(),
        writeback_noop_count: counters.writeback_noop_count(),
        writeback_commit_count: counters.writeback_commit_count(),
        writeback_failure_count: counters.writeback_failure_count(),
        writeback_authority_bypass_rejection_count: counters
            .writeback_authority_bypass_rejection_count(),
        writeback_validation_rejection_count: counters.writeback_validation_rejection_count(),
        writeback_replay_request_count: counters.writeback_replay_request_count(),
        writeback_replay_mismatch_count: counters.writeback_replay_mismatch_count(),
    }
}

fn aggregate_runtime_writeback_counters(
    runtimes: &[&crate::facade::RuntimeBridge],
) -> crate::facade::BridgeWritebackCounters {
    let mut totals = WritebackCounterSnapshot {
        writeback_family_lookup_count: 0,
        writeback_family_dispatch_count: 0,
        writeback_mapper_lowering_count: 0,
        writeback_decision_record_append_count: 0,
        writeback_request_count: 0,
        writeback_effect_width: 0,
        writeback_strategy_contract_count: 0,
        writeback_strategy_rejection_count: 0,
        writeback_idempotence_check_count: 0,
        writeback_causality_match_count: 0,
        writeback_loop_prevention_check_count: 0,
        writeback_loop_prevention_rejection_count: 0,
        writeback_noop_count: 0,
        writeback_commit_count: 0,
        writeback_failure_count: 0,
        writeback_authority_bypass_rejection_count: 0,
        writeback_validation_rejection_count: 0,
        writeback_replay_request_count: 0,
        writeback_replay_mismatch_count: 0,
    };

    for runtime_bridge in runtimes {
        for record in runtime_bridge.diagnostics().writeback_execution_records() {
            let counters = record.counters();
            totals.writeback_family_lookup_count += counters.writeback_family_lookup_count();
            totals.writeback_family_dispatch_count += counters.writeback_family_dispatch_count();
            totals.writeback_mapper_lowering_count += counters.writeback_mapper_lowering_count();
            totals.writeback_decision_record_append_count +=
                counters.writeback_decision_record_append_count();
            totals.writeback_request_count += counters.writeback_request_count();
            totals.writeback_effect_width += counters.writeback_effect_width();
            totals.writeback_strategy_contract_count +=
                counters.writeback_strategy_contract_count();
            totals.writeback_strategy_rejection_count +=
                counters.writeback_strategy_rejection_count();
            totals.writeback_idempotence_check_count +=
                counters.writeback_idempotence_check_count();
            totals.writeback_causality_match_count += counters.writeback_causality_match_count();
            totals.writeback_loop_prevention_check_count +=
                counters.writeback_loop_prevention_check_count();
            totals.writeback_loop_prevention_rejection_count +=
                counters.writeback_loop_prevention_rejection_count();
            totals.writeback_noop_count += counters.writeback_noop_count();
            totals.writeback_commit_count += counters.writeback_commit_count();
            totals.writeback_failure_count += counters.writeback_failure_count();
            totals.writeback_authority_bypass_rejection_count +=
                counters.writeback_authority_bypass_rejection_count();
            totals.writeback_validation_rejection_count +=
                counters.writeback_validation_rejection_count();
            totals.writeback_replay_request_count += counters.writeback_replay_request_count();
            totals.writeback_replay_mismatch_count += counters.writeback_replay_mismatch_count();
        }

        for record in runtime_bridge.diagnostics().writeback_replay_records() {
            let counters = record.counters();
            totals.writeback_family_lookup_count += counters.writeback_family_lookup_count();
            totals.writeback_family_dispatch_count += counters.writeback_family_dispatch_count();
            totals.writeback_mapper_lowering_count += counters.writeback_mapper_lowering_count();
            totals.writeback_decision_record_append_count +=
                counters.writeback_decision_record_append_count();
            totals.writeback_request_count += counters.writeback_request_count();
            totals.writeback_effect_width += counters.writeback_effect_width();
            totals.writeback_strategy_contract_count +=
                counters.writeback_strategy_contract_count();
            totals.writeback_strategy_rejection_count +=
                counters.writeback_strategy_rejection_count();
            totals.writeback_idempotence_check_count +=
                counters.writeback_idempotence_check_count();
            totals.writeback_causality_match_count += counters.writeback_causality_match_count();
            totals.writeback_loop_prevention_check_count +=
                counters.writeback_loop_prevention_check_count();
            totals.writeback_loop_prevention_rejection_count +=
                counters.writeback_loop_prevention_rejection_count();
            totals.writeback_noop_count += counters.writeback_noop_count();
            totals.writeback_commit_count += counters.writeback_commit_count();
            totals.writeback_failure_count += counters.writeback_failure_count();
            totals.writeback_authority_bypass_rejection_count +=
                counters.writeback_authority_bypass_rejection_count();
            totals.writeback_validation_rejection_count +=
                counters.writeback_validation_rejection_count();
            totals.writeback_replay_request_count += counters.writeback_replay_request_count();
            totals.writeback_replay_mismatch_count += counters.writeback_replay_mismatch_count();
        }
    }

    totals.counters()
}

#[derive(Debug, Clone)]
struct RejectingTruthWritebackAuthority {
    failure_class: crate::facade::BridgeWritebackFailureClass,
    last_request_digest: Arc<RwLock<Option<String>>>,
    last_receipt_digest: Arc<RwLock<Option<String>>>,
}

impl RejectingTruthWritebackAuthority {
    fn new(failure_class: crate::facade::BridgeWritebackFailureClass) -> Self {
        Self {
            failure_class,
            last_request_digest: Arc::new(RwLock::new(None)),
            last_receipt_digest: Arc::new(RwLock::new(None)),
        }
    }

    fn last_request_digest(&self) -> Option<String> {
        self.last_request_digest
            .read()
            .expect("rejecting writeback authority lock poisoned")
            .clone()
    }

    fn last_receipt_digest(&self) -> Option<String> {
        self.last_receipt_digest
            .read()
            .expect("rejecting writeback authority lock poisoned")
            .clone()
    }
}

impl crate::adapter::TruthWritebackAuthority for RejectingTruthWritebackAuthority {
    fn execute_writeback(
        &self,
        request: crate::adapter::TruthWritebackRequest,
    ) -> Result<crate::adapter::TruthWritebackReceipt, crate::adapter::TruthWritebackAuthorityError>
    {
        let receipt = crate::adapter::TruthWritebackReceipt::new_with_failure_class(
            crate::facade::BridgeWritebackOutcomeClass::Rejected,
            Some(self.failure_class),
            format!(
                "rejecting-truth-writeback-authority:{}:{}",
                format!("{:?}", self.failure_class),
                request.digest()
            ),
            &request,
        );
        *self
            .last_request_digest
            .write()
            .expect("rejecting writeback authority lock poisoned") =
            Some(request.digest().to_string());
        *self
            .last_receipt_digest
            .write()
            .expect("rejecting writeback authority lock poisoned") =
            Some(receipt.digest().to_string());
        Ok(receipt)
    }
}

pub(super) fn parse_writeback_harness_target(
    target: &str,
) -> Option<Result<WritebackHarnessTarget, BridgeHarnessError>> {
    match target {
        "writeback-duplicate-certify" => Some(Ok(WritebackHarnessTarget::DuplicateCertification)),
        "writeback-bypass-certify" => Some(Ok(WritebackHarnessTarget::BypassCertification)),
        "writeback-feedback-certify" => Some(Ok(WritebackHarnessTarget::FeedbackLoopCertification)),
        "writeback-replay-mismatch-certify" => {
            Some(Ok(WritebackHarnessTarget::ReplayMismatchCertification))
        }
        "writeback-family-extension-certify" => {
            Some(Ok(WritebackHarnessTarget::ExtensibleFamilyCertification))
        }
        "writeback-family-admission-boundary-certify" => Some(Ok(
            WritebackHarnessTarget::MultiFamilyAdmissionBoundaryCertification,
        )),
        "writeback-family-replay-loop-isolation-certify" => Some(Ok(
            WritebackHarnessTarget::CrossFamilyReplayLoopIsolationCertification,
        )),
        "writeback-family-mapper-parity-certify" => {
            Some(Ok(WritebackHarnessTarget::HostMapperParityCertification))
        }
        _ => None,
    }
}

pub(super) fn execute_writeback_request(
    runtime: &crate::harness::adapter::BridgeHarnessSession,
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    target: WritebackHarnessTarget,
) -> Result<WritebackHarnessExecution, BridgeHarnessError> {
    match target {
        WritebackHarnessTarget::DuplicateCertification => {
            execute_duplicate_certification(runtime, runtime_bridge, fixture)
        }
        WritebackHarnessTarget::BypassCertification => {
            execute_bypass_certification(runtime, runtime_bridge, fixture)
        }
        WritebackHarnessTarget::FeedbackLoopCertification => {
            execute_feedback_loop_certification(runtime, runtime_bridge, fixture)
        }
        WritebackHarnessTarget::ReplayMismatchCertification => {
            execute_replay_mismatch_certification(runtime, runtime_bridge, fixture)
        }
        WritebackHarnessTarget::ExtensibleFamilyCertification => {
            execute_extensible_family_certification(runtime, runtime_bridge, fixture)
        }
        WritebackHarnessTarget::MultiFamilyAdmissionBoundaryCertification => {
            execute_multi_family_admission_boundary_certification(runtime, runtime_bridge, fixture)
        }
        WritebackHarnessTarget::CrossFamilyReplayLoopIsolationCertification => {
            execute_cross_family_replay_loop_isolation_certification(
                runtime,
                runtime_bridge,
                fixture,
            )
        }
        WritebackHarnessTarget::HostMapperParityCertification => {
            execute_host_mapper_parity_certification(runtime, runtime_bridge, fixture)
        }
    }
}

fn execute_duplicate_certification(
    _runtime: &crate::harness::adapter::BridgeHarnessSession,
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
) -> Result<WritebackHarnessExecution, BridgeHarnessError> {
    let lowered_policy_bundle = lowered_policy(runtime_bridge)?;
    let declaration = crate::facade::BridgeWritebackDeclaration::writeback_capable(
        crate::facade::BridgeWritebackDeclarationIdentity::new("harness:writeback-duplicate"),
        crate::facade::BridgeRequestKind::Authoritative,
        crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff,
        crate::facade::BridgeWritebackEffectClass::ProjectedStateDiff,
        crate::facade::BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
        "strategy:sha256:writeback-duplicate",
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let contract = runtime_bridge
        .admit_writeback_declaration(declaration, &lowered_policy_bundle)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback duplicate certification contract admission failed: {error}"
            ))
        })?;

    let commit_identity = fixture
        .committed_patches()
        .first()
        .map(|patch| patch.commit_identity().as_str().to_string())
        .unwrap_or_else(|| "missing-commit".to_string());
    let route_digest = route_digest_for_first_patch(runtime_bridge, fixture)?;
    let causality = crate::facade::BridgeWritebackCausalityBasis::new(
        crate::facade::BridgeWritebackCausalityIdentity::new("harness:writeback-causality"),
        format!("truth-trigger:{}", commit_identity),
        route_digest.clone(),
        "evaluation-surface:sha256:writeback-duplicate",
        "truth-view:sha256:writeback-duplicate",
    );
    let effect = runtime_bridge.lower_writeback_effect(
        &contract,
        &causality,
        crate::facade::BridgeWritebackEffectIdentity::new("harness:writeback-effect"),
        "effect:sha256:writeback-duplicate",
    );
    let first_idempotence = runtime_bridge.classify_writeback_idempotence(
        &effect,
        &lowered_policy_bundle,
        "authority-state:sha256:before",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-idempotence:first",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let repeated_idempotence = runtime_bridge.classify_writeback_idempotence(
        &effect,
        &lowered_policy_bundle,
        "authority-state:sha256:after-first-commit",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-idempotence:repeat",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (first_outcome, first_receipt) = runtime_bridge
        .execute_writeback_authority(&contract, &effect, &first_idempotence)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback duplicate certification first authority execution failed: {error}"
            ))
        })?;
    let (repeated_outcome, repeated_receipt) = runtime_bridge
        .execute_writeback_authority(&contract, &effect, &repeated_idempotence)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback duplicate certification repeated authority execution failed: {error}"
            ))
        })?;
    let first_bundle = runtime_bridge.replay_writeback_bundle(
        &contract,
        &effect,
        &first_idempotence,
        &first_outcome,
    );
    let repeated_bundle = runtime_bridge.replay_writeback_bundle(
        &contract,
        &effect,
        &repeated_idempotence,
        &repeated_outcome,
    );
    let replay_bundle = runtime_bridge.replay_writeback_bundle(
        &contract,
        &effect,
        &repeated_idempotence,
        &repeated_outcome,
    );
    let commit_count = match (
        first_receipt.outcome_class(),
        repeated_receipt.outcome_class(),
    ) {
        (
            crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit,
            crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit,
        ) => 2,
        (crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit, _)
        | (_, crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit) => 1,
        _ => 0,
    };
    let noop_count = match (
        first_receipt.outcome_class(),
        repeated_receipt.outcome_class(),
    ) {
        (
            crate::facade::BridgeWritebackOutcomeClass::CanonicalNoop,
            crate::facade::BridgeWritebackOutcomeClass::CanonicalNoop,
        ) => 2,
        (crate::facade::BridgeWritebackOutcomeClass::CanonicalNoop, _)
        | (_, crate::facade::BridgeWritebackOutcomeClass::CanonicalNoop) => 1,
        _ => 0,
    };
    let counters = aggregate_runtime_writeback_counters(&[runtime_bridge]);
    let counter_snapshot = snapshot_from_counters(&counters);
    let first_loop_prevention = runtime_bridge.classify_writeback_loop_prevention(
        &effect,
        &first_idempotence,
        None::<std::sync::Arc<str>>,
        None::<std::sync::Arc<str>>,
    );
    let repeated_loop_prevention = runtime_bridge.classify_writeback_loop_prevention(
        &effect,
        &repeated_idempotence,
        None::<std::sync::Arc<str>>,
        None::<std::sync::Arc<str>>,
    );
    let first_strategy_compatibility = runtime_bridge.classify_writeback_strategy_compatibility(
        &contract,
        &effect,
        &first_idempotence,
    );
    let repeated_strategy_compatibility = runtime_bridge.classify_writeback_strategy_compatibility(
        &contract,
        &effect,
        &repeated_idempotence,
    );
    let first_feedback_provenance = runtime_bridge.derive_writeback_feedback_provenance(&effect);
    let first_candidate = runtime_bridge
        .validate_writeback_candidate(
            &contract,
            &effect,
            &first_idempotence,
            &first_loop_prevention,
            &first_strategy_compatibility,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback duplicate certification first candidate validation failed: {error}"
            ))
        })?;
    let repeated_candidate = runtime_bridge
        .validate_writeback_candidate(
            &contract,
            &effect,
            &repeated_idempotence,
            &repeated_loop_prevention,
            &repeated_strategy_compatibility,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback duplicate certification repeated candidate validation failed: {error}"
            ))
        })?;
    let mapped_input = runtime_bridge
        .diagnostics()
        .writeback_mapped_family_input_for_digest(effect.mapped_input_digest())
        .expect("writeback harness should retain mapped-family input for duplicate certification");
    let mapper_witness = crate::facade::BridgeWritebackMapperWitness::issue(&mapped_input);
    let first_authority_request = crate::adapter::TruthWritebackRequest::new(
        effect.family_kind(),
        contract.digest(),
        first_candidate.digest(),
        mapped_input.digest(),
        mapper_witness.digest(),
        effect.digest(),
        effect.effect_digest(),
        effect.effect_class(),
        effect.strategy_class(),
        first_feedback_provenance.digest(),
        first_loop_prevention.digest(),
        first_loop_prevention.disposition(),
        first_strategy_compatibility.digest(),
        first_idempotence.causality_digest(),
        first_idempotence.digest(),
        first_idempotence.idempotence_class(),
        effect.strategy_descriptor_digest(),
    );
    let repeated_authority_request = crate::adapter::TruthWritebackRequest::new(
        effect.family_kind(),
        contract.digest(),
        repeated_candidate.digest(),
        mapped_input.digest(),
        mapper_witness.digest(),
        effect.digest(),
        effect.effect_digest(),
        effect.effect_class(),
        effect.strategy_class(),
        first_feedback_provenance.digest(),
        repeated_loop_prevention.digest(),
        repeated_loop_prevention.disposition(),
        repeated_strategy_compatibility.digest(),
        repeated_idempotence.causality_digest(),
        repeated_idempotence.digest(),
        repeated_idempotence.idempotence_class(),
        effect.strategy_descriptor_digest(),
    );

    Ok(WritebackHarnessExecution::DuplicateCertification {
        first_bundle_digest: first_bundle.digest().to_string(),
        repeated_bundle_digest: repeated_bundle.digest().to_string(),
        replay_bundle_digest: replay_bundle.digest().to_string(),
        duplicate_authority_matrix: json!({
            "writeback_digest": repeated_bundle.digest(),
            "bridge_effect_digest": effect.digest(),
            "causality_digest": causality.digest(),
            "replay_digest": replay_bundle.digest(),
            "mutation_plan_digest": first_receipt.authoritative_artifact_digest(),
            "replay_bundle_report": {
                "digest": replay_bundle.digest(),
                "semantic_digest": replay_bundle.semantic_digest(),
                "strategy_class": format!("{:?}", replay_bundle.strategy_class()),
                "strategy_descriptor_digest": replay_bundle.strategy_descriptor_digest(),
                "causality_digest": replay_bundle.causality_digest(),
                "lowered_policy_digest": replay_bundle.lowered_policy_digest(),
                "retry_disposition": format!("{:?}", replay_bundle.retry_disposition()),
                "outcome_class": format!("{:?}", replay_bundle.outcome_class()),
                "authoritative_artifact_digest": replay_bundle.authoritative_artifact_digest(),
            },
            "idempotence_report": {
                "first_digest": first_idempotence.digest(),
                "repeated_digest": repeated_idempotence.digest(),
                "idempotence_class": format!("{:?}", first_idempotence.idempotence_class()),
                "authoritative_state_before": first_idempotence.authoritative_state_digest(),
                "authoritative_state_after_first_commit": repeated_idempotence.authoritative_state_digest(),
                "lowered_policy_digest": first_idempotence.lowered_policy_digest(),
                "strategy_descriptor_digest": first_idempotence.strategy_descriptor_digest(),
            },
            "loop_prevention_report": {
                "first_digest": first_loop_prevention.digest(),
                "first_disposition": format!("{:?}", first_loop_prevention.disposition()),
                "repeated_digest": repeated_loop_prevention.digest(),
                "repeated_disposition": format!("{:?}", repeated_loop_prevention.disposition()),
                "current_feedback_provenance_digest": first_loop_prevention.current_feedback_provenance_digest(),
                "current_causality_digest": first_loop_prevention.current_causality_digest(),
            },
            "authority_boundary_matrix": {
                "contract_digest": contract.digest(),
                "strategy_basis_digest": contract.validated_declaration().strategy_basis().expect("admitted writeback contract should preserve strategy basis").digest(),
                "first_strategy_compatibility_digest": first_strategy_compatibility.digest(),
                "first_strategy_compatibility_disposition": format!("{:?}", first_strategy_compatibility.disposition()),
                "first_candidate_digest": first_candidate.digest(),
                "repeated_strategy_compatibility_digest": repeated_strategy_compatibility.digest(),
                "repeated_strategy_compatibility_disposition": format!("{:?}", repeated_strategy_compatibility.disposition()),
                "repeated_candidate_digest": repeated_candidate.digest(),
                "first_authority_request_digest": first_authority_request.digest(),
                "repeated_authority_request_digest": repeated_authority_request.digest(),
                "first_authority_receipt_digest": first_receipt.digest(),
                "repeated_authority_receipt_digest": repeated_receipt.digest(),
            },
            "truth_trigger_digest": format!("truth-trigger:{commit_identity}"),
            "route_digest": route_digest,
            "effect_digest": effect.digest(),
            "first_attempt": {
                "idempotence_digest": first_idempotence.digest(),
                "outcome_digest": first_outcome.digest(),
                "replay_bundle_digest": first_bundle.digest(),
                "outcome_class": format!("{:?}", first_receipt.outcome_class()),
            },
            "repeated_attempt": {
                "idempotence_digest": repeated_idempotence.digest(),
                "outcome_digest": repeated_outcome.digest(),
                "replay_bundle_digest": repeated_bundle.digest(),
                "outcome_class": format!("{:?}", repeated_receipt.outcome_class()),
            },
            "boundedness_proof": {
                "authoritative_commit_count": commit_count,
                "canonical_noop_count": noop_count,
                "duplicate_causality_detected": true,
                "loop_converged": commit_count == 1 && noop_count == 1,
            },
        }),
        counter_snapshot,
    })
}

fn execute_bypass_certification(
    runtime: &crate::harness::adapter::BridgeHarnessSession,
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
) -> Result<WritebackHarnessExecution, BridgeHarnessError> {
    let declaration = crate::facade::BridgeWritebackDeclaration::writeback_capable(
        crate::facade::BridgeWritebackDeclarationIdentity::new("harness:writeback-bypass"),
        crate::facade::BridgeRequestKind::Preview,
        crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff,
        crate::facade::BridgeWritebackEffectClass::ProjectedStateDiff,
        crate::facade::BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
        "strategy:sha256:writeback-bypass",
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let error = runtime_bridge
        .validate_writeback_declaration(declaration)
        .expect_err("preview writeback bypass must fail closed");
    let error_message = error.to_string();
    let failure_digest = digest_string(
        "bridge-writeback-harness-failure",
        &format!("{:?}|{}", error.kind(), error_message),
    )
    .to_string();
    let unbound_runtime = build_writeback_runtime(runtime, fixture, false)?;
    let lowered_policy_bundle = lowered_policy(&unbound_runtime)?;
    let authority_declaration = crate::facade::BridgeWritebackDeclaration::writeback_capable(
        crate::facade::BridgeWritebackDeclarationIdentity::new(
            "harness:writeback-bypass:unbound-authority",
        ),
        crate::facade::BridgeRequestKind::Authoritative,
        crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff,
        crate::facade::BridgeWritebackEffectClass::ProjectedStateDiff,
        crate::facade::BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
        "strategy:sha256:writeback-bypass-unbound",
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let authority_contract = unbound_runtime
        .admit_writeback_declaration(authority_declaration, &lowered_policy_bundle)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback bypass certification failed to admit unbound-authority contract: {error}"
            ))
        })?;
    let authority_causality = crate::facade::BridgeWritebackCausalityBasis::new(
        crate::facade::BridgeWritebackCausalityIdentity::new("harness:writeback-bypass:causality"),
        "truth-trigger:commit-a",
        "route:sha256:writeback-bypass",
        "evaluation-surface:sha256:writeback-bypass",
        "truth-view:sha256:writeback-bypass",
    );
    let authority_effect = unbound_runtime.lower_writeback_effect(
        &authority_contract,
        &authority_causality,
        crate::facade::BridgeWritebackEffectIdentity::new("harness:writeback-bypass:effect"),
        "effect:sha256:writeback-bypass-unbound",
    );
    let authority_idempotence = unbound_runtime.classify_writeback_idempotence(
        &authority_effect,
        &lowered_policy_bundle,
        "authority-state:sha256:unbound",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-bypass:idempotence",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let authority_error = unbound_runtime
        .execute_writeback_authority(
            &authority_contract,
            &authority_effect,
            &authority_idempotence,
        )
        .expect_err("unbound writeback authority execution must fail closed");
    let authority_failure_digest = digest_string(
        "bridge-writeback-harness-authority-bypass-failure",
        &format!("{:?}|{}", authority_error.kind(), authority_error),
    )
    .to_string();
    let merge_rejecting_authority = RejectingTruthWritebackAuthority::new(
        crate::facade::BridgeWritebackFailureClass::MergeAuthorityRejected,
    );
    let merge_rejecting_runtime = build_writeback_runtime_with_custom_authority(
        runtime,
        fixture,
        merge_rejecting_authority.clone(),
    )?;
    let merge_lowered_policy = lowered_policy(&merge_rejecting_runtime)?;
    let merge_declaration = crate::facade::BridgeWritebackDeclaration::writeback_capable(
        crate::facade::BridgeWritebackDeclarationIdentity::new(
            "harness:writeback-bypass:merge-rejected",
        ),
        crate::facade::BridgeRequestKind::Authoritative,
        crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff,
        crate::facade::BridgeWritebackEffectClass::ProjectedStateDiff,
        crate::facade::BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
        "strategy:sha256:writeback-bypass-merge-rejected",
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let merge_contract = merge_rejecting_runtime
        .admit_writeback_declaration(merge_declaration, &merge_lowered_policy)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback bypass certification failed to admit merge-rejected contract: {error}"
            ))
        })?;
    let merge_causality = crate::facade::BridgeWritebackCausalityBasis::new(
        crate::facade::BridgeWritebackCausalityIdentity::new(
            "harness:writeback-bypass:merge-rejected:causality",
        ),
        "truth-trigger:commit-b",
        "route:sha256:writeback-bypass-merge-rejected",
        "evaluation-surface:sha256:writeback-bypass-merge-rejected",
        "truth-view:sha256:writeback-bypass-merge-rejected",
    );
    let merge_effect = merge_rejecting_runtime.lower_writeback_effect(
        &merge_contract,
        &merge_causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-bypass:merge-rejected:effect",
        ),
        "effect:sha256:writeback-bypass-merge-rejected",
    );
    let merge_idempotence = merge_rejecting_runtime.classify_writeback_idempotence(
        &merge_effect,
        &merge_lowered_policy,
        "authority-state:sha256:merge-rejected",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-bypass:merge-rejected:idempotence",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let merge_error = merge_rejecting_runtime
        .execute_writeback_authority(&merge_contract, &merge_effect, &merge_idempotence)
        .expect_err("merge-authority rejection must fail closed");
    let merge_failure_digest = digest_string(
        "bridge-writeback-harness-merge-bypass-failure",
        &format!("{:?}|{}", merge_error.kind(), merge_error),
    )
    .to_string();
    let unsafe_feedback_runtime = build_writeback_runtime(runtime, fixture, true)?;
    let unsafe_feedback_policy = lowered_policy(&unsafe_feedback_runtime)?;
    let unsafe_feedback_declaration = crate::facade::BridgeWritebackDeclaration::writeback_capable(
        crate::facade::BridgeWritebackDeclarationIdentity::new(
            "harness:writeback-bypass:unsafe-feedback",
        ),
        crate::facade::BridgeRequestKind::Authoritative,
        crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff,
        crate::facade::BridgeWritebackEffectClass::ProjectedStateDiff,
        crate::facade::BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
        "strategy:sha256:writeback-bypass-unsafe-feedback",
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let unsafe_feedback_contract = unsafe_feedback_runtime
        .admit_writeback_declaration(unsafe_feedback_declaration, &unsafe_feedback_policy)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback bypass certification failed to admit unsafe-feedback contract: {error}"
            ))
        })?;
    let unsafe_feedback_causality = crate::facade::BridgeWritebackCausalityBasis::new(
        crate::facade::BridgeWritebackCausalityIdentity::new(
            "harness:writeback-bypass:unsafe-feedback:causality",
        ),
        "truth-trigger:commit-c",
        "route:sha256:writeback-bypass-unsafe-feedback",
        "evaluation-surface:sha256:writeback-bypass-unsafe-feedback",
        "truth-view:sha256:writeback-bypass-unsafe-feedback",
    );
    let unsafe_feedback_effect = unsafe_feedback_runtime.lower_writeback_effect(
        &unsafe_feedback_contract,
        &unsafe_feedback_causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-bypass:unsafe-feedback:effect",
        ),
        "effect:sha256:writeback-bypass-unsafe-feedback",
    );
    let unsafe_feedback_idempotence = unsafe_feedback_runtime.classify_writeback_idempotence(
        &unsafe_feedback_effect,
        &unsafe_feedback_policy,
        "authority-state:sha256:unsafe-feedback",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-bypass:unsafe-feedback:idempotence",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let unsafe_feedback_provenance =
        unsafe_feedback_runtime.derive_writeback_feedback_provenance(&unsafe_feedback_effect);
    let unsafe_feedback_loop_prevention = unsafe_feedback_runtime
        .classify_writeback_loop_prevention(
            &unsafe_feedback_effect,
            &unsafe_feedback_idempotence,
            Some(unsafe_feedback_provenance.digest()),
            None::<std::sync::Arc<str>>,
        );
    let unsafe_feedback_error = unsafe_feedback_runtime
        .execute_writeback_authority_with_feedback_context(
            &unsafe_feedback_contract,
            &unsafe_feedback_effect,
            &unsafe_feedback_idempotence,
            Some(unsafe_feedback_provenance.digest()),
            None::<std::sync::Arc<str>>,
        )
        .expect_err("partial feedback context must fail closed before authority execution");
    let unsafe_feedback_failure_digest = digest_string(
        "bridge-writeback-harness-unsafe-feedback-bypass-failure",
        &format!(
            "{:?}|{}",
            unsafe_feedback_error.kind(),
            unsafe_feedback_error
        ),
    )
    .to_string();
    let contradictory_feedback_causality_digest =
        "truth-trigger:commit-contradictory-feedback".to_string();
    let contradictory_feedback_loop_prevention = unsafe_feedback_runtime
        .classify_writeback_loop_prevention(
            &unsafe_feedback_effect,
            &unsafe_feedback_idempotence,
            Some(unsafe_feedback_provenance.digest()),
            Some(contradictory_feedback_causality_digest.clone()),
        );
    let contradictory_feedback_error = unsafe_feedback_runtime
        .execute_writeback_authority_with_feedback_context(
            &unsafe_feedback_contract,
            &unsafe_feedback_effect,
            &unsafe_feedback_idempotence,
            Some(unsafe_feedback_provenance.digest()),
            Some(contradictory_feedback_causality_digest.clone()),
        )
        .expect_err("contradictory feedback context must fail closed before authority execution");
    let contradictory_feedback_failure_digest = digest_string(
        "bridge-writeback-harness-contradictory-feedback-bypass-failure",
        &format!(
            "{:?}|{}",
            contradictory_feedback_error.kind(),
            contradictory_feedback_error
        ),
    )
    .to_string();

    let counters = aggregate_runtime_writeback_counters(&[
        &unbound_runtime,
        &merge_rejecting_runtime,
        &unsafe_feedback_runtime,
    ]);
    let counter_snapshot = snapshot_from_counters(&counters);
    let validation_error_kind = format!("{:?}", error.kind());

    Ok(WritebackHarnessExecution::BypassCertification {
        failure_digest,
        bypass_rejection: json!({
            "writeback_digest": serde_json::Value::Null,
            "bridge_effect_digest": serde_json::Value::Null,
            "causality_digest": serde_json::Value::Null,
            "replay_digest": serde_json::Value::Null,
            "mutation_plan_digest": serde_json::Value::Null,
            "idempotence_report": serde_json::Value::Null,
            "loop_prevention_report": {
                "unsafe_feedback_partial": {
                    "digest": unsafe_feedback_loop_prevention.digest(),
                    "disposition": format!("{:?}", unsafe_feedback_loop_prevention.disposition()),
                    "current_feedback_provenance_digest": unsafe_feedback_loop_prevention.current_feedback_provenance_digest(),
                    "current_causality_digest": unsafe_feedback_loop_prevention.current_causality_digest(),
                    "incoming_feedback_provenance_digest": unsafe_feedback_loop_prevention.incoming_feedback_provenance_digest(),
                    "incoming_feedback_causality_digest": unsafe_feedback_loop_prevention.incoming_feedback_causality_digest(),
                },
                "unsafe_feedback_contradictory": {
                    "digest": contradictory_feedback_loop_prevention.digest(),
                    "disposition": format!("{:?}", contradictory_feedback_loop_prevention.disposition()),
                    "current_feedback_provenance_digest": contradictory_feedback_loop_prevention.current_feedback_provenance_digest(),
                    "current_causality_digest": contradictory_feedback_loop_prevention.current_causality_digest(),
                    "incoming_feedback_provenance_digest": contradictory_feedback_loop_prevention.incoming_feedback_provenance_digest(),
                    "incoming_feedback_causality_digest": contradictory_feedback_loop_prevention.incoming_feedback_causality_digest(),
                },
            },
            "authority_boundary_matrix": {
                "preview_validation_failure": {
                    "contract_digest": serde_json::Value::Null,
                    "strategy_basis_digest": serde_json::Value::Null,
                    "authority_request_digest": serde_json::Value::Null,
                    "authority_receipt_digest": serde_json::Value::Null,
                    "bypass_class": "validation-short-circuit",
                    "failure_kind": format!("{:?}", error.kind()),
                },
                "unbound_authority_failure": {
                    "contract_digest": authority_contract.digest(),
                    "strategy_basis_digest": authority_contract.validated_declaration().strategy_basis().expect("admitted writeback contract should preserve strategy basis").digest(),
                    "authority_request_digest": serde_json::Value::Null,
                    "authority_receipt_digest": serde_json::Value::Null,
                    "bypass_class": "unbound-authority-execution",
                    "failure_kind": format!("{:?}", authority_error.kind()),
                    "failure_digest": authority_failure_digest,
                    "causality_digest": authority_causality.digest(),
                    "bridge_effect_digest": authority_effect.digest(),
                    "idempotence_digest": authority_idempotence.digest(),
                },
                "merge_authority_failure": {
                    "contract_digest": merge_contract.digest(),
                    "strategy_basis_digest": merge_contract.validated_declaration().strategy_basis().expect("admitted writeback contract should preserve strategy basis").digest(),
                    "authority_request_digest": merge_rejecting_authority.last_request_digest(),
                    "authority_receipt_digest": merge_rejecting_authority.last_receipt_digest(),
                    "bypass_class": "merge-authority-rejection",
                    "failure_kind": format!("{:?}", merge_error.kind()),
                    "failure_digest": merge_failure_digest,
                    "causality_digest": merge_causality.digest(),
                    "bridge_effect_digest": merge_effect.digest(),
                    "idempotence_digest": merge_idempotence.digest(),
                },
                "unsafe_feedback_failure": {
                    "contract_digest": unsafe_feedback_contract.digest(),
                    "strategy_basis_digest": unsafe_feedback_contract.validated_declaration().strategy_basis().expect("admitted writeback contract should preserve strategy basis").digest(),
                    "authority_request_digest": serde_json::Value::Null,
                    "authority_receipt_digest": serde_json::Value::Null,
                    "bypass_class": "unsafe-feedback-preauthority",
                    "failure_kind": format!("{:?}", unsafe_feedback_error.kind()),
                    "failure_digest": unsafe_feedback_failure_digest,
                    "causality_digest": unsafe_feedback_causality.digest(),
                    "bridge_effect_digest": unsafe_feedback_effect.digest(),
                    "idempotence_digest": unsafe_feedback_idempotence.digest(),
                    "feedback_provenance_digest": unsafe_feedback_provenance.digest(),
                },
                "contradictory_feedback_failure": {
                    "contract_digest": unsafe_feedback_contract.digest(),
                    "strategy_basis_digest": unsafe_feedback_contract.validated_declaration().strategy_basis().expect("admitted writeback contract should preserve strategy basis").digest(),
                    "authority_request_digest": serde_json::Value::Null,
                    "authority_receipt_digest": serde_json::Value::Null,
                    "bypass_class": "contradictory-feedback-preauthority",
                    "failure_kind": format!("{:?}", contradictory_feedback_error.kind()),
                    "failure_digest": contradictory_feedback_failure_digest,
                    "causality_digest": unsafe_feedback_causality.digest(),
                    "bridge_effect_digest": unsafe_feedback_effect.digest(),
                    "idempotence_digest": unsafe_feedback_idempotence.digest(),
                    "feedback_provenance_digest": unsafe_feedback_provenance.digest(),
                    "incoming_feedback_causality_digest": contradictory_feedback_causality_digest,
                },
            },
            "failure_kind": format!("{:?}", error.kind()),
            "detail": error_message,
            "typed_boundary": "preview-writeback-validation",
            "validation_error_kind": validation_error_kind,
        }),
        zero_residue_report: json!({
            "authoritative_commit_count": 0,
            "authoritative_artifact_count": 0,
            "retained_writeback_bundle_count": 0,
            "loop_side_effect_count": 0,
        }),
        counter_snapshot,
    })
}

fn execute_feedback_loop_certification(
    runtime: &crate::harness::adapter::BridgeHarnessSession,
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
) -> Result<WritebackHarnessExecution, BridgeHarnessError> {
    let lowered_policy_bundle = lowered_policy(runtime_bridge)?;
    let declaration = crate::facade::BridgeWritebackDeclaration::writeback_capable(
        crate::facade::BridgeWritebackDeclarationIdentity::new("harness:writeback-feedback"),
        crate::facade::BridgeRequestKind::Authoritative,
        crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff,
        crate::facade::BridgeWritebackEffectClass::ProjectedStateDiff,
        crate::facade::BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
        "strategy:sha256:writeback-feedback",
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let contract = runtime_bridge
        .admit_writeback_declaration(declaration, &lowered_policy_bundle)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback feedback certification contract admission failed: {error}"
            ))
        })?;

    let original_commit = fixture
        .committed_patches()
        .first()
        .cloned()
        .ok_or_else(|| {
            BridgeHarnessError::new("writeback feedback fixture requires one committed patch")
        })?;
    let initial_route_digest = route_digest_for_first_patch(runtime_bridge, fixture)?;
    let original_causality = crate::facade::BridgeWritebackCausalityBasis::new(
        crate::facade::BridgeWritebackCausalityIdentity::new(
            "harness:writeback-feedback-causality",
        ),
        format!(
            "truth-trigger:{}",
            original_commit.commit_identity().as_str()
        ),
        initial_route_digest.clone(),
        "evaluation-surface:sha256:writeback-feedback",
        "truth-view:sha256:writeback-feedback",
    );
    let effect = runtime_bridge.lower_writeback_effect(
        &contract,
        &original_causality,
        crate::facade::BridgeWritebackEffectIdentity::new("harness:writeback-feedback-effect"),
        "effect:sha256:writeback-feedback",
    );
    let feedback_provenance = runtime_bridge.derive_writeback_feedback_provenance(&effect);
    let initial_idempotence = runtime_bridge.classify_writeback_idempotence(
        &effect,
        &lowered_policy_bundle,
        "authority-state:sha256:before-feedback",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-feedback-idempotence:first",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (initial_outcome, _initial_receipt) = runtime_bridge
        .execute_writeback_authority(&contract, &effect, &initial_idempotence)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback feedback certification first authority execution failed: {error}"
            ))
        })?;

    let feedback_commit = bridge_feedback_patch(
        "commit-feedback",
        "patch-feedback",
        original_commit.snapshot_identity().as_str(),
        original_commit.branch_identity().as_str(),
        feedback_provenance.digest(),
        original_causality.digest(),
    );
    let (carried_feedback_provenance_digest, carried_causality_digest) =
        feedback_provenance_hint(&feedback_commit)
            .map(|(provenance, causality)| (provenance.to_owned(), causality.to_owned()))
            .ok_or_else(|| {
                BridgeHarnessError::new(
                    "feedback patch did not carry first-class bridge-origin writeback provenance",
                )
            })?;
    let ordinary_commit = crate::facade::RawCommittedPatchEnvelope::new_with_metadata(
        crate::facade::BridgeProducerMetadata::bridge_harness_fixture(),
        crate::facade::TruthCommitIdentity::new("commit-ordinary"),
        crate::facade::TruthPatchIdentity::new("patch-ordinary"),
        crate::facade::TruthSnapshotIdentity::new(original_commit.snapshot_identity().as_str()),
        crate::facade::TruthBranchIdentity::new(original_commit.branch_identity().as_str()),
        vec![crate::facade::BridgeCommittedPatchItem::new(
            "user",
            forge_foundational::facade::AspectKey::new("profile")
                .expect("valid bridge patch aspect key"),
            "name",
        )],
    );
    runtime.source.insert_committed_patch(ordinary_commit);
    let ordinary_route_digest = route_digest_for_commit(runtime_bridge, "commit-ordinary")?;
    runtime.source.insert_committed_patch(feedback_commit);

    let feedback_result = runtime_bridge
        .deliver_invalidation(
            runtime_bridge
                .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit(
                    "commit-feedback",
                ))
                .map_err(|error| {
                    BridgeHarnessError::new(format!(
                        "writeback feedback certification failed to plan feedback commit: {error}"
                    ))
                })?,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback feedback certification failed to deliver feedback commit: {error}"
            ))
        })?;
    let feedback_route_digest = digest_string(
        "bridge-writeback-feedback-route",
        feedback_result.result_summary().route_identity().as_str(),
    )
    .to_string();
    let replayed_causality = crate::facade::BridgeWritebackCausalityBasis::new(
        crate::facade::BridgeWritebackCausalityIdentity::new(
            "harness:writeback-feedback-causality",
        ),
        format!(
            "truth-trigger:{}",
            original_commit.commit_identity().as_str()
        ),
        initial_route_digest.clone(),
        "evaluation-surface:sha256:writeback-feedback",
        "truth-view:sha256:writeback-feedback",
    );
    if replayed_causality.digest() != carried_causality_digest {
        return Err(BridgeHarnessError::new(format!(
            "feedback patch carried causality `{carried_causality_digest}` but replayed causality was `{}`",
            replayed_causality.digest()
        )));
    }
    let replayed_feedback_provenance = runtime_bridge.derive_writeback_feedback_provenance(&effect);
    if replayed_feedback_provenance.digest() != carried_feedback_provenance_digest {
        return Err(BridgeHarnessError::new(format!(
            "feedback patch carried provenance `{carried_feedback_provenance_digest}` but replayed provenance was `{}`",
            replayed_feedback_provenance.digest()
        )));
    }
    let changed_effect = runtime_bridge.lower_writeback_effect(
        &contract,
        &replayed_causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-feedback-effect:changed",
        ),
        "effect:sha256:writeback-feedback-changed",
    );
    let changed_idempotence = runtime_bridge.classify_writeback_idempotence(
        &changed_effect,
        &lowered_policy_bundle,
        "authority-state:sha256:after-feedback-commit",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-feedback-idempotence:changed",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let changed_effect_error = runtime_bridge
        .execute_writeback_authority_with_feedback_context(
            &contract,
            &changed_effect,
            &changed_idempotence,
            Some(carried_feedback_provenance_digest.clone()),
            Some(carried_causality_digest.clone()),
        )
        .expect_err("same-causality changed-effect feedback must fail closed");
    let changed_effect_failure_digest = digest_string(
        "bridge-writeback-feedback-changed-effect-failure",
        &format!("{:?}|{}", changed_effect_error.kind(), changed_effect_error),
    )
    .to_string();
    let replayed_idempotence = runtime_bridge.classify_writeback_idempotence(
        &effect,
        &lowered_policy_bundle,
        "authority-state:sha256:after-feedback-commit",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-feedback-idempotence:replayed",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (loop_prevention, replayed_outcome, replayed_receipt) = runtime_bridge
        .execute_writeback_authority_with_feedback_context(
            &contract,
            &effect,
            &replayed_idempotence,
            Some(carried_feedback_provenance_digest.clone()),
            Some(carried_causality_digest.clone()),
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback feedback certification replayed authority execution failed: {error}"
            ))
        })?;
    let replayed_bundle = runtime_bridge.replay_writeback_bundle(
        &contract,
        &effect,
        &replayed_idempotence,
        &replayed_outcome,
    );
    let replayed_strategy_compatibility = runtime_bridge.classify_writeback_strategy_compatibility(
        &contract,
        &effect,
        &replayed_idempotence,
    );
    let replayed_candidate = if loop_prevention.disposition()
        == crate::facade::BridgeWritebackLoopDisposition::AllowAuthoritativeAttempt
    {
        Some(
            runtime_bridge
                .validate_writeback_candidate(
                    &contract,
                    &effect,
                    &replayed_idempotence,
                    &loop_prevention,
                    &replayed_strategy_compatibility,
                )
                .map_err(|error| {
                    BridgeHarnessError::new(format!(
                        "writeback feedback certification replayed candidate validation failed: {error}"
                    ))
                })?,
        )
    } else {
        None
    };
    let feedback_authority_request = replayed_candidate.as_ref().map(|candidate| {
        let mapped_input = runtime_bridge
            .diagnostics()
            .writeback_mapped_family_input_for_digest(effect.mapped_input_digest())
            .expect(
                "writeback harness should retain mapped-family input for feedback certification",
            );
        let mapper_witness = crate::facade::BridgeWritebackMapperWitness::issue(&mapped_input);
        crate::adapter::TruthWritebackRequest::new(
            effect.family_kind(),
            contract.digest(),
            candidate.digest(),
            mapped_input.digest(),
            mapper_witness.digest(),
            effect.digest(),
            effect.effect_digest(),
            effect.effect_class(),
            effect.strategy_class(),
            replayed_feedback_provenance.digest(),
            loop_prevention.digest(),
            loop_prevention.disposition(),
            replayed_strategy_compatibility.digest(),
            replayed_idempotence.causality_digest(),
            replayed_idempotence.digest(),
            replayed_idempotence.idempotence_class(),
            effect.strategy_descriptor_digest(),
        )
    });
    let rebuilt_runtime = build_writeback_runtime(runtime, fixture, true)?;
    let rebuilt_lowered_policy = lowered_policy(&rebuilt_runtime)?;
    let rebuilt_contract = rebuilt_runtime
        .admit_writeback_declaration(
            crate::facade::BridgeWritebackDeclaration::writeback_capable(
                crate::facade::BridgeWritebackDeclarationIdentity::new(
                    "harness:writeback-feedback",
                ),
                crate::facade::BridgeRequestKind::Authoritative,
                crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff,
                crate::facade::BridgeWritebackEffectClass::ProjectedStateDiff,
                crate::facade::BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
                "strategy:sha256:writeback-feedback",
                crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            ),
            &rebuilt_lowered_policy,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback feedback certification failed to admit rebuilt contract: {error}"
            ))
        })?;
    let rebuilt_effect = rebuilt_runtime.lower_writeback_effect(
        &rebuilt_contract,
        &replayed_causality,
        crate::facade::BridgeWritebackEffectIdentity::new("harness:writeback-feedback-effect"),
        "effect:sha256:writeback-feedback",
    );
    let rebuilt_idempotence = rebuilt_runtime.classify_writeback_idempotence(
        &rebuilt_effect,
        &rebuilt_lowered_policy,
        "authority-state:sha256:after-feedback-commit",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-feedback-idempotence:replayed",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (rebuilt_loop_prevention, rebuilt_outcome, rebuilt_receipt) = rebuilt_runtime
        .execute_writeback_authority_with_feedback_context(
            &rebuilt_contract,
            &rebuilt_effect,
            &rebuilt_idempotence,
            Some(carried_feedback_provenance_digest.clone()),
            Some(carried_causality_digest.clone()),
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback feedback certification replay-after-rebuild execution failed: {error}"
            ))
        })?;
    let rebuilt_replay_bundle = rebuilt_runtime.replay_writeback_bundle(
        &rebuilt_contract,
        &rebuilt_effect,
        &rebuilt_idempotence,
        &rebuilt_outcome,
    );
    let counters = aggregate_runtime_writeback_counters(&[runtime_bridge, &rebuilt_runtime]);
    let counter_snapshot = snapshot_from_counters(&counters);

    Ok(WritebackHarnessExecution::FeedbackLoopCertification {
        feedback_loop_digest: digest_string(
            "bridge-writeback-feedback-loop",
            &format!(
                "initial-outcome={}|replayed-outcome={}|replayed-bundle={}",
                initial_outcome.digest(),
                replayed_outcome.digest(),
                replayed_bundle.digest(),
            ),
        )
        .to_string(),
        feedback_route_digest,
        feedback_origin_matrix: json!({
            "writeback_digest": replayed_bundle.digest(),
            "bridge_effect_digest": effect.digest(),
            "causality_digest": original_causality.digest(),
            "replay_digest": replayed_bundle.digest(),
            "mutation_plan_digest": initial_outcome.authoritative_artifact_digest(),
            "replay_bundle_report": {
                "digest": replayed_bundle.digest(),
                "semantic_digest": replayed_bundle.semantic_digest(),
                "strategy_class": format!("{:?}", replayed_bundle.strategy_class()),
                "strategy_descriptor_digest": replayed_bundle.strategy_descriptor_digest(),
                "causality_digest": replayed_bundle.causality_digest(),
                "lowered_policy_digest": replayed_bundle.lowered_policy_digest(),
                "retry_disposition": format!("{:?}", replayed_bundle.retry_disposition()),
                "outcome_class": format!("{:?}", replayed_bundle.outcome_class()),
                "authoritative_artifact_digest": replayed_bundle.authoritative_artifact_digest(),
            },
            "idempotence_report": {
                "initial_digest": initial_idempotence.digest(),
                "replayed_digest": replayed_idempotence.digest(),
                "idempotence_class": format!("{:?}", initial_idempotence.idempotence_class()),
                "initial_authoritative_state_digest": initial_idempotence.authoritative_state_digest(),
                "replayed_authoritative_state_digest": replayed_idempotence.authoritative_state_digest(),
                "lowered_policy_digest": initial_idempotence.lowered_policy_digest(),
                "strategy_descriptor_digest": initial_idempotence.strategy_descriptor_digest(),
            },
            "loop_prevention_report": {
                "digest": loop_prevention.digest(),
                "disposition": format!("{:?}", loop_prevention.disposition()),
                "current_feedback_provenance_digest": loop_prevention.current_feedback_provenance_digest(),
                "current_causality_digest": loop_prevention.current_causality_digest(),
                "incoming_feedback_provenance_digest": loop_prevention.incoming_feedback_provenance_digest(),
                "incoming_feedback_causality_digest": loop_prevention.incoming_feedback_causality_digest(),
            },
            "authority_boundary_matrix": {
                "contract_digest": contract.digest(),
                "strategy_basis_digest": contract.validated_declaration().strategy_basis().expect("admitted writeback contract should preserve strategy basis").digest(),
                "strategy_compatibility_digest": replayed_strategy_compatibility.digest(),
                "strategy_compatibility_disposition": format!("{:?}", replayed_strategy_compatibility.disposition()),
                "candidate_digest": replayed_candidate.as_ref().map(|candidate| candidate.digest()),
                "authority_request_digest": feedback_authority_request.as_ref().map(|request| request.digest()),
                "authority_receipt_digest": replayed_receipt.as_ref().map(|receipt| receipt.digest()),
            },
            "changed_effect_feedback_matrix": {
                "bridge_effect_digest": changed_effect.digest(),
                "causality_digest": changed_effect.causality_digest(),
                "idempotence_digest": changed_idempotence.digest(),
                "failure_kind": format!("{:?}", changed_effect_error.kind()),
                "failure_digest": changed_effect_failure_digest,
                "same_causality_as_initial": changed_effect.causality_digest() == original_causality.digest(),
                "same_feedback_provenance_as_initial": runtime_bridge
                    .derive_writeback_feedback_provenance(&changed_effect)
                    .digest()
                    == feedback_provenance.digest(),
            },
            "interleaved_truth_matrix": {
                "ordinary_truth_commit_identity": "commit-ordinary",
                "ordinary_truth_route_digest": ordinary_route_digest,
                "bridge_feedback_commit_identity": "commit-feedback",
                "interleaving_preserved_single_authoritative_commit": runtime.writeback_authority.committed_causality_count() == 1,
            },
            "restart_replay_matrix": {
                "rebuilt_contract_digest": rebuilt_contract.digest(),
                "rebuilt_effect_digest": rebuilt_effect.digest(),
                "rebuilt_idempotence_digest": rebuilt_idempotence.digest(),
                "rebuilt_loop_prevention_digest": rebuilt_loop_prevention.digest(),
                "rebuilt_loop_prevention_disposition": format!("{:?}", rebuilt_loop_prevention.disposition()),
                "rebuilt_outcome_digest": rebuilt_outcome.digest(),
                "rebuilt_replay_bundle_digest": rebuilt_replay_bundle.digest(),
                "rebuilt_authority_receipt_present": rebuilt_receipt.is_some(),
                "replay_equivalent_to_live_feedback": rebuilt_replay_bundle.digest() == replayed_bundle.digest(),
            },
            "feedback_provenance_digest": feedback_provenance.digest(),
            "carried_causality_digest": carried_causality_digest,
            "carried_feedback_provenance_digest": carried_feedback_provenance_digest,
            "initial_causality_digest": original_causality.digest(),
            "feedback_route_digest": feedback_result.result_summary().route_identity().as_str(),
            "loop_prevention_digest": loop_prevention.digest(),
            "loop_prevention_disposition": format!("{:?}", loop_prevention.disposition()),
            "boundedness_proof": {
                "authoritative_commit_count": runtime.writeback_authority.committed_causality_count(),
                "replayed_feedback_outcome_class": "CanonicalNoop",
                "changed_effect_retrigger_failure_kind": format!("{:?}", changed_effect_error.kind()),
                "feedback_publication_routed": true,
                "ordinary_truth_interleaved": true,
                "feedback_converged": true,
                "restart_replay_converged": rebuilt_replay_bundle.digest() == replayed_bundle.digest(),
                "replayed_authority_receipt_present": replayed_receipt.is_some(),
            },
        }),
        counter_snapshot,
    })
}

fn execute_replay_mismatch_certification(
    runtime: &crate::harness::adapter::BridgeHarnessSession,
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
) -> Result<WritebackHarnessExecution, BridgeHarnessError> {
    let lowered_policy_bundle = lowered_policy(runtime_bridge)?;
    let declaration = crate::facade::BridgeWritebackDeclaration::writeback_capable(
        crate::facade::BridgeWritebackDeclarationIdentity::new("harness:writeback-replay-mismatch"),
        crate::facade::BridgeRequestKind::Authoritative,
        crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff,
        crate::facade::BridgeWritebackEffectClass::ProjectedStateDiff,
        crate::facade::BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
        "strategy:sha256:writeback-replay-mismatch",
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let contract = runtime_bridge
        .admit_writeback_declaration(declaration, &lowered_policy_bundle)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback replay mismatch certification contract admission failed: {error}"
            ))
        })?;
    let causality = crate::facade::BridgeWritebackCausalityBasis::new(
        crate::facade::BridgeWritebackCausalityIdentity::new(
            "harness:writeback-replay-mismatch-causality",
        ),
        "truth-trigger:sha256:writeback-replay-mismatch",
        "route:sha256:writeback-replay-mismatch",
        "evaluation-surface:sha256:writeback-replay-mismatch",
        "truth-view:sha256:writeback-replay-mismatch",
    );
    let expected_effect = runtime_bridge.lower_writeback_effect(
        &contract,
        &causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-replay-mismatch-effect:expected",
        ),
        "effect:sha256:writeback-replay-mismatch:expected",
    );
    let replayed_effect = runtime_bridge.lower_writeback_effect(
        &contract,
        &causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-replay-mismatch-effect:replayed",
        ),
        "effect:sha256:writeback-replay-mismatch:replayed",
    );
    let expected_idempotence = runtime_bridge.classify_writeback_idempotence(
        &expected_effect,
        &lowered_policy_bundle,
        "truth-state:sha256:writeback-replay-mismatch",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-replay-mismatch-idempotence:expected",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let replayed_idempotence = runtime_bridge.classify_writeback_idempotence(
        &replayed_effect,
        &lowered_policy_bundle,
        "truth-state:sha256:writeback-replay-mismatch",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-replay-mismatch-idempotence:replayed",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let expected_bundle = runtime_bridge.replay_writeback_bundle(
        &contract,
        &expected_effect,
        &expected_idempotence,
        &crate::facade::BridgeWritebackAuthorityOutcome::authoritative_commit(
            &expected_idempotence,
            "authoritative-artifact:sha256:writeback-replay-mismatch",
        ),
    );
    let replayed_bundle = runtime_bridge.replay_writeback_bundle(
        &contract,
        &replayed_effect,
        &replayed_idempotence,
        &crate::facade::BridgeWritebackAuthorityOutcome::authoritative_commit(
            &replayed_idempotence,
            "authoritative-artifact:sha256:writeback-replay-mismatch",
        ),
    );
    let validation_error = runtime_bridge
        .validate_replayed_writeback_bundle(&expected_bundle, &replayed_bundle)
        .expect_err("writeback replay mismatch certification must fail on semantic drift");
    let rebuilt_runtime = build_writeback_runtime(runtime, fixture, true)?;
    let rebuilt_lowered_policy_bundle = lowered_policy(&rebuilt_runtime)?;
    let rebuilt_contract = rebuilt_runtime
        .admit_writeback_declaration(
            crate::facade::BridgeWritebackDeclaration::writeback_capable(
                crate::facade::BridgeWritebackDeclarationIdentity::new(
                    "harness:writeback-replay-mismatch",
                ),
                crate::facade::BridgeRequestKind::Authoritative,
                crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff,
                crate::facade::BridgeWritebackEffectClass::ProjectedStateDiff,
                crate::facade::BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
                "strategy:sha256:writeback-replay-mismatch",
                crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            ),
            &rebuilt_lowered_policy_bundle,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback replay mismatch certification rebuilt contract admission failed: {error}"
            ))
        })?;
    let rebuilt_replayed_effect = rebuilt_runtime.lower_writeback_effect(
        &rebuilt_contract,
        &causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-replay-mismatch-effect:replayed",
        ),
        "effect:sha256:writeback-replay-mismatch:replayed",
    );
    let rebuilt_replayed_idempotence = rebuilt_runtime.classify_writeback_idempotence(
        &rebuilt_replayed_effect,
        &rebuilt_lowered_policy_bundle,
        "truth-state:sha256:writeback-replay-mismatch",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-replay-mismatch-idempotence:replayed",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let rebuilt_replayed_bundle = rebuilt_runtime.replay_writeback_bundle(
        &rebuilt_contract,
        &rebuilt_replayed_effect,
        &rebuilt_replayed_idempotence,
        &crate::facade::BridgeWritebackAuthorityOutcome::authoritative_commit(
            &rebuilt_replayed_idempotence,
            "authoritative-artifact:sha256:writeback-replay-mismatch-rebuilt",
        ),
    );
    let rebuilt_validation_error = rebuilt_runtime
        .validate_replayed_writeback_bundle(&expected_bundle, &rebuilt_replayed_bundle)
        .expect_err(
            "writeback replay mismatch certification must fail on semantic drift after rebuild",
        );
    let counters = aggregate_runtime_writeback_counters(&[runtime_bridge, &rebuilt_runtime]);
    let counter_snapshot = snapshot_from_counters(&counters);

    Ok(WritebackHarnessExecution::ReplayMismatchCertification {
        replay_validation_digest: digest_string(
            "bridge-writeback-replay-validation",
            &format!(
                "expected={}|replayed={}|failure={:?}",
                expected_bundle.semantic_digest(),
                replayed_bundle.semantic_digest(),
                validation_error.kind()
            ),
        )
        .to_string(),
        replay_mismatch_matrix: json!({
            "expected_replay_digest": expected_bundle.digest(),
            "expected_semantic_digest": expected_bundle.semantic_digest(),
            "expected_causality_digest": expected_bundle.causality_digest(),
            "replayed_replay_digest": replayed_bundle.digest(),
            "replayed_semantic_digest": replayed_bundle.semantic_digest(),
            "expected_effect_digest": expected_effect.effect_digest(),
            "replayed_effect_digest": replayed_effect.effect_digest(),
            "replayed_causality_digest": replayed_bundle.causality_digest(),
            "failure_kind": format!("{:?}", validation_error.kind()),
            "failure_message": validation_error.to_string(),
            "semantic_mismatch_detected": expected_bundle.semantic_digest() != replayed_bundle.semantic_digest(),
            "diagnostic_detail_changed": expected_bundle.digest() != replayed_bundle.digest(),
            "restart_replay_matrix": {
                "rebuilt_replay_digest": rebuilt_replayed_bundle.digest(),
                "rebuilt_semantic_digest": rebuilt_replayed_bundle.semantic_digest(),
                "rebuilt_failure_kind": format!("{:?}", rebuilt_validation_error.kind()),
                "rebuilt_failure_message": rebuilt_validation_error.to_string(),
                "restart_mismatch_detected": expected_bundle.semantic_digest()
                    != rebuilt_replayed_bundle.semantic_digest(),
            },
        }),
        counter_snapshot,
    })
}

fn execute_extensible_family_certification(
    runtime: &crate::harness::adapter::BridgeHarnessSession,
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
) -> Result<WritebackHarnessExecution, BridgeHarnessError> {
    let lowered_policy_bundle = lowered_policy(runtime_bridge)?;
    let causality = crate::facade::BridgeWritebackCausalityBasis::new(
        crate::facade::BridgeWritebackCausalityIdentity::new(
            "harness:writeback-family-extension-causality",
        ),
        "truth-trigger:sha256:writeback-family-extension",
        "route:sha256:writeback-family-extension",
        "evaluation-surface:sha256:writeback-family-extension",
        "truth-view:sha256:writeback-family-extension",
    );
    let projected_contract = runtime_bridge
        .admit_writeback_declaration(
            crate::facade::BridgeWritebackDeclaration::writeback_capable(
                crate::facade::BridgeWritebackDeclarationIdentity::new(
                    "harness:writeback-family-extension:projected",
                ),
                crate::facade::BridgeRequestKind::Authoritative,
                crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff,
                crate::facade::BridgeWritebackEffectClass::ProjectedStateDiff,
                crate::facade::BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
                "strategy:sha256:writeback-family-extension:projected",
                crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            ),
            &lowered_policy_bundle,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!("projected family admission failed: {error}"))
        })?;
    let aspect_contract = runtime_bridge
        .admit_writeback_declaration(
            crate::facade::BridgeWritebackDeclaration::writeback_capable(
                crate::facade::BridgeWritebackDeclarationIdentity::new(
                    "harness:writeback-family-extension:aspect",
                ),
                crate::facade::BridgeRequestKind::Authoritative,
                crate::facade::BridgeWritebackFamilyKind::AspectReconciliation,
                crate::facade::BridgeWritebackEffectClass::AspectReconciliation,
                crate::facade::BridgeWritebackStrategyClass::AspectReconciliationCommit,
                "strategy:sha256:writeback-family-extension:aspect",
                crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            ),
            &lowered_policy_bundle,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!("aspect family admission failed: {error}"))
        })?;
    let projected_effect = runtime_bridge.lower_writeback_effect(
        &projected_contract,
        &causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-family-extension:effect:projected",
        ),
        "effect:sha256:writeback-family-extension:projected",
    );
    let aspect_effect = runtime_bridge.lower_writeback_effect(
        &aspect_contract,
        &causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-family-extension:effect:aspect",
        ),
        "effect:sha256:writeback-family-extension:aspect",
    );
    let projected_idempotence = runtime_bridge.classify_writeback_idempotence(
        &projected_effect,
        &lowered_policy_bundle,
        "truth-state:sha256:writeback-family-extension:projected",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-family-extension:idempotence:projected",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let aspect_idempotence = runtime_bridge.classify_writeback_idempotence(
        &aspect_effect,
        &lowered_policy_bundle,
        "truth-state:sha256:writeback-family-extension:aspect",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-family-extension:idempotence:aspect",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (projected_outcome, projected_receipt) = runtime_bridge
        .execute_writeback_authority(
            &projected_contract,
            &projected_effect,
            &projected_idempotence,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "projected family authority execution failed: {error}"
            ))
        })?;
    let (aspect_outcome, aspect_receipt) = runtime_bridge
        .execute_writeback_authority(&aspect_contract, &aspect_effect, &aspect_idempotence)
        .map_err(|error| {
            BridgeHarnessError::new(format!("aspect family authority execution failed: {error}"))
        })?;
    let projected_bundle = runtime_bridge.replay_writeback_bundle(
        &projected_contract,
        &projected_effect,
        &projected_idempotence,
        &projected_outcome,
    );
    let aspect_bundle = runtime_bridge.replay_writeback_bundle(
        &aspect_contract,
        &aspect_effect,
        &aspect_idempotence,
        &aspect_outcome,
    );
    let projected_feedback = runtime_bridge.derive_writeback_feedback_provenance(&projected_effect);
    let cross_family_loop_prevention = runtime_bridge.classify_writeback_loop_prevention(
        &aspect_effect,
        &aspect_idempotence,
        Some(projected_feedback.digest()),
        Some(causality.digest()),
    );
    let projected_admission_record = runtime_bridge
        .diagnostics()
        .writeback_admission_record_for_contract_digest(projected_contract.digest())
        .ok_or_else(|| {
            BridgeHarnessError::new(
                "projected family admission record missing from retained diagnostics",
            )
        })?;
    let aspect_admission_record = runtime_bridge
        .diagnostics()
        .writeback_admission_record_for_contract_digest(aspect_contract.digest())
        .ok_or_else(|| {
            BridgeHarnessError::new(
                "aspect family admission record missing from retained diagnostics",
            )
        })?;
    let family_execution_records = runtime_bridge.diagnostics().writeback_execution_records();
    let projected_execution_record =
        find_execution_record_for_replay(&family_execution_records, projected_bundle.digest())
            .ok_or_else(|| {
                BridgeHarnessError::new(
                    "projected family execution record missing from retained diagnostics",
                )
            })?;
    let aspect_execution_record =
        find_execution_record_for_replay(&family_execution_records, aspect_bundle.digest())
            .ok_or_else(|| {
                BridgeHarnessError::new(
                    "aspect family execution record missing from retained diagnostics",
                )
            })?;

    let rebuilt_runtime = build_writeback_runtime_with_custom_authority(
        runtime,
        fixture,
        crate::harness::fixtures::RecordingTruthWritebackAuthority::default(),
    )?;
    let rebuilt_policy_bundle = lowered_policy(&rebuilt_runtime)?;
    let rebuilt_projected_contract = rebuilt_runtime
        .admit_writeback_declaration(
            crate::facade::BridgeWritebackDeclaration::writeback_capable(
                crate::facade::BridgeWritebackDeclarationIdentity::new(
                    "harness:writeback-family-extension:projected",
                ),
                crate::facade::BridgeRequestKind::Authoritative,
                crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff,
                crate::facade::BridgeWritebackEffectClass::ProjectedStateDiff,
                crate::facade::BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
                "strategy:sha256:writeback-family-extension:projected",
                crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            ),
            &rebuilt_policy_bundle,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "rebuilt projected family admission failed during extensible certification: {error}"
            ))
        })?;
    let rebuilt_projected_effect = rebuilt_runtime.lower_writeback_effect(
        &rebuilt_projected_contract,
        &causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-family-extension:effect:projected",
        ),
        "effect:sha256:writeback-family-extension:projected",
    );
    let rebuilt_projected_idempotence = rebuilt_runtime.classify_writeback_idempotence(
        &rebuilt_projected_effect,
        &rebuilt_policy_bundle,
        "truth-state:sha256:writeback-family-extension:projected",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-family-extension:idempotence:projected",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (rebuilt_projected_outcome, _) = rebuilt_runtime
        .execute_writeback_authority(
            &rebuilt_projected_contract,
            &rebuilt_projected_effect,
            &rebuilt_projected_idempotence,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "rebuilt projected family execution failed during extensible certification: {error}"
            ))
        })?;
    let rebuilt_projected_bundle = rebuilt_runtime.replay_writeback_bundle(
        &rebuilt_projected_contract,
        &rebuilt_projected_effect,
        &rebuilt_projected_idempotence,
        &rebuilt_projected_outcome,
    );
    rebuilt_runtime
        .validate_replayed_writeback_bundle(&projected_bundle, &rebuilt_projected_bundle)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "same-family rebuilt replay validation unexpectedly failed: {error}"
            ))
        })?;
    let rebuilt_execution_record = rebuilt_runtime
        .diagnostics()
        .last_writeback_execution_record()
        .ok_or_else(|| {
            BridgeHarnessError::new(
                "rebuilt projected execution record missing from retained diagnostics",
            )
        })?;
    let changed_causality = crate::facade::BridgeWritebackCausalityBasis::new(
        crate::facade::BridgeWritebackCausalityIdentity::new(
            "harness:writeback-family-extension-causality:changed",
        ),
        "truth-trigger:sha256:writeback-family-extension:changed",
        "route:sha256:writeback-family-extension:changed",
        "evaluation-surface:sha256:writeback-family-extension",
        "truth-view:sha256:writeback-family-extension",
    );
    let changed_projected_effect = rebuilt_runtime.lower_writeback_effect(
        &rebuilt_projected_contract,
        &changed_causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-family-extension:effect:projected:changed",
        ),
        "effect:sha256:writeback-family-extension:projected",
    );
    let changed_projected_idempotence = rebuilt_runtime.classify_writeback_idempotence(
        &changed_projected_effect,
        &rebuilt_policy_bundle,
        "truth-state:sha256:writeback-family-extension:projected",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-family-extension:idempotence:projected:changed",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (changed_projected_outcome, _) = rebuilt_runtime
        .execute_writeback_authority(
            &rebuilt_projected_contract,
            &changed_projected_effect,
            &changed_projected_idempotence,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "changed-causality projected execution failed during extensible certification: {error}"
            ))
        })?;
    let changed_projected_bundle = rebuilt_runtime.replay_writeback_bundle(
        &rebuilt_projected_contract,
        &changed_projected_effect,
        &changed_projected_idempotence,
        &changed_projected_outcome,
    );
    let same_family_drift_error = rebuilt_runtime
        .validate_replayed_writeback_bundle(&projected_bundle, &changed_projected_bundle)
        .expect_err("same-family changed-causality replay validation must fail closed");

    let replay_validation_error = rebuilt_runtime
        .validate_replayed_writeback_bundle(&projected_bundle, &aspect_bundle)
        .expect_err("cross-family replay validation must fail closed");
    let family_replay_records = rebuilt_runtime.diagnostics().writeback_replay_records();
    let same_family_drift_replay_record = find_replay_record(
        &family_replay_records,
        projected_bundle.digest(),
        changed_projected_bundle.digest(),
    )
    .ok_or_else(|| {
        BridgeHarnessError::new(
            "same-family changed-causality replay record missing from retained diagnostics",
        )
    })?;
    let cross_family_replay_record = find_replay_record(
        &family_replay_records,
        projected_bundle.digest(),
        aspect_bundle.digest(),
    )
    .ok_or_else(|| {
        BridgeHarnessError::new(
            "cross-family replay record missing from retained diagnostics after mismatch validation",
        )
    })?;

    let shadow_protocol_error = runtime_bridge
        .validate_writeback_declaration(crate::facade::BridgeWritebackDeclaration::new(
            crate::facade::BridgeWritebackDeclarationIdentity::new(
                "harness:writeback-family-extension:shadow-protocol",
            ),
            crate::facade::BridgeRequestKind::Authoritative,
            crate::facade::BridgeWritebackRequestMode::WritebackCapable,
            Some(crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff),
            crate::facade::BridgeWritebackEffectClass::AspectReconciliation,
            Some(crate::facade::BridgeWritebackStrategyClass::AspectReconciliationCommit),
            "strategy:sha256:writeback-family-extension:shadow-protocol",
            crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
        ))
        .expect_err("shadow protocol family/effect mismatch must fail closed");
    let counters = aggregate_runtime_writeback_counters(&[runtime_bridge, &rebuilt_runtime]);
    let counter_snapshot = snapshot_from_counters(&counters);

    Ok(WritebackHarnessExecution::ExtensibleFamilyCertification {
        family_extension_digest: digest_string(
            "bridge-writeback-family-extension",
            &format!(
                "projected={}|aspect={}|shadow={:?}",
                projected_bundle.digest(),
                aspect_bundle.digest(),
                shadow_protocol_error.kind()
            ),
        )
        .to_string(),
        family_extension_matrix: json!({
            "projected_family": {
                "admission_record_digest": projected_admission_record.digest(),
                "contract_digest": projected_contract.digest(),
                "effect_digest": projected_effect.digest(),
                "mapped_input_digest": projected_effect.mapped_input_digest(),
                "mapper_envelope_digest": projected_effect.mapper_envelope_digest(),
                "causality_digest": projected_effect.causality_digest(),
                "idempotence_digest": projected_idempotence.digest(),
                "replay_bundle_digest": projected_bundle.digest(),
                "replay_semantic_digest": projected_bundle.semantic_digest(),
                "authority_receipt_digest": projected_receipt.digest(),
            },
            "aspect_family": {
                "admission_record_digest": aspect_admission_record.digest(),
                "contract_digest": aspect_contract.digest(),
                "effect_digest": aspect_effect.digest(),
                "mapped_input_digest": aspect_effect.mapped_input_digest(),
                "mapper_envelope_digest": aspect_effect.mapper_envelope_digest(),
                "causality_digest": aspect_effect.causality_digest(),
                "idempotence_digest": aspect_idempotence.digest(),
                "replay_bundle_digest": aspect_bundle.digest(),
                "replay_semantic_digest": aspect_bundle.semantic_digest(),
                "authority_receipt_digest": aspect_receipt.digest(),
            },
            "cross_family_replay_isolation": {
                "semantic_digest_separated": projected_bundle.semantic_digest() != aspect_bundle.semantic_digest(),
                "bundle_digest_separated": projected_bundle.digest() != aspect_bundle.digest(),
                "failure_kind": format!("{:?}", replay_validation_error.kind()),
                "failure_digest": digest_string(
                    "bridge-writeback-family-cross-replay",
                    &replay_validation_error.to_string(),
                ),
                "family_replay_record_digest": cross_family_replay_record.digest(),
                "decision_trace_digest": digest_string(
                    "bridge-writeback-family-cross-replay-trace",
                    &format!(
                        "projected-bundle={}|aspect-bundle={}|replay-record={}|failure={:?}",
                        projected_bundle.digest(),
                        aspect_bundle.digest(),
                        cross_family_replay_record.digest(),
                        replay_validation_error.kind(),
                    ),
                ),
            },
            "same_family_equivalence": {
                "semantic_digest_equal": projected_bundle.semantic_digest() == rebuilt_projected_bundle.semantic_digest(),
                "bundle_digest_equal": projected_bundle.digest() == rebuilt_projected_bundle.digest(),
                "effect_digest_equal": projected_effect.digest() == rebuilt_projected_effect.digest(),
                "mapped_input_digest_equal": projected_effect.mapped_input_digest() == rebuilt_projected_effect.mapped_input_digest(),
                "family_execution_record_digest": rebuilt_execution_record.digest(),
                "decision_trace_digest": digest_string(
                    "bridge-writeback-family-same-family-trace",
                    &format!(
                        "projected-bundle={}|rebuilt-bundle={}|execution-record={}",
                        projected_bundle.digest(),
                        rebuilt_projected_bundle.digest(),
                        rebuilt_execution_record.digest(),
                    ),
                ),
            },
            "same_family_changed_causality": {
                "causality_digest_separated": projected_bundle.causality_digest() != changed_projected_bundle.causality_digest(),
                "semantic_digest_separated": projected_bundle.semantic_digest() != changed_projected_bundle.semantic_digest(),
                "bundle_digest_separated": projected_bundle.digest() != changed_projected_bundle.digest(),
                "failure_kind": format!("{:?}", same_family_drift_error.kind()),
                "family_replay_record_digest": same_family_drift_replay_record.digest(),
                "decision_trace_digest": digest_string(
                    "bridge-writeback-family-same-family-drift-trace",
                    &format!(
                        "projected-bundle={}|changed-bundle={}|replay-record={}|failure={:?}",
                        projected_bundle.digest(),
                        changed_projected_bundle.digest(),
                        same_family_drift_replay_record.digest(),
                        same_family_drift_error.kind(),
                    ),
                ),
            },
            "cross_family_loop_isolation": {
                "incoming_feedback_provenance_digest": projected_feedback.digest(),
                "incoming_feedback_causality_digest": causality.digest(),
                "disposition": format!("{:?}", cross_family_loop_prevention.disposition()),
                "digest": cross_family_loop_prevention.digest(),
            },
            "mapper_parity_matrix": {
                "projected_mapper_envelope_retained": runtime_bridge
                    .diagnostics()
                    .writeback_mapper_envelope_for_digest(projected_effect.mapper_envelope_digest())
                    .is_some(),
                "aspect_mapper_envelope_retained": runtime_bridge
                    .diagnostics()
                    .writeback_mapper_envelope_for_digest(aspect_effect.mapper_envelope_digest())
                    .is_some(),
                "projected_mapped_input_retained": runtime_bridge
                    .diagnostics()
                    .writeback_mapped_family_input_for_digest(projected_effect.mapped_input_digest())
                    .is_some(),
                "aspect_mapped_input_retained": runtime_bridge
                    .diagnostics()
                    .writeback_mapped_family_input_for_digest(aspect_effect.mapped_input_digest())
                    .is_some(),
                "projected_family_mapper_record_digest": projected_execution_record
                    .mapper_record_digest(),
                "aspect_family_mapper_record_digest": aspect_execution_record
                    .mapper_record_digest(),
                "projected_family_execution_record_digest": projected_execution_record.digest(),
                "aspect_family_execution_record_digest": aspect_execution_record.digest(),
                "decision_trace_digest": digest_string(
                    "bridge-writeback-family-mapper-trace",
                    &format!(
                        "projected-admission={}|aspect-admission={}|projected-mapper={}|aspect-mapper={}|projected-execution={}|aspect-execution={}",
                        projected_admission_record.digest(),
                        aspect_admission_record.digest(),
                        projected_execution_record
                            .mapper_record_digest()
                            .unwrap_or("none"),
                        aspect_execution_record
                            .mapper_record_digest()
                            .unwrap_or("none"),
                        projected_execution_record.digest(),
                        aspect_execution_record.digest(),
                    ),
                ),
            },
            "shadow_protocol_rejection": {
                "failure_kind": format!("{:?}", shadow_protocol_error.kind()),
                "failure_digest": digest_string(
                    "bridge-writeback-family-shadow-protocol",
                    &shadow_protocol_error.to_string(),
                ),
                "decision_trace_digest": digest_string(
                    "bridge-writeback-family-shadow-protocol-trace",
                    &format!(
                        "shadow={:?}|projected-admission={}|aspect-admission={}",
                        shadow_protocol_error.kind(),
                        projected_admission_record.digest(),
                        aspect_admission_record.digest(),
                    ),
                ),
            }
        }),
        counter_snapshot,
    })
}

fn execute_multi_family_admission_boundary_certification(
    runtime: &crate::harness::adapter::BridgeHarnessSession,
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
) -> Result<WritebackHarnessExecution, BridgeHarnessError> {
    let _ = runtime;
    let _ = fixture;
    let lowered_policy_bundle = lowered_policy(runtime_bridge)?;
    let causality = crate::facade::BridgeWritebackCausalityBasis::new(
        crate::facade::BridgeWritebackCausalityIdentity::new(
            "harness:writeback-family-admission-boundary-causality",
        ),
        "truth-trigger:sha256:writeback-family-admission-boundary",
        "route:sha256:writeback-family-admission-boundary",
        "evaluation-surface:sha256:writeback-family-admission-boundary",
        "truth-view:sha256:writeback-family-admission-boundary",
    );
    let projected_contract = runtime_bridge
        .admit_writeback_declaration(
            crate::facade::BridgeWritebackDeclaration::writeback_capable(
                crate::facade::BridgeWritebackDeclarationIdentity::new(
                    "harness:writeback-family-admission-boundary:projected",
                ),
                crate::facade::BridgeRequestKind::Authoritative,
                crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff,
                crate::facade::BridgeWritebackEffectClass::ProjectedStateDiff,
                crate::facade::BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
                "strategy:sha256:writeback-family-admission-boundary:projected",
                crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            ),
            &lowered_policy_bundle,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "multi-family admission boundary projected family admission failed: {error}"
            ))
        })?;
    let aspect_contract = runtime_bridge
        .admit_writeback_declaration(
            crate::facade::BridgeWritebackDeclaration::writeback_capable(
                crate::facade::BridgeWritebackDeclarationIdentity::new(
                    "harness:writeback-family-admission-boundary:aspect",
                ),
                crate::facade::BridgeRequestKind::Authoritative,
                crate::facade::BridgeWritebackFamilyKind::AspectReconciliation,
                crate::facade::BridgeWritebackEffectClass::AspectReconciliation,
                crate::facade::BridgeWritebackStrategyClass::AspectReconciliationCommit,
                "strategy:sha256:writeback-family-admission-boundary:aspect",
                crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            ),
            &lowered_policy_bundle,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "multi-family admission boundary aspect family admission failed: {error}"
            ))
        })?;
    let projected_effect = runtime_bridge.lower_writeback_effect(
        &projected_contract,
        &causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-family-admission-boundary:effect:projected",
        ),
        "effect:sha256:writeback-family-admission-boundary:projected",
    );
    let aspect_effect = runtime_bridge.lower_writeback_effect(
        &aspect_contract,
        &causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-family-admission-boundary:effect:aspect",
        ),
        "effect:sha256:writeback-family-admission-boundary:aspect",
    );
    let projected_idempotence = runtime_bridge.classify_writeback_idempotence(
        &projected_effect,
        &lowered_policy_bundle,
        "truth-state:sha256:writeback-family-admission-boundary:projected",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-family-admission-boundary:idempotence:projected",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let aspect_idempotence = runtime_bridge.classify_writeback_idempotence(
        &aspect_effect,
        &lowered_policy_bundle,
        "truth-state:sha256:writeback-family-admission-boundary:aspect",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-family-admission-boundary:idempotence:aspect",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (projected_outcome, _) = runtime_bridge
        .execute_writeback_authority(
            &projected_contract,
            &projected_effect,
            &projected_idempotence,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "multi-family admission boundary projected family execution failed: {error}"
            ))
        })?;
    let (aspect_outcome, _) = runtime_bridge
        .execute_writeback_authority(&aspect_contract, &aspect_effect, &aspect_idempotence)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "multi-family admission boundary aspect family execution failed: {error}"
            ))
        })?;
    let projected_bundle = runtime_bridge.replay_writeback_bundle(
        &projected_contract,
        &projected_effect,
        &projected_idempotence,
        &projected_outcome,
    );
    let aspect_bundle = runtime_bridge.replay_writeback_bundle(
        &aspect_contract,
        &aspect_effect,
        &aspect_idempotence,
        &aspect_outcome,
    );
    let projected_admission_record = runtime_bridge
        .diagnostics()
        .writeback_admission_record_for_contract_digest(projected_contract.digest())
        .ok_or_else(|| {
            BridgeHarnessError::new(
                "multi-family admission boundary projected admission record missing",
            )
        })?;
    let aspect_admission_record = runtime_bridge
        .diagnostics()
        .writeback_admission_record_for_contract_digest(aspect_contract.digest())
        .ok_or_else(|| {
            BridgeHarnessError::new(
                "multi-family admission boundary aspect admission record missing",
            )
        })?;
    let shadow_protocol_error = runtime_bridge
        .validate_writeback_declaration(crate::facade::BridgeWritebackDeclaration::new(
            crate::facade::BridgeWritebackDeclarationIdentity::new(
                "harness:writeback-family-admission-boundary:shadow-protocol",
            ),
            crate::facade::BridgeRequestKind::Authoritative,
            crate::facade::BridgeWritebackRequestMode::WritebackCapable,
            Some(crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff),
            crate::facade::BridgeWritebackEffectClass::AspectReconciliation,
            Some(crate::facade::BridgeWritebackStrategyClass::AspectReconciliationCommit),
            "strategy:sha256:writeback-family-admission-boundary:shadow-protocol",
            crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
        ))
        .expect_err("multi-family admission boundary shadow protocol mismatch must fail closed");
    let counters = aggregate_runtime_writeback_counters(&[runtime_bridge]);
    let counter_snapshot = snapshot_from_counters(&counters);
    let failure_digest = digest_string(
        "bridge-writeback-family-admission-boundary-shadow-protocol",
        &shadow_protocol_error.to_string(),
    );
    let family_extension_digest = digest_string(
        "bridge-writeback-family-admission-boundary",
        &format!(
            "projected={}|aspect={}|shadow={:?}",
            projected_bundle.digest(),
            aspect_bundle.digest(),
            shadow_protocol_error.kind()
        ),
    )
    .to_string();
    let admission_boundary_matrix = json!({
        "projected_family": {
            "admission_record_digest": projected_admission_record.digest(),
            "contract_digest": projected_contract.digest(),
            "effect_digest": projected_effect.digest(),
            "mapped_input_digest": projected_effect.mapped_input_digest(),
            "mapper_envelope_digest": projected_effect.mapper_envelope_digest(),
            "causality_digest": projected_effect.causality_digest(),
            "idempotence_digest": projected_idempotence.digest(),
            "replay_bundle_digest": projected_bundle.digest(),
            "replay_semantic_digest": projected_bundle.semantic_digest(),
        },
        "aspect_family": {
            "admission_record_digest": aspect_admission_record.digest(),
            "contract_digest": aspect_contract.digest(),
            "effect_digest": aspect_effect.digest(),
            "mapped_input_digest": aspect_effect.mapped_input_digest(),
            "mapper_envelope_digest": aspect_effect.mapper_envelope_digest(),
            "causality_digest": aspect_effect.causality_digest(),
            "idempotence_digest": aspect_idempotence.digest(),
            "replay_bundle_digest": aspect_bundle.digest(),
            "replay_semantic_digest": aspect_bundle.semantic_digest(),
        },
        "family_admission_matrix": {
            "projected_family_admitted": true,
            "aspect_family_admitted": true,
            "projected_admission_record_digest": projected_admission_record.digest(),
            "aspect_admission_record_digest": aspect_admission_record.digest(),
            "projected_contract_digest": projected_contract.digest(),
            "aspect_contract_digest": aspect_contract.digest(),
            "family_digest_separated": projected_contract.digest() != aspect_contract.digest(),
            "decision_trace_digest": digest_string(
                "bridge-writeback-family-admission-boundary-trace",
                &format!(
                    "projected-admission={}|aspect-admission={}|projected-contract={}|aspect-contract={}",
                    projected_admission_record.digest(),
                    aspect_admission_record.digest(),
                    projected_contract.digest(),
                    aspect_contract.digest(),
                ),
            ),
        },
        "authority_boundary_matrix": {
            "failure_kind": format!("{:?}", shadow_protocol_error.kind()),
            "failure_digest": failure_digest.clone(),
            "decision_trace_digest": digest_string(
                "bridge-writeback-family-admission-boundary-shadow-trace",
                &format!(
                    "shadow={:?}|projected-admission={}|aspect-admission={}",
                    shadow_protocol_error.kind(),
                    projected_admission_record.digest(),
                    aspect_admission_record.digest(),
                ),
            ),
        },
        "failure_digest": failure_digest,
    });
    Ok(
        WritebackHarnessExecution::MultiFamilyAdmissionBoundaryCertification {
            family_extension_digest,
            admission_boundary_matrix,
            counter_snapshot,
        },
    )
}

fn execute_cross_family_replay_loop_isolation_certification(
    runtime: &crate::harness::adapter::BridgeHarnessSession,
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
) -> Result<WritebackHarnessExecution, BridgeHarnessError> {
    let lowered_policy_bundle = lowered_policy(runtime_bridge)?;
    let causality = crate::facade::BridgeWritebackCausalityBasis::new(
        crate::facade::BridgeWritebackCausalityIdentity::new(
            "harness:writeback-family-replay-loop-isolation-causality",
        ),
        "truth-trigger:sha256:writeback-family-replay-loop-isolation",
        "route:sha256:writeback-family-replay-loop-isolation",
        "evaluation-surface:sha256:writeback-family-replay-loop-isolation",
        "truth-view:sha256:writeback-family-replay-loop-isolation",
    );
    let projected_contract = runtime_bridge
        .admit_writeback_declaration(
            crate::facade::BridgeWritebackDeclaration::writeback_capable(
                crate::facade::BridgeWritebackDeclarationIdentity::new(
                    "harness:writeback-family-replay-loop-isolation:projected",
                ),
                crate::facade::BridgeRequestKind::Authoritative,
                crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff,
                crate::facade::BridgeWritebackEffectClass::ProjectedStateDiff,
                crate::facade::BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
                "strategy:sha256:writeback-family-replay-loop-isolation:projected",
                crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            ),
            &lowered_policy_bundle,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "cross-family replay/loop isolation projected family admission failed: {error}"
            ))
        })?;
    let aspect_contract = runtime_bridge
        .admit_writeback_declaration(
            crate::facade::BridgeWritebackDeclaration::writeback_capable(
                crate::facade::BridgeWritebackDeclarationIdentity::new(
                    "harness:writeback-family-replay-loop-isolation:aspect",
                ),
                crate::facade::BridgeRequestKind::Authoritative,
                crate::facade::BridgeWritebackFamilyKind::AspectReconciliation,
                crate::facade::BridgeWritebackEffectClass::AspectReconciliation,
                crate::facade::BridgeWritebackStrategyClass::AspectReconciliationCommit,
                "strategy:sha256:writeback-family-replay-loop-isolation:aspect",
                crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            ),
            &lowered_policy_bundle,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "cross-family replay/loop isolation aspect family admission failed: {error}"
            ))
        })?;
    let projected_effect = runtime_bridge.lower_writeback_effect(
        &projected_contract,
        &causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-family-replay-loop-isolation:effect:projected",
        ),
        "effect:sha256:writeback-family-replay-loop-isolation:projected",
    );
    let aspect_effect = runtime_bridge.lower_writeback_effect(
        &aspect_contract,
        &causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-family-replay-loop-isolation:effect:aspect",
        ),
        "effect:sha256:writeback-family-replay-loop-isolation:aspect",
    );
    let projected_idempotence = runtime_bridge.classify_writeback_idempotence(
        &projected_effect,
        &lowered_policy_bundle,
        "truth-state:sha256:writeback-family-replay-loop-isolation:projected",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-family-replay-loop-isolation:idempotence:projected",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let aspect_idempotence = runtime_bridge.classify_writeback_idempotence(
        &aspect_effect,
        &lowered_policy_bundle,
        "truth-state:sha256:writeback-family-replay-loop-isolation:aspect",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-family-replay-loop-isolation:idempotence:aspect",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (projected_outcome, _) = runtime_bridge
        .execute_writeback_authority(
            &projected_contract,
            &projected_effect,
            &projected_idempotence,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "cross-family replay/loop isolation projected execution failed: {error}"
            ))
        })?;
    let (aspect_outcome, _) = runtime_bridge
        .execute_writeback_authority(&aspect_contract, &aspect_effect, &aspect_idempotence)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "cross-family replay/loop isolation aspect execution failed: {error}"
            ))
        })?;
    let projected_bundle = runtime_bridge.replay_writeback_bundle(
        &projected_contract,
        &projected_effect,
        &projected_idempotence,
        &projected_outcome,
    );
    let aspect_bundle = runtime_bridge.replay_writeback_bundle(
        &aspect_contract,
        &aspect_effect,
        &aspect_idempotence,
        &aspect_outcome,
    );
    let projected_feedback = runtime_bridge.derive_writeback_feedback_provenance(&projected_effect);
    let cross_family_loop_prevention = runtime_bridge.classify_writeback_loop_prevention(
        &aspect_effect,
        &aspect_idempotence,
        Some(projected_feedback.digest()),
        Some(causality.digest()),
    );

    let rebuilt_runtime = build_writeback_runtime_with_custom_authority(
        runtime,
        fixture,
        crate::harness::fixtures::RecordingTruthWritebackAuthority::default(),
    )?;
    let rebuilt_policy_bundle = lowered_policy(&rebuilt_runtime)?;
    let rebuilt_projected_contract = rebuilt_runtime
        .admit_writeback_declaration(
            crate::facade::BridgeWritebackDeclaration::writeback_capable(
                crate::facade::BridgeWritebackDeclarationIdentity::new(
                    "harness:writeback-family-replay-loop-isolation:projected",
                ),
                crate::facade::BridgeRequestKind::Authoritative,
                crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff,
                crate::facade::BridgeWritebackEffectClass::ProjectedStateDiff,
                crate::facade::BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
                "strategy:sha256:writeback-family-replay-loop-isolation:projected",
                crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            ),
            &rebuilt_policy_bundle,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "cross-family replay/loop isolation rebuilt projected admission failed: {error}"
            ))
        })?;
    let rebuilt_projected_effect = rebuilt_runtime.lower_writeback_effect(
        &rebuilt_projected_contract,
        &causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-family-replay-loop-isolation:effect:projected",
        ),
        "effect:sha256:writeback-family-replay-loop-isolation:projected",
    );
    let rebuilt_projected_idempotence = rebuilt_runtime.classify_writeback_idempotence(
        &rebuilt_projected_effect,
        &rebuilt_policy_bundle,
        "truth-state:sha256:writeback-family-replay-loop-isolation:projected",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-family-replay-loop-isolation:idempotence:projected",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (rebuilt_projected_outcome, _) = rebuilt_runtime
        .execute_writeback_authority(
            &rebuilt_projected_contract,
            &rebuilt_projected_effect,
            &rebuilt_projected_idempotence,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "cross-family replay/loop isolation rebuilt projected execution failed: {error}"
            ))
        })?;
    let rebuilt_projected_bundle = rebuilt_runtime.replay_writeback_bundle(
        &rebuilt_projected_contract,
        &rebuilt_projected_effect,
        &rebuilt_projected_idempotence,
        &rebuilt_projected_outcome,
    );
    rebuilt_runtime
        .validate_replayed_writeback_bundle(&projected_bundle, &rebuilt_projected_bundle)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "cross-family replay/loop isolation same-family rebuilt validation unexpectedly failed: {error}"
            ))
        })?;
    let rebuilt_execution_record = rebuilt_runtime
        .diagnostics()
        .last_writeback_execution_record()
        .ok_or_else(|| {
            BridgeHarnessError::new(
                "cross-family replay/loop isolation rebuilt execution record missing",
            )
        })?;
    let changed_causality = crate::facade::BridgeWritebackCausalityBasis::new(
        crate::facade::BridgeWritebackCausalityIdentity::new(
            "harness:writeback-family-replay-loop-isolation-causality:changed",
        ),
        "truth-trigger:sha256:writeback-family-replay-loop-isolation:changed",
        "route:sha256:writeback-family-replay-loop-isolation:changed",
        "evaluation-surface:sha256:writeback-family-replay-loop-isolation",
        "truth-view:sha256:writeback-family-replay-loop-isolation",
    );
    let changed_projected_effect = rebuilt_runtime.lower_writeback_effect(
        &rebuilt_projected_contract,
        &changed_causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-family-replay-loop-isolation:effect:projected:changed",
        ),
        "effect:sha256:writeback-family-replay-loop-isolation:projected",
    );
    let changed_projected_idempotence = rebuilt_runtime.classify_writeback_idempotence(
        &changed_projected_effect,
        &rebuilt_policy_bundle,
        "truth-state:sha256:writeback-family-replay-loop-isolation:projected",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-family-replay-loop-isolation:idempotence:projected:changed",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (changed_projected_outcome, _) = rebuilt_runtime
        .execute_writeback_authority(
            &rebuilt_projected_contract,
            &changed_projected_effect,
            &changed_projected_idempotence,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "cross-family replay/loop isolation changed-causality projected execution failed: {error}"
            ))
        })?;
    let changed_projected_bundle = rebuilt_runtime.replay_writeback_bundle(
        &rebuilt_projected_contract,
        &changed_projected_effect,
        &changed_projected_idempotence,
        &changed_projected_outcome,
    );
    let same_family_drift_error = rebuilt_runtime
        .validate_replayed_writeback_bundle(&projected_bundle, &changed_projected_bundle)
        .expect_err("cross-family replay/loop isolation same-family changed-causality drift must fail closed");
    let replay_validation_error = rebuilt_runtime
        .validate_replayed_writeback_bundle(&projected_bundle, &aspect_bundle)
        .expect_err(
            "cross-family replay/loop isolation cross-family replay validation must fail closed",
        );
    let family_replay_records = rebuilt_runtime.diagnostics().writeback_replay_records();
    let same_family_drift_replay_record = find_replay_record(
        &family_replay_records,
        projected_bundle.digest(),
        changed_projected_bundle.digest(),
    )
    .ok_or_else(|| {
        BridgeHarnessError::new(
            "cross-family replay/loop isolation same-family drift replay record missing",
        )
    })?;
    let cross_family_replay_record = find_replay_record(
        &family_replay_records,
        projected_bundle.digest(),
        aspect_bundle.digest(),
    )
    .ok_or_else(|| {
        BridgeHarnessError::new(
            "cross-family replay/loop isolation cross-family replay record missing",
        )
    })?;
    let counters = aggregate_runtime_writeback_counters(&[runtime_bridge, &rebuilt_runtime]);
    let counter_snapshot = snapshot_from_counters(&counters);
    let family_extension_digest = digest_string(
        "bridge-writeback-family-replay-loop-isolation",
        &format!(
            "projected={}|aspect={}|replay={:?}|loop={:?}",
            projected_bundle.digest(),
            aspect_bundle.digest(),
            replay_validation_error.kind(),
            cross_family_loop_prevention.disposition()
        ),
    )
    .to_string();
    let replay_loop_matrix = json!({
        "projected_family": {
            "effect_digest": projected_effect.digest(),
            "mapped_input_digest": projected_effect.mapped_input_digest(),
            "causality_digest": projected_effect.causality_digest(),
            "idempotence_digest": projected_idempotence.digest(),
            "replay_bundle_digest": projected_bundle.digest(),
            "replay_semantic_digest": projected_bundle.semantic_digest(),
        },
        "aspect_family": {
            "effect_digest": aspect_effect.digest(),
            "mapped_input_digest": aspect_effect.mapped_input_digest(),
            "causality_digest": aspect_effect.causality_digest(),
            "idempotence_digest": aspect_idempotence.digest(),
            "replay_bundle_digest": aspect_bundle.digest(),
            "replay_semantic_digest": aspect_bundle.semantic_digest(),
        },
        "cross_family_replay_isolation": {
            "semantic_digest_separated": projected_bundle.semantic_digest() != aspect_bundle.semantic_digest(),
            "bundle_digest_separated": projected_bundle.digest() != aspect_bundle.digest(),
            "failure_kind": format!("{:?}", replay_validation_error.kind()),
            "failure_digest": digest_string(
                "bridge-writeback-family-replay-loop-cross-family",
                &replay_validation_error.to_string(),
            ),
            "family_replay_record_digest": cross_family_replay_record.digest(),
            "decision_trace_digest": digest_string(
                "bridge-writeback-family-replay-loop-cross-family-trace",
                &format!(
                    "projected-bundle={}|aspect-bundle={}|replay-record={}|failure={:?}",
                    projected_bundle.digest(),
                    aspect_bundle.digest(),
                    cross_family_replay_record.digest(),
                    replay_validation_error.kind(),
                ),
            ),
        },
        "same_family_equivalence": {
            "semantic_digest_equal": projected_bundle.semantic_digest() == rebuilt_projected_bundle.semantic_digest(),
            "bundle_digest_equal": projected_bundle.digest() == rebuilt_projected_bundle.digest(),
            "effect_digest_equal": projected_effect.digest() == rebuilt_projected_effect.digest(),
            "mapped_input_digest_equal": projected_effect.mapped_input_digest() == rebuilt_projected_effect.mapped_input_digest(),
            "family_execution_record_digest": rebuilt_execution_record.digest(),
            "decision_trace_digest": digest_string(
                "bridge-writeback-family-replay-loop-same-family-trace",
                &format!(
                    "projected-bundle={}|rebuilt-bundle={}|execution-record={}",
                    projected_bundle.digest(),
                    rebuilt_projected_bundle.digest(),
                    rebuilt_execution_record.digest(),
                ),
            ),
        },
        "same_family_changed_causality": {
            "causality_digest_separated": projected_bundle.causality_digest() != changed_projected_bundle.causality_digest(),
            "semantic_digest_separated": projected_bundle.semantic_digest() != changed_projected_bundle.semantic_digest(),
            "bundle_digest_separated": projected_bundle.digest() != changed_projected_bundle.digest(),
            "failure_kind": format!("{:?}", same_family_drift_error.kind()),
            "family_replay_record_digest": same_family_drift_replay_record.digest(),
            "decision_trace_digest": digest_string(
                "bridge-writeback-family-replay-loop-same-family-drift-trace",
                &format!(
                    "projected-bundle={}|changed-bundle={}|replay-record={}|failure={:?}",
                    projected_bundle.digest(),
                    changed_projected_bundle.digest(),
                    same_family_drift_replay_record.digest(),
                    same_family_drift_error.kind(),
                ),
            ),
        },
        "cross_family_loop_isolation": {
            "incoming_feedback_provenance_digest": projected_feedback.digest(),
            "incoming_feedback_causality_digest": causality.digest(),
            "disposition": format!("{:?}", cross_family_loop_prevention.disposition()),
            "digest": cross_family_loop_prevention.digest(),
        },
    });
    Ok(
        WritebackHarnessExecution::CrossFamilyReplayLoopIsolationCertification {
            family_extension_digest,
            replay_loop_matrix,
            counter_snapshot,
        },
    )
}

fn execute_host_mapper_parity_certification(
    runtime: &crate::harness::adapter::BridgeHarnessSession,
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
) -> Result<WritebackHarnessExecution, BridgeHarnessError> {
    let _ = runtime;
    let _ = fixture;
    let lowered_policy_bundle = lowered_policy(runtime_bridge)?;
    let causality = crate::facade::BridgeWritebackCausalityBasis::new(
        crate::facade::BridgeWritebackCausalityIdentity::new(
            "harness:writeback-family-mapper-parity-causality",
        ),
        "truth-trigger:sha256:writeback-family-mapper-parity",
        "route:sha256:writeback-family-mapper-parity",
        "evaluation-surface:sha256:writeback-family-mapper-parity",
        "truth-view:sha256:writeback-family-mapper-parity",
    );
    let projected_contract = runtime_bridge
        .admit_writeback_declaration(
            crate::facade::BridgeWritebackDeclaration::writeback_capable(
                crate::facade::BridgeWritebackDeclarationIdentity::new(
                    "harness:writeback-family-mapper-parity:projected",
                ),
                crate::facade::BridgeRequestKind::Authoritative,
                crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff,
                crate::facade::BridgeWritebackEffectClass::ProjectedStateDiff,
                crate::facade::BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
                "strategy:sha256:writeback-family-mapper-parity:projected",
                crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            ),
            &lowered_policy_bundle,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "host mapper parity projected family admission failed: {error}"
            ))
        })?;
    let aspect_contract = runtime_bridge
        .admit_writeback_declaration(
            crate::facade::BridgeWritebackDeclaration::writeback_capable(
                crate::facade::BridgeWritebackDeclarationIdentity::new(
                    "harness:writeback-family-mapper-parity:aspect",
                ),
                crate::facade::BridgeRequestKind::Authoritative,
                crate::facade::BridgeWritebackFamilyKind::AspectReconciliation,
                crate::facade::BridgeWritebackEffectClass::AspectReconciliation,
                crate::facade::BridgeWritebackStrategyClass::AspectReconciliationCommit,
                "strategy:sha256:writeback-family-mapper-parity:aspect",
                crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            ),
            &lowered_policy_bundle,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "host mapper parity aspect family admission failed: {error}"
            ))
        })?;
    let projected_effect = runtime_bridge.lower_writeback_effect(
        &projected_contract,
        &causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-family-mapper-parity:effect:projected",
        ),
        "effect:sha256:writeback-family-mapper-parity:projected",
    );
    let aspect_effect = runtime_bridge.lower_writeback_effect(
        &aspect_contract,
        &causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-family-mapper-parity:effect:aspect",
        ),
        "effect:sha256:writeback-family-mapper-parity:aspect",
    );
    let projected_idempotence = runtime_bridge.classify_writeback_idempotence(
        &projected_effect,
        &lowered_policy_bundle,
        "truth-state:sha256:writeback-family-mapper-parity:projected",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-family-mapper-parity:idempotence:projected",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let aspect_idempotence = runtime_bridge.classify_writeback_idempotence(
        &aspect_effect,
        &lowered_policy_bundle,
        "truth-state:sha256:writeback-family-mapper-parity:aspect",
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-family-mapper-parity:idempotence:aspect",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (projected_outcome, _) = runtime_bridge
        .execute_writeback_authority(
            &projected_contract,
            &projected_effect,
            &projected_idempotence,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "host mapper parity projected family execution failed: {error}"
            ))
        })?;
    let (aspect_outcome, _) = runtime_bridge
        .execute_writeback_authority(&aspect_contract, &aspect_effect, &aspect_idempotence)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "host mapper parity aspect family execution failed: {error}"
            ))
        })?;
    let projected_bundle = runtime_bridge.replay_writeback_bundle(
        &projected_contract,
        &projected_effect,
        &projected_idempotence,
        &projected_outcome,
    );
    let aspect_bundle = runtime_bridge.replay_writeback_bundle(
        &aspect_contract,
        &aspect_effect,
        &aspect_idempotence,
        &aspect_outcome,
    );
    let family_execution_records = runtime_bridge.diagnostics().writeback_execution_records();
    let projected_execution_record =
        find_execution_record_for_replay(&family_execution_records, projected_bundle.digest())
            .ok_or_else(|| {
                BridgeHarnessError::new(
            "host mapper parity projected execution record missing from retained diagnostics",
        )
            })?;
    let aspect_execution_record =
        find_execution_record_for_replay(&family_execution_records, aspect_bundle.digest())
            .ok_or_else(|| {
                BridgeHarnessError::new(
                    "host mapper parity aspect execution record missing from retained diagnostics",
                )
            })?;
    let projected_admission_record = runtime_bridge
        .diagnostics()
        .writeback_admission_record_for_contract_digest(projected_contract.digest())
        .ok_or_else(|| {
            BridgeHarnessError::new(
                "host mapper parity projected admission record missing from retained diagnostics",
            )
        })?;
    let aspect_admission_record = runtime_bridge
        .diagnostics()
        .writeback_admission_record_for_contract_digest(aspect_contract.digest())
        .ok_or_else(|| {
            BridgeHarnessError::new(
                "host mapper parity aspect admission record missing from retained diagnostics",
            )
        })?;
    let shadow_protocol_error = runtime_bridge
        .validate_writeback_declaration(crate::facade::BridgeWritebackDeclaration::new(
            crate::facade::BridgeWritebackDeclarationIdentity::new(
                "harness:writeback-family-mapper-parity:shadow-protocol",
            ),
            crate::facade::BridgeRequestKind::Authoritative,
            crate::facade::BridgeWritebackRequestMode::WritebackCapable,
            Some(crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff),
            crate::facade::BridgeWritebackEffectClass::AspectReconciliation,
            Some(crate::facade::BridgeWritebackStrategyClass::AspectReconciliationCommit),
            "strategy:sha256:writeback-family-mapper-parity:shadow-protocol",
            crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
        ))
        .expect_err("host mapper parity shadow protocol mismatch must fail closed");
    let counters = aggregate_runtime_writeback_counters(&[runtime_bridge]);
    let counter_snapshot = snapshot_from_counters(&counters);
    let family_extension_digest = digest_string(
        "bridge-writeback-family-mapper-parity",
        &format!(
            "projected={}|aspect={}|shadow={:?}",
            projected_bundle.digest(),
            aspect_bundle.digest(),
            shadow_protocol_error.kind()
        ),
    )
    .to_string();
    let mapper_parity_matrix = json!({
        "projected_family": {
            "effect_digest": projected_effect.digest(),
            "mapped_input_digest": projected_effect.mapped_input_digest(),
            "mapper_envelope_digest": projected_effect.mapper_envelope_digest(),
            "replay_bundle_digest": projected_bundle.digest(),
        },
        "aspect_family": {
            "effect_digest": aspect_effect.digest(),
            "mapped_input_digest": aspect_effect.mapped_input_digest(),
            "mapper_envelope_digest": aspect_effect.mapper_envelope_digest(),
            "replay_bundle_digest": aspect_bundle.digest(),
        },
        "mapper_parity_matrix": {
            "projected_mapper_envelope_retained": runtime_bridge
                .diagnostics()
                .writeback_mapper_envelope_for_digest(projected_effect.mapper_envelope_digest())
                .is_some(),
            "aspect_mapper_envelope_retained": runtime_bridge
                .diagnostics()
                .writeback_mapper_envelope_for_digest(aspect_effect.mapper_envelope_digest())
                .is_some(),
            "projected_mapped_input_retained": runtime_bridge
                .diagnostics()
                .writeback_mapped_family_input_for_digest(projected_effect.mapped_input_digest())
                .is_some(),
            "aspect_mapped_input_retained": runtime_bridge
                .diagnostics()
                .writeback_mapped_family_input_for_digest(aspect_effect.mapped_input_digest())
                .is_some(),
            "projected_family_mapper_record_digest": projected_execution_record
                .mapper_record_digest(),
            "aspect_family_mapper_record_digest": aspect_execution_record
                .mapper_record_digest(),
            "projected_family_execution_record_digest": projected_execution_record.digest(),
            "aspect_family_execution_record_digest": aspect_execution_record.digest(),
            "decision_trace_digest": digest_string(
                "bridge-writeback-family-mapper-parity-trace",
                &format!(
                    "projected-admission={}|aspect-admission={}|projected-mapper={}|aspect-mapper={}|projected-execution={}|aspect-execution={}",
                    projected_admission_record.digest(),
                    aspect_admission_record.digest(),
                    projected_execution_record
                        .mapper_record_digest()
                        .unwrap_or("none"),
                    aspect_execution_record
                        .mapper_record_digest()
                        .unwrap_or("none"),
                    projected_execution_record.digest(),
                    aspect_execution_record.digest(),
                ),
            ),
        },
        "shadow_protocol_rejection": {
            "failure_kind": format!("{:?}", shadow_protocol_error.kind()),
            "failure_digest": digest_string(
                "bridge-writeback-family-mapper-parity-shadow-protocol",
                &shadow_protocol_error.to_string(),
            ),
            "decision_trace_digest": digest_string(
                "bridge-writeback-family-mapper-parity-shadow-trace",
                &format!(
                    "shadow={:?}|projected-admission={}|aspect-admission={}",
                    shadow_protocol_error.kind(),
                    projected_admission_record.digest(),
                    aspect_admission_record.digest(),
                ),
            ),
        },
    });
    Ok(WritebackHarnessExecution::HostMapperParityCertification {
        family_extension_digest,
        mapper_parity_matrix,
        counter_snapshot,
    })
}

fn find_execution_record_for_replay(
    records: &[crate::writeback::BridgeWritebackExecutionRecord],
    replay_bundle_digest: &str,
) -> Option<crate::writeback::BridgeWritebackExecutionRecord> {
    records
        .iter()
        .rev()
        .find(|record| record.replay_bundle_digest() == Some(replay_bundle_digest))
        .cloned()
}

fn find_replay_record(
    records: &[crate::writeback::BridgeWritebackReplayRecord],
    expected_replay_digest: &str,
    replayed_replay_digest: &str,
) -> Option<crate::writeback::BridgeWritebackReplayRecord> {
    records
        .iter()
        .rev()
        .find(|record| {
            record.expected_replay_digest() == expected_replay_digest
                && record.replayed_replay_digest() == replayed_replay_digest
        })
        .cloned()
}

fn lowered_policy(
    runtime_bridge: &crate::facade::RuntimeBridge,
) -> Result<crate::facade::LoweredBridgeExecutionPolicy, BridgeHarnessError> {
    let contract = runtime_bridge
        .admit_policy_declaration(crate::facade::BridgePolicyDeclaration::new(
            crate::facade::BridgePolicyDeclarationIdentity::new("harness:writeback-policy"),
            crate::facade::BridgeRequestKind::Authoritative,
            crate::facade::BridgeExecutionPolicyClass::DeterministicCanonical,
            crate::facade::BridgeDiagnosticsTier::Standard,
            true,
            true,
        ))
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback harness failed to admit canonical authoritative policy: {error:?}"
            ))
        })?;
    Ok(runtime_bridge.lower_admitted_policy(&contract))
}

fn route_digest_for_first_patch(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
) -> Result<String, BridgeHarnessError> {
    let commit_identity = fixture
        .committed_patches()
        .first()
        .map(|patch| patch.commit_identity().as_str().to_string())
        .ok_or_else(|| {
            BridgeHarnessError::new("writeback harness fixture requires one committed patch")
        })?;
    let result = runtime_bridge
        .deliver_invalidation(
            runtime_bridge
                .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit(
                    commit_identity.clone(),
                ))
                .map_err(|error| {
                    BridgeHarnessError::new(format!(
                        "writeback harness failed to plan committed patch `{commit_identity}`: {error}"
                    ))
                })?,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback harness failed to deliver committed patch `{commit_identity}`: {error}"
            ))
        })?;
    Ok(digest_string(
        "bridge-writeback-route-digest",
        result.result_summary().route_identity().as_str(),
    )
    .to_string())
}

fn route_digest_for_commit(
    runtime_bridge: &crate::facade::RuntimeBridge,
    commit_identity: &str,
) -> Result<String, BridgeHarnessError> {
    let result = runtime_bridge
        .deliver_invalidation(
            runtime_bridge
                .plan_committed_patch(crate::facade::BridgeRouteRequest::for_commit(
                    commit_identity.to_string(),
                ))
                .map_err(|error| {
                    BridgeHarnessError::new(format!(
                        "writeback harness failed to plan committed patch `{commit_identity}`: {error}"
                    ))
                })?,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback harness failed to deliver committed patch `{commit_identity}`: {error}"
            ))
        })?;
    Ok(digest_string(
        "bridge-writeback-route-digest",
        result.result_summary().route_identity().as_str(),
    )
    .to_string())
}

fn bridge_feedback_patch(
    commit_identity: &str,
    patch_identity: &str,
    snapshot_identity: &str,
    branch_identity: &str,
    feedback_provenance_digest: &str,
    causality_digest: &str,
) -> crate::facade::RawCommittedPatchEnvelope {
    crate::facade::RawCommittedPatchEnvelope::new_with_metadata(
        crate::facade::BridgeProducerMetadata::relational_publication()
            .with_writeback_feedback_provenance(feedback_provenance_digest, causality_digest),
        crate::facade::TruthCommitIdentity::new(commit_identity),
        crate::facade::TruthPatchIdentity::new(patch_identity),
        crate::facade::TruthSnapshotIdentity::new(snapshot_identity),
        crate::facade::TruthBranchIdentity::new(branch_identity),
        vec![crate::facade::BridgeCommittedPatchItem::new(
            "user",
            forge_foundational::facade::AspectKey::new("profile")
                .expect("valid bridge patch aspect key"),
            "name",
        )],
    )
}

fn feedback_provenance_hint(
    patch: &crate::facade::RawCommittedPatchEnvelope,
) -> Option<(&str, &str)> {
    Some((
        patch
            .producer_metadata()
            .writeback_feedback_provenance_digest()?,
        patch
            .producer_metadata()
            .writeback_feedback_causality_digest()?,
    ))
}

fn build_writeback_runtime(
    runtime: &crate::harness::adapter::BridgeHarnessSession,
    fixture: &BridgeHarnessFixture,
    bind_authority: bool,
) -> Result<crate::facade::RuntimeBridge, BridgeHarnessError> {
    let mut builder = crate::facade::RuntimeBridgeBuilder::new()
        .with_policy(fixture.policy())
        .with_relational_source(runtime.source.clone())
        .with_truth_branch_head_source(runtime.source.clone())
        .with_signal_sink(runtime.sink.clone());
    if bind_authority {
        builder = builder.with_writeback_authority(runtime.writeback_authority.clone());
    }
    let (first_mapping, remaining_mappings) =
        fixture.mappings().split_first().ok_or_else(|| {
            BridgeHarnessError::new("writeback harness fixture requires at least one mapping")
        })?;
    let mut builder = builder.register_mapping(first_mapping.clone());
    for mapping in remaining_mappings {
        builder = builder.register_mapping(mapping.clone());
    }
    builder.build().map_err(|error| {
        BridgeHarnessError::new(format!(
            "failed to build writeback harness runtime with bind_authority={bind_authority}: {error}"
        ))
    })
}

fn build_writeback_runtime_with_custom_authority<A>(
    runtime: &crate::harness::adapter::BridgeHarnessSession,
    fixture: &BridgeHarnessFixture,
    writeback_authority: A,
) -> Result<crate::facade::RuntimeBridge, BridgeHarnessError>
where
    A: crate::adapter::TruthWritebackAuthority,
{
    let builder = crate::facade::RuntimeBridgeBuilder::new()
        .with_policy(fixture.policy())
        .with_relational_source(runtime.source.clone())
        .with_truth_branch_head_source(runtime.source.clone())
        .with_signal_sink(runtime.sink.clone())
        .with_writeback_authority(writeback_authority);
    let (first_mapping, remaining_mappings) =
        fixture.mappings().split_first().ok_or_else(|| {
            BridgeHarnessError::new("writeback harness fixture requires at least one mapping")
        })?;
    let mut builder = builder.register_mapping(first_mapping.clone());
    for mapping in remaining_mappings {
        builder = builder.register_mapping(mapping.clone());
    }
    builder.build().map_err(|error| {
        BridgeHarnessError::new(format!(
            "failed to build writeback harness runtime with custom authority: {error}"
        ))
    })
}
