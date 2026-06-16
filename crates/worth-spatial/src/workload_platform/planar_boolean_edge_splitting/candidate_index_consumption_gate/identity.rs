use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::gate::PlanarBooleanCandidateIndexConsumptionGate;

pub(crate) fn consumption_gate_identity(
    gate: &PlanarBooleanCandidateIndexConsumptionGate,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-candidate-index-consumption-gate".to_string(),
            format!("event-ledger:{}", gate.event_ledger_identity()),
            format!(
                "downstream-consumption:{}",
                gate.downstream_consumption_identity()
            ),
            format!("reduced-pair:{}", gate.reduced_pair_identity()),
            format!(
                "segment-pair-enumeration:{}",
                gate.segment_pair_enumeration_identity()
            ),
            format!(
                "candidate-index-product:{}",
                gate.candidate_index_product_identity()
            ),
            format!(
                "query-declaration:{}",
                gate.query_index_declaration_digest()
            ),
            format!("query-plan:{}", gate.query_index_plan_digest()),
            format!("query-envelope:{}", gate.query_index_envelope_digest()),
            format!(
                "candidate-index-strategy:{}",
                candidate_index_strategy_key(gate)
            ),
            format!("fallback-posture:{}", fallback_posture_key(gate)),
            format!("lifecycle-outcome:{}", lifecycle_outcome_key(gate)),
            format!(
                "expected-pair-breadth:{}",
                gate.counters().expected_pair_breadth()
            ),
            format!(
                "indexed-candidates:{}",
                gate.counters().indexed_candidate_pair_count()
            ),
            format!("culled-pairs:{}", gate.counters().culled_pair_count()),
            format!("emitted-pairs:{}", gate.counters().emitted_pair_count()),
            format!("fallback-used:{}", gate.counters().fallback_used()),
        ],
    )
}

fn candidate_index_strategy_key(gate: &PlanarBooleanCandidateIndexConsumptionGate) -> &'static str {
    match gate.candidate_index_strategy() {
        crate::workload_platform::planar_boolean_events::PlanarBooleanCandidateIndexStrategy::AabbSweep => {
            "aabb-sweep-v1"
        }
    }
}

fn fallback_posture_key(gate: &PlanarBooleanCandidateIndexConsumptionGate) -> &'static str {
    match gate.fallback_posture() {
        crate::workload_platform::planar_boolean_events::PlanarBooleanCandidateIndexFallbackPosture::NotUsed => {
            "not-used"
        }
        crate::workload_platform::planar_boolean_events::PlanarBooleanCandidateIndexFallbackPosture::FullBreadthNonProduction => {
            "full-breadth-non-production"
        }
    }
}

fn lifecycle_outcome_key(gate: &PlanarBooleanCandidateIndexConsumptionGate) -> &'static str {
    match gate.lifecycle_outcome() {
        crate::workload_platform::planar_boolean_events::PlanarBooleanCandidateIndexLifecycleOutcome::Bound => {
            "bound"
        }
    }
}
