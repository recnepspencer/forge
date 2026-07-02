use crate::checkpoint_interlock::ReadDuringCheckpointVerdict;
use crate::compaction_interlock::ReadDuringCompactionVerdict;
use crate::epoch::{EpochRetryDecision, PhysicalEpochComparisonEvidence};
use crate::latch::LatchOrderProof;
use crate::publication::PhysicalPublicationReceipt;
use crate::reclaim_reachability::ReclaimEligibilityProof;
use crate::stable_read_execution::StablePhysicalReadReceipt;
use forge_foundational::performance_api::lower_lane::basis::{
    FoundationalPerformanceAttachmentConstructionDenial, FoundationalPerformanceBundle,
    FoundationalPerformanceBundleConstructionDenial, FoundationalPerformanceContractName,
    FoundationalPerformanceCounterName, FoundationalPerformanceCounterSpec,
};
use forge_foundational::performance_api::lower_lane::receipts::{
    counter_backed_performance_receipt,
    FoundationalCounterBackedPerformanceReceiptConstructionDenial,
    FoundationalPerformanceCounterRow,
};
use forge_foundational::{
    performance, performance_bundle, FoundationalAuthoritativePerformanceClaim,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
};

use super::basis::S6FoundationalCounterReceipt;
use super::{
    PhysicalIsolationCounterSnapshot, S5PhysicalIsolationCloseoutBasis,
    S6IoQosIsolationReadinessDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutedS5IsolationCloseout {
    basis: S5PhysicalIsolationCloseoutBasis,
    counters: PhysicalIsolationCounterSnapshot,
    foundational_counter_receipt: S6FoundationalCounterReceipt,
    proof_progression_identity: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct ExecutedS5IsolationCloseoutReceipts<'a> {
    pub stable_read: StablePhysicalReadReceipt,
    pub latch_order_proof: &'a LatchOrderProof,
    pub epoch_freshness: &'a PhysicalEpochComparisonEvidence,
    pub publication: &'a PhysicalPublicationReceipt,
    pub reclaim: &'a ReclaimEligibilityProof,
    pub compaction: &'a ReadDuringCompactionVerdict,
    pub checkpoint: &'a ReadDuringCheckpointVerdict,
}

impl ExecutedS5IsolationCloseout {
    pub fn from_physical_isolation_receipts(
        receipts: ExecutedS5IsolationCloseoutReceipts<'_>,
    ) -> Result<Self, S6IoQosIsolationReadinessDenial> {
        let _latch_order_proof = receipts.latch_order_proof;
        let counters = closeout_counters(receipts)?;
        let foundational_counter_receipt = s6_store_executed_counter_receipt(counters)?;
        let basis = S5PhysicalIsolationCloseoutBasis::from_executed_isolation(
            proof_progression_identity(receipts, counters),
            counters,
        );
        Ok(Self {
            basis,
            counters,
            foundational_counter_receipt,
            proof_progression_identity: proof_progression_identity(receipts, counters),
        })
    }

    pub const fn basis(&self) -> S5PhysicalIsolationCloseoutBasis {
        self.basis
    }

    pub const fn counters(&self) -> PhysicalIsolationCounterSnapshot {
        self.counters
    }

    pub(crate) const fn foundational_counter_receipt(&self) -> &S6FoundationalCounterReceipt {
        &self.foundational_counter_receipt
    }

    pub(crate) const fn proof_progression_identity(&self) -> u64 {
        self.proof_progression_identity
    }
}

fn closeout_counters(
    receipts: ExecutedS5IsolationCloseoutReceipts<'_>,
) -> Result<PhysicalIsolationCounterSnapshot, S6IoQosIsolationReadinessDenial> {
    let stable = receipts.stable_read.counters();
    let compaction_pre = receipts.compaction.pre_cutover_read().counters();
    let compaction_post = receipts.compaction.post_cutover_read().counters();
    let checkpoint_pre = receipts.checkpoint.pre_publication_read().counters();
    let checkpoint_post = receipts.checkpoint.post_publication_read().counters();
    let reclaim = receipts.reclaim.counters();
    let publication = receipts.publication.counters();
    let epoch_retry = match receipts.epoch_freshness.freshness().decision() {
        EpochRetryDecision::Current => 0,
        EpochRetryDecision::Retry | EpochRetryDecision::RebindRequired => 1,
    };
    PhysicalIsolationCounterSnapshot::from_store_executed_counts(
        1,
        stable.blocking_io_events()
            + compaction_pre.blocking_io_events()
            + compaction_post.blocking_io_events()
            + checkpoint_pre.blocking_io_events()
            + checkpoint_post.blocking_io_events(),
        stable.retry_decisions()
            + compaction_pre.retry_decisions()
            + compaction_post.retry_decisions()
            + checkpoint_pre.retry_decisions()
            + checkpoint_post.retry_decisions()
            + epoch_retry,
        1,
        1,
        reclaim.executed_reachability_inputs(),
        publication.readiness_joins() + reclaim.blocked_reclaims(),
        reclaim.blocked_reclaims(),
        receipts
            .stable_read
            .read_plan_release()
            .protected_references_released(),
    )
}

fn s6_store_executed_counter_receipt(
    counters: PhysicalIsolationCounterSnapshot,
) -> Result<S6FoundationalCounterReceipt, S6IoQosIsolationReadinessDenial> {
    let bundle = s6_store_executed_performance_bundle(counters)?;
    counter_backed_performance_receipt(bundle)
        .attach_counter_row(counter_row(
            "s5.closeout.outcomes",
            counters.outcome_count(),
        )?)
        .attach_counter_row(counter_row("s5.closeout.waits", counters.wait_count())?)
        .attach_counter_row(counter_row("s5.closeout.retries", counters.retry_count())?)
        .attach_counter_row(counter_row(
            "s5.closeout.latch-counter-rows",
            counters.latch_counter_rows(),
        )?)
        .attach_counter_row(counter_row(
            "s5.closeout.latch-waits",
            counters.latch_wait_count(),
        )?)
        .attach_counter_row(counter_row(
            "s5.closeout.reclaim-counter-rows",
            counters.reclaim_counter_rows(),
        )?)
        .attach_counter_row(counter_row(
            "s5.closeout.blocked-maintenance",
            counters.blocked_maintenance_count(),
        )?)
        .attach_counter_row(counter_row(
            "s5.closeout.reclaim-blocks",
            counters.reclaim_block_count(),
        )?)
        .attach_counter_row(counter_row(
            "s5.closeout.protected-byte-footprint",
            counters.protected_byte_footprint(),
        )?)
        .finish()
        .map_err(map_receipt_denial)
}

fn s6_store_executed_performance_bundle(
    counters: PhysicalIsolationCounterSnapshot,
) -> Result<
    FoundationalPerformanceBundle<FoundationalAuthoritativePerformanceClaim>,
    S6IoQosIsolationReadinessDenial,
> {
    let claim = performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::PointLocal)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::TraversalLocal)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::ValidationPlanning)
        .include_work(FoundationalPerformanceWorkClass::AuthoritativeMutation)
        .exclude_work(FoundationalPerformanceWorkClass::ReplayReconstruction)
        .finish()
        .map_err(|_| S6IoQosIsolationReadinessDenial::MissingExecutedCounter)?;

    performance_bundle(claim)
        .attach_contract_name(contract_name()?)
        .attach_counter_spec(counter_spec(
            "s5.closeout.outcomes",
            counters.outcome_count(),
        )?)
        .attach_counter_spec(counter_spec("s5.closeout.waits", counters.wait_count())?)
        .attach_counter_spec(counter_spec("s5.closeout.retries", counters.retry_count())?)
        .attach_counter_spec(counter_spec(
            "s5.closeout.latch-counter-rows",
            counters.latch_counter_rows(),
        )?)
        .attach_counter_spec(counter_spec(
            "s5.closeout.latch-waits",
            counters.latch_wait_count(),
        )?)
        .attach_counter_spec(counter_spec(
            "s5.closeout.reclaim-counter-rows",
            counters.reclaim_counter_rows(),
        )?)
        .attach_counter_spec(counter_spec(
            "s5.closeout.blocked-maintenance",
            counters.blocked_maintenance_count(),
        )?)
        .attach_counter_spec(counter_spec(
            "s5.closeout.reclaim-blocks",
            counters.reclaim_block_count(),
        )?)
        .attach_counter_spec(counter_spec(
            "s5.closeout.protected-byte-footprint",
            counters.protected_byte_footprint(),
        )?)
        .finish()
        .map_err(map_bundle_denial)
}

