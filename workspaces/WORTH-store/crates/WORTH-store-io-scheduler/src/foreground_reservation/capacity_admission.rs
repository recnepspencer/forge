use worth_foundational::performance_api::lower_lane::policy::FoundationalPolicyAdmissionReceipt;
use worth_foundational::FoundationalPerformanceBudgetKind;
use worth_proof::prelude::{AuthorityMarker, AuthorityWitness};
use worth_store_physical_backend::{BackendTargetProfile, CapabilityEvidenceClass};
use worth_store_security::StoreSecurityScopeIdentity;

use crate::{
    IoSchedulerBackendCapabilityAdmission, IoSchedulerBackendCapabilityRequirement,
    IoSchedulerS6ReadinessAdmission, IoSchedulerS6SecurityScopeAdmission,
};

use super::capacity::require_capacity;
use super::{
    ForegroundArbitrationDeclaration, ForegroundIoLaneKind, ForegroundLaneDeclaration,
    ForegroundLatencyEnvelope, ForegroundReservationAdmissionDenial,
    ForegroundReservationResourceShortfall, ForegroundResourceBudget,
};

#[derive(Debug, Eq, PartialEq)]
pub struct ForegroundReservationCapacityAuthority {
    _private: (),
}

impl AuthorityMarker for ForegroundReservationCapacityAuthority {}

