use crate::identity::hash_parts;
use crate::policy_execution_seam::{
    PolicyAwareExecutionMode, PolicyAwareExecutionSeam, PolicyAwareSeamCounters,
};
use crate::policy_narrowing::NarrowedPolicyQueryArtifact;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwarePlanDigest(String);

impl PolicyAwarePlanDigest {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyAwarePlanCostPosture {
    RuntimeCurrentBounded,
    RuntimeBranchBounded,
    RuntimeHistoricalBounded,
    StoreHistoricalRetainedBounded,
    RuntimeDiffBounded,
    LiveSparseAuthorized,
    DeliveryWidthBounded,
    DeferredStoreBacked,
    DeniedWouldWiden,
}

impl PolicyAwarePlanCostPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RuntimeCurrentBounded => "runtime_current_bounded",
            Self::RuntimeBranchBounded => "runtime_branch_bounded",
            Self::RuntimeHistoricalBounded => "runtime_historical_bounded",
            Self::StoreHistoricalRetainedBounded => "store_historical_retained_bounded",
            Self::RuntimeDiffBounded => "runtime_diff_bounded",
            Self::LiveSparseAuthorized => "live_sparse_authorized",
            Self::DeliveryWidthBounded => "delivery_width_bounded",
            Self::DeferredStoreBacked => "deferred_store_backed",
            Self::DeniedWouldWiden => "denied_would_widen",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PolicyAwarePlanWorkBudget {
    authorized_field_width: usize,
    proof_descriptor_count: usize,
    proof_topology_width: usize,
    tenant_schema_basis_count: usize,
    delivery_field_width: usize,
    live_relevance_field_width: usize,
    allocation_scope_width: usize,
    digest_part_count: usize,
}

impl PolicyAwarePlanWorkBudget {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        authorized_field_width: usize,
        proof_descriptor_count: usize,
        proof_topology_width: usize,
        delivery_field_width: usize,
        live_relevance_field_width: usize,
        digest_part_count: usize,
    ) -> Self {
        Self {
            authorized_field_width,
            proof_descriptor_count,
            proof_topology_width,
            tenant_schema_basis_count: 2,
            delivery_field_width,
            live_relevance_field_width,
            allocation_scope_width: 1,
            digest_part_count,
        }
    }

    pub(crate) fn from_narrowed(
        artifact: &NarrowedPolicyQueryArtifact,
        delivery_field_width: usize,
        live_relevance_field_width: usize,
        digest_part_count: usize,
    ) -> Self {
        Self::new(
            artifact.authorized_projection().visible_field_paths().len(),
            artifact.relationship_proof().descriptor_count(),
            artifact
                .relationship_proof()
                .topology_classes()
                .iter()
                .map(|topology| topology.as_str().len().min(1))
                .sum(),
            delivery_field_width,
            live_relevance_field_width,
            digest_part_count,
        )
    }

    pub fn authorized_field_width(&self) -> usize {
        self.authorized_field_width
    }

    pub fn proof_descriptor_count(&self) -> usize {
        self.proof_descriptor_count
    }

    pub fn proof_topology_width(&self) -> usize {
        self.proof_topology_width
    }

    pub fn tenant_schema_basis_count(&self) -> usize {
        self.tenant_schema_basis_count
    }

    pub fn delivery_field_width(&self) -> usize {
        self.delivery_field_width
    }

    pub fn live_relevance_field_width(&self) -> usize {
        self.live_relevance_field_width
    }

    pub fn allocation_scope_width(&self) -> usize {
        self.allocation_scope_width
    }

