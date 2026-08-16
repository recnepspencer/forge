use super::{OwnerBody, OwnerKind};

pub(super) const PLANNER_OWNER_BODIES: &[OwnerBody] = &[
    owner!(
        "evaluation plan construction",
        "logic/planner/planning/mod.rs",
        "../../../../../../../logic/planner/planning/mod.rs",
        Function,
        "build_evaluation_plan_with_policy_resolver",
        "13f6a0c373d9c88fa808c9d72b232e43031ae1c944ca5aaab5a9ad19b11fa784"
    ),
    owner!(
        "readiness prevalidation",
        "logic/planner/precompute/eligibility.rs",
        "../../../../../../../logic/planner/precompute/eligibility.rs",
        Function,
        "prevalidate_stage_tasks",
        "4c9b879401b35e067bf371f7258dbbc7fe775f0e8ad06ec26f4f3b9166d0b41c"
    ),
    owner!(
        "prepared plan execution",
        "logic/planner/execution/mod.rs",
        "../../../../../../../logic/planner/execution/mod.rs",
        Function,
        "execute_prepared_plan_with_policy_and_temporal_lowering",
        "2ee63a69b218191ef30e68fa2a38ac833d6c8bbf3bfbf68631e166b6a669984b"
    ),
    owner!(
        "stage execution",
        "logic/planner/execution/stage.rs",
        "../../../../../../../logic/planner/execution/stage.rs",
        Function,
        "execute_stage",
        "f239c714242d985089d9575b6ab495e5e767c8f10b5b12b0d43eef8c74cc6acd"
    ),
    owner!(
        "stage precompute",
        "logic/planner/precompute/stage.rs",
        "../../../../../../../logic/planner/precompute/stage.rs",
        Function,
        "perform_stage_precompute",
        "353a3689f70f37d99cf69a83846bd7821884c4679cbbc6ee507866e547184f95"
    ),
    owner!(
        "precompute dispatch",
        "logic/planner/precompute/dispatch.rs",
        "../../../../../../../logic/planner/precompute/dispatch.rs",
        Function,
        "dispatch_stage_precompute",
        "efb8a691a3c02b530fdebfeecf7b260399ef770e5f1fdedb55bc5fbe40f516d6"
    ),
    owner!(
        "stage apply",
        "logic/planner/apply/stage.rs",
        "../../../../../../../logic/planner/apply/stage.rs",
        Function,
        "apply_stage",
        "fd92262087a58a395894012d087b67a8218a4a06bb9064c8dc3f95040b2f85af"
    ),
    owner!(
        "lowered serial apply",
        "logic/planner/apply/stage.rs",
        "../../../../../../../logic/planner/apply/stage.rs",
        Function,
        "run_lowered_apply_pass",
        "a18ecefa9a52fc2ca7ad65285da8b7a187d1b0a37c9a971170baa7585d5279bf"
    ),
    owner!(
        "snapshot publication",
        "logic/planner/apply/stage.rs",
        "../../../../../../../logic/planner/apply/stage.rs",
        Function,
        "publish_pending_snapshots",
        "b2157a0391742d036519b677afdc700ccb330529f68e5fd5ba3f98934824ee96"
    ),
    owner!(
        "stage result finalization",
        "logic/planner/apply/stage.rs",
        "../../../../../../../logic/planner/apply/stage.rs",
        Function,
        "finalize_stage_results",
        "56898cb87702d0dd88652471ff7a868afe5f3d6155de888265c4bb71b268fe97"
    ),
    owner!(
        "grouped parallel apply",
        "logic/planner/apply/stage/concurrent.rs",
        "../../../../../../../logic/planner/apply/stage/concurrent.rs",
        Function,
        "run_grouped_concurrent_apply_pass",
        "db67e6aeed6340e5c6d54ce72b5fc7a391c148dafc9f4c1eda916c44cd132737"
    ),
    owner!(
        "parallel packet construction",
        "logic/planner/apply/stage/concurrent_packets.rs",
        "../../../../../../../logic/planner/apply/stage/concurrent_packets.rs",
        Function,
        "build_group_packet",
        "58c1c9f646482428e411e5a90c1937ede52be4613a69f3abdb69848dc28c1861"
    ),
    owner!(
        "parallel packet reduction",
        "logic/planner/apply/stage/concurrent_packets.rs",
        "../../../../../../../logic/planner/apply/stage/concurrent_packets.rs",
        Function,
        "reduce_grouped_concurrent_packets",
        "40109f5204bca3a7f4a8db58a658dde9194e072ce45517f60511679f57884e2d"
    ),
    owner!(
        "parallel task output publication",
        "logic/planner/apply/stage/concurrent_packets.rs",
        "../../../../../../../logic/planner/apply/stage/concurrent_packets.rs",
        Function,
        "publish_group_local_task_commit",
        "c65244660f95ffe3ef178d77ee327e28f827e891709f5b1c42964ba97e838216"
    ),
    owner!(
        "parallel input lowering",
        "logic/planner/apply/stage/concurrent_packets.rs",
        "../../../../../../../logic/planner/apply/stage/concurrent_packets.rs",
        Function,
        "build_concurrent_apply_group_inputs",
        "e23f99ccb12b97c94ee56374cbdcaf9310eea4b296c905bb779d81150a36dacc"
    ),
    owner!(
        "serial batch preparation",
        "logic/planner/apply/serial_batch/preparation.rs",
        "../../../../../../../logic/planner/apply/serial_batch/preparation.rs",
        Method,
        "prepare",
        "03bc435ba7013d23fd11c82ff8136005d0198818f5cf34ca6e741fbde2390d99"
    ),
    owner!(
        "serial batch application",
        "logic/planner/apply/serial_batch/application.rs",
        "../../../../../../../logic/planner/apply/serial_batch/application.rs",
        Method,
        "apply",
        "d8d80858e9549dd6976279cda10d6f493d585b28792325a62b5382d38e81ee7d"
    ),
    owner!(
        "serial task application",
        "logic/planner/apply/serial_batch/application.rs",
        "../../../../../../../logic/planner/apply/serial_batch/application.rs",
        Function,
        "apply_serial_input",
        "f4a85bf4c2f54ffa7526f10675f215706a694b9039e4cd1aacb870eaf81ac513"
    ),
    owner!(
        "serial snapshot reduction",
        "logic/planner/apply/serial_batch/finalization.rs",
        "../../../../../../../logic/planner/apply/serial_batch/finalization.rs",
        Method,
        "split_pending_snapshots",
        "a474159353f8e805b6b7f0e6c355cec7518f2c5df03115d435cac2a9b43a8bb3"
    ),
    owner!(
        "serial finalize promotion",
        "logic/planner/apply/serial_batch/finalization.rs",
        "../../../../../../../logic/planner/apply/serial_batch/finalization.rs",
        Method,
        "into_ready_for_finalize",
        "f5a3cbbbb20b2448426df53f0a5756385688cc85f2c8d8c0b3512ef1a793de7c"
    ),
    owner!(
        "prepared evaluation application",
        "logic/evaluation/engine/prepared_apply/evaluation.rs",
        "../../../../../../../logic/evaluation/engine/prepared_apply/evaluation.rs",
        Function,
        "apply_prepared_evaluation_after_dependencies_with_policy",
        "d7e41f1e9def137c2be04956159b5cbb5659c3f4d67dfc767470fe5cf1571706"
    ),
    owner!(
        "evaluation effect application",
        "logic/evaluation/engine/apply.rs",
        "../../../../../../../logic/evaluation/engine/apply.rs",
        Function,
        "apply_effect_with_policy_and_condition",
        "266f00f62f5161950484f83a9b34bc9192f1f20d6c88da12e9e8ba028da46dbf"
    ),
    owner!(
        "serial effect promotion",
        "logic/evaluation/engine/apply/mutation.rs",
        "../../../../../../../logic/evaluation/engine/apply/mutation.rs",
        Function,
        "apply_evaluation_effect",
        "5cceef06765142c0cce0305cc12aeb0d2e7649f9f40770d7a666ec56df0ab81b"
    ),
];