fn contract_name() -> Result<FoundationalPerformanceContractName, S6IoQosIsolationReadinessDenial> {
    FoundationalPerformanceContractName::new("forge-store.s5.executed-isolation-closeout")
        .map_err(map_attachment_denial)
}

fn counter_spec(
    name: &'static str,
    expected_exact_count: u64,
) -> Result<FoundationalPerformanceCounterSpec, S6IoQosIsolationReadinessDenial> {
    Ok(FoundationalPerformanceCounterSpec::new(
        counter_name(name)?,
        FoundationalPerformanceWorkClass::ValidationPlanning,
        expected_exact_count,
    ))
}

fn counter_row(
    name: &'static str,
    observed_count: u64,
) -> Result<FoundationalPerformanceCounterRow, S6IoQosIsolationReadinessDenial> {
    Ok(FoundationalPerformanceCounterRow::new(
        counter_name(name)?,
        observed_count,
    ))
}

fn counter_name(
    name: &'static str,
) -> Result<FoundationalPerformanceCounterName, S6IoQosIsolationReadinessDenial> {
    FoundationalPerformanceCounterName::new(name).map_err(map_attachment_denial)
}

fn map_attachment_denial(
    _: FoundationalPerformanceAttachmentConstructionDenial,
) -> S6IoQosIsolationReadinessDenial {
    S6IoQosIsolationReadinessDenial::MissingExecutedCounter
}

