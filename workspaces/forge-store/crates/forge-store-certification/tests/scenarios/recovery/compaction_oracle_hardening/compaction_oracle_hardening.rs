#[path = "../../../support/recovery/counter_strength/support.rs"]
mod support;

use forge_store_physical_certification::{
    admit_physical_counter_evidence, BlockedReclaimUntilReleaseOracle, CounterMismatchEvidence,
    OldReaderSeesOldRootOracle, PhysicalOracleJudgment, PhysicalProofOracle,
    PhysicalProofOracleKind, PhysicalSimulationPlan, PostSwapReaderSeesNewRootOracle,
    ReusablePhysicalOracleFamily,
};

#[test]
fn publication_only_compaction_observation_cannot_satisfy_reader_oracles() {
    let plan = support::lower_physical_isolation_plan();
    let trace = support::publication_only_trace(&plan);
    let family = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape();

    assert_oracle_denied(
        family.oracle(OldReaderSeesOldRootOracle),
        &plan,
        &trace,
        PhysicalProofOracleKind::OldReaderSeesOldRoot,
    );
    assert_oracle_denied(
        family.oracle(PostSwapReaderSeesNewRootOracle),
        &plan,
        &trace,
        PhysicalProofOracleKind::PostSwapReaderSeesNewRoot,
    );
    assert_oracle_denied(
        family.oracle(BlockedReclaimUntilReleaseOracle),
        &plan,
        &trace,
        PhysicalProofOracleKind::BlockedReclaimUntilRelease,
    );
}

#[test]
fn publication_only_compaction_observation_cannot_satisfy_counter_contracts() {
    let plan = support::lower_physical_isolation_shortcut_plan();
    let evidence =
        support::executed_counter_evidence(&plan, support::publication_only_trace(&plan));
    let denial = admit_physical_counter_evidence(&plan, evidence)
        .expect_err("publication-only evidence must not satisfy blocked reclaim counters");

    assert_eq!(
        denial,
        CounterMismatchEvidence::PositiveCounterNotPositive {
            kind: forge_store_physical_certification::CounterContractKind::BlockedReclaimAttempts,
            actual: 0,
        }
    );
}

fn assert_oracle_denied<O: PhysicalProofOracle>(
    oracle: PhysicalOracleJudgment<O>,
    plan: &PhysicalSimulationPlan,
    trace: &forge_store_physical_certification::ObservedPhysicalTrace,
    expected: PhysicalProofOracleKind,
) {
    let denial = oracle
        .judge(plan, trace)
        .expect_err("publication-only evidence must not satisfy compaction oracles");
    assert!(matches!(
        denial,
        forge_store_physical_certification::OracleDenial::CompactionInterlockObservationDenied {
            oracle
        } if oracle == expected
    ));
}
