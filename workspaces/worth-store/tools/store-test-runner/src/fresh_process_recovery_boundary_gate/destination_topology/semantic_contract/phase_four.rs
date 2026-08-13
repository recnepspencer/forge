pub(super) fn phase_four_responsibility(path: &str) -> Option<&'static str> {
    match path {
        "crates/worth-store-recovery-runtime/src/orchestration/planning/context.rs" => {
            Some("linear-selected-planning-context")
        }
        "crates/worth-store-recovery-runtime/src/orchestration/planning/admitted_basis.rs" => {
            Some("effect-free-planning-basis-admission")
        }
        "crates/worth-store-recovery-runtime/src/orchestration/planning/resolved_basis.rs" => {
            Some("bounded-observation-and-redo-resolution")
        }
        "crates/worth-store-recovery-runtime/src/orchestration/planning/completion.rs" => {
            Some("immutable-plan-cost-and-handoff-completion")
        }
        "crates/worth-store-recovery-runtime/src/orchestration/planning/counters.rs" => {
            Some("progressive-planning-stage-counters")
        }
        "crates/worth-store-recovery-runtime/src/progression/planned/basis.rs" => {
            Some("immutable-staging-publication-quiescence-basis")
        }
        "crates/worth-store-recovery-runtime/src/progression/planned/basis/derivation.rs" => {
            Some("preflighted-execution-basis-derivation")
        }
        "crates/worth-store-recovery-runtime/src/progression/planned/basis/derivation/pending.rs" => {
            Some("admitted-pending-projection-basis")
        }
        "crates/worth-store-recovery-runtime/src/progression/planned/basis/derivation/materialization.rs" => {
            Some("projected-materialization-aggregation")
        }
        "crates/worth-store-recovery-runtime/src/progression/planned/basis/derivation/actions.rs" => {
            Some("exact-staging-action-derivation")
        }
        "crates/worth-store-recovery-runtime/src/progression/planned/basis/derivation/layout.rs" => {
            Some("immutable-staging-layout-assembly")
        }
        "crates/worth-store-recovery-runtime/src/progression/planned/basis/derivation/closeout.rs" => {
            Some("publication-quiescence-and-identity-closeout")
        }
        "crates/worth-store-recovery-runtime/src/progression/planned/basis/staging_cost.rs" => {
            Some("preallocation-staging-cost-admission")
        }
        "crates/worth-store-recovery-runtime/src/progression/planned/basis/identity.rs" => {
            Some("complete-plan-causal-identity")
        }
        "crates/worth-store-recovery-runtime/src/orchestration/planning/page_observation/failure.rs" => {
            Some("typed-page-media-denial")
        }
        "crates/worth-store-recovery-runtime/src/orchestration/planning/page_observation/allocation_truth.rs" => {
            Some("checksum-bound-free-space-frontier-and-capacity-admission")
        }
        "crates/worth-store-recovery-runtime/src/orchestration/planning/page_observation/selected_basis.rs" => {
            Some("manifest-authoritative-prior-observation")
        }
        "crates/worth-store-recovery-runtime/src/orchestration/planning/page_observation/materialized.rs" => {
            Some("manifest-addressed-inline-and-extent-page-observation")
        }
        "crates/worth-store-recovery-runtime/src/orchestration/planning/selected_source_inventory.rs" => {
            Some("bounded-selected-source-inventory")
        }
        "crates/worth-store-recovery-physics/src/redo_replay/record/target_decode.rs" => {
            Some("bounded-canonical-redo-target-decoder")
        }
        "crates/worth-store-recovery-physics/src/redo_replay/plan/accessors.rs" => {
            Some("sealed-redo-plan-construction-and-views")
        }
        "crates/worth-store-recovery-physics/src/redo_replay/plan/admission.rs" => {
            Some("grouped-projection-admission")
        }
        "crates/worth-store-recovery-physics/src/redo_replay/plan/group_admission.rs" => {
            Some("pre-observation-group-structural-admission")
        }
        "crates/worth-store-recovery-physics/src/redo_replay/plan/projection_materialization.rs" => {
            Some("projected-record-byte-closure")
        }
        "crates/worth-store-recovery-physics/src/redo_replay/plan/projection_validation.rs" => {
            Some("projection-routing-and-frame-closure")
        }
        "crates/worth-store-physical-format/src/manifest/durable_segment_routing/codec_primitives.rs" => {
            Some("segment-membership-codec-primitives")
        }
        "crates/worth-store-physical-format/src/recovery_projection.rs" => {
            Some("canonical-recovery-projection-v3")
        }
        "crates/worth-store-physical-format/src/recovery_projection/codec.rs" => {
            Some("bounded-recovery-projection-codec")
        }
        "crates/worth-store-physical-format/src/recovery_projection/codec/cursor.rs" => {
            Some("preallocation-projection-decode-cursor")
        }
        "crates/worth-store-physical-format/src/recovery_projection/root_state.rs" => {
            Some("complete-recovered-root-state")
        }
        "crates/worth-store/src/physical_runtime/recovery_freshness/binding/accessors.rs" => {
            Some("read-only-sampled-binding-evidence")
        }
        _ => None,
    }
}