fn map_bundle_denial(
    _: FoundationalPerformanceBundleConstructionDenial,
) -> S6IoQosIsolationReadinessDenial {
    S6IoQosIsolationReadinessDenial::MissingExecutedCounter
}

fn map_receipt_denial(
    _: FoundationalCounterBackedPerformanceReceiptConstructionDenial,
) -> S6IoQosIsolationReadinessDenial {
    S6IoQosIsolationReadinessDenial::MissingExecutedCounter
}

fn proof_progression_identity(
    receipts: ExecutedS5IsolationCloseoutReceipts<'_>,
    counters: PhysicalIsolationCounterSnapshot,
) -> u64 {
    let mut identity = 0x5355_0000_0000_0001_u64;
    identity = mix_u64(identity, counters.outcome_count());
    identity = mix_u64(identity, counters.wait_count());
    identity = mix_u64(identity, counters.retry_count());
    identity = mix_u64(identity, counters.latch_counter_rows());
    identity = mix_u64(identity, counters.reclaim_counter_rows());
    identity = mix_u64(identity, counters.protected_byte_footprint());
    identity = mix_u64(
        identity,
        receipts.stable_read.read_plan_release().root_epoch().get(),
    );
    identity = mix_u64(identity, receipts.publication.epochs().root().old().get());
    identity = mix_u64(identity, receipts.publication.epochs().root().new().get());
    identity = mix_u64(
        identity,
        u64::from(
            receipts
                .compaction
                .pre_cutover_reader_retained_old_structure(),
        ),
    );
    identity = mix_u64(
        identity,
        u64::from(
            receipts
                .checkpoint
                .post_publication_reader_observed_new_epoch(),
        ),
    );
    mix_u64(identity, receipts.reclaim.counters().candidate_ranges())
}

const fn mix_u64(mut digest: u64, value: u64) -> u64 {
    let bytes = value.to_le_bytes();
    let mut index = 0;
    while index < bytes.len() {
        digest ^= bytes[index] as u64;
        digest = digest.wrapping_mul(0x1000_0000_01b3);
        index += 1;
    }
    digest
}
