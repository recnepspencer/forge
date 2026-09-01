pub(super) enum OwnerKind {
    Function(&'static str),
    Method(&'static str),
}

impl OwnerKind {
    pub(super) fn name(&self) -> &'static str {
        match self {
            Self::Function(name) | Self::Method(name) => name,
        }
    }
}

pub(super) struct OwnerBody {
    pub(super) responsibility: &'static str,
    pub(super) source_path: &'static str,
    pub(super) source: &'static str,
    pub(super) kind: OwnerKind,
    pub(super) expected_digest: &'static str,
}

macro_rules! owner {
    ($responsibility:literal, $path:literal, $source:literal, $kind:ident, $name:literal, $digest:literal) => {
        OwnerBody {
            responsibility: $responsibility,
            source_path: $path,
            source: include_str!($source),
            kind: OwnerKind::$kind($name),
            expected_digest: $digest,
        }
    };
}

pub(super) const RUNTIME_OWNER_BODIES: &[OwnerBody] = &[
    owner!(
        "root invalidation admission",
        "logic/invalidation/routing.rs",
        "../../../../../../logic/invalidation/routing.rs",
        Function,
        "mark_dirty_batch",
        "f29afe1da2da0fd4d45a84b44ace8cddc7c97e4119a4174f4f7a805f82e72457"
    ),
    owner!(
        "source-only invalidation planning",
        "logic/invalidation/routing/planning.rs",
        "../../../../../../logic/invalidation/routing/planning.rs",
        Function,
        "plan_invalidation_frontier",
        "274f28490ff6c7a63e0d32c81404f1ca52383b6a1eca43e43196d314c80dd3da"
    ),
    owner!(
        "frontier execution diagnostics sidecar",
        "logic/invalidation/routing/application.rs",
        "../../../../../../logic/invalidation/routing/application.rs",
        Function,
        "execute_invalidation_frontier",
        "af590bcdf760cd202e428d6b32b450457f0099cdb9705fcc7ad6a229c5ca60ff"
    ),
    owner!(
        "producer-local reverse index query",
        "data/graph/topology/subscriber_index/buckets/query.rs",
        "../../../../../../data/graph/topology/subscriber_index/buckets/query.rs",
        Method,
        "query_scope",
        "69f86d3fb7825eecf0007af03599b91440a39b0096f37d5ccebfc401876aa281"
    ),
    owner!(
        "reverse index membership replacement",
        "data/graph/topology/subscriber_index/buckets.rs",
        "../../../../../../data/graph/topology/subscriber_index/buckets.rs",
        Method,
        "replace_consumer",
        "fe616dc3f20d08fba93429099f2fe455d44129d102658f51e2ed0065c8807e26"
    ),
    owner!(
        "reverse index authority rebuild",
        "data/graph/topology/subscriber_index/rebuild.rs",
        "../../../../../../data/graph/topology/subscriber_index/rebuild.rs",
        Method,
        "rebuild_reverse_subscription_index_from_dependencies",
        "b1dc1860b20f1ecee63dd176c63e445d29c2317ca6ef81d22ed59370d995cd82"
    ),
    owner!(
        "routing counter projection",
        "logic/invalidation/routing/counters.rs",
        "../../../../../../logic/invalidation/routing/counters.rs",
        Function,
        "record_diagnostic_projection",
        "659e54a587b6f50b395a2612defb073739e7776648256fc53b8218b496a4a0f6"
    ),
    owner!(
        "effect telemetry writers",
        "data/graph/runtime/effect/evidence.rs",
        "../../../../../../data/graph/runtime/effect/evidence.rs",
        Method,
        "record_effect_telemetry",
        "2bb879290c6608c49c296c0007afc882d2bc1d39e2c36085f1e6a8c2a6e34219"
    ),
    owner!(
        "effect state publication",
        "data/graph/runtime/effect.rs",
        "../../../../../../data/graph/runtime/effect.rs",
        Method,
        "transition_effect_state",
        "0ea93e5c41ec04bfdcea6f11952d0d2bb6932d2afc1b2186181f5a37113faf8f"
    ),
    owner!(
        "effect node state mutation",
        "data/graph/runtime/effect.rs",
        "../../../../../../data/graph/runtime/effect.rs",
        Method,
        "apply_effect_node_state",
        "1bcecc59f3f3a384d1d0442bbd34c8c65120997f4b3e517587d7b91143492263"
    ),
    owner!(
        "producer lifecycle publication",
        "data/graph/runtime/effect.rs",
        "../../../../../../data/graph/runtime/effect.rs",
        Method,
        "apply_effect_node_lifecycle",
        "6d1d356e32947ca73eb2bfd566fc2447d171c1b36447855144d6f5ddde181b75"
    ),
    owner!(
        "effect snapshot publication",
        "data/graph/runtime/effect/application.rs",
        "../../../../../../data/graph/runtime/effect/application.rs",
        Method,
        "commit_effect_snapshot",
        "8917127d018103f6af225ff6e69a39089764ae44b303ac32cafa406091e29be5"
    ),
    owner!(
        "serial output promotion entry",
        "data/graph/runtime/effect/output_commit.rs",
        "../../../../../../data/graph/runtime/effect/output_commit.rs",
        Method,
        "apply_effect",
        "6d6b6177901e04dbc1816cdf092d37e31617b5124addce9a162b43186ba5ec17"
    ),
    owner!(
        "output packet preparation",
        "data/graph/runtime/effect/output_commit.rs",
        "../../../../../../data/graph/runtime/effect/output_commit.rs",
        Method,
        "prepare_output_commit_packet_with_probe",
        "891e1e5d55c6ec792308d093ad95bc629a3ef67db3ada669fc694e9ae4223679"
    ),
    owner!(
        "atomic output publication",
        "data/graph/runtime/effect/output_commit.rs",
        "../../../../../../data/graph/runtime/effect/output_commit.rs",
        Method,
        "publish_output_commit_packet",
        "25ba2961ff8959554669823a6ca6181b83552b2c9c3c69abd365193933902fe9"
    ),
    owner!(
        "parallel output promotion entry",
        "data/graph/runtime/effect/output_commit.rs",
        "../../../../../../data/graph/runtime/effect/output_commit.rs",
        Method,
        "publish_prepared_parallel_apply_commit_packet",
        "79631a5cf8b4fe17552ee6ff2d157be5765273a5682346c7d658714e37fe89da"
    ),
    owner!(
        "changed cause preparation",
        "logic/invalidation/causality/dependency_admission.rs",
        "../../../../../../logic/invalidation/causality/dependency_admission.rs",
        Method,
        "prepare_direct_output_causes",
        "0c62e42f5ab5bc2da463606b342bc5b468a0d96fe6cd5160bf6e318068ea3a0b"
    ),
    owner!(
        "stable predecessor resolution",
        "logic/invalidation/causality/dependency_admission.rs",
        "../../../../../../logic/invalidation/causality/dependency_admission.rs",
        Method,
        "prepare_stable_output_resolution",
        "99261c4037e31f92305f78801358020bf71d77cbb2fb889ac0ea136399ece4c1"
    ),
    owner!(
        "consumer cause admission",
        "logic/invalidation/causality/dependency_admission.rs",
        "../../../../../../logic/invalidation/causality/dependency_admission.rs",
        Method,
        "prepare_consumer_cause_set",
        "d4ca67f2b6a07139c90f86e621d150c55dbb12a305fd1756331fb10c71c2b4f2"
    ),
    owner!(
        "direct cause publication",
        "logic/invalidation/causality/dependency_admission.rs",
        "../../../../../../logic/invalidation/causality/dependency_admission.rs",
        Method,
        "publish_direct_output_causes",
        "0b08dfe9363963306ec3ff37f68a6bf35bc886bf66561f0f5cd0f9b994fc18f7"
    ),
    owner!(
        "edge cause reconciliation",
        "logic/invalidation/causality/cause_aggregation.rs",
        "../../../../../../logic/invalidation/causality/cause_aggregation.rs",
        Function,
        "reconcile_edge_cause",
        "8715860ecb6ff99f58078ed12466ba61264761c40c8d91b0657b6fb6e6fb9f61"
    ),
    owner!(
        "pending cause projection",
        "data/graph/storage/invalidation_causes/application.rs",
        "../../../../../../data/graph/storage/invalidation_causes/application.rs",
        Method,
        "replace_pending_causes",
        "229607787712eaf6eb02dfc2b63de44f8b87e2edbed5a758b97d83a5839dd0c4"
    ),
    owner!(
        "prepared pending cause publication",
        "data/graph/storage/invalidation_causes/application.rs",
        "../../../../../../data/graph/storage/invalidation_causes/application.rs",
        Method,
        "replace_prepared_pending_causes",
        "2ee6a5b604e23485a7ae0adbb3ae6f62f80956f6a5e5cdd29f179024de9e23a9"
    ),
    owner!(
        "derived dirty cache rebuild",
        "data/graph/storage/invalidation_causes/application.rs",
        "../../../../../../data/graph/storage/invalidation_causes/application.rs",
        Method,
        "rebuild_dirty_caches_from_pending_causes",
        "bc91961b3ac65df8fdfbfd73fc85dad242cc550c6d9938b809f55d78c3aa56ab"
    ),
    owner!(
        "output ordinal publication",
        "data/graph/storage/invalidation_causes/cause_sets.rs",
        "../../../../../../data/graph/storage/invalidation_causes/cause_sets.rs",
        Method,
        "publish_output_commit_ordinal",
        "d6b9468e264663aeab969c86e61d365efcc8e55559c5df1f24d76923f9f77c48"
    ),
    owner!(
        "output commit ledger publication",
        "data/graph/storage/invalidation_causes/cause_sets.rs",
        "../../../../../../data/graph/storage/invalidation_causes/cause_sets.rs",
        Method,
        "publish_output_commit",
        "60338be2dd18e21668b06eab033d38ea64e8069708bfa828e209e61465216b3c"
    ),
    owner!(
        "canonical cause insertion",
        "data/graph/storage/invalidation_causes/cause_sets.rs",
        "../../../../../../data/graph/storage/invalidation_causes/cause_sets.rs",
        Method,
        "insert",
        "fa6c0cc9cc34eb49ff62e862d5443dcab668ac3a7bea80823d3656a25c8e09a2"
    ),
    owner!(
        "canonical cause replacement",
        "data/graph/storage/invalidation_causes/cause_sets.rs",
        "../../../../../../data/graph/storage/invalidation_causes/cause_sets.rs",
        Method,
        "replace",
        "9271f2bd8302ca64626dbd08ac17978f454fbff69a942c788580f8c166c73ee6"
    ),
    owner!(
        "canonical cause slot write",
        "data/graph/storage/invalidation_causes/cause_sets.rs",
        "../../../../../../data/graph/storage/invalidation_causes/cause_sets.rs",
        Method,
        "replace_set",
        "5b0d8a672055d4c887086478a7930a77ccc19b4483436ad0bc5170b3f5faa7dc"
    ),
];
mod planner;

pub(super) fn planner_owner_bodies() -> &'static [OwnerBody] {
    planner::PLANNER_OWNER_BODIES
}