    pub fn digest_part_count(&self) -> usize {
        self.digest_part_count
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "plan_budget:{}:{}:{}:{}:{}:{}:{}:{}",
            self.authorized_field_width,
            self.proof_descriptor_count,
            self.proof_topology_width,
            self.tenant_schema_basis_count,
            self.delivery_field_width,
            self.live_relevance_field_width,
            self.allocation_scope_width,
            self.digest_part_count
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwarePlanLoweringReport {
    digest: String,
    posture: PolicyAwarePlanCostPosture,
    mode: PolicyAwareExecutionMode,
    executor_semantic_rediscovery_count: usize,
}

impl PolicyAwarePlanLoweringReport {
    pub(crate) fn new(
        core_digest: &PolicyAwarePlanDigest,
        posture: PolicyAwarePlanCostPosture,
        mode: PolicyAwareExecutionMode,
    ) -> Self {
        Self {
            digest: hash_parts(&[
                format!("core:{}", core_digest.as_str()),
                format!("posture:{}", posture.as_str()),
                format!("mode:{}", mode.as_str()),
                "executor_rediscovery:0".to_string(),
            ]),
            posture,
            mode,
            executor_semantic_rediscovery_count: 0,
        }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn posture(&self) -> PolicyAwarePlanCostPosture {
        self.posture
    }

    pub fn mode(&self) -> PolicyAwareExecutionMode {
        self.mode
    }

    pub fn executor_semantic_rediscovery_count(&self) -> usize {
        self.executor_semantic_rediscovery_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwarePlanCore {
    digest: PolicyAwarePlanDigest,
    seam: PolicyAwareExecutionSeam,
    work_budget: PolicyAwarePlanWorkBudget,
    report: PolicyAwarePlanLoweringReport,
}

impl PolicyAwarePlanCore {
    pub(crate) fn from_narrowed(
        artifact: &NarrowedPolicyQueryArtifact,
        mode: PolicyAwareExecutionMode,
        posture: PolicyAwarePlanCostPosture,
        delivery_field_width: usize,
        live_relevance_field_width: usize,
    ) -> Self {
        Self::from_narrowed_with_counter_adjustment(
            artifact,
            mode,
            posture,
            delivery_field_width,
            live_relevance_field_width,
            |counters| counters,
        )
    }

    pub(crate) fn from_narrowed_with_counter_adjustment(
        artifact: &NarrowedPolicyQueryArtifact,
        mode: PolicyAwareExecutionMode,
        posture: PolicyAwarePlanCostPosture,
        delivery_field_width: usize,
        live_relevance_field_width: usize,
        adjust_counters: impl FnOnce(PolicyAwareSeamCounters) -> PolicyAwareSeamCounters,
    ) -> Self {
        let digest_part_count = 12;
        let work_budget = PolicyAwarePlanWorkBudget::from_narrowed(
            artifact,
            delivery_field_width,
            live_relevance_field_width,
            digest_part_count,
        );
        let counters = PolicyAwareSeamCounters::admitted(
            work_budget.authorized_field_width(),
            work_budget.proof_topology_width(),
            work_budget.delivery_field_width(),
            work_budget.live_relevance_field_width(),
            work_budget.digest_part_count(),
        );
        let counters = adjust_counters(counters);
        let seam = PolicyAwareExecutionSeam::from_narrowed(artifact, mode, counters);
        let digest = PolicyAwarePlanDigest::new(hash_parts(&[
            format!("seam:{}", seam.identity().as_str()),
            format!("narrowed:{}", artifact.digest()),
            format!("posture:{}", posture.as_str()),
            work_budget.digest_part(),
        ]));
        let report = PolicyAwarePlanLoweringReport::new(&digest, posture, mode);
        Self {
            digest,
            seam,
            work_budget,
            report,
        }
    }

    pub fn digest(&self) -> &PolicyAwarePlanDigest {
        &self.digest
    }

    pub fn seam(&self) -> &PolicyAwareExecutionSeam {
        &self.seam
    }

    pub fn work_budget(&self) -> PolicyAwarePlanWorkBudget {
        self.work_budget
    }

    pub fn report(&self) -> &PolicyAwarePlanLoweringReport {
        &self.report
    }
}
