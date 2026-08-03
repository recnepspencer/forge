use worth_store::physical_runtime::{
    lower_physical_durability_performance_receipt, ClosedRuntime, ServingShutdownOutcome,
};

pub(super) fn emit(close: &ServingShutdownOutcome<ClosedRuntime>) -> Result<(), String> {
    let profile = close
        .durability_closeout()
        .recovery_handoff()
        .ok_or_else(|| "C.7 performance evidence requires a recovery handoff".to_owned())?
        .backend_evidence()
        .profile()
        .label();
    let summary = close.performance();
    for contract in summary.contracts() {
        let claim = contract.claim();
        let evidence = lower_physical_durability_performance_receipt(contract, summary)
            .map_err(|denial| format!("C.7 performance receipt denied: {denial:?}"))?;
        let rows = evidence.receipt().counter_rows();
        let encoded = rows
            .iter()
            .map(|row| format!("{}={}", row.name().as_str(), row.observed_count()))
            .collect::<Vec<_>>()
            .join(",");
        println!(
            "BOUNDED_RESIDENCY_PERFORMANCE {} {} {} {}",
            claim.label(),
            profile,
            rows.len(),
            encoded,
        );
    }
    Ok(())
}