impl ForegroundReservationCapacityAuthority {
    #[allow(dead_code)]
    pub(crate) fn store_owned() -> AuthorityWitness<Self> {
        AuthorityWitness::from_authority_marker(Self { _private: () })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ForegroundReservationCapacityAdmission {
    authority_witness: AuthorityWitness<ForegroundReservationCapacityAuthority>,
    lane: ForegroundIoLaneKind,
    backend_requirement: IoSchedulerBackendCapabilityRequirement,
    backend_profile: BackendTargetProfile,
    backend_evidence_class: CapabilityEvidenceClass,
    envelope: ForegroundLatencyEnvelope,
    arbitration: ForegroundArbitrationDeclaration,
    security_scope_identity: StoreSecurityScopeIdentity,
    stable_read_wait_count: u64,
    stable_read_retry_count: u64,
    requested: ForegroundResourceBudget,
    admitted: ForegroundResourceBudget,
    assumed_backend_limits: ForegroundResourceBudget,
    policy_receipt: FoundationalPolicyAdmissionReceipt,
    freshness: ForegroundReservationCapacityFreshness,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ForegroundReservationCapacityAdmissionRequest {
    authority_witness: AuthorityWitness<ForegroundReservationCapacityAuthority>,
    lane: ForegroundLaneDeclaration,
    backend_requirement: IoSchedulerBackendCapabilityRequirement,
    backend_profile: BackendTargetProfile,
    backend_evidence_class: CapabilityEvidenceClass,
    arbitration: ForegroundArbitrationDeclaration,
    security_scope_identity: StoreSecurityScopeIdentity,
    stable_read_wait_count: u64,
    stable_read_retry_count: u64,
    requested: ForegroundResourceBudget,
    admitted: ForegroundResourceBudget,
    assumed_backend_limits: ForegroundResourceBudget,
    policy_receipt: FoundationalPolicyAdmissionReceipt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForegroundReservationCapacityAdmissionDenial {
    InsufficientCapacity(ForegroundReservationResourceShortfall),
    PolicyReceiptHasNoBudgetDecision,
    PolicyReceiptRejectedOrWidenedWork,
    PolicyReceiptMissingBudgetKind(FoundationalPerformanceBudgetKind),
    PolicyReceiptDuplicateBudgetKind(FoundationalPerformanceBudgetKind),
    PolicyReceiptBudgetMismatch {
        kind: FoundationalPerformanceBudgetKind,
        requested_units: u32,
        admitted_units: u32,
        expected_requested_units: u32,
        expected_admitted_units: u32,
    },
    PolicyReceiptBudgetOverflow(FoundationalPerformanceBudgetKind),
    MissingLaneEnvelope,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ForegroundReservationCapacityFreshness {
    Current,
    #[allow(dead_code)]
    RebindRequired,
}

impl ForegroundReservationCapacityAdmission {
    pub const fn lane(&self) -> ForegroundIoLaneKind {
        self.lane
    }

    pub const fn backend_requirement(&self) -> IoSchedulerBackendCapabilityRequirement {
        self.backend_requirement
    }

    pub const fn backend_profile(&self) -> BackendTargetProfile {
        self.backend_profile
    }

    pub const fn backend_evidence_class(&self) -> CapabilityEvidenceClass {
        self.backend_evidence_class
    }

    pub const fn envelope(&self) -> ForegroundLatencyEnvelope {
        self.envelope
    }

    pub const fn arbitration(&self) -> ForegroundArbitrationDeclaration {
        self.arbitration
    }

    pub const fn security_scope_identity(&self) -> StoreSecurityScopeIdentity {
        self.security_scope_identity
    }

    pub const fn stable_read_wait_count(&self) -> u64 {
        self.stable_read_wait_count
    }

    pub const fn stable_read_retry_count(&self) -> u64 {
        self.stable_read_retry_count
    }

    pub const fn requested_budget(&self) -> ForegroundResourceBudget {
        self.requested
    }

    pub const fn admitted_budget(&self) -> ForegroundResourceBudget {
        self.admitted
    }

    pub const fn assumed_backend_limits(&self) -> ForegroundResourceBudget {
        self.assumed_backend_limits
    }

    pub const fn policy_receipt(&self) -> &FoundationalPolicyAdmissionReceipt {
        &self.policy_receipt
    }

    pub(super) const fn freshness(&self) -> ForegroundReservationCapacityFreshness {
        self.freshness
    }

    pub(super) const fn authority_witness(
        &self,
    ) -> &AuthorityWitness<ForegroundReservationCapacityAuthority> {
        &self.authority_witness
    }
}

impl ForegroundReservationCapacityAdmissionRequest {
    pub fn new(
        authority_witness: AuthorityWitness<ForegroundReservationCapacityAuthority>,
        lane: ForegroundLaneDeclaration,
        backend: &IoSchedulerBackendCapabilityAdmission,
        stable_readiness: &IoSchedulerS6ReadinessAdmission,
        security_scope: &IoSchedulerS6SecurityScopeAdmission,
        arbitration: ForegroundArbitrationDeclaration,
        admitted: ForegroundResourceBudget,
        assumed_backend_limits: ForegroundResourceBudget,
        policy_receipt: FoundationalPolicyAdmissionReceipt,
    ) -> Self {
        let readiness_counters = stable_readiness.counters();
        Self {
            authority_witness,
            lane,
            backend_requirement: backend.requirement(),
            backend_profile: backend.profile(),
            backend_evidence_class: backend.evidence_class(),
            arbitration,
            security_scope_identity: security_scope.permission().identity(),
            stable_read_wait_count: readiness_counters.wait_count(),
            stable_read_retry_count: readiness_counters.retry_count(),
            requested: lane.requested_budget(),
            admitted,
            assumed_backend_limits,
            policy_receipt,
        }
    }
}

pub fn admit_foreground_reservation_capacity(
    request: ForegroundReservationCapacityAdmissionRequest,
) -> Result<ForegroundReservationCapacityAdmission, ForegroundReservationCapacityAdmissionDenial> {
    let envelope = request
        .lane
        .envelope()
        .ok_or(ForegroundReservationCapacityAdmissionDenial::MissingLaneEnvelope)?;
    require_policy_receipt(&request.policy_receipt, request.requested, request.admitted)?;
    require_capacity(request.requested, request.admitted).map_err(|(denial, _)| match denial {
        ForegroundReservationAdmissionDenial::InsufficientCapacity(shortfall) => {
            ForegroundReservationCapacityAdmissionDenial::InsufficientCapacity(shortfall)
        }
        _ => ForegroundReservationCapacityAdmissionDenial::PolicyReceiptRejectedOrWidenedWork,
    })?;
    require_capacity(request.admitted, request.assumed_backend_limits).map_err(|(denial, _)| {
        match denial {
            ForegroundReservationAdmissionDenial::InsufficientCapacity(shortfall) => {
                ForegroundReservationCapacityAdmissionDenial::InsufficientCapacity(shortfall)
            }
            _ => ForegroundReservationCapacityAdmissionDenial::PolicyReceiptRejectedOrWidenedWork,
        }
    })?;
    Ok(ForegroundReservationCapacityAdmission {
        authority_witness: request.authority_witness,
        lane: request.lane.lane(),
        backend_requirement: request.backend_requirement,
        backend_profile: request.backend_profile,
        backend_evidence_class: request.backend_evidence_class,
        envelope,
        arbitration: request.arbitration,
        security_scope_identity: request.security_scope_identity,
        stable_read_wait_count: request.stable_read_wait_count,
        stable_read_retry_count: request.stable_read_retry_count,
        requested: request.requested,
        admitted: request.admitted,
        assumed_backend_limits: request.assumed_backend_limits,
        policy_receipt: request.policy_receipt,
        freshness: ForegroundReservationCapacityFreshness::Current,
    })
}

fn require_policy_receipt(
    receipt: &FoundationalPolicyAdmissionReceipt,
    requested: ForegroundResourceBudget,
    admitted: ForegroundResourceBudget,
) -> Result<(), ForegroundReservationCapacityAdmissionDenial> {
    if receipt.budget_decisions().is_empty() {
        return Err(ForegroundReservationCapacityAdmissionDenial::PolicyReceiptHasNoBudgetDecision);
    }
    if !receipt.denied_work().is_empty() || !receipt.widened_work().is_empty() {
        return Err(
            ForegroundReservationCapacityAdmissionDenial::PolicyReceiptRejectedOrWidenedWork,
        );
    }
    require_policy_budget_kind(
        receipt,
        FoundationalPerformanceBudgetKind::Breadth,
        breadth_units(requested)?,
        breadth_units(admitted)?,
    )?;
    require_policy_budget_kind(
        receipt,
        FoundationalPerformanceBudgetKind::Density,
        density_units(requested)?,
        density_units(admitted)?,
    )?;
    require_policy_budget_kind(
        receipt,
        FoundationalPerformanceBudgetKind::Locality,
        locality_units(requested)?,
        locality_units(admitted)?,
    )?;
    require_policy_budget_kind(
        receipt,
        FoundationalPerformanceBudgetKind::FreshnessSensitive,
        freshness_units(requested)?,
        freshness_units(admitted)?,
    )?;
    Ok(())
}

fn require_policy_budget_kind(
    receipt: &FoundationalPolicyAdmissionReceipt,
    kind: FoundationalPerformanceBudgetKind,
    expected_requested_units: u32,
    expected_admitted_units: u32,
) -> Result<(), ForegroundReservationCapacityAdmissionDenial> {
    let mut matched = None;
    for decision in receipt.budget_decisions() {
        if decision.kind() != kind {
            continue;
        }
        if matched.is_some() {
            return Err(
                ForegroundReservationCapacityAdmissionDenial::PolicyReceiptDuplicateBudgetKind(
                    kind,
                ),
            );
        }
        matched = Some(decision);
    }
    let Some(decision) = matched else {
        if expected_requested_units == 0 && expected_admitted_units == 0 {
            return Ok(());
        }
        return Err(
            ForegroundReservationCapacityAdmissionDenial::PolicyReceiptMissingBudgetKind(kind),
        );
    };
    if decision.requested_units() != expected_requested_units
        || decision.admitted_units() != expected_admitted_units
    {
        return Err(
            ForegroundReservationCapacityAdmissionDenial::PolicyReceiptBudgetMismatch {
                kind,
                requested_units: decision.requested_units(),
                admitted_units: decision.admitted_units(),
                expected_requested_units,
                expected_admitted_units,
            },
        );
    }
    Ok(())
}

fn breadth_units(
    budget: ForegroundResourceBudget,
) -> Result<u32, ForegroundReservationCapacityAdmissionDenial> {
    u32_budget_sum(
        FoundationalPerformanceBudgetKind::Breadth,
        &[budget.queue_slots(), budget.worker_permits()],
    )
}

fn density_units(
    budget: ForegroundResourceBudget,
) -> Result<u32, ForegroundReservationCapacityAdmissionDenial> {
    u32_budget_sum(
        FoundationalPerformanceBudgetKind::Density,
        &[
            budget.bandwidth_tokens(),
            budget.dirty_page_budget(),
            budget.cache_residency_hints(),
        ],
    )
}

fn locality_units(
    budget: ForegroundResourceBudget,
) -> Result<u32, ForegroundReservationCapacityAdmissionDenial> {
    u32_budget_sum(
        FoundationalPerformanceBudgetKind::Locality,
        &[
            budget.read_ahead_window(),
            budget.write_back_window(),
            budget.reclaim_permits(),
        ],
    )
}

fn freshness_units(
    budget: ForegroundResourceBudget,
) -> Result<u32, ForegroundReservationCapacityAdmissionDenial> {
    u32_budget_sum(
        FoundationalPerformanceBudgetKind::FreshnessSensitive,
        &[budget.flush_permits(), budget.sync_debt()],
    )
}

fn u32_budget_sum(
    kind: FoundationalPerformanceBudgetKind,
    units: &[u64],
) -> Result<u32, ForegroundReservationCapacityAdmissionDenial> {
    let mut total = 0_u64;
    for unit in units {
        total = total.checked_add(*unit).ok_or(
            ForegroundReservationCapacityAdmissionDenial::PolicyReceiptBudgetOverflow(kind),
        )?;
    }
    u32::try_from(total).map_err(|_| {
        ForegroundReservationCapacityAdmissionDenial::PolicyReceiptBudgetOverflow(kind)
    })
}

#[cfg(test)]
pub(crate) fn rebind_required_capacity_admission_for_test(
    mut admission: ForegroundReservationCapacityAdmission,
) -> ForegroundReservationCapacityAdmission {
    admission.freshness = ForegroundReservationCapacityFreshness::RebindRequired;
    admission
}
